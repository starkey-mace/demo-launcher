mod config;

use actix_files::Files;
use actix_web::{ App, HttpServer, middleware };
use config::Config;
use std::env;
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    let exe_dir = std::fs::canonicalize(env::current_exe()?)?.parent().unwrap().to_path_buf();
    let config_path = exe_dir.join("config.json");
    let current_dir = exe_dir.clone();

    let config = Config::load(&config_path)?;

    if config.demos.is_empty() {
        eprintln!("Error: No demos configured in config.json");
        std::process::exit(1);
    }

    let demo_names: Vec<String> = config.demos
        .iter()
        .map(|d| d.name.clone())
        .collect();

    clearscreen::clear().unwrap();

    println!("\n╔════════════════════════════════════╗");
    println!("║       Demo Launcher                ║");
    println!("╚════════════════════════════════════╝\n");

    let selected = inquire::Select::new("Select a demo to launch:", demo_names.clone()).prompt()?;

    let selected_demo = config.demos
        .iter()
        .find(|d| d.name == selected)
        .ok_or_else(|| anyhow::anyhow!("Demo not found"))?;

    let demo_path = current_dir.join(&selected_demo.path);

    if !demo_path.exists() {
        eprintln!("Error: Demo path does not exist: {}", demo_path.display());
        std::process::exit(1);
    }

    println!("\nStarting server for: {}\nPath: {}", selected_demo.name, demo_path.display());
    println!("Server running at http://localhost:3000");
    println!("Press Ctrl+C to exit\n");

    let demo_path = demo_path.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders
                    ::new()
                    .add(("Cache-Control", "no-cache, no-store, must-revalidate"))
            )
            .wrap(middleware::DefaultHeaders::new().add(("Pragma", "no-cache")))
            .wrap(middleware::DefaultHeaders::new().add(("Expires", "0")))
            .service(Files::new("/", demo_path.clone()).index_file("index.html"))
    })
        .bind("127.0.0.1:3000")?
        .run();

    let _ = open::that(&format!("http://localhost:3000/?v={}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()));

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        _ = server => {},
        _ = ctrl_c => {
            println!("\n✓ Shutting down server...");
            println!("✓ Closing application...\n");
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg("tell application \"Terminal\" to close (every window whose name contains \"Demo Launcher\")")
                    .output();
            }
        }
    }

    std::process::exit(0);
}
