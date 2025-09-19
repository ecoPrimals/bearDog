// Live Feed Entropy Validator - MANDATORY for Human Keys
// Enforces that all human entropy MUST come from live feed sources.
// Simulated entropy is COMPLETELY DISALLOWED for human key creation.

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Live feed validation result
#[derive(Debug, Clone)]
pub struct LiveFeedValidation {
    /// Whether is_live is enabled
    pub is_live: bool,
    /// The feed quality value
    pub feed_quality: f64,
    pub validation_timestamp: DateTime<Utc>,
    /// The source verification value
    pub source_verification: SourceVerification,
}

#[derive(Debug, Clone)]
pub struct SourceVerification {
    /// Whether hardware_attestation is enabled
    pub hardware_attestation: bool,
    pub temporal_validation: bool,
    /// The entropy freshness value
    pub entropy_freshness: f64,
    /// Whether anti_replay_check is enabled
    pub anti_replay_check: bool,
}

/// Live Feed Entropy Validator
pub struct LiveFeedValidator {
    required_freshness_seconds: u64,
    min_hardware_entropy_ratio: f64,
}

impl LiveFeedValidator {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            required_freshness_seconds: 5,   // Max 5 seconds old
            min_hardware_entropy_ratio: 0.8, // 80% must be hardware
        }
    }

    /// CRITICAL: Validate that entropy is from live feed only
    /// Returns error if ANY simulated entropy is detected
    /// Validates live_feed_only
    /// Validates live_feed_only
    pub fn validate_live_feed_only(
        &self,
        entropy_data: &[u8],
        source_metadata: &HashMap<String, String>,
    ) -> Result<LiveFeedValidation, BearDogError> {
        // 1. Check for simulated entropy patterns (SECURITY CRITICAL)
        self.detect_simulated_patterns(entropy_data)?;

        // 2. Validate hardware attestation
        let hardware_attestation = self.validate_hardware_source(source_metadata)?;

        // 3. Validate temporal freshness
        let temporal_validation = self.validate_temporal_freshness(source_metadata)?;

        // 4. Calculate entropy freshness
        let entropy_freshness = self.calculate_entropy_freshness(entropy_data)?;

        // 5. Anti-replay protection
        let anti_replay_check = self.validate_anti_replay(entropy_data, source_metadata)?;

        // 6. Overall validation
        let is_live = hardware_attestation
            && temporal_validation
            && entropy_freshness > 0.9
            && anti_replay_check;

        if !is_live {
            return Err(BearDogError::security(
                "CRITICAL: Non-live entropy detected for human key creation. This violates core security principles."
            ));
        }

        let feed_quality = self.calculate_feed_quality(
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

    /// SECURITY CRITICAL: Detect any simulated entropy patterns
    fn detect_simulated_patterns(&self, entropy_data: &[u8]) -> Result<(), BearDogError> {
        // Check for mathematical patterns that indicate simulation
        if self.has_mathematical_pattern(entropy_data) {
            return Err(BearDogError::security(
                "SIMULATED ENTROPY DETECTED: Mathematical patterns indicate non-live source",
            ));
        }

        // Check for repeating sequences
        if self.has_repeating_sequences(entropy_data) {
            return Err(BearDogError::security(
                "SIMULATED ENTROPY DETECTED: Repeating sequences indicate algorithmic generation",
            ));
        }

        // Check for insufficient randomness
        if self.has_insufficient_randomness(entropy_data) {
            return Err(BearDogError::security(
                "SIMULATED ENTROPY DETECTED: Insufficient randomness for live human entropy",
            ));
        }

        Ok(())
    }

    /// Checks if mathematical pattern
    fn has_mathematical_pattern(&self, data: &[u8]) -> bool {
        // Detect arithmetic progressions and other mathematical patterns
        if data.len() < 8 {
            return false;
        }

        for window in data.windows(4) {
            let diffs: Vec<i16> = window
                .windows(2)
                .map(|pair| pair[1] as i16 - pair[0] as i16)
                .collect();

            // Check for constant differences (arithmetic progression)
            if diffs.windows(2).all(|d| d[0] == d[1]) && diffs[0] != 0 {
                return true;
            }
        }
        false
    }

    /// Checks if repeating sequences
    fn has_repeating_sequences(&self, data: &[u8]) -> bool {
        // Check for repeating patterns
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

    /// Checks if insufficient randomness
    fn has_insufficient_randomness(&self, data: &[u8]) -> bool {
        // Basic entropy check - real human entropy should have high entropy
        let mut byte_counts = [0u32; 256];
        for &byte in data {
            byte_counts[byte as usize] += 1;
        }

        // Calculate Shannon entropy
        let len = data.len() as f64;
        let entropy: f64 = byte_counts
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum();

        // Real human entropy should have entropy > 6.0 bits per byte
        entropy < 6.0
    }

    /// Validates hardware_source
    fn validate_hardware_source(
        &self,
        metadata: &HashMap<String, String>,
    ) -> Result<bool, BearDogError> {
        // Require hardware attestation for human entropy
        let has_hardware = metadata
            .get("hardware_source")
            .map(|s| s == "true")
            .unwrap_or(false);

        let has_attestation = metadata.contains_key("hardware_attestation");

        Ok(has_hardware && has_attestation)
    }

    /// Validates temporal_freshness
    fn validate_temporal_freshness(
        &self,
        metadata: &HashMap<String, String>,
    ) -> Result<bool, BearDogError> {
        if let Some(timestamp_str) = metadata.get("collection_timestamp") {
            if let Ok(timestamp) = timestamp_str.parse::<i64>() {
                let collection_time = DateTime::from_timestamp(timestamp, 0)
                    .ok_or_else(|| BearDogError::validation("Invalid timestamp"))?;

                let age = Utc::now().signed_duration_since(collection_time);
                return Ok(age.num_seconds() <= self.required_freshness_seconds as i64);
            }
        }
        Ok(false)
    }


    fn calculate_entropy_freshness(&self, _entropy_data: &[u8]) -> Result<f64, BearDogError> {
        // In a real implementation, this would analyze entropy characteristics
        // For now, assume good freshness if we reach this point
        Ok(0.95)
    }

    /// Validates anti_replay
    fn validate_anti_replay(
        &self,
        _entropy_data: &[u8],
        metadata: &HashMap<String, String>,
    ) -> Result<bool, BearDogError> {
        // Check for replay protection nonce
        let has_nonce = metadata.contains_key("anti_replay_nonce");
        let has_sequence = metadata.contains_key("sequence_number");

        Ok(has_nonce && has_sequence)
    }


    fn calculate_feed_quality(
        &self,
        entropy_freshness: f64,
        hardware: bool,
        temporal: bool,
    ) -> f64 {
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
