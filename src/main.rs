//! QWARTZ - R-Labs Next-Generation Cryptographic System
//!
//! Run with: cargo run -- <command>

use qwartz::{init, Command, execute};

fn main() {
    if let Err(e) = init() {
        eprintln!("❌ Initialization failed: {}", e);
        std::process::exit(1);
    }

    let cmd = Command::parse();
    if let Err(e) = execute(cmd) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}
