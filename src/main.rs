use clap::Parser;
use std::{net::ToSocketAddrs, sync::Arc, sync::atomic::{AtomicBool, Ordering}, thread, time::Duration};
use ctrlc;

// Define a command-line argument parser using Clap
#[derive(Parser)]
#[command(name = "Ping Resolver")]
#[command(about = "Ping a domain or IP address to resolve its IP", long_about = None)]
struct Args {
    /// Domain or IP address to ping
    target: String,
}

fn main() {
    let args = Args::parse();
    let target = &args.target;

    // Flag to track when the program should exit
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up the Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, exiting...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    println!("Pinging to resolve IP address for: {}", target);

    while running.load(Ordering::SeqCst) {
        // Resolve the domain/IP to its socket address (including the IP)
        match resolve_ip(target) {
            Ok(ip) => println!("Resolved IP: {}", ip),
            Err(e) => eprintln!("Error resolving IP: {}", e),
        }

        // Pause for 2 seconds before resolving again
        thread::sleep(Duration::from_secs(2));
    }

    println!("Program terminated.");
}

// Function to resolve the IP of the target
fn resolve_ip(target: &str) -> Result<String, String> {
    let socket_addrs = (target, 0)
        .to_socket_addrs()
        .map_err(|e| format!("Failed to resolve {}: {}", target, e))?;

    if let Some(addr) = socket_addrs.into_iter().next() {
        Ok(addr.ip().to_string())
    } else {
        Err(format!("No IP address found for {}", target))
    }
}