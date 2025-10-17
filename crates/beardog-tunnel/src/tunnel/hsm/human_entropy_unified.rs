//! Unified Human Entropy Classifier
//!
//! This module provides a unified classifier for human entropy from various sources
//! for use in HSM tier elevation and key generation.

use super::types::HsmTier;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Unified human entropy classifier
pub struct UnifiedHumanEntropyClassifier {
    tier_criteria: TierElevationCriteria,
}

/// Tier elevation criteria
#[derive(Debug, Clone)]
pub struct TierElevationCriteria {
    pub min_quality_score: f64,
    pub require_biometric: bool,
    pub require_multiple_sources: bool,
    pub require_realtime: bool,
}

impl Default for TierElevationCriteria {
    fn default() -> Self {
        Self {
            min_quality_score: 0.7,
            require_biometric: true, // Default to requiring biometric
            require_multiple_sources: false,
            require_realtime: false,
        }
    }
}

// NOTE: UnifiedHumanEntropyClassifier Default implementation removed - use Type::new() instead since it returns Result

impl UnifiedHumanEntropyClassifier {
    /// Creates a new unified human entropy classifier
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            tier_criteria: TierElevationCriteria::default(),
        })
    }

    /// Creates classifier with custom criteria
    pub fn with_criteria(criteria: TierElevationCriteria) -> Result<Self, BearDogError> {
        Ok(Self {
            tier_criteria: criteria,
        })
    }

    /// Classifies human entropy from mobile device
    pub fn classify_mobile_entropy(
        &self,
        sensor_data: &HashMap<String, f64>,
    ) -> Result<HumanEntropyClassification, BearDogError> {
        info!("📊 Classifying mobile human entropy");

        let quality_score = self.calculate_quality_score(sensor_data)?;
        let source_count = sensor_data.len();
        let has_biometric =
            sensor_data.contains_key("fingerprint") || sensor_data.contains_key("face_scan");

        let meets_criteria = quality_score >= self.tier_criteria.min_quality_score
            && (!self.tier_criteria.require_biometric || has_biometric)
            && (!self.tier_criteria.require_multiple_sources || source_count >= 2);

        let classification = HumanEntropyClassification {
            quality_score,
            source_count,
            has_biometric,
            is_realtime: true,
            meets_elevation_criteria: meets_criteria,
            recommended_tier: self.recommend_tier(quality_score, has_biometric)?,
        };

        info!(
            "✅ Entropy classification complete: score={:.2}",
            quality_score
        );
        Ok(classification)
    }

    /// Checks if capabilities support ephemeral seeds
    pub fn supports_ephemeral_seeds(
        &self,
        capabilities: &HsmCapabilities,
    ) -> Result<bool, BearDogError> {
        Ok(capabilities.supports_ephemeral_seeds)
    }

    /// Gets human entropy sources for a tier
    pub fn get_sources_for_tier(&self, tier: &HsmTier) -> Result<Vec<EntropySource>, BearDogError> {
        match tier {
            HsmTier::Mobile | HsmTier::SecureEnclave => {
                // NOTE: Device type info no longer embedded in tier variant
                // Defaulting to full mobile entropy sources
                Ok(vec![
                    EntropySource::Biometric,
                    EntropySource::TouchPattern,
                    EntropySource::Sensors,
                ])
            }
            _ => Ok(vec![EntropySource::Manual]),
        }
    }

    /// Calculates quality score from sensor data
    fn calculate_quality_score(
        &self,
        sensor_data: &HashMap<String, f64>,
    ) -> Result<f64, BearDogError> {
        if sensor_data.is_empty() {
            return Ok(0.0);
        }

        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        for (sensor_type, value) in sensor_data {
            let weight = self.get_sensor_weight(sensor_type);
            total_score += value * weight;
            weight_sum += weight;
        }

        let score = if weight_sum > 0.0 {
            total_score / weight_sum
        } else {
            0.0
        };

        Ok(score.min(1.0))
    }

    /// Gets weight for a sensor type
    fn get_sensor_weight(&self, sensor_type: &str) -> f64 {
        match sensor_type {
            "fingerprint" | "face_scan" => 1.0,
            "touch_pressure" | "touch_duration" => 0.8,
            "accelerometer" | "gyroscope" => 0.6,
            "ambient_light" | "proximity" => 0.4,
            _ => 0.3,
        }
    }

    /// Recommends tier based on quality and biometric data
    fn recommend_tier(
        &self,
        quality_score: f64,
        has_biometric: bool,
    ) -> Result<HsmTier, BearDogError> {
        if quality_score >= 0.8 && has_biometric {
            // NOTE: Simplified to unit variant - device info managed separately
            Ok(HsmTier::SecureEnclave)
        } else {
            // NOTE: Simplified to unit variant
            Ok(HsmTier::Software)
        }
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsafe implementation used .expect() which could panic
// Use Type::new()? or Type::new().unwrap_or_else(|e| { /* handle error */ }) instead

/// Human entropy classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyClassification {
    pub quality_score: f64,
    pub source_count: usize,
    pub has_biometric: bool,
    pub is_realtime: bool,
    pub meets_elevation_criteria: bool,
    pub recommended_tier: HsmTier,
}

