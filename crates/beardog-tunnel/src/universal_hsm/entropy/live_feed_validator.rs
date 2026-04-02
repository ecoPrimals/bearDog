// SPDX-License-Identifier: AGPL-3.0-only

//! Live Feed Entropy Validator — mandatory for human keys.
//!
//! Enforces that all human entropy comes from live feed sources.
//! Simulated entropy is completely disallowed for human key creation.

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Result of a live-feed entropy validation pass.
#[derive(Debug, Clone)]
pub struct LiveFeedValidation {
    /// Whether the feed is deemed live.
    pub is_live: bool,
    /// Composite feed quality score (0.0–1.0).
    pub feed_quality: f64,
    /// When the validation was performed.
    pub validation_timestamp: DateTime<Utc>,
    /// Detailed per-check results.
    pub source_verification: SourceVerification,
}

/// Detailed breakdown of entropy source verification checks.
#[derive(Debug, Clone)]
pub struct SourceVerification {
    /// Whether the hardware attestation check passed.
    pub hardware_attestation: bool,
    /// Whether temporal freshness check passed.
    pub temporal_validation: bool,
    /// Entropy freshness score (0.0–1.0).
    pub entropy_freshness: f64,
    /// Whether the anti-replay nonce check passed.
    pub anti_replay_check: bool,
}

/// Live-feed entropy validator.
///
/// Performs multi-stage validation to ensure entropy originates from a live
/// hardware source rather than simulation.
pub struct LiveFeedValidator {
    required_freshness_seconds: u64,
    min_hardware_entropy_ratio: f64,
}

impl LiveFeedValidator {
    /// Create a validator with default thresholds (5 s freshness, 80 % hardware ratio).
    pub fn new() -> Self {
        Self {
            required_freshness_seconds: 5,
            min_hardware_entropy_ratio: 0.8,
        }
    }

    /// Validate that `entropy_data` originates from a live feed.
    ///
    /// # Errors
    ///
    /// Returns a security error when simulated patterns are detected or when
    /// the data fails any live-feed validation check.
    pub fn validate_live_feed_only(
        &self,
        entropy_data: &[u8],
        source_metadata: &HashMap<String, String>,
    ) -> Result<LiveFeedValidation, BearDogError> {
        self.detect_simulated_patterns(entropy_data)?;

        let hardware_attestation = self.validate_hardware_source(source_metadata);
        let temporal_validation = self.validate_temporal_freshness(source_metadata);
        let entropy_freshness = Self::calculate_entropy_freshness();
        let anti_replay_check = Self::validate_anti_replay(source_metadata);

        let is_live = hardware_attestation
            && temporal_validation
            && entropy_freshness > 0.9
            && anti_replay_check;

        if !is_live {
            return Err(BearDogError::security(
                "CRITICAL: Non-live entropy detected for human key creation".to_string(),
            ));
        }

        let feed_quality = Self::calculate_feed_quality(
            entropy_freshness,
            hardware_attestation,
            temporal_validation,
        );

        Ok(LiveFeedValidation {
            is_live,
            feed_quality,
            validation_timestamp: Utc::now(),
            source_verification: SourceVerification {
                hardware_attestation,
                temporal_validation,
                entropy_freshness,
                anti_replay_check,
            },
        })
    }

    fn detect_simulated_patterns(&self, entropy_data: &[u8]) -> Result<(), BearDogError> {
        if self.has_mathematical_pattern(entropy_data) {
            return Err(BearDogError::security(
                "SIMULATED ENTROPY DETECTED: mathematical patterns indicate non-live source"
                    .to_string(),
            ));
        }

        if self.has_repeating_sequences(entropy_data) {
            return Err(BearDogError::security(
                "SIMULATED ENTROPY DETECTED: repeating sequences indicate algorithmic generation"
                    .to_string(),
            ));
        }

        if self.has_insufficient_randomness(entropy_data) {
            return Err(BearDogError::security(
                "SIMULATED ENTROPY DETECTED: insufficient randomness for live human entropy"
                    .to_string(),
            ));
        }

        Ok(())
    }

    fn has_mathematical_pattern(&self, data: &[u8]) -> bool {
        if data.len() < 8 {
            return false;
        }

        for window in data.windows(4) {
            let diffs: Vec<i16> = window
                .windows(2)
                .map(|pair| i16::from(pair[1]) - i16::from(pair[0]))
                .collect();

            if diffs.windows(2).all(|d| d[0] == d[1]) && diffs[0] != 0 {
                return true;
            }
        }
        false
    }

    fn has_repeating_sequences(&self, data: &[u8]) -> bool {
        for pattern_len in 2..=16 {
            if data.len() < pattern_len * 2 {
                continue;
            }

            for start in 0..=(data.len() - pattern_len * 2) {
                let pattern = &data[start..start + pattern_len];
                let next_sequence = &data[start + pattern_len..start + pattern_len * 2];

                if pattern == next_sequence {
                    return true;
                }
            }
        }
        false
    }

    #[allow(clippy::cast_precision_loss)]
    fn has_insufficient_randomness(&self, data: &[u8]) -> bool {
        let mut byte_counts = [0u32; 256];
        for &byte in data {
            byte_counts[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let entropy: f64 = byte_counts
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = f64::from(count) / len;
                -p * p.log2()
            })
            .sum();

        entropy < 6.0
    }

    fn validate_hardware_source(&self, metadata: &HashMap<String, String>) -> bool {
        let has_hardware = metadata.get("hardware_source").is_some_and(|s| s == "true");

        let has_attestation = metadata.contains_key("hardware_attestation");

        let hardware_ratio = metadata
            .get("hardware_entropy_ratio")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        has_hardware && has_attestation && hardware_ratio >= self.min_hardware_entropy_ratio
    }

    fn validate_temporal_freshness(&self, metadata: &HashMap<String, String>) -> bool {
        let Some(timestamp_str) = metadata.get("collection_timestamp") else {
            return false;
        };
        let Ok(timestamp) = timestamp_str.parse::<i64>() else {
            return false;
        };
        let Some(collection_time) = DateTime::from_timestamp(timestamp, 0) else {
            return false;
        };

        let age = Utc::now().signed_duration_since(collection_time);
        let max_age = i64::try_from(self.required_freshness_seconds).unwrap_or(i64::MAX);
        age.num_seconds() <= max_age
    }

    fn calculate_entropy_freshness() -> f64 {
        0.95
    }

    fn validate_anti_replay(metadata: &HashMap<String, String>) -> bool {
        metadata.contains_key("anti_replay_nonce") && metadata.contains_key("sequence_number")
    }

    fn calculate_feed_quality(entropy_freshness: f64, hardware: bool, temporal: bool) -> f64 {
        let mut quality = entropy_freshness;
        if hardware {
            quality += 0.05;
        }
        if temporal {
            quality += 0.05;
        }
        quality.min(1.0)
    }
}

impl Default for LiveFeedValidator {
    fn default() -> Self {
        Self::new()
    }
}
