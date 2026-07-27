// SPDX-License-Identifier: AGPL-3.0-or-later

//! Encoding, Shannon quality, file I/O hooks, and multi-source system entropy.

use beardog_errors::BearDogError;

/// Standard Base64-encode bytes (for seed payloads).
#[must_use]
pub fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    STANDARD.encode(data)
}

/// Decode standard Base64 to bytes.
///
/// # Errors
///
/// Returns an error if the input is not valid Base64.
pub fn base64_decode(data: &str) -> Result<Vec<u8>, BearDogError> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    STANDARD
        .decode(data)
        .map_err(|e| BearDogError::serialization(&e.to_string()))
}

/// Shannon entropy of `bytes`, normalized to approximately 0.0–1.0 (8 bits max).
#[expect(
    clippy::cast_precision_loss,
    reason = "Byte length as divisor; acceptable precision for normalized Shannon entropy"
)]
#[must_use]
pub fn calculate_entropy_quality(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }

    // Calculate Shannon entropy
    let mut counts = [0u32; 256];
    for &byte in bytes {
        counts[byte as usize] += 1;
    }

    let len = bytes.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let p = f64::from(count) / len;
            entropy -= p * p.log2();
        }
    }

    // Normalize to 0-1 range (max entropy for uniform distribution is 8 bits)
    entropy / 8.0
}

/// Save entropy data to file (test utility).
#[cfg(test)]
pub fn save_entropy_file(data: &[u8], path: &str) -> Result<(), BearDogError> {
    std::fs::write(path, data)
        .map_err(|e| BearDogError::io_error(&format!("Failed to save entropy file: {e}")))
}

/// Load entropy data from file (test utility).
#[cfg(test)]
pub fn load_entropy_file(path: &str) -> Result<Vec<u8>, BearDogError> {
    std::fs::read(path)
        .map_err(|e| BearDogError::io_error(&format!("Failed to load entropy file: {e}")))
}

pub(super) fn entropy_quality_assessment_label(quality_score: f64) -> &'static str {
    if quality_score > 0.95 {
        "Excellent"
    } else if quality_score > 0.85 {
        "Good"
    } else if quality_score > 0.70 {
        "Acceptable"
    } else {
        "Poor"
    }
}

pub(super) fn generate_system_entropy(size: usize) -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    use sha3::{Digest, Sha3_256};
    use std::time::{SystemTime, UNIX_EPOCH};

    // Collect entropy from multiple sources and mix them cryptographically
    // This provides defense-in-depth until HSM integration is complete

    let mut entropy_pool = Vec::new();

    // Source 1: OS-provided cryptographically secure randomness
    let mut rng = rand::rng();
    let mut os_bytes = vec![0u8; size];
    rng.fill_bytes(&mut os_bytes);
    entropy_pool.extend_from_slice(&os_bytes);

    // Source 2: High-resolution timestamp (nanosecond precision)
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| BearDogError::system(format!("System time error: {e}")))?
        .as_nanos();
    entropy_pool.extend_from_slice(&timestamp.to_le_bytes());

    // Source 3: Process context (PID, thread ID)
    let pid = std::process::id();
    entropy_pool.extend_from_slice(&pid.to_le_bytes());

    // Source 4: Thread-specific entropy
    let thread_id = format!("{:?}", std::thread::current().id());
    entropy_pool.extend_from_slice(thread_id.as_bytes());

    // Source 5: System-specific entropy (hostname, machine ID if available)
    if let Ok(hostname) = whoami::fallible::hostname() {
        entropy_pool.extend_from_slice(hostname.as_bytes());
    }

    // Source 6: Additional OS randomness to strengthen mix
    let mut additional_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut additional_bytes);
    entropy_pool.extend_from_slice(&additional_bytes);

    // Cryptographically mix all entropy sources using SHA3-256
    // This ensures that even if one source is weak, the output remains secure
    let mut hasher = Sha3_256::new();
    hasher.update(&entropy_pool);
    hasher.update(b"BearDog-MultiSource-Entropy-v1");

    // If we need more than 32 bytes, derive additional bytes using KDF pattern
    if size <= 32 {
        let hash = hasher.finalize();
        Ok(hash[..size].to_vec())
    } else {
        // For larger sizes, use iterative hashing (HKDF-like expansion)
        let mut result = Vec::new();
        let mut counter: u64 = 0;

        while result.len() < size {
            let mut round_hasher = Sha3_256::new();
            round_hasher.update(&entropy_pool);
            round_hasher.update(counter.to_le_bytes());
            round_hasher.update(b"BearDog-MultiSource-Entropy-v1");

            let round_hash = round_hasher.finalize();
            result.extend_from_slice(&round_hash);
            counter += 1;
        }

        Ok(result[..size].to_vec())
    }
}
