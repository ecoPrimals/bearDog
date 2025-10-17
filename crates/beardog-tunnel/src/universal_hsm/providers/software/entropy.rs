//! Software HSM entropy collection

use beardog_errors::BearDogError;

/// Software entropy collector
#[derive(Debug, Clone, Default)]
pub struct SoftwareEntropyCollector;

impl SoftwareEntropyCollector {
    /// Create new entropy collector
    pub fn new() -> Self {
        Self
    }

    /// Collect entropy
    ///
    /// Stub implementation for entropy collection
    pub fn collect_entropy(&self, num_bytes: usize) -> Result<Vec<u8>, BearDogError> {
        // TODO: Implement actual entropy collection
        Ok(vec![0; num_bytes])
    }

    /// Get entropy quality score
    ///
    /// Stub implementation returning fixed quality score
    pub fn get_quality_score(&self) -> f64 {
        // TODO: Implement actual quality assessment
        0.8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_collector() {
        let collector = SoftwareEntropyCollector::new();
        assert!(collector.collect_entropy(32).is_ok());
        assert!(collector.get_quality_score() > 0.0);
    }
}
