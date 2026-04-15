//! QWARTZ - R-Labs Next-Generation Cryptographic System
//!
//! Run with: cargo run -- <command>

use qwartz::{cli::Command, init};

fn main() {
    // Initialize QWARTZ
    if let Err(e) = init() {
        eprintln!("❌ Initialization failed: {}", e);
        std::process::exit(1);
    }

    // Parse and execute command
    let cmd = Command::parse();

    if let Err(e) = cli::execute(cmd) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}
