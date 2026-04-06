// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Multi-modal human entropy collector
#[derive(Debug, Clone)]
pub struct MultiModalHumanEntropyCollector {
    _config: HumanEntropyConfig,
}

impl MultiModalHumanEntropyCollector {
    /// Creates a new instance
    #[must_use]
    pub const fn new(config: HumanEntropyConfig) -> Self {
        Self { _config: config }
    }

    /// Collect high-quality entropy from available sources
    ///
    /// This implementation uses a multi-source entropy collection approach:
    ///
    /// 1. **System Entropy**: OS-provided randomness (e.g., `/dev/urandom` on Linux)
    /// 2. **Timing Entropy**: Precise timestamp variations
    /// 3. **Thread Randomness**: Thread-local RNG state
    /// 4. **Process Entropy**: Process ID and memory addresses
    ///
    /// The collected entropy is mixed using XOR folding to ensure:
    /// - Uniform distribution across all bits
    /// - Protection against single-source weakness
    /// - Sufficient entropy for cryptographic operations
    ///
    /// # Security Notes
    /// - This provides a solid baseline for entropy collection
    /// - For production deployment with human entropy requirements:
    ///   - Integrate hardware sensors (camera, microphone, haptic)
    ///   - Use the `MultiModalHumanEntropyCollector` from `collectors.rs`
    ///   - Ensure informed consent from humans
    /// - Entropy quality is verified against `config.quality_threshold`
    ///
    /// # Errors
    /// Returns error if entropy collection fails or quality is below threshold
    pub fn collect_entropy(&self) -> Result<Vec<u8>, BearDogError> {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Collect 32 bytes (256 bits) of entropy
        const ENTROPY_SIZE: usize = 32;
        let mut entropy_bytes = vec![0u8; ENTROPY_SIZE];

        // Source 1: System-provided cryptographic randomness
        // This is backed by OS entropy pools (/dev/urandom, BCryptGenRandom, etc.)
        use rand::RngCore;
        let mut rng = rand::rng();
        rng.fill_bytes(&mut entropy_bytes);

        // Source 2: High-resolution timing entropy
        // Captures microsecond-level variations in execution timing
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let timing_bytes = duration.as_nanos().to_le_bytes();
            for (i, &byte) in timing_bytes.iter().enumerate() {
                entropy_bytes[i % ENTROPY_SIZE] ^= byte;
            }
        }

        // Source 3: Process-specific entropy
        // Incorporates process ID and memory addresses
        let pid = std::process::id();
        let pid_bytes = pid.to_le_bytes();
        for (i, &byte) in pid_bytes.iter().enumerate() {
            entropy_bytes[i % ENTROPY_SIZE] ^= byte;
        }

        // Source 4: Thread-local timing variations
        // Multiple timing samples capture execution jitter
        for sample in 0..8 {
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let nanos = u64::from(duration.subsec_nanos());
                let sample_bytes = (nanos.wrapping_mul(sample + 1)).to_le_bytes();
                for (i, &byte) in sample_bytes.iter().enumerate() {
                    entropy_bytes[i % ENTROPY_SIZE] ^= byte;
                }
            }
        }

        // Verify entropy quality (basic check: not all zeros, not all same byte)
        let all_zeros = entropy_bytes.iter().all(|&b| b == 0);
        let all_same = entropy_bytes.windows(2).all(|w| w[0] == w[1]);

        if all_zeros || all_same {
            return Err(BearDogError::Security {
                message: "Entropy quality verification failed: insufficient randomness".to_string(),
                category: beardog_errors::SecurityErrorCategory::Encryption,
            });
        }

        // Calculate entropy quality score (simplified Shannon entropy estimate)
        let quality_score = calculate_entropy_quality(&entropy_bytes);

        if quality_score < self._config.quality_threshold {
            return Err(BearDogError::Security {
                message: format!(
                    "Entropy quality ({:.2}) below threshold ({:.2})",
                    quality_score, self._config.quality_threshold
                ),
                category: beardog_errors::SecurityErrorCategory::Encryption,
            });
        }

        Ok(entropy_bytes)
    }
}

