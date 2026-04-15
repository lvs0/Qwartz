//! QWARTZ - R-Labs Next-Generation Cryptographic System
//! 
//! ZAB (Zero-Architecture Bypass) Generation Z Protocol

#![deny(unused_must_use)]
#![forbid(unsafe_code)]

pub mod entropy;
pub mod zab_r;
pub mod zab_k;
pub mod sync;
pub mod cli;

pub use entropy::EntropyEngine;
pub use zab_r::ZABAdaptive;
pub use zab_k::ZABKey;
pub use sync::ZABSync;

/// ZAB Mode selection
#[derive(Debug, Clone, Copy)]
pub enum ZABMode {
    /// Adaptive mode - keys generated from environment entropy
    Adaptive,
    /// Key-based mode - post-quantum hybrid keys
    KeyBased,
    /// Fusion mode - ZAB-GenZ combination
    Fusion,
}

/// ZAB Generation Z version
pub const ZAB_VERSION: &str = "ZAB-GenZ-0.1.0";

/// Initialize QWARTZ
pub fn init() -> Result<(), QwartzError> {
    EntropyEngine::init()?;
    Ok(())
}

/// QWARTZ Error types
#[derive(Debug, thiserror::Error)]
pub enum QwartzError {
    #[error("Entropy source unavailable: {0}")]
    EntropyUnavailable(String),
    
    #[error("Key generation failed: {0}")]
    KeyGenFailed(String),
    
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    
    #[error("Sync protocol error: {0}")]
    SyncError(String),
    
    #[error("Invalid key format: {0}")]
    InvalidKey(String),
    
    #[error("Verification failed")]
    VerificationFailed,
}
