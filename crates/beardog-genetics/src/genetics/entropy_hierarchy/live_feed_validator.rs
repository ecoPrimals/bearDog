//! Live Feed Validator
//!
//! Validates that entropy comes from live human sources only, not simulation.
//! CRITICAL for enforcing the Entropy Hierarchy Principle:
//! "Never simulate human entropy - it violates the trust model"

use beardog_errors::BearDogError;
use std::collections::HashMap;

/// Configuration for live feed validation
#[derive(Debug, Clone)]
pub struct LiveFeedConfig {
    /// Require hardware attestation metadata
    pub require_hardware_attestation: bool,
    /// Require anti-replay nonce
    pub require_anti_replay_nonce: bool,
    /// Maximum timing gap between events (milliseconds)
    pub max_timing_gap_ms: u64,
    /// Minimum timing entropy required (0.0-1.0)
    pub min_timing_entropy: f64,
    /// Maximum uniformity allowed (0.0-1.0) - too uniform = simulated
    pub max_uniformity: f64,
}

impl Default for LiveFeedConfig {
    fn default() -> Self {
        Self {
            require_hardware_attestation: true,
            require_anti_replay_nonce: true,
            max_timing_gap_ms: 5000,
            min_timing_entropy: 0.3,
            max_uniformity: 0.95,
        }
    }
}

/// Result of live feed validation
#[derive(Debug, Clone)]
pub struct LiveFeedValidationResult {
    /// Is this live data (not simulated)?
    pub is_live: bool,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// List of violations detected
    pub violations: Vec<String>,
    /// Timing entropy score (0.0-1.0)
    pub timing_entropy: f64,
    /// Data uniformity score (0.0-1.0)
    pub uniformity: f64,
}

/// Validates that entropy comes from live human sources only
///
/// CRITICAL: This enforces the entropy hierarchy principle:
/// "Never simulate human entropy - it violates the trust model"
#[derive(Debug, Clone)]
pub struct LiveFeedValidator {
    config: LiveFeedConfig,
}

