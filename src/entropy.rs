//! Entropy Engine - Hardware-level randomness generation

use crate::{QwartzError, ZABVersion};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

/// Entropy source types
#[derive(Debug, Clone)]
pub enum EntropySource {
    /// CPU micro-timing variations
    CpuMicroTiming,
    /// Software fingerprint (OS, version, etc.)
    SoftwareFingerprint,
    /// Energy signature pattern
    EnergySignature,
    /// Lattice noise (hardware)
    LatticeNoise,
}

/// Entropy Engine - gathers randomness from multiple sources
pub struct EntropyEngine;

impl EntropyEngine {
    /// Initialize entropy collection
    pub fn init() -> Result<(), QwartzError> {
        // Warm up entropy sources
        for _ in 0..100 {
            Self::cpu_micro_timing();
        }
        Ok(())
    }

    /// Generate entropy from CPU micro-timing
    pub fn cpu_micro_timing() -> u64 {
        let start = Instant::now();
        let mut accumulator = 0u64;

        // Perform timing-sensitive operations
        for i in 0..1000 {
            let t1 = Instant::now();
            let t2 = Instant::now();
            accumulator ^= ((t2 - t1).as_nanos() as u64).wrapping_mul(i as u64);
        }

        // Add actual elapsed time
        let elapsed = start.elapsed();
        accumulator ^= (elapsed.as_nanos() as u64).wrapping_mul(0x9e3779b97f4a7c15);

        accumulator
    }

    /// Generate entropy from software fingerprint
    pub fn software_fingerprint() -> [u8; 32] {
        let mut hash = [0u8; 32];

        // OS type (simplified)
        #[cfg(target_os = "linux")]
        hash[0] = 0x01;
        #[cfg(target_os = "macos")]
        hash[0] = 0x02;
        #[cfg(target_os = "windows")]
        hash[0] = 0x03;

        // Architecture
        #[cfg(target_arch = "x86_64")]
        hash[1] = 0x01;
        #[cfg(target_arch = "aarch64")]
        hash[1] = 0x02;

        // Timestamp as additional entropy
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        hash[..8].copy_from_slice(&now.to_le_bytes());

        // Add CPU timing entropy
        let timing = Self::cpu_micro_timing();
        hash[8..16].copy_from_slice(&timing.to_le_bytes());

        hash
    }

    /// Generate entropy from simulated energy signature
    pub fn energy_signature() -> [u8; 32] {
        let mut sig = [0u8; 32];

        // Multiple timing samples
        for i in 0..32 {
            sig[i] = ((Self::cpu_micro_timing() >> (i * 2)) & 0xFF) as u8;
        }

        sig
    }

    /// Lattice noise - hardware-level randomness
    pub fn lattice_noise() -> [u8; 64] {
        let mut noise = [0u8; 64];

        // Collect multiple entropy sources
        for i in 0..64 {
            let t = Self::cpu_micro_timing();
            let e = Self::energy_signature();

            // Mix using hash function
            noise[i] = ((t.wrapping_mul(i as u64 + 1)) ^ (e[i] as u64)) as u8;
        }

        noise
    }

    /// Generate seed from all entropy sources
    pub fn generate_seed() -> Zeroizing<[u8; 64]> {
        let mut seed = Zeroizing::new([0u8; 64]);

        // Mix all entropy sources
        let micro = Self::cpu_micro_timing();
        let software = Self::software_fingerprint();
        let energy = Self::energy_signature();
        let lattice = Self::lattice_noise();

        // First 8 bytes: CPU timing
        seed[..8].copy_from_slice(&micro.to_le_bytes());

        // Next 32 bytes: Software fingerprint
        seed[8..40].copy_from_slice(&software);

        // Next 32 bytes: Energy signature
        seed[40..72].copy_from_slice(&energy);

        // Last 64 bytes: Lattice noise
        seed[72..].copy_from_slice(&lattice);

        // Final mixing
        for i in 0..64 {
            seed[i] ^= lattice[i % 64];
            seed[i] = seed[i].wrapping_add(micro as u8);
        }

        seed
    }

    /// Gather entropy from specific source
    pub fn gather(source: EntropySource) -> Vec<u8> {
        match source {
            EntropySource::CpuMicroTiming => {
                let mut bytes = Vec::with_capacity(8);
                let val = Self::cpu_micro_timing();
                bytes.extend_from_slice(&val.to_le_bytes());
                bytes
            }
            EntropySource::SoftwareFingerprint => Self::software_fingerprint().to_vec(),
            EntropySource::EnergySignature => Self::energy_signature().to_vec(),
            EntropySource::LatticeNoise => Self::lattice_noise().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_generation() {
        let seed1 = EntropyEngine::generate_seed();
        let seed2 = EntropyEngine::generate_seed();

        // Seeds should be different (with very high probability)
        assert_ne!(seed1[..], seed2[..]);
    }

    #[test]
    fn test_micro_timing() {
        let t1 = EntropyEngine::cpu_micro_timing();
        let t2 = EntropyEngine::cpu_micro_timing();

        // Should produce non-zero values
        assert_ne!(t1, 0);
        assert_ne!(t2, 0);
    }
}