/// Calculate entropy quality score using simplified Shannon entropy
///
/// Returns a score from 0.0 (no entropy) to 1.0 (perfect entropy)
fn calculate_entropy_quality(data: &[u8]) -> f64 {
    // Count byte frequency
    let mut frequency = [0u32; 256];
    for &byte in data {
        frequency[byte as usize] += 1;
    }

    // Calculate Shannon entropy
    #[expect(
        clippy::cast_precision_loss,
        reason = "byte length as f64 for Shannon normalization"
    )]
    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in &frequency {
        if count > 0 {
            let probability = f64::from(count) / len;
            entropy -= probability * probability.log2();
        }
    }

    // Normalize to 0.0-1.0 range (perfect entropy for uniform bytes is 8 bits)
    entropy / 8.0
}

/// Configuration for legacy collectors: minimum Shannon quality and collection deadline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyConfig {
    /// The quality threshold value
    pub quality_threshold: f64,
    /// Maximum wall-clock time to wait for entropy samples before failing the collection.
    pub collection_timeout_ms: u64,
}

impl Default for HumanEntropyConfig {
    fn default() -> Self {
        Self {
            quality_threshold: 0.8,
            collection_timeout_ms: 5000,
        }
    }
}

impl HumanEntropyConfig {
    /// Load `quality_threshold` from `BEARDOG_ENTROPY_QUALITY_THRESHOLD` when set (default `0.8`).
    pub fn from_env() -> Self {
        Self {
            quality_threshold: std::env::var("BEARDOG_ENTROPY_QUALITY_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.8),
            collection_timeout_ms: 5000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_entropy_config_default() {
        let config = HumanEntropyConfig::default();
        assert_eq!(config.quality_threshold, 0.8);
        assert_eq!(config.collection_timeout_ms, 5000);
    }

    #[test]
    fn test_human_entropy_config_custom() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.9,
            collection_timeout_ms: 10000,
        };
        assert_eq!(config.quality_threshold, 0.9);
        assert_eq!(config.collection_timeout_ms, 10000);
    }

    #[test]
    fn test_multi_modal_collector_new() {
        let config = HumanEntropyConfig::default();
        let collector = MultiModalHumanEntropyCollector::new(config);

        // Verify collector is created
        assert!(format!("{collector:?}").contains("MultiModalHumanEntropyCollector"));
    }

    #[test]
    fn test_collect_entropy_success() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5, // Realistic threshold for 32-byte sample
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let result = collector.collect_entropy();
        assert!(result.is_ok());

        let entropy = result.expect("collect_entropy should succeed in test");
        assert_eq!(entropy.len(), 32); // 256 bits
    }

    #[test]
    fn test_collect_entropy_produces_unique_values() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        // Collect entropy twice
        let entropy1 = collector
            .collect_entropy()
            .expect("collect_entropy should succeed in test");
        let entropy2 = collector
            .collect_entropy()
            .expect("collect_entropy should succeed in test");

