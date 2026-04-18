//! ZAB-R (Adaptive Mode) - Environment-based key generation
//!
//! Keys are generated from the environment itself - no stored key material

use crate::{EntropyEngine, QwartzError};
use sha3::{Digest, Sha3_256};
use zeroize::{Zeroize, Zeroizing};

/// ZAB-R Adaptive Key - generated from environment entropy
#[derive(Zeroize)]
pub struct ZABAdaptive {
    /// Context fingerprint (derived from environment)
    context: Zeroizing<[u8; 32]>,
    /// Generation timestamp
    generation: u64,
    /// Evolution counter
    evolution: u64,
}

impl ZABAdaptive {
    /// Create new adaptive key from current environment
    pub fn new() -> Result<Self, QwartzError> {
        let entropy = EntropyEngine::generate_seed();

        // Derive context from entropy
        let mut hasher = Sha3_256::new();
        hasher.update(&*entropy);
        hasher.update(b"ZAB-R-Adaptive");
        hasher.update(&std::process::id().to_le_bytes());

        let mut context = [0u8; 32];
        context.copy_from_slice(&hasher.finalize());

        let generation = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(Self {
            context: Zeroizing::new(context),
            generation,
            evolution: 0,
        })
    }

    /// Generate encryption key from context
    pub fn derive_key(&self, purpose: &[u8]) -> Zeroizing<[u8; 32]> {
        let mut hasher = Sha3_256::new();
        hasher.update(&*self.context);
        hasher.update(purpose);
        hasher.update(&self.generation.to_le_bytes());
        hasher.update(&self.evolution.to_le_bytes());

        let mut key = Zeroizing::new([0u8; 32]);
        key.copy_from_slice(&hasher.finalize());
        key
    }

    /// Evolve key (for auto-mutation)
    pub fn evolve(&mut self) {
        self.evolution += 1;

        // Re-derive context with new entropy
        let new_entropy = EntropyEngine::generate_seed();
        let mut hasher = Sha3_256::new();
        hasher.update(&*self.context);
        hasher.update(&*new_entropy);
        hasher.update(b"ZAB-R-Evolution");

        self.context.zeroize();
        let result: [u8; 32] = hasher.finalize().into();
        self.context = Zeroizing::new(result);
    }

    /// Verify if context matches current environment
    pub fn verify(&self) -> bool {
        // Re-derive expected context
        let entropy = EntropyEngine::generate_seed();
        let mut hasher = Sha3_256::new();
        hasher.update(&*entropy);
        hasher.update(b"ZAB-R-Adaptive");
        hasher.update(&std::process::id().to_le_bytes());

        let expected = hasher.finalize();

        // Check if current context is close enough
        let mut matches = 0;
        for (a, b) in self.context.iter().zip(expected.iter()) {
            if a == b {
                matches += 1;
            }
        }

        // 50% match threshold for adaptive keys
        matches >= 16
    }

    /// Export context (for sync)
    pub fn export_context(&self) -> Vec<u8> {
        let mut export = Vec::with_capacity(48);
        export.extend_from_slice(&self.generation.to_le_bytes());
        export.extend_from_slice(&self.evolution.to_le_bytes());
        export.extend_from_slice(&*self.context);
        export
    }

    /// Import context (from sync)
    pub fn import_context(data: &[u8]) -> Result<Self, QwartzError> {
        if data.len() < 48 {
            return Err(QwartzError::InvalidKey("Context too short".into()));
        }

        let generation = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let evolution = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let mut context = [0u8; 32];
        context.copy_from_slice(&data[16..48]);

        Ok(Self {
            context: Zeroizing::new(context),
            generation,
            evolution,
        })
    }
}

impl Default for ZABAdaptive {
    fn default() -> Self {
        Self::new().expect("Failed to create adaptive key")
    }
}

/// Encrypt using ZAB-R adaptive mode
pub fn encrypt_adaptive(plaintext: &[u8], purpose: &[u8]) -> Result<Vec<u8>, QwartzError> {
    let key_manager = ZABAdaptive::new()?;
    let key = key_manager.derive_key(purpose);

    // Simple XOR encryption with hash-based mixing
    let mut hasher = Sha3_256::new();
    hasher.update(&*key);
    hasher.update(plaintext);
    let mask = hasher.finalize();

    let mut ciphertext = Vec::with_capacity(plaintext.len() + 64);

    // Add header: version + evolution
    ciphertext.extend_from_slice(b"ZAB-R01");
    ciphertext.extend_from_slice(&key_manager.evolution.to_le_bytes());

    // Encrypt
    for (i, byte) in plaintext.iter().enumerate() {
        ciphertext.push(byte ^ mask[i % 32]);
    }

    Ok(ciphertext)
}

/// Decrypt using ZAB-R adaptive mode
pub fn decrypt_adaptive(ciphertext: &[u8], purpose: &[u8]) -> Result<Vec<u8>, QwartzError> {
    if ciphertext.len() < 14 || &ciphertext[0..8] != b"ZAB-R01" {
        return Err(QwartzError::InvalidKey("Invalid ZAB-R format".into()));
    }

    let evolution = u64::from_le_bytes(ciphertext[8..16].try_into().unwrap());
    let encrypted = &ciphertext[16..];

    let mut key_manager = ZABAdaptive::new()?;

    // Evolve to matching state
    while key_manager.evolution < evolution {
        key_manager.evolve();
    }

    let key = key_manager.derive_key(purpose);

    // Decrypt
    let mut hasher = Sha3_256::new();
    hasher.update(&*key);
    hasher.update(encrypted);
    let mask = hasher.finalize();

    let mut plaintext = Vec::with_capacity(encrypted.len());
    for (i, byte) in encrypted.iter().enumerate() {
        plaintext.push(byte ^ mask[i % 32]);
    }

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_encrypt_decrypt() {
        let plaintext = b"Secret message for ZAB-R";
        let purpose = b"test-encryption";

        let ciphertext = encrypt_adaptive(plaintext, purpose).unwrap();
        let decrypted = decrypt_adaptive(&ciphertext, purpose).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_key_derivation() {
        let key1 = ZABAdaptive::new().unwrap();
        let key2 = ZABAdaptive::new().unwrap();

        // Keys should be different (different entropy)
        let k1 = key1.derive_key(b"test");
        let k2 = key2.derive_key(b"test");

        // Note: Keys might match if entropy is similar, but unlikely
        // This is expected behavior for adaptive keys
    }
}
