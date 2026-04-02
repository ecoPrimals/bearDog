// SPDX-License-Identifier: AGPL-3.0-only

//! Entropy collection implementation.
//!
//! Collects entropy from multiple sources and mixes them cryptographically
//! (SHA3-256) to produce high-quality random data for key generation.

use beardog_errors::BearDogError;
use rand::RngCore;
use sha3::{Digest, Sha3_256};

/// Multi-source entropy collector with quality assessment.
///
/// Collects from OS CSPRNG, system timing jitter, and process context,
/// then mixes all sources via SHA3-256 to ensure uniform distribution.
#[derive(Debug, Clone)]
pub struct EntropyCollector {
    quality_threshold: f64,
}

impl Default for EntropyCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl EntropyCollector {
    /// Create a collector with the default quality threshold (0.95).
    pub fn new() -> Self {
        Self {
            quality_threshold: 0.95,
        }
    }

    /// Create a collector with a custom quality threshold (clamped to 0.0–1.0).
    pub fn with_quality_threshold(threshold: f64) -> Self {
        Self {
            quality_threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Collect `num_bytes` of high-quality entropy.
    ///
    /// # Errors
    ///
    /// Returns an error when the collected entropy falls below the quality
    /// threshold.
    pub async fn collect(&self, num_bytes: usize) -> Result<Vec<u8>, BearDogError> {
        let mut entropy_pool = Vec::with_capacity(num_bytes * 3);

        let mut os_entropy = vec![0u8; num_bytes];
        rand::rng().fill_bytes(&mut os_entropy);
        entropy_pool.extend_from_slice(&os_entropy);

        let timing_entropy = Self::collect_timing_entropy(num_bytes);
        entropy_pool.extend_from_slice(&timing_entropy);

        let context_entropy = Self::collect_context_entropy(num_bytes);
        entropy_pool.extend_from_slice(&context_entropy);

        let mixed_entropy = Self::mix_entropy_sources(&entropy_pool, num_bytes);

        // Only enforce quality threshold on samples large enough for meaningful
        // Shannon entropy measurement (512+ bytes fill the byte distribution).
        if num_bytes >= 512 {
            let quality = Self::assess_entropy_quality(&mixed_entropy);
            if quality < self.quality_threshold {
                return Err(BearDogError::security(format!(
                    "Entropy quality {quality:.3} below threshold {:.3}",
                    self.quality_threshold
                )));
            }
        }

        Ok(mixed_entropy)
    }

    fn collect_timing_entropy(num_bytes: usize) -> Vec<u8> {
        use std::time::{SystemTime, UNIX_EPOCH};

        (0..num_bytes)
            .map(|_| {
                let nanos = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.subsec_nanos())
                    .unwrap_or(0);
                #[allow(clippy::cast_possible_truncation)]
                let byte = (nanos & 0xFF) as u8;
                byte
            })
            .collect()
    }

    fn collect_context_entropy(num_bytes: usize) -> Vec<u8> {
        let mut context_data = Vec::with_capacity(num_bytes);

        let pid = std::process::id();
        context_data.extend_from_slice(&pid.to_le_bytes());

        let thread_id = std::thread::current().id();
        let thread_hash = format!("{thread_id:?}").into_bytes();
        context_data.extend_from_slice(&thread_hash);

        context_data.resize(num_bytes, 0);
        context_data
    }

    fn mix_entropy_sources(entropy_pool: &[u8], output_size: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(output_size);
        let mut hasher = Sha3_256::new();

        let iterations = output_size.div_ceil(32);

        for i in 0..iterations {
            hasher.update(entropy_pool);
            #[allow(clippy::cast_possible_truncation)]
            let counter = (i as u64).to_le_bytes();
            hasher.update(counter);

            let hash = hasher.finalize_reset();
            result.extend_from_slice(&hash);
        }

        result.truncate(output_size);
        result
    }

    /// Shannon entropy normalised to 0.0–1.0.
    #[allow(clippy::cast_precision_loss)]
    fn assess_entropy_quality(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut frequency = [0u32; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &frequency {
            if count > 0 {
                let p = f64::from(count) / len;
                entropy -= p * p.log2();
            }
        }

        entropy / 8.0
    }

    /// Current quality threshold.
    pub fn get_quality(&self) -> f64 {
        self.quality_threshold
    }

    /// Comprehensive quality assessment for diagnostics.
    pub fn assess_quality(&self, data: &[u8]) -> EntropyQualityReport {
        let shannon_entropy = Self::assess_entropy_quality(data);
        let chi_square = Self::chi_square_test(data);

        EntropyQualityReport {
            shannon_entropy,
            chi_square_statistic: chi_square,
            passes_chi_square: chi_square < 350.0,
            byte_count: data.len(),
            quality_threshold: self.quality_threshold,
            overall_quality: shannon_entropy,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn chi_square_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let expected = data.len() as f64 / 256.0;
        let mut frequency = [0u32; 256];

        for &byte in data {
            frequency[byte as usize] += 1;
        }

        frequency
            .iter()
            .map(|&count| {
                let diff = f64::from(count) - expected;
                (diff * diff) / expected
            })
            .sum()
    }
}

/// Entropy quality assessment report.
#[derive(Debug, Clone)]
pub struct EntropyQualityReport {
    /// Shannon entropy (0.0–1.0, higher is better).
    pub shannon_entropy: f64,
    /// Chi-square statistic.
    pub chi_square_statistic: f64,
    /// Whether chi-square test passes (95 % confidence).
    pub passes_chi_square: bool,
    /// Number of bytes analyzed.
    pub byte_count: usize,
    /// Quality threshold used.
    pub quality_threshold: f64,
    /// Overall quality score.
    pub overall_quality: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_entropy_collection() -> Result<(), BearDogError> {
        let collector = EntropyCollector::new();
        let entropy = collector.collect(32).await?;

        assert_eq!(entropy.len(), 32);
        assert!(
            !entropy.iter().all(|&b| b == 0),
            "Entropy should not be all zeros"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_entropy_quality_assessment() -> Result<(), BearDogError> {
        let collector = EntropyCollector::new();
        let entropy = collector.collect(1024).await?;

        let report = collector.assess_quality(&entropy);

        assert!(
            report.shannon_entropy > 0.90,
            "Shannon entropy {} should be > 0.90",
            report.shannon_entropy
        );
        assert!(
            report.passes_chi_square,
            "Chi-square test failed: {}",
            report.chi_square_statistic
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_low_quality_rejection() {
        let collector = EntropyCollector::with_quality_threshold(0.999);
        let report = collector.assess_quality(&vec![42u8; 100]);
        assert!(
            report.shannon_entropy < 0.1,
            "Low-quality data should have low entropy"
        );
    }

    #[tokio::test]
    async fn test_entropy_uniqueness() -> Result<(), BearDogError> {
        let collector = EntropyCollector::new();

        let sample1 = collector.collect(32).await?;
        let sample2 = collector.collect(32).await?;

        assert_ne!(
            sample1, sample2,
            "Consecutive entropy samples should be unique"
        );

        Ok(())
    }

    #[test]
    fn test_chi_square_perfect_distribution() {
        let perfect: Vec<u8> = (0..=255).collect();
        let chi_square = EntropyCollector::chi_square_test(&perfect);

        assert!(
            chi_square < 100.0,
            "Perfect distribution should have low chi-square: {chi_square}",
        );
    }

    #[test]
    fn test_shannon_entropy_bounds() {
        let zeros = vec![0u8; 100];
        let entropy_min = EntropyCollector::assess_entropy_quality(&zeros);
        assert!(
            entropy_min < 0.1,
            "All zeros should have entropy near 0: {entropy_min}",
        );

        let uniform: Vec<u8> = (0..=255).cycle().take(1024).collect();
        let entropy_max = EntropyCollector::assess_entropy_quality(&uniform);
        assert!(
            entropy_max > 0.95,
            "Uniform distribution should have entropy near 1.0: {entropy_max}",
        );
    }
}

#[cfg(test)]
#[path = "collector_production_tests.rs"]
mod collector_production_tests;
