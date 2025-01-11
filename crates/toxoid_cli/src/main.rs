use clap::{Parser, Subcommand};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs;
use std::thread;

// TODO: Make this configurable CLI argument
const HOST_ADDRESS: &str = "127.0.0.1:7878";

#[derive(Parser, Debug)]
#[command(name = "toxoid_cli")]
#[command(about = "A CLI tool for Toxoid Engine, watching file changes and building WASM scripts, reloading them from the host", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Watch for file changes and build WASM
    Watch {
        /// Path to watch for changes
        #[arg(short, long, default_value = "app/guest")]
        path: String,

        /// Out path for the guest WASM file build
        #[arg(short, long, default_value = "target/wasm32-wasip1/debug/guest.wasm")]
        out_path: PathBuf,

        // Host path for the WASM file
        #[arg(short = 'x', long, default_value = "app/host/guest.wasm")]
        host_path: PathBuf,

        // TODO: Add a static linking feature that doesn't use WASM hot reloading
        // cargo run --features static-linking --no-default-features
        // #[arg(long)]
        // static_linking: bool,
    },
}

fn build_guest(path: &str, out_path: &PathBuf, host_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Build the WASM file
    let build_status = Command::new("cargo")
        .args(&["component", "build"])
        .current_dir(path)
        .status()
        .expect("Failed to build WASM");

    // Ensure the build process is complete
    if build_status.success() {
        println!("Build completed successfully.");

        // Introduce a small delay to ensure file handles are released
        thread::sleep(Duration::from_millis(500));

        // Move the WASM file to the final output path
        println!("Moving WASM file from {} to {}...", out_path.display(), host_path.display());
        fs::rename(out_path, host_path)
            .expect("Failed to move WASM file");

        // Connect to the server using TcpStream
        let mut conn = std::net::TcpStream::connect(HOST_ADDRESS)?;
        conn.write_all(format!("reload {}", "guest.wasm").as_bytes())?;
        println!("Sent reload message to host...");
    } else {
        println!("Build failed, skipping file move.");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Watch { path, out_path, host_path } => {
            // Start the host application in a separate process
            // let mut host_process = Command::new("cargo")
            //     .args(&["run", "--package", "host"])
            //     .stdout(Stdio::inherit())
            //     .stderr(Stdio::inherit())
            //     .spawn()
            //     .expect("Failed to run host");
            // println!("Host running...");

            let mut host_process = Command::new("./target/debug/host")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to run host");
            println!("Host running...");
            
            // Build the guest WASM file
            println!("Building guest WASM...");
            build_guest(path, out_path, host_path)?;
            // Wait 5 seconds to watch for changes
            thread::sleep(Duration::from_secs(5));
            
            // Watch for file changes
            let (tx, rx) = channel();
            let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
            watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
            let debounce_duration = Duration::from_secs(2);
            let mut last_event_time = Instant::now();
            println!("Watching directory: {}", path);
            
            for res in rx {
                match res {
                    Ok(Event { paths, .. }) if !paths.is_empty() => {
                        let now = Instant::now();
                        if now.duration_since(last_event_time) >= debounce_duration {
                            last_event_time = now;
                            println!("File change detected, rebuilding guest WASM...");
                            build_guest(path, out_path, host_path)?;
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                    _ => (),
                }
            }

            // Ensure the host process is terminated when the CLI exits
            host_process.kill().expect("Failed to kill host process");
        }
    }

    Ok(())
}