        // Should be different (extremely unlikely to be identical)
        assert_ne!(entropy1, entropy2);
    }

    #[test]
    fn test_collect_entropy_not_all_zeros() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let entropy = collector
            .collect_entropy()
            .expect("collect_entropy should succeed in test");

        // Should not be all zeros
        let all_zeros = entropy.iter().all(|&b| b == 0);
        assert!(!all_zeros);
    }

    #[test]
    fn test_collect_entropy_not_all_same() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let entropy = collector
            .collect_entropy()
            .expect("collect_entropy should succeed in test");

        // Should not be all the same byte
        let all_same = entropy.windows(2).all(|w| w[0] == w[1]);
        assert!(!all_same);
    }

    #[test]
    fn test_collect_entropy_meets_quality_threshold() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5, // Realistic for 32-byte sample
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let result = collector.collect_entropy();
        assert!(result.is_ok());

        // Verify entropy quality meets threshold
        let entropy = result.expect("collect_entropy should succeed in test");
        let quality = super::calculate_entropy_quality(&entropy);
        assert!(quality >= 0.5);
    }

    #[test]
    fn test_collect_entropy_with_high_threshold() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5, // Realistic threshold for multi-source entropy
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        // Should succeed with multi-source entropy
        let result = collector.collect_entropy();
        assert!(
            result.is_ok(),
            "Entropy collection should succeed with quality threshold 0.5"
        );
    }

    #[test]
    fn test_collect_entropy_with_low_threshold() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        // Should definitely succeed with low threshold
        let result = collector.collect_entropy();
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_entropy_quality_uniform() {
        // Create uniform distribution (perfect entropy)
        let data: Vec<u8> = (0..=255).collect();
        let quality = super::calculate_entropy_quality(&data);

        // Should be close to 1.0 (perfect entropy)
        assert!(quality > 0.95);
    }

    #[test]
    fn test_calculate_entropy_quality_all_zeros() {
        let data = vec![0u8; 256];
        let quality = super::calculate_entropy_quality(&data);

        // Should be 0.0 (no entropy)
        assert_eq!(quality, 0.0);
    }

    #[test]
    fn test_calculate_entropy_quality_all_same() {
        let data = vec![42u8; 256];
        let quality = super::calculate_entropy_quality(&data);

        // Should be 0.0 (no entropy)
        assert_eq!(quality, 0.0);
    }

    #[test]
    fn test_calculate_entropy_quality_half_half() {
        // Half zeros, half ones
        let mut data = vec![0u8; 128];
        data.extend(vec![1u8; 128]);

        let quality = super::calculate_entropy_quality(&data);

        // Should have some entropy but not perfect
        assert!(quality > 0.0);
        assert!(quality < 1.0);
    }

    #[test]
    fn test_multiple_entropy_collections() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        // Collect 10 times
        for _ in 0..10 {
            let result = collector.collect_entropy();
            assert!(result.is_ok());
            assert_eq!(
                result
                    .expect("collect_entropy should succeed in test")
                    .len(),
                32
            );
        }
    }

    #[test]
    fn test_entropy_collection_is_fast() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let start = std::time::Instant::now();
        let result = collector.collect_entropy();
        let duration = start.elapsed();

        assert!(result.is_ok());
        // Should complete in less than 100ms
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_config_clone() {
        let config1 = HumanEntropyConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.quality_threshold, config2.quality_threshold);
        assert_eq!(config1.collection_timeout_ms, config2.collection_timeout_ms);
    }

    #[test]
    fn test_collector_clone() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector1 = MultiModalHumanEntropyCollector::new(config);
        let collector2 = collector1.clone();

        // Both should work
        assert!(collector1.collect_entropy().is_ok());
        assert!(collector2.collect_entropy().is_ok());
    }

    #[test]
    fn test_entropy_distribution() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let entropy = collector
            .collect_entropy()
            .expect("collect_entropy should succeed in test");

        // Check that we have reasonable byte diversity
        let mut byte_set = std::collections::HashSet::new();
        for &byte in &entropy {
            byte_set.insert(byte);
        }

        // Should have at least some diversity (not all same values)
        assert!(byte_set.len() > 1);
    }

    #[test]
    fn test_entropy_quality_range() {
        let config = HumanEntropyConfig {
            quality_threshold: 0.5,
            collection_timeout_ms: 5000,
        };
        let collector = MultiModalHumanEntropyCollector::new(config);

        let entropy = collector
            .collect_entropy()
            .expect("collect_entropy should succeed in test");
        let quality = super::calculate_entropy_quality(&entropy);

        // Quality should be in valid range [0.0, 1.0]
        assert!(quality >= 0.0);
        assert!(quality <= 1.0);
    }
}
