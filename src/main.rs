mod config;

use actix_files::Files;
use actix_web::{App, HttpServer};
use config::Config;
use crossterm::event::{self, Event, KeyCode};
use std::env;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

    let demo_names: Vec<String> = config.demos.iter().map(|d| d.name.clone()).collect();

    clearscreen::clear().unwrap();

    println!("\n╔════════════════════════════════════╗");
    println!("║       Demo Launcher                ║");
    println!("╚════════════════════════════════════╝\n");

    let selected = inquire::Select::new("Select a demo to launch:", demo_names.clone())
        .prompt()?;

    let selected_demo = config
        .demos
        .iter()
        .find(|d| d.name == selected)
        .ok_or_else(|| anyhow::anyhow!("Demo not found"))?;

    let demo_path = current_dir.join(&selected_demo.path);

    if !demo_path.exists() {
        eprintln!(
            "Error: Demo path does not exist: {}",
            demo_path.display()
        );
        std::process::exit(1);
    }

    println!(
        "\nStarting server for: {}\nPath: {}",
        selected_demo.name,
        demo_path.display()
    );
    println!("Server running at http://localhost:3000");
    println!("Press ESC to exit\n");

    let demo_path = demo_path.clone();
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    let server = HttpServer::new(move || {
        App::new().service(Files::new("/", demo_path.clone()).index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run();

    let _ = open::that("http://localhost:3000");

    let server_handle = tokio::spawn(async move {
        server.await
    });

    let keyboard_handle = tokio::spawn(async move {
        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
                if let Event::Key(key) = event::read().unwrap_or(Event::FocusLost) {
                    if key.code == KeyCode::Esc {
                        shutdown_clone.store(true, Ordering::SeqCst);
                        break;
                    }
                }
            }
        }
    });

    loop {
        if shutdown.load(Ordering::SeqCst) {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    server_handle.abort();
    keyboard_handle.abort();

    std::process::exit(0);
}