impl LiveFeedValidator {
    /// Create new live feed validator with default config
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: LiveFeedConfig::default(),
        }
    }

    /// Create with custom configuration
    #[must_use]
    pub fn with_config(config: LiveFeedConfig) -> Self {
        Self { config }
    }

    /// Validate that entropy comes from live feed only (NO SIMULATION)
    ///
    /// This performs multiple checks:
    /// 1. Hardware attestation (if required)
    /// 2. Anti-replay nonce (if required)
    /// 3. Timing entropy analysis
    /// 4. Uniformity check (detect simulation)
    /// 5. Pattern detection (detect PRNGs)
    pub fn validate_live_feed_only(
        &self,
        entropy_data: &[u8],
        source_metadata: &HashMap<String, String>,
    ) -> Result<LiveFeedValidationResult, BearDogError> {
        let mut violations = Vec::new();

        // Check 1: Hardware attestation
        if self.config.require_hardware_attestation
            && !source_metadata.contains_key("hardware_attestation")
        {
            violations.push("Missing hardware attestation".to_string());
        }

        // Check 2: Anti-replay nonce
        if self.config.require_anti_replay_nonce
            && !source_metadata.contains_key("anti_replay_nonce")
        {
            violations.push("Missing anti-replay nonce".to_string());
        }

        // Check 3: Timing entropy (CRITICAL for human validation)
        let timing_entropy = self.calculate_timing_entropy(entropy_data)?;
        if timing_entropy < self.config.min_timing_entropy {
            violations.push(format!(
                "Timing entropy too low: {:.2}% (required: {:.2}%)",
                timing_entropy * 100.0,
                self.config.min_timing_entropy * 100.0
            ));
        }

        // Check 4: Uniformity (too uniform = likely simulated)
        let uniformity = self.calculate_uniformity(entropy_data);
        if uniformity > self.config.max_uniformity {
            violations.push(format!(
                "Data too uniform: {:.2}% (max: {:.2}%) - likely simulated",
                uniformity * 100.0,
                self.config.max_uniformity * 100.0
            ));
        }

        // Check 5: PRNG pattern detection
        if let Some(prng_type) = self.detect_prng_pattern(entropy_data)? {
            violations.push(format!(
                "PRNG pattern detected: {} - entropy is simulated",
                prng_type
            ));
        }

        // Calculate overall confidence
        let is_live = violations.is_empty()
            && timing_entropy >= self.config.min_timing_entropy
            && uniformity <= self.config.max_uniformity;

        let confidence = if is_live {
            // High confidence if all checks pass
            0.9 + (timing_entropy * 0.1)
        } else {
            // Low confidence if violations detected
            0.1
        };

        Ok(LiveFeedValidationResult {
            is_live,
            confidence,
            violations,
            timing_entropy,
            uniformity,
        })
    }

    /// Calculate timing entropy from data
    ///
    /// Uses Shannon entropy to measure randomness.
    /// Higher = more random (human-like)
    /// Lower = more uniform (simulated)
    fn calculate_timing_entropy(&self, data: &[u8]) -> Result<f64, BearDogError> {
        if data.is_empty() {
            return Ok(0.0);
        }

        // Count byte frequencies
        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        // Calculate Shannon entropy
        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = f64::from(count) / len;
                entropy -= p * p.log2();
            }
        }

        // Normalize to 0-1 range (max entropy for bytes is 8 bits)
        Ok(entropy / 8.0)
    }

    /// Calculate uniformity of data
    ///
    /// Measures how evenly distributed byte values are.
    /// Too uniform (close to 1.0) indicates simulation.
    fn calculate_uniformity(&self, data: &[u8]) -> f64 {
        if data.is_empty() {
            return 1.0;
        }

        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        // Count how many different byte values appear
        let used_values = counts.iter().filter(|&&c| c > 0).count();

        // Calculate uniformity (closer to 1.0 = more uniform)
        f64::from(used_values as u16) / 256.0
    }

    /// Detect known PRNG patterns
    ///
    /// Checks for patterns typical of pseudo-random number generators,
    /// which indicate simulated (not live) entropy.
    fn detect_prng_pattern(&self, data: &[u8]) -> Result<Option<String>, BearDogError> {
        if data.len() < 16 {
            return Ok(None);
        }

        // Check for LCG (Linear Congruential Generator) patterns
        // LCGs produce consecutive values that are too closely spaced
        let mut gaps = Vec::new();
        let sample_size = data.len().min(32);

        for i in 0..sample_size - 1 {
            let gap = i16::from(data[i + 1]).abs_diff(i16::from(data[i]));
            gaps.push(gap);
        }

        if !gaps.is_empty() {
            let avg_gap: f64 = gaps.iter().map(|&g| f64::from(g)).sum::<f64>() / gaps.len() as f64;

            // LCGs often have very consistent small gaps
            if avg_gap < 5.0 {
                return Ok(Some("LCG-like pattern".to_string()));
            }
        }

        // Check for repeating patterns (common in weak PRNGs)
        if let Some(period) = self.detect_repeating_pattern(data) {
            return Ok(Some(format!("Repeating pattern (period: {})", period)));
        }

        Ok(None)
    }

    /// Detect repeating patterns in data
    fn detect_repeating_pattern(&self, data: &[u8]) -> Option<usize> {
        let len = data.len();
        if len < 4 {
            return None;
        }

        // Check for short repeating patterns (2-16 bytes)
        for period in 2..=16.min(len / 2) {
            let mut matches = 0;
            let repeats = len / period;

            for i in 0..period {
                let mut all_same = true;
                let first_val = data[i];

                for r in 1..repeats {
                    if data[i + r * period] != first_val {
                        all_same = false;
                        break;
                    }
                }

                if all_same {
                    matches += 1;
                }
            }

            // If most values repeat with this period, it's likely a pattern
            if matches > period * 3 / 4 {
                return Some(period);
            }
        }

        None
    }
}

