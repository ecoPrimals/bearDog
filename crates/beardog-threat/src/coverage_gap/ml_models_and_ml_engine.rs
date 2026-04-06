// SPDX-License-Identifier: AGPL-3.0-or-later

// ============================================================================
// types/engine/ml_models.rs - 37 uncovered lines, 83.41% coverage
// ============================================================================

#[cfg(test)]
mod ml_models_gap_tests {
    use crate::threat::types::engine::ml_models::{
        ConfusionMatrix, MlModel, MlModelType, ModelPerformanceMetrics, ModelPrediction,
        PredictionValue,
    };

    #[test]
    fn test_model_summary() {
        let model = MlModel::new(
            "Summary Test",
            "desc",
            MlModelType::AnomalyDetection,
            vec!["f1".to_string(), "f2".to_string()],
        );
        let summary = model.summary();
        assert_eq!(summary.name, "Summary Test");
        assert_eq!(summary.model_type, MlModelType::AnomalyDetection);
        assert_eq!(summary.feature_count, 2);
        assert!(!summary.is_active);
    }

    #[test]
    fn test_model_get_age_days() {
        let model = MlModel::new("Age Test", "desc", MlModelType::Classification, vec![]);
        assert_eq!(model.get_age_days(), 0);
    }

    #[test]
    fn test_model_update_training_timestamp() {
        let mut model = MlModel::new("Train Test", "desc", MlModelType::Regression, vec![]);
        let before = model.last_trained;
        model.update_training_timestamp();
        assert!(model.last_trained >= before);
    }

    #[test]
    fn test_model_add_metadata() {
        let mut model = MlModel::new("Meta Test", "desc", MlModelType::Clustering, vec![]);
        model.add_metadata("framework", "pytorch");
        model.add_metadata("gpu", "A100");
        assert_eq!(
            model.metadata.get("framework"),
            Some(&"pytorch".to_string())
        );
        assert_eq!(model.metadata.len(), 2);
    }

    #[test]
    fn test_confusion_matrix_all_zeros() {
        let cm = ConfusionMatrix::default();
        assert_eq!(cm.accuracy(), 0.0);
        assert_eq!(cm.precision(), 0.0);
        assert_eq!(cm.recall(), 0.0);
        assert_eq!(cm.f1_score(), 0.0);
    }

    #[test]
    fn test_confusion_matrix_perfect() {
        let cm = ConfusionMatrix {
            true_positives: 100,
            true_negatives: 100,
            false_positives: 0,
            false_negatives: 0,
        };
        assert_eq!(cm.accuracy(), 1.0);
        assert_eq!(cm.precision(), 1.0);
        assert_eq!(cm.recall(), 1.0);
        assert_eq!(cm.f1_score(), 1.0);
    }

    #[test]
    fn test_prediction_value_variants() {
        let _ = PredictionValue::Binary(true);
        let _ = PredictionValue::Class("threat".to_string());
        let _ = PredictionValue::Numeric(0.95);
        let _ = PredictionValue::AnomalyScore(0.8);
        let _ = PredictionValue::Custom(serde_json::json!({"key": "value"}));

        let mut probs = std::collections::HashMap::new();
        probs.insert("class_a".to_string(), 0.7);
        let _ = PredictionValue::Probability(probs);
    }

    #[test]
    fn test_model_prediction_low_confidence() {
        let pred = ModelPrediction::new("m1", PredictionValue::Binary(false), 0.3);
        assert!(!pred.is_high_confidence());
    }

    #[test]
    fn test_model_prediction_add_feature_importance() {
        let mut pred = ModelPrediction::new("m1", PredictionValue::Binary(true), 0.9);
        pred.add_feature_importance("ip_address", 0.8);
        pred.add_feature_importance("user_agent", 0.6);
        assert_eq!(pred.feature_importance.len(), 2);
        assert_eq!(pred.feature_importance.get("ip_address"), Some(&0.8));
    }

    #[test]
    fn test_ml_model_type_display_all() {
        assert_eq!(MlModelType::Classification.to_string(), "Classification");
        assert_eq!(MlModelType::Regression.to_string(), "Regression");
        assert_eq!(MlModelType::Clustering.to_string(), "Clustering");
        assert_eq!(
            MlModelType::AnomalyDetection.to_string(),
            "Anomaly Detection"
        );
        assert_eq!(
            MlModelType::NaturalLanguageProcessing.to_string(),
            "Natural Language Processing"
        );
        assert_eq!(MlModelType::TimeSeries.to_string(), "Time Series");
        assert_eq!(MlModelType::DeepLearning.to_string(), "Deep Learning");
        assert_eq!(MlModelType::Ensemble.to_string(), "Ensemble");
        assert_eq!(
            MlModelType::Custom("BERT".to_string()).to_string(),
            "Custom: BERT"
        );
    }

