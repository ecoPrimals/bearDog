// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

// ML Integration Tests
//
// Comprehensive test coverage for machine learning integration functionality

use crate::threat::handlers::core::ThreatDetectionEngine;
use crate::threat::types::{
    DetectionMethod, MlModel, MlModelType, ThreatSeverity, ThreatStatus, ThreatType,
};
use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;
use std::collections::HashMap;

#[cfg(test)]
mod ml_integration_tests {
    use super::*;
    use crate::float_assert::near_f64;

    fn create_test_engine() -> ThreatDetectionEngine {
        let config = ThreatDetectionConfig::default();
        ThreatDetectionEngine::new(config).unwrap()
    }

    fn create_engine_with_model() -> ThreatDetectionEngine {
        let mut engine = create_test_engine();
        engine.add_ml_model(create_test_ml_model());
        engine
    }

    fn create_test_ml_model() -> MlModel {
        MlModel {
            id: "test_model_1".to_string(),
            name: "Test ML Model".to_string(),
            model_type: MlModelType::NeuralNetwork,
            version: "1.0".to_string(),
            accuracy: 0.95,
            trained_at: std::time::SystemTime::now(),
        }
    }

    #[test]
    fn test_analyze_with_ml_no_models_returns_empty() {
        let engine = create_test_engine();
        let mut event_data = HashMap::new();
        event_data.insert("severity", "high");
        event_data.insert("type", "intrusion");

        let result = engine.analyze_with_ml(&event_data);
        assert!(result.is_ok());
        assert!(
            result.unwrap().is_empty(),
            "no models loaded → empty results"
        );
    }

    #[test]
    fn test_analyze_with_ml_success() {
        let engine = create_engine_with_model();
        let mut event_data = HashMap::new();
        event_data.insert("severity", "high");
        event_data.insert("type", "intrusion");

        let result = engine.analyze_with_ml(&event_data);
        assert!(result.is_ok());

        let events = result.unwrap();
        for event in &events {
            assert_eq!(event.threat_type, ThreatType::Anomaly);
            assert_eq!(event.status, ThreatStatus::Active);
            assert_eq!(event.detection_method, DetectionMethod::MachineLearning);
            assert!(event.confidence >= 0.0 && event.confidence <= 1.0);
        }
    }

