# Demo Launcher

A cross-platform Rust application that serves demo web applications locally and opens them in your default browser. Perfect for marketing teams who need to launch multiple different demos from a Windows tablet or any other device.

## Features

- **Interactive Demo Selection** - Beautiful terminal UI for selecting from a list of demos
- **Local Web Server** - Runs on `http://localhost:3000` with zero external dependencies
- **Automatic Browser Launch** - Opens your default browser automatically to the selected demo
- **Flexible Directory Structure** - Each demo can have its own custom path to `index.html`
- **JSON Configuration** - Simple configuration file to define your demos

## Setup

### Prerequisites

- **Windows**: You need to have Rust installed. Download from [rustup.rs](https://rustup.rs/)
- **Mac/Linux**: Same as above

### Building from Source

1. Clone or extract this project
2. Open a terminal in the project directory
3. Run:
   ```bash
   cargo build --release
   ```
4. The executable will be created at `target/release/demo-launcher.exe` (Windows) or `target/release/demo-launcher` (Mac/Linux)

### Windows-Specific Build Instructions

To build a Windows executable (recommended for Surface tablets):

**On Windows:**
```bash
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

The executable will be at: `target/x86_64-pc-windows-msvc/release/demo-launcher.exe`

**Cross-compiling from Mac:**
Install `mingw-w64`:
```bash
brew install mingw-w64
```
Then build:
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Configuration

### config.json

Create a `config.json` file in the same directory as your executable. This file defines the list of available demos.

**Example:**
```json
{
  "demos": [
    {
      "name": "Demo 1 - Product Overview",
      "path": "demos/demo1"
    },
    {
      "name": "Demo 2 - Feature Deep Dive",
      "path": "demos/demo2"
    },
    {
      "name": "Demo 3 - ROI Calculator",
      "path": "demos/demo3/www"
    },
    {
      "name": "Demo 4 - Customer Success",
      "path": "demos/demo4"
    },
    {
      "name": "Demo 5 - Interactive Walkthrough",
      "path": "demos/demo5/build"
    },
    {
      "name": "Demo 6 - Pricing Comparison",
      "path": "demos/demo6"
    }
  ]
}
```

### Path Configuration

- **path**: Relative path from the executable to the directory containing your `index.html` file
- Each demo can have a different directory structure - just point to wherever the index.html lives
- Paths are case-sensitive on Mac/Linux, but case-insensitive on Windows

## Usage

### On Windows (Recommended for Surface Tablets)

1. Copy `demo-launcher.exe` to a folder on your Surface tablet
2. Copy `config.json` to the same folder as the executable
3. Create a `demos/` directory next to the executable and organize your demo folders inside it
4. Double-click `demo-launcher.exe`
5. Select which demo you want to launch from the interactive menu
6. Your default browser will open with the selected demo

### Directory Structure Example

```
C:\Users\User\Desktop\Demo Launcher\
├── demo-launcher.exe
├── config.json
└── demos/
    ├── demo1/
    │   ├── index.html
    │   ├── css/
    │   └── js/
    ├── demo2/
    │   ├── index.html
    │   ├── styles/
    │   └── scripts/
    └── demo3/
        └── www/
            ├── index.html
            └── assets/
```

## How It Works

1. The application reads `config.json` from the same directory as the executable
2. Shows an interactive menu with your configured demo names
3. When you select a demo, it:
   - Starts a web server on `http://localhost:3000`
   - Serves all files from the specified demo directory
   - Opens your browser to `http://localhost:3000`
   - Runs indefinitely until you close the terminal/command window

## Requirements for Your Demos

Each demo folder needs:
- An `index.html` file (at the path specified in config.json)
- All assets (CSS, JS, images, etc.) referenced with relative paths from the index.html file

### Example HTML Asset Paths

**If index.html is in `demos/demo1/`:**
```html
<link rel="stylesheet" href="css/style.css">  <!-- Loads demos/demo1/css/style.css -->
<script src="js/app.js"></script>              <!-- Loads demos/demo1/js/app.js -->
<img src="images/logo.png">                    <!-- Loads demos/demo1/images/logo.png -->
```

**If index.html is in `demos/demo3/www/`:**
```html
<link rel="stylesheet" href="css/style.css">  <!-- Loads demos/demo3/www/css/style.css -->
<script src="js/app.js"></script>              <!-- Loads demos/demo3/www/js/app.js -->
```

## Stopping the Server

Press `Ctrl+C` in the terminal/command window where the application is running to stop the server and close the application.

## Troubleshooting

### Port 3000 Already in Use
If port 3000 is already in use, close the other application using that port or wait for it to release the port, then restart the launcher.

### config.json Not Found
Make sure `config.json` is in the same directory as the executable and that it's valid JSON.

### Demo Path Not Found
Check that the paths in `config.json` are correct relative to where the executable is located. Use forward slashes `/` or backslashes `\` (both work on Windows).

### Browser Doesn't Open Automatically
The application will still start the server even if the browser can't open. You can manually open `http://localhost:3000` in your browser.

## Development

### Building for Development
```bash
cargo build
cargo run
```

### Testing
Run with your configured demos to ensure everything works as expected.

## License

This project is provided as-is for use by your marketing team.
