//! Software HSM entropy collection

use beardog_errors::BearDogError;
use rand::RngCore;

/// Software entropy collector
///
/// Collects entropy from the system's cryptographically secure random number generator.
/// This is suitable for software-based HSM implementations where hardware entropy
/// sources are not available.
#[derive(Debug, Clone, Default)]
pub struct SoftwareEntropyCollector;

impl SoftwareEntropyCollector {
    /// Create new entropy collector
    pub fn new() -> Self {
        Self
    }

    /// Collect cryptographically secure entropy
    ///
    /// Uses the system's CSPRNG to collect high-quality entropy.
    ///
    /// # Errors
    /// Returns an error if the system RNG fails
    pub fn collect_entropy(&self, num_bytes: usize) -> Result<Vec<u8>, BearDogError> {
        let mut entropy = vec![0u8; num_bytes];
        rand::thread_rng()
            .try_fill_bytes(&mut entropy)
            .map_err(|e| BearDogError::security(
                format!("Failed to collect entropy: {}", e),
                e.into()
            ))?;
        Ok(entropy)
    }

    /// Get entropy quality score
    ///
    /// For software-based entropy using system CSPRNG, the quality is high but not
    /// as high as hardware-based sources. Returns a score between 0.0 and 1.0.
    ///
    /// # Quality Score
    /// - 0.85: Software CSPRNG (good quality, validated by OS)
    /// - 0.95+: Would require hardware entropy source
    pub fn get_quality_score(&self) -> f64 {
        // System CSPRNG provides good quality entropy, but not as high as
        // dedicated hardware sources (TRNG)
        0.85
    }

    /// Assess the quality of collected entropy
    ///
    /// Performs basic statistical tests on the entropy to ensure it appears random.
    /// Returns a score between 0.0 (poor) and 1.0 (excellent).
    pub fn assess_entropy_quality(&self, entropy: &[u8]) -> f64 {
        if entropy.is_empty() {
            return 0.0;
        }

        // Basic entropy assessment using Shannon entropy
        let mut byte_counts = [0u64; 256];
        for &byte in entropy {
            byte_counts[byte as usize] += 1;
        }

        let len = entropy.len() as f64;
        let mut shannon_entropy = 0.0;
        
        for &count in &byte_counts {
            if count > 0 {
                let probability = count as f64 / len;
                shannon_entropy -= probability * probability.log2();
            }
        }

        // Normalize to 0.0-1.0 range (8 bits = perfect randomness)
        (shannon_entropy / 8.0).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_collector_creation() {
        let collector = SoftwareEntropyCollector::new();
        assert!(collector.collect_entropy(32).is_ok());
        assert!(collector.get_quality_score() > 0.0);
    }

    #[test]
    fn test_collect_entropy() {
        let collector = SoftwareEntropyCollector::new();
        
        // Collect entropy
        let entropy1 = collector.collect_entropy(32)?;
        let entropy2 = collector.collect_entropy(32)?;
        
        // Should be correct length
        assert_eq!(entropy1.len(), 32);
        assert_eq!(entropy2.len(), 32);
        
        // Should be different (extremely high probability)
        assert_ne!(entropy1, entropy2);
        
        // Should not be all zeros
        assert_ne!(entropy1, vec![0u8; 32]);
    }

    #[test]
    fn test_quality_score() {
        let collector = SoftwareEntropyCollector::new();
        let score = collector.get_quality_score();
        
        // Should be high quality but not perfect (0.85 for software)
        assert!(score > 0.8);
        assert!(score < 0.9);
        assert_eq!(score, 0.85);
    }

    #[test]
    fn test_assess_entropy_quality() {
        let collector = SoftwareEntropyCollector::new();
        
        // Test with actual random data
        let entropy = collector.collect_entropy(1024)?;
        let quality = collector.assess_entropy_quality(&entropy);
        
        // Should assess as high quality (near 1.0)
        assert!(quality > 0.95, "Quality score too low: {}", quality);
        
        // Test with poor entropy (all zeros)
        let poor_entropy = vec![0u8; 1024];
        let poor_quality = collector.assess_entropy_quality(&poor_entropy);
        
        // Should assess as very poor quality
        assert!(poor_quality < 0.1, "Poor entropy scored too high: {}", poor_quality);
        
        // Test with empty data
        let empty_quality = collector.assess_entropy_quality(&[]);
        assert_eq!(empty_quality, 0.0);
    }

    #[test]
    fn test_entropy_different_sizes() {
        let collector = SoftwareEntropyCollector::new();
        
        // Test various sizes
        for size in [0, 1, 16, 32, 64, 128, 256, 1024] {
            let entropy = collector.collect_entropy(size)?;
            assert_eq!(entropy.len(), size);
            
            if size > 0 {
                // Assess quality for non-empty entropy
                let quality = collector.assess_entropy_quality(&entropy);
                assert!(quality > 0.9, "Size {} quality too low: {}", size, quality);
            }
        }
    }
}