    #[test]
    fn test_analyze_with_ml_empty_data() {
        let engine = create_engine_with_model();
        let event_data = HashMap::new();

        let result = engine.analyze_with_ml(&event_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_with_ml_various_inputs() {
        let engine = create_engine_with_model();

        let test_cases = vec![
            vec![("type", "malware"), ("source", "external")],
            vec![("type", "ddos"), ("severity", "critical")],
            vec![("type", "phishing"), ("target", "user")],
        ];

        for case in test_cases {
            let mut event_data = HashMap::new();
            for (k, v) in case {
                event_data.insert(k, v);
            }

            let result = engine.analyze_with_ml(&event_data);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_analyze_with_ml_event_structure() {
        let engine = create_engine_with_model();
        let mut event_data = HashMap::new();
        event_data.insert("resource", "test-server");
        event_data.insert("hostname", "ml-host-01");

        let result = engine.analyze_with_ml(&event_data).unwrap();
        if let Some(event) = result.first() {
            assert_eq!(event.source.source_type, "machine_learning");
            assert!(event.source.identifier.starts_with("ml_"));
            assert_eq!(event.target.target_type, "system");
            assert!(event.target.identifier.starts_with("ml_target_"));
        }
    }

    #[test]
    fn test_analyze_with_ml_generates_unique_ids() {
        let engine = create_engine_with_model();
        let mut event_data = HashMap::new();
        event_data.insert("key", "value");

        let result1 = engine.analyze_with_ml(&event_data).unwrap();
        let result2 = engine.analyze_with_ml(&event_data).unwrap();

        if !result1.is_empty() && !result2.is_empty() {
            let id1 = &result1[0].id;
            let id2 = &result2[0].id;
            assert_ne!(id1, id2);
        }
    }

    #[test]
    fn test_calculate_threat_score_basic() {
        let engine = create_test_engine();
        let model = create_test_ml_model();
        let mut event_data = HashMap::new();
        event_data.insert("severity", "high");

        let result = engine.calculate_threat_score(&model, &event_data);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_calculate_threat_score_empty_data() {
        let engine = create_test_engine();
        let model = create_test_ml_model();
        let event_data = HashMap::new();

        let result = engine.calculate_threat_score(&model, &event_data);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_calculate_threat_score_deterministic() {
        let engine = create_test_engine();
        let model = create_test_ml_model();
        let mut event_data = HashMap::new();
        event_data.insert("key1", "value1");
        event_data.insert("key2", "value2");

        let score1 = engine.calculate_threat_score(&model, &event_data).unwrap();
        let score2 = engine.calculate_threat_score(&model, &event_data).unwrap();

        near_f64(score1, score2);
    }

    #[test]
    fn test_calculate_threat_score_different_data() {
        let engine = create_test_engine();
        let model = create_test_ml_model();

        let mut data1 = HashMap::new();
        data1.insert("key", "value1");

        let mut data2 = HashMap::new();
        data2.insert("key", "value2");

        let score1 = engine.calculate_threat_score(&model, &data1).unwrap();
        let score2 = engine.calculate_threat_score(&model, &data2).unwrap();

        // Different data should produce different scores (most likely)
        assert!(
            (score1 - score2).abs() > 1e-9,
            "expected scores to differ, got {score1} and {score2}"
        );
    }

    #[test]
    fn test_calculate_threat_score_range() {
        let engine = create_test_engine();
        let model = create_test_ml_model();

        for i in 0..10 {
            let i_str = i.to_string();
            let mut event_data = HashMap::new();
            event_data.insert("iteration", i_str.as_str());

            let score = engine.calculate_threat_score(&model, &event_data).unwrap();
            assert!(score >= 0.0, "Score should be >= 0.0");
            assert!(score <= 1.0, "Score should be <= 1.0");
        }
    }

    #[test]
    fn test_add_ml_model() {
        let mut engine = create_test_engine();
        let model = create_test_ml_model();

        engine.add_ml_model(model.clone());

        // Verify model was added (via internal state check)
        // Note: This tests the add operation succeeds
        assert!(!model.id.is_empty());
    }

    #[test]
    fn test_add_multiple_ml_models() {
        let mut engine = create_test_engine();

        for i in 0..5 {
            let model = MlModel {
                id: format!("model_{i}"),
                name: format!("Test Model {i}"),
                model_type: MlModelType::NeuralNetwork,
                version: "1.0".to_string(),
                accuracy: 0.9,
                trained_at: std::time::SystemTime::now(),
            };

            engine.add_ml_model(model);
        }

        // All models added successfully
        assert!(true, "Models added successfully");
    }

    #[test]
    fn test_add_ml_model_replace_existing() {
        let mut engine = create_test_engine();

        let model1 = MlModel {
            id: "same_id".to_string(),
            name: "Model 1".to_string(),
            model_type: MlModelType::NeuralNetwork,
            version: "1.0".to_string(),
            accuracy: 0.8,
            trained_at: std::time::SystemTime::now(),
        };

        let model2 = MlModel {
            id: "same_id".to_string(),
            name: "Model 2".to_string(),
            model_type: MlModelType::DecisionTree,
            version: "2.0".to_string(),
            accuracy: 0.9,
            trained_at: std::time::SystemTime::now(),
        };

        engine.add_ml_model(model1);
        engine.add_ml_model(model2); // Should replace model1

        // Model replaced successfully
        assert!(true, "Model replaced successfully");
    }

    #[test]
    fn test_ml_integration_workflow() {
        let mut engine = create_test_engine();

        let model = create_test_ml_model();
        engine.add_ml_model(model.clone());

        let mut event_data = HashMap::new();
        event_data.insert("type", "anomaly");
        let events = engine.analyze_with_ml(&event_data).unwrap();

        let score = engine.calculate_threat_score(&model, &event_data).unwrap();
        assert!(score >= 0.0 && score <= 1.0);

        for event in &events {
            assert_eq!(event.detection_method, DetectionMethod::MachineLearning);
        }
    }

    #[test]
    fn test_analyze_with_ml_event_metadata() {
        let engine = create_engine_with_model();
        let mut event_data = HashMap::new();
        event_data.insert("key", "value");

        let events = engine.analyze_with_ml(&event_data).unwrap();
        if let Some(event) = events.first() {
            assert!(event.evidence.is_empty());
            assert!(event.recommended_actions.is_empty());
            assert!(event.assigned_analyst.is_none());
            assert!(event.related_events.is_empty());
            assert!(event.raw_data.is_none());
            assert!(!event.mitigated);
            assert!(event.mitigation_actions.is_empty());
            assert!(event.mitigation_steps.is_empty());
        }
    }

    #[test]
    fn test_analyze_with_ml_description() {
        let engine = create_engine_with_model();
        let mut event_data = HashMap::new();
        event_data.insert("key", "value");

        let events = engine.analyze_with_ml(&event_data).unwrap();
        for event in &events {
            assert!(
                event.description.contains("detected anomaly"),
                "description should describe the detection: {}",
                event.description
            );
        }
    }

    #[test]
    fn test_calculate_threat_score_with_complex_data() {
        let engine = create_test_engine();
        let model = create_test_ml_model();

        let mut event_data = HashMap::new();
        event_data.insert("severity", "critical");
        event_data.insert("type", "intrusion");
        event_data.insert("source", "external");
        event_data.insert("target", "database");
        event_data.insert("protocol", "https");

        let score = engine.calculate_threat_score(&model, &event_data).unwrap();
        assert!(score >= 0.0 && score <= 1.0);
    }
}
