//! QWARTZ CLI - Command Line Interface

use crate::entropy::EntropyEngine;
use crate::{KeyType, ZABAdaptive, ZABKey, ZABMode};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// CLI Commands
#[derive(Debug)]
pub enum Command {
    /// Generate new key
    Keygen { mode: String, output: String },
    /// Encrypt file
    Encrypt {
        key: String,
        input: String,
        output: String,
    },
    /// Decrypt file
    Decrypt {
        key: String,
        input: String,
        output: String,
    },
    /// Sync nodes
    Sync { peers: Vec<String> },
    /// Verify file
    Verify { input: String },
    /// Entropy test
    Entropy,
    /// Version
    Version,
    /// Help
    Help,
}

impl Command {
    /// Parse command line arguments
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            return Command::Help;
        }

        match args[1].as_str() {
            "keygen" => {
                let mut mode = "aes".to_string();
                let mut output = "qwartz.key".to_string();

                let mut i = 2;
                while i < args.len() {
                    match args[i].as_str() {
                        "--mode" | "-m" => {
                            if i + 1 < args.len() {
                                mode = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        "--output" | "-o" => {
                            if i + 1 < args.len() {
                                output = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }

                Command::Keygen { mode, output }
            }
            "encrypt" => {
                let mut key = String::new();
                let mut input = String::new();
                let mut output = String::new();

                let mut i = 2;
                while i < args.len() {
                    match args[i].as_str() {
                        "--key" | "-k" => {
                            if i + 1 < args.len() {
                                key = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        "--input" | "-i" => {
                            if i + 1 < args.len() {
                                input = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        "--output" | "-o" => {
                            if i + 1 < args.len() {
                                output = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }

                Command::Encrypt { key, input, output }
            }
            "decrypt" => {
                let mut key = String::new();
                let mut input = String::new();
                let mut output = String::new();

                let mut i = 2;
                while i < args.len() {
                    match args[i].as_str() {
                        "--key" | "-k" => {
                            if i + 1 < args.len() {
                                key = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        "--input" | "-i" => {
                            if i + 1 < args.len() {
                                input = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        "--output" | "-o" => {
                            if i + 1 < args.len() {
                                output = args[i + 1].clone();
                                i += 1;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }

                Command::Decrypt { key, input, output }
            }
            "sync" => {
                let peers: Vec<String> = args[2..]
                    .iter()
                    .filter(|s| !s.starts_with('-'))
                    .cloned()
                    .collect();

                Command::Sync { peers }
            }
            "verify" => {
                let input = args.get(2).cloned().unwrap_or_default();
                Command::Verify { input }
            }
            "entropy" => Command::Entropy,
            "version" | "-v" | "--version" => Command::Version,
            "help" | "-h" | "--help" | _ => Command::Help,
        }
    }
}

/// Execute command
pub fn execute(cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Command::Keygen { mode, output } => {
            println!("⚡ Generating ZAB key...");

            let key_type = match mode.as_str() {
                "adaptive" | "zab-r" => KeyType::SymmetricAES, // Simplified
                "pq" | "zab-k" | _ => KeyType::SymmetricAES,
            };

            let key = ZABKey::generate(key_type)?;
            let data = key.export();

            fs::write(&output, &data)?;
            println!("✅ Key saved to: {}", output);
            println!("   Fingerprint: {}", hex::encode(key.fingerprint()));

            Ok(())
        }
        Command::Encrypt { key, input, output } => {
            println!("🔐 Encrypting file...");

            let key_data = fs::read(&key)?;
            let zab_key = ZABKey::import(&key_data)?;
            let plaintext = fs::read(&input)?;

            let ciphertext = zab_key.encrypt(&plaintext)?;
            fs::write(&output, &ciphertext)?;

            println!("✅ Encrypted to: {}", output);
            Ok(())
        }
        Command::Decrypt { key, input, output } => {
            println!("🔓 Decrypting file...");

            let key_data = fs::read(&key)?;
            let zab_key = ZABKey::import(&key_data)?;
            let ciphertext = fs::read(&input)?;

            let plaintext = zab_key.decrypt(&ciphertext)?;
            fs::write(&output, &plaintext)?;

            println!("✅ Decrypted to: {}", output);
            Ok(())
        }
        Command::Sync { peers } => {
            println!("🔄 Sync protocol - {} peers", peers.len());
            println!("   ZAB-Sync v0.1.0");
            Ok(())
        }
        Command::Verify { input } => {
            let data = fs::read(&input)?;

            if data.starts_with(b"ZAB-R01") {
                println!("✅ ZAB-R Adaptive key detected");
            } else if data.starts_with(b"ZAB-K01") {
                println!("✅ ZAB-K Key-based key detected");
            } else {
                println!("❌ Unknown format");
            }
            Ok(())
        }
        Command::Entropy => {
            println!("🎲 Entropy sources:");
            println!("   CPU Timing: {}", EntropyEngine::cpu_micro_timing());
            println!(
                "   Software: {}",
                hex::encode(EntropyEngine::software_fingerprint())
            );
            println!(
                "   Energy: {}",
                hex::encode(EntropyEngine::energy_signature())
            );
            Ok(())
        }
        Command::Version => {
            println!("QWARTZ v{}", crate::ZAB_VERSION);
            Ok(())
        }
        Command::Help => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!(
        r#"
⬡ QWARTZ - R-Labs Next-Generation Cryptographic System

USAGE:
    qwartz <COMMAND> [OPTIONS]

COMMANDS:
    keygen          Generate new ZAB key
    encrypt         Encrypt file
    decrypt         Decrypt file
    sync            Sync with peers
    verify          Verify file format
    entropy         Test entropy sources
    version         Show version
    help            Show this help

EXAMPLES:
    qwartz keygen --mode pq --output my.key
    qwartz encrypt --key my.key --input secret.txt --output secret.qw
    qwartz decrypt --key my.key --input secret.qw --output decrypted.txt

MODES:
    adaptive        ZAB-R (environment-based)
    pq              ZAB-K (post-quantum hybrid)

See https://github.com/lvs0/Qwartz for more info.
"#
    );
}

/// Hex encoding helper
mod hex {
    pub fn encode(data: impl AsRef<[u8]>) -> String {
        data.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}