    #[test]
    fn test_model_performance_metrics_default() {
        let metrics = ModelPerformanceMetrics::default();
        assert_eq!(metrics.accuracy, 0.0);
        assert_eq!(metrics.precision, 0.0);
        assert_eq!(metrics.recall, 0.0);
        assert_eq!(metrics.f1_score, 0.0);
        assert_eq!(metrics.specificity, 0.0);
        assert_eq!(metrics.sensitivity, 0.0);
        assert_eq!(metrics.auc_roc, 0.0);
        assert_eq!(metrics.training_time_ms, 0);
        assert_eq!(metrics.inference_time_ms, 0);
        assert_eq!(metrics.model_size_bytes, 0);
    }

    #[test]
    fn test_model_needs_retraining_high_accuracy() {
        let mut model = MlModel::new("Test", "desc", MlModelType::Classification, vec![]);
        model.update_accuracy(0.95);
        assert!(!model.needs_retraining());
    }
}
// ============================================================================
// ml_engine.rs - 77 uncovered lines, 84.57% coverage
// ============================================================================

#[cfg(test)]
mod ml_engine_gap_tests {
    use crate::threat::ml_engine::{MlEngine, MlModel, UniversalComputeAdapter};
    use beardog_errors::BearDogError;
    use std::future::Future;
    use std::pin::Pin;

    struct MockAdapter;

    impl UniversalComputeAdapter for MockAdapter {
        fn request_compute(
            &self,
            _endpoint: &str,
            _request: &serde_json::Value,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, BearDogError>> + Send + '_>>
        {
            Box::pin(async {
                Ok(serde_json::json!({
                    "confidence": 0.92,
                    "risk_score": 0.85,
                    "reasoning": ["Network ML analysis", "Pattern matched"]
                }))
            })
        }
    }

    struct FailingAdapter;

    impl UniversalComputeAdapter for FailingAdapter {
        fn request_compute(
            &self,
            _endpoint: &str,
            _request: &serde_json::Value,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, BearDogError>> + Send + '_>>
        {
            Box::pin(async { Err(BearDogError::unavailable("ML service unavailable".into())) })
        }
    }

    #[test]
    fn test_get_stats() {
        let mut engine = MlEngine::new();
        let model = MlModel {
            name: "test-model".to_string(),
            version: "1.0".to_string(),
            model_type: "neural_network".to_string(),
            accuracy: 0.95,
            last_updated: chrono::Utc::now(),
        };
        engine.add_model(model);

        let stats = engine.get_stats();
        assert_eq!(stats.local_predictions, 0);
        assert_eq!(stats.network_predictions, 0);
        assert_eq!(stats.models_loaded, 1);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let engine = MlEngine::new();

        // Predict something to populate cache
        let event = crate::threat::types::SecurityEvent {
            id: "cache-test".to_string(),
            event_type: "test_event".to_string(),
            severity: crate::threat::types::ThreatSeverity::Low,
            timestamp: chrono::Utc::now().into(),
            source: "test".to_string(),
            description: "test".to_string(),
            data: std::collections::HashMap::new(),
        };
        engine
            .predict_threat(&event)
            .await
            .expect("predict threat for cache test");

        // Clear the cache - exercises the code path
        engine.clear_cache().await;

        // After clearing, a new prediction should still work
        let pred = engine
            .predict_threat(&event)
            .await
            .expect("predict threat after cache clear");
        assert!(pred.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_predict_via_universal_adapter_success() {
        let engine = MlEngine::new().with_universal_adapter(Box::new(MockAdapter));

        let event = crate::threat::types::SecurityEvent {
            id: "adapter-test".to_string(),
            event_type: "network_scan".to_string(),
            severity: crate::threat::types::ThreatSeverity::High,
            timestamp: chrono::Utc::now().into(),
            source: "scanner".to_string(),
            description: "scan detected".to_string(),
            data: std::collections::HashMap::new(),
        };

        let prediction = engine
            .predict_threat(&event)
            .await
            .expect("predict threat via universal adapter");
        assert_eq!(prediction.confidence, 0.92);
        assert_eq!(prediction.model_version, "beardog-network-v2.0");
        assert!(prediction.reasoning.iter().any(|r| r.contains("Network")));
    }

    #[tokio::test]
    async fn test_predict_via_universal_adapter_fallback() {
        let engine = MlEngine::new().with_universal_adapter(Box::new(FailingAdapter));

        let event = crate::threat::types::SecurityEvent {
            id: "fallback-test".to_string(),
            event_type: "auth_failure".to_string(),
            severity: crate::threat::types::ThreatSeverity::Medium,
            timestamp: chrono::Utc::now().into(),
            source: "auth".to_string(),
            description: "auth failed".to_string(),
            data: std::collections::HashMap::new(),
        };

        // Should fallback to local prediction
        let prediction = engine
            .predict_threat(&event)
            .await
            .expect("predict threat local fallback");
        assert_eq!(prediction.model_version, "beardog-local-v1.0");
        assert_eq!(prediction.confidence, 0.75);
    }
}
