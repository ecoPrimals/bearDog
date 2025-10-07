use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::info;

/// Confidence analysis for mathematical certainty in BearDog operations
pub struct ConfidenceAnalyzer {
    confidence_threshold: f64,
    statistical_samples: usize,
}

impl Default for ConfidenceAnalyzer {
    fn default() -> Self {
        Self {
            confidence_threshold: 0.95, // 95% confidence level
            statistical_samples: 1000,
        }
    }
}

impl ConfidenceAnalyzer {
    pub fn new(threshold: f64, samples: usize) -> Self {
        Self {
            confidence_threshold: threshold,
            statistical_samples: samples,
        }
    }

    /// Analyze confidence level for cryptographic operations
    pub async fn analyze_crypto_confidence(&self, operation_results: &[bool]) -> Result<f64, BearDogError> {
        if operation_results.is_empty() {
            return Err(BearDogError::validation("No operation results provided"));
        }

        let success_count = operation_results.iter().filter(|&&result| result).count();
        let success_rate = success_count as f64 / operation_results.len() as f64;
        
        info!("Crypto confidence analysis: {}/{} operations successful ({:.2}%)", 
              success_count, operation_results.len(), success_rate * 100.0);

        Ok(success_rate)
    }

    /// Analyze confidence level for system operations
    pub async fn analyze_system_confidence(&self, metrics: &HashMap<String, f64>) -> Result<f64, BearDogError> {
        if metrics.is_empty() {
            return Err(BearDogError::validation("No metrics provided"));
        }

        let mut weighted_confidence = 0.0;
        let mut total_weight = 0.0;

        for (metric_name, value) in metrics {
            let weight = self.get_metric_weight(metric_name);
            weighted_confidence += value * weight;
            total_weight += weight;
        }

        if total_weight == 0.0 {
            return Err(BearDogError::internal("No valid metric weights"));
        }

        let final_confidence = weighted_confidence / total_weight;
        info!("System confidence analysis: {:.2}% overall confidence", final_confidence * 100.0);

        Ok(final_confidence)
    }

    /// Calculate confidence intervals for performance metrics
    pub fn calculate_confidence_interval(&self, samples: &[f64]) -> Result<(f64, f64), BearDogError> {
        if samples.len() < 2 {
            return Err(BearDogError::validation("Need at least 2 samples for confidence interval"));
        }

        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        let variance = samples.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (samples.len() - 1) as f64;
        let std_dev = variance.sqrt();
        
        // 95% confidence interval (approximately ±1.96 standard errors)
        let margin_of_error = 1.96 * (std_dev / (samples.len() as f64).sqrt());
        
        Ok((mean - margin_of_error, mean + margin_of_error))
    }

    fn get_metric_weight(&self, metric_name: &str) -> f64 {
        match metric_name {
            "security_score" => 0.4,
            "performance_score" => 0.3,
            "reliability_score" => 0.2,
            "compliance_score" => 0.1,
            _ => 0.05, // Default weight for unknown metrics
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crypto_confidence_analysis() {
        let analyzer = ConfidenceAnalyzer::default();
        let results = vec![true, true, false, true, true]; // 80% success rate
        
        let confidence = analyzer.analyze_crypto_confidence(&results).unwrap();
        assert!((confidence - 0.8).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_system_confidence_analysis() {
        let analyzer = ConfidenceAnalyzer::default();
        let mut metrics = HashMap::new();
        metrics.insert("security_score".to_string(), 0.95);
        metrics.insert("performance_score".to_string(), 0.85);
        
        let confidence = analyzer.analyze_system_confidence(&metrics).unwrap();
        assert!(confidence > 0.8);
    }

    #[test]
    fn test_confidence_interval() {
        let analyzer = ConfidenceAnalyzer::default();
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let (lower, upper) = analyzer.calculate_confidence_interval(&samples).unwrap();
        assert!(lower < 3.0 && upper > 3.0); // Mean should be within interval
    }
}