/// HSM capabilities
#[derive(Debug, Clone)]
pub struct HsmCapabilities {
    pub supports_ephemeral_seeds: bool,
    pub supports_biometric: bool,
    pub supports_realtime_entropy: bool,
}

impl Default for HsmCapabilities {
    fn default() -> Self {
        Self {
            supports_ephemeral_seeds: true, // Default to supporting ephemeral seeds
            supports_biometric: true,       // Default to supporting biometric
            supports_realtime_entropy: false,
        }
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsafe implementation used .expect() which could panic
// Use Type::new()? or Type::new().unwrap_or_else(|e| { /* handle error */ }) instead

/// Entropy source types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropySource {
    Biometric,
    TouchPattern,
    Sensors,
    Manual,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classifier_creation() {
        let classifier = UnifiedHumanEntropyClassifier::new();
        assert!(classifier.is_ok());
    }

    #[test]
    fn test_criteria_default() {
        let criteria = TierElevationCriteria::default();
        assert_eq!(criteria.min_quality_score, 0.7);
        assert!(criteria.require_biometric);
    }

    #[test]
    fn test_quality_score_empty() {
        let classifier = UnifiedHumanEntropyClassifier::new().unwrap();
        let sensor_data = HashMap::new();
        let score = classifier.calculate_quality_score(&sensor_data).unwrap();
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_quality_score_with_data() {
        let classifier = UnifiedHumanEntropyClassifier::new().unwrap();
        let mut sensor_data = HashMap::new();
        sensor_data.insert("fingerprint".to_string(), 0.9);
        sensor_data.insert("touch_pressure".to_string(), 0.8);

        let score = classifier.calculate_quality_score(&sensor_data).unwrap();
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_sensor_weights() {
        let classifier = UnifiedHumanEntropyClassifier::new().unwrap();
        assert_eq!(classifier.get_sensor_weight("fingerprint"), 1.0);
        assert_eq!(classifier.get_sensor_weight("touch_pressure"), 0.8);
        assert_eq!(classifier.get_sensor_weight("accelerometer"), 0.6);
    }

    #[test]
    fn test_classification() {
        let classifier = UnifiedHumanEntropyClassifier::new().unwrap();
        let mut sensor_data = HashMap::new();
        sensor_data.insert("fingerprint".to_string(), 0.9);

        let result = classifier.classify_mobile_entropy(&sensor_data);
        assert!(result.is_ok());

        let classification = result.unwrap();
        assert!(classification.has_biometric);
        assert!(classification.quality_score > 0.0);
    }

    #[test]
    fn test_capabilities() {
        let caps = HsmCapabilities::default();
        assert!(caps.supports_ephemeral_seeds);
        assert!(caps.supports_biometric);
    }

    #[test]
    fn test_criteria_with_custom() {
        let criteria = TierElevationCriteria {
            min_quality_score: 0.9,
            require_biometric: false,
            require_multiple_sources: false,
            require_realtime: false,
        };

        let classifier = UnifiedHumanEntropyClassifier::with_criteria(criteria);
        assert!(classifier.is_ok());
    }

    #[test]
    fn test_tier_sources() {
        let classifier = UnifiedHumanEntropyClassifier::new().unwrap();
        // NOTE: Simplified to unit variant - device info managed separately
        let tier = HsmTier::Mobile;

        let sources = classifier.get_sources_for_tier(&tier).unwrap();
        assert!(!sources.is_empty());
    }
}
