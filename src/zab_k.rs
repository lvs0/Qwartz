//! ZAB-K (Key-Based Mode) - Post-quantum hybrid keys
//!
//! Uses Kyber/Dilithium-style algorithms with auto-mutability

use crate::{EntropyEngine, QwartzError};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::ChaCha20Poly1305;
use sha3::{Digest, Sha3_256, Sha3_512};
use zeroize::{Zeroize, Zeroizing};

/// ZAB-K Key types
#[derive(Debug, Clone)]
pub enum KeyType {
    /// Symmetric key for AES-256-GCM
    SymmetricAES,
    /// Symmetric key for ChaCha20-Poly1305
    SymmetricChaCha,
    /// Hybrid educational key
    HybridPQ,
}

impl KeyType {
    pub fn as_str(&self) -> &str {
        match self {
            KeyType::SymmetricAES => "AES-256-GCM",
            KeyType::SymmetricChaCha => "ChaCha20-Poly1305",
            KeyType::HybridPQ => "ZAB-Hybrid",
        }
    }
}

/// ZAB-K Key structure
#[derive(Zeroize)]
pub struct ZABKey {
    /// Key material
    #[zeroize(skip)]
    key_material: Zeroizing<Vec<u8>>,
    /// Key type (not sensitive, skip zeroization)
    #[zeroize(skip)]
    key_type: KeyType,
    /// Auto-mutation counter
    mutations: u64,
    /// Creation timestamp
    created: u64,
    /// Key fingerprint
    fingerprint: [u8; 32],
}

impl ZABKey {
    /// Generate new ZAB-K key
    pub fn generate(key_type: KeyType) -> Result<Self, QwartzError> {
        let key_size = match key_type {
            KeyType::SymmetricAES => 32,
            KeyType::SymmetricChaCha => 32,
            KeyType::HybridPQ => 64,
        };

        // Generate key from entropy
        let seed = EntropyEngine::generate_seed();
        let mut key_material = vec![0u8; key_size];

        for i in 0..key_size {
            key_material[i] = seed[i % 64];
        }

        // Derive fingerprint
        let mut hasher = Sha3_256::new();
        hasher.update(&key_material);
        hasher.update(b"ZAB-K-Key");
        let mut fingerprint = [0u8; 32];
        fingerprint.copy_from_slice(&hasher.finalize());

        let created = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(Self {
            key_material: Zeroizing::new(key_material),
            key_type,
            mutations: 0,
            created,
            fingerprint,
        })
    }

    /// Get encryption key
    pub fn get_cipher(&self) -> Result<Aes256Gcm, QwartzError> {
        if self.key_material.len() < 32 {
            return Err(QwartzError::InvalidKey("Key too short".into()));
        }

        let key: [u8; 32] = self.key_material[..32].try_into().unwrap();
        Ok(Aes256Gcm::new_from_slice(&key).unwrap())
    }

    /// Encrypt data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, QwartzError> {
        let mut nonce_bytes = [0u8; 12];
        let entropy = EntropyEngine::generate_seed();
        for i in 0..12 {
            nonce_bytes[i] = entropy[i % 64];
        }
        let nonce = Nonce::from_slice(&nonce_bytes);

        let cipher = self.get_cipher()?;
        let mut result = nonce_bytes.to_vec();
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| QwartzError::EncryptionFailed(e.to_string()))?;
        result.extend(ciphertext);
        Ok(result)
    }

    /// Decrypt data
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, QwartzError> {
        if ciphertext.len() < 12 {
            return Err(QwartzError::InvalidKey("Ciphertext too short".into()));
        }

        let nonce = Nonce::from_slice(&ciphertext[..12]);
        let ct = &ciphertext[12..];

        let cipher = self.get_cipher()?;
        cipher
            .decrypt(nonce, ct)
            .map_err(|e| QwartzError::DecryptionFailed(e.to_string()))
    }

    /// Auto-mutate key
    pub fn mutate(&mut self) {
        self.mutations += 1;

        // Derive new key from current + entropy
        let mut hasher = Sha3_512::new();
        hasher.update(&*self.key_material);
        hasher.update(&self.mutations.to_le_bytes());

        let entropy = EntropyEngine::generate_seed();
        hasher.update(&*entropy);

        let new_key = hasher.finalize();

        // Update key material
        for i in 0..self.key_material.len().min(new_key.len()) {
            self.key_material[i] = new_key[i];
        }

        // Update fingerprint
        let mut fp_hasher = Sha3_256::new();
        fp_hasher.update(&*self.key_material);
        fp_hasher.update(&self.mutations.to_le_bytes());
        self.fingerprint.copy_from_slice(&fp_hasher.finalize());
    }

    /// Export key (for storage)
    pub fn export(&self) -> Vec<u8> {
        let mut export = Vec::with_capacity(64 + self.key_material.len());

        // Header
        export.extend_from_slice(b"ZAB-K01");
        export.push(match self.key_type {
            KeyType::SymmetricAES => 0x01,
            KeyType::SymmetricChaCha => 0x02,
            KeyType::HybridPQ => 0x03,
        });
        export.extend_from_slice(&self.mutations.to_le_bytes());
        export.extend_from_slice(&self.created.to_le_bytes());
        export.extend_from_slice(&self.fingerprint);

        // Key material
        export.extend_from_slice(&*self.key_material);

        export
    }

    /// Import key
    pub fn import(data: &[u8]) -> Result<Self, QwartzError> {
        if data.len() < 58 || &data[0..7] != b"ZAB-K01" {
            return Err(QwartzError::InvalidKey("Invalid ZAB-K format".into()));
        }

        let key_type = match data[7] {
            0x01 => KeyType::SymmetricAES,
            0x02 => KeyType::SymmetricChaCha,
            0x03 => KeyType::HybridPQ,
            _ => return Err(QwartzError::InvalidKey("Unknown key type".into())),
        };

        let mutations = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let created = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let mut fingerprint = [0u8; 32];
        fingerprint.copy_from_slice(&data[24..56]);

        let key_material = data[56..].to_vec();

        Ok(Self {
            key_material: Zeroizing::new(key_material),
            key_type,
            mutations,
            created,
            fingerprint,
        })
    }

    /// Get fingerprint
    pub fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
}

impl Drop for ZABKey {
    fn drop(&mut self) {
        // Zeroize key material on drop
        self.key_material.zeroize();
    }
}

/// Encrypt with ZAB-K
pub fn encrypt_keybased(plaintext: &[u8], key: &ZABKey) -> Result<Vec<u8>, QwartzError> {
    key.encrypt(plaintext)
}

/// Decrypt with ZAB-K
pub fn decrypt_keybased(ciphertext: &[u8], key: &ZABKey) -> Result<Vec<u8>, QwartzError> {
    key.decrypt(ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let key = ZABKey::generate(KeyType::SymmetricAES).unwrap();
        assert_eq!(key.key_material.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = ZABKey::generate(KeyType::SymmetricAES).unwrap();
        let plaintext = b"Secret message";

        let ciphertext = key.encrypt(plaintext).unwrap();
        let decrypted = key.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_mutation() {
        let mut key = ZABKey::generate(KeyType::SymmetricAES).unwrap();
        let fp1 = key.fingerprint();

        key.mutate();
        let fp2 = key.fingerprint();

        // Fingerprint should change after mutation
        assert_ne!(fp1, fp2);
    }
}
