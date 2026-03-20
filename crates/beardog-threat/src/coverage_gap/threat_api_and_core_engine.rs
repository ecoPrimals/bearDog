// SPDX-License-Identifier: AGPL-3.0-only

// ============================================================================
// ThreatAPI (mod.rs) - 8 uncovered lines, 0% coverage
// ============================================================================

#[cfg(test)]
mod threat_api_tests {
    use crate::threat::ThreatAPI;
    use beardog_errors::BearDogError;

    #[test]
    fn test_create_default() -> Result<(), BearDogError> {
        let engine = ThreatAPI::create_default()?;
        assert!(engine.detection_rules.is_empty());
        assert!(engine.active_threats.is_empty());
        Ok(())
    }

    #[test]
    fn test_new_with_ml() -> Result<(), BearDogError> {
        let (threat_engine, _ml_engine) = ThreatAPI::new_with_ml()?;
        assert!(threat_engine.detection_rules.is_empty());
        Ok(())
    }
}
// ============================================================================
// handlers/core.rs - 5 uncovered lines, 80.77% coverage
// ============================================================================

#[cfg(test)]
mod core_engine_gap_tests {
    use crate::threat::handlers::core::ThreatDetectionEngine;
    use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

    #[test]
    fn test_engine_with_ml_enabled() {
        let mut config = ThreatDetectionConfig::default();
        config.ml_enhancement = true;
        let engine = ThreatDetectionEngine::new(config).unwrap();
        assert!(engine.ml_engine.is_some());
    }

    #[test]
    fn test_engine_without_ml() {
        let mut config = ThreatDetectionConfig::default();
        config.ml_enhancement = false;
        let engine = ThreatDetectionEngine::new(config).unwrap();
        assert!(engine.ml_engine.is_none());
    }

    #[test]
    fn test_engine_remove_rule_delegates() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.remove_rule("nonexistent");
        assert!(!result);
    }
}