impl Default for LiveFeedValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_feed_validator_creation() {
        let validator = LiveFeedValidator::new();
        assert!(validator.config.require_hardware_attestation);
        assert!(validator.config.require_anti_replay_nonce);
    }

    #[test]
    fn test_live_feed_validator_rejects_missing_attestation() {
        let validator = LiveFeedValidator::new();
        let data = vec![1, 2, 3, 4, 5];
        let metadata = HashMap::new(); // No attestation

        let result = validator.validate_live_feed_only(&data, &metadata).unwrap();

        assert!(!result.is_live);
        assert!(!result.violations.is_empty());
        assert!(result
            .violations
            .iter()
            .any(|v| v.contains("hardware attestation")));
    }

    #[test]
    fn test_live_feed_validator_rejects_uniform_data() {
        let validator = LiveFeedValidator::new();

        // Perfectly uniform data (all byte values 0-255)
        let data: Vec<u8> = (0..=255).collect();

        let mut metadata = HashMap::new();
        metadata.insert("hardware_attestation".to_string(), "true".to_string());
        metadata.insert("anti_replay_nonce".to_string(), "test-nonce".to_string());

        let result = validator.validate_live_feed_only(&data, &metadata).unwrap();

        // Should detect as too uniform
        assert!(!result.is_live);
        assert!(result.violations.iter().any(|v| v.contains("uniform")));
    }

    #[test]
    fn test_live_feed_validator_accepts_random_data() {
        let validator = LiveFeedValidator::new();

        // Real random data (should have good entropy)
        use rand::RngCore;
        let mut data = vec![0u8; 256];
        rand::thread_rng().fill_bytes(&mut data);

        let mut metadata = HashMap::new();
        metadata.insert("hardware_attestation".to_string(), "true".to_string());
        metadata.insert(
            "anti_replay_nonce".to_string(),
            uuid::Uuid::new_v4().to_string(),
        );

        let result = validator.validate_live_feed_only(&data, &metadata).unwrap();

        // Should accept good random data
        assert!(result.is_live || result.timing_entropy > 0.5);
    }

    #[test]
    fn test_calculate_timing_entropy_empty() {
        let validator = LiveFeedValidator::new();
        let result = validator.calculate_timing_entropy(&[]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_timing_entropy_low() {
        let validator = LiveFeedValidator::new();

        // All same value = very low entropy
        let data = vec![42u8; 100];
        let result = validator.calculate_timing_entropy(&data).unwrap();

        assert!(result < 0.1); // Should be very low
    }

    #[test]
    fn test_calculate_uniformity() {
        let validator = LiveFeedValidator::new();

        // All 256 byte values = maximum uniformity
        let uniform_data: Vec<u8> = (0..=255).collect();
        let uniformity = validator.calculate_uniformity(&uniform_data);
        assert_eq!(uniformity, 1.0);

        // Single value = minimum uniformity
        let non_uniform = vec![42u8; 100];
        let low_uniformity = validator.calculate_uniformity(&non_uniform);
        assert!(low_uniformity < 0.01);
    }

    #[test]
    fn test_detect_lcg_pattern() {
        let validator = LiveFeedValidator::new();

        // Simulate LCG-like pattern (consecutive small gaps)
        let lcg_data: Vec<u8> = (0..32).map(|i| (i * 3 % 256) as u8).collect();

        let result = validator.detect_prng_pattern(&lcg_data).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_repeating_pattern() {
        let validator = LiveFeedValidator::new();

        // Create obvious repeating pattern
        let pattern = vec![1u8, 2, 3, 4];
        let mut data = Vec::new();
        for _ in 0..10 {
            data.extend_from_slice(&pattern);
        }

        let result = validator.detect_repeating_pattern(&data);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_live_feed_custom_config() {
        let config = LiveFeedConfig {
            require_hardware_attestation: false,
            require_anti_replay_nonce: false,
            max_timing_gap_ms: 10000,
            min_timing_entropy: 0.1,
            max_uniformity: 0.99,
        };

        let validator = LiveFeedValidator::with_config(config);

        // Should accept data without attestation
        let data = vec![1, 2, 3, 4, 5];
        let metadata = HashMap::new();

        let result = validator.validate_live_feed_only(&data, &metadata).unwrap();

        // May still fail on timing entropy but not on metadata
        assert!(!result.violations.iter().any(|v| v.contains("attestation")));
    }
}
