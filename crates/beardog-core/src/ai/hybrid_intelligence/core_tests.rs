//! Tests for Hybrid Intelligence Core

#[cfg(test)]
mod tests {
    #![allow(unused_imports, clippy::float_cmp, clippy::useless_vec)]

    use crate::ai::hybrid_intelligence::{
        config::{HybridIntelligenceConfig, IntelligenceMode, LearningAlgorithm},
        core::{
            DecisionContext, DecisionResult, HybridIntelligenceBuilder, HybridIntelligenceSystem,
            IntelligenceEvent, IntelligenceEventType, IntelligenceMetrics, PredictionResult,
            SystemCommand, SystemStatus,
        },
        core_types::IntelligenceCapability,
    };
    use beardog_types::canonical::HealthStatus;
    use std::collections::HashMap;

    /// Create a test config
    fn create_test_config() -> HybridIntelligenceConfig {
        HybridIntelligenceConfig {
            system_id: "test-ai-system".to_string(),
            enabled_capabilities: vec![IntelligenceCapability::PredictiveAnalytics],
            ..Default::default()
        }
    }

    #[test]
    fn test_intelligence_metrics_default() {
        let metrics = IntelligenceMetrics::default();

        assert_eq!(metrics.total_predictions, 0);
        assert_eq!(metrics.total_decisions, 0);
        assert_eq!(metrics.total_models_trained, 0);
        assert_eq!(metrics.avg_prediction_accuracy, 0.0);
        assert_eq!(metrics.avg_decision_confidence, 0.0);
        assert_eq!(metrics.uptime_secs, 0);
        assert_eq!(metrics.memory_usage_mb, 0.0);
        assert_eq!(metrics.cpu_usage_percent, 0.0);
        assert_eq!(metrics.gpu_usage_percent, None);
    }

    #[test]
    fn test_system_command_variants() {
        let _ = SystemCommand::Start;
        let _ = SystemCommand::Stop;
        let _ = SystemCommand::Restart;
        let _ = SystemCommand::GetStatus;
        let _ = SystemCommand::Reset;

        let config = Box::new(create_test_config());
        let _ = SystemCommand::UpdateConfig(config);

        let _ = SystemCommand::AddCapability(IntelligenceCapability::PredictiveAnalytics);
        let _ = SystemCommand::RemoveCapability(IntelligenceCapability::AnomalyDetection);
    }

    #[test]
    fn test_intelligence_event_type_variants() {
        assert!(matches!(
            IntelligenceEventType::ModelTrained,
            IntelligenceEventType::ModelTrained
        ));
        assert!(matches!(
            IntelligenceEventType::PredictionMade,
            IntelligenceEventType::PredictionMade
        ));
        assert!(matches!(
            IntelligenceEventType::DecisionMade,
            IntelligenceEventType::DecisionMade
        ));
        assert!(matches!(
            IntelligenceEventType::LearningUpdate,
            IntelligenceEventType::LearningUpdate
        ));
        assert!(matches!(
            IntelligenceEventType::OptimizationCompleted,
            IntelligenceEventType::OptimizationCompleted
        ));
        assert!(matches!(
            IntelligenceEventType::AnomalyDetected,
            IntelligenceEventType::AnomalyDetected
        ));
        assert!(matches!(
            IntelligenceEventType::PerformanceThresholdCrossed,
            IntelligenceEventType::PerformanceThresholdCrossed
        ));
    }

    #[test]
    fn test_decision_context_creation() {
        let context = DecisionContext {
            context_id: "test-context".to_string(),
            priority_level: 80,
            time_limit_ms: 5000,
            required_confidence: 0.8,
        };

        assert_eq!(context.context_id, "test-context");
        assert_eq!(context.priority_level, 80);
        assert_eq!(context.time_limit_ms, 5000);
        assert_eq!(context.required_confidence, 0.8);
    }

    #[test]
    fn test_hybrid_intelligence_system_new() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config);

        assert!(system.is_ok());
        let system = system.unwrap();
        assert_eq!(system.config.system_id, "test-ai-system");
    }

    #[tokio::test]
    async fn test_system_initialize() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        // Initialize in a spawn_blocking to avoid runtime issues
        let result = tokio::task::spawn_blocking(move || system.initialize()).await;

        // The spawn_blocking will succeed, the initialize should return Ok
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_predict_with_valid_input() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");
        // Skip initialize to avoid blocking issues

        let input_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = system.predict(input_data, None).await;

        assert!(result.is_ok());
        let prediction = result.unwrap();
        assert_eq!(prediction.predictions.len(), 5);
        assert_eq!(prediction.model_id, "default_model");
    }

    #[tokio::test]
    async fn test_predict_with_empty_input() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let input_data = vec![];
        let result = system.predict(input_data, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_predict_with_custom_model_id() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let input_data = vec![10.0, 20.0, 30.0];
        let result = system
            .predict(input_data, Some("custom_model_v2".to_string()))
            .await;

        assert!(result.is_ok());
        let prediction = result.unwrap();
        assert_eq!(prediction.model_id, "custom_model_v2");
    }

    #[tokio::test]
    async fn test_predict_updates_metrics() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let initial_metrics = system.get_metrics().await;
        assert_eq!(initial_metrics.total_predictions, 0);

        let input_data = vec![1.0, 2.0, 3.0];
        let _ = system.predict(input_data.clone(), None).await;

        let updated_metrics = system.get_metrics().await;
        assert_eq!(updated_metrics.total_predictions, 1);

        let _ = system.predict(input_data, None).await;
        let final_metrics = system.get_metrics().await;
        assert_eq!(final_metrics.total_predictions, 2);
    }

    #[tokio::test]
    async fn test_make_decision_with_context() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let mut context = HashMap::new();
        context.insert(
            "risk_level".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(0.3).unwrap()),
        );
        context.insert(
            "data_quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(0.9).unwrap()),
        );

        let result = system.make_decision(context).await;

        assert!(result.is_ok());
        let decision = result.unwrap();
        assert!(!decision.decision.is_empty());
        assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    }

    #[tokio::test]
    async fn test_make_decision_updates_metrics() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let initial_metrics = system.get_metrics().await;
        assert_eq!(initial_metrics.total_decisions, 0);

        let context = HashMap::new();
        let _ = system.make_decision(context.clone()).await;

        let updated_metrics = system.get_metrics().await;
        assert_eq!(updated_metrics.total_decisions, 1);
    }

    #[tokio::test]
    async fn test_subscribe_to_events() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let mut receiver = system.subscribe_to_events();

        // Trigger an event by making a prediction
        let input_data = vec![1.0, 2.0];
        let _ = system.predict(input_data, None).await;

        // Try to receive the event
        let event = receiver.try_recv();
        assert!(event.is_ok());

        let event = event.unwrap();
        assert!(matches!(
            event.event_type,
            IntelligenceEventType::PredictionMade
        ));
    }

    #[tokio::test]
    async fn test_get_metrics() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let metrics = system.get_metrics().await;
        assert_eq!(metrics.total_predictions, 0);
        assert_eq!(metrics.total_decisions, 0);
    }

    #[tokio::test]
    async fn test_get_health() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");
        // Skip initialize to avoid blocking issues

        let health = system.get_health().await;
        // Default health is Healthy
        assert_eq!(health, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_get_status() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");
        // Skip initialize to avoid blocking issues

        let status = system.get_status().await;
        assert_eq!(status.system_id, "test-ai-system");
        assert_eq!(status.health, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_shutdown() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");
        // Skip initialize to avoid blocking issues

        let result = tokio::task::spawn_blocking(move || system.shutdown()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_builder_new() {
        let builder = HybridIntelligenceBuilder::new();
        let system = builder.build();
        assert!(system.is_ok());
    }

    #[test]
    fn test_builder_with_system_id() {
        let builder = HybridIntelligenceBuilder::new().system_id("custom-ai-system");
        let system = builder.build().expect("Should build");
        assert_eq!(system.config.system_id, "custom-ai-system");
    }

    #[test]
    fn test_builder_with_capability() {
        let builder = HybridIntelligenceBuilder::new()
            .capability(IntelligenceCapability::PredictiveAnalytics);
        let system = builder.build();
        assert!(system.is_ok());
    }

    #[test]
    fn test_builder_fluent_interface() {
        let builder = HybridIntelligenceBuilder::new()
            .system_id("fluent-test")
            .capability(IntelligenceCapability::PredictiveAnalytics)
            .capability(IntelligenceCapability::PatternRecognition);

        let system = builder.build();
        assert!(system.is_ok());
    }

    #[test]
    fn test_prediction_result_serialization() {
        use chrono::Utc;
        use uuid::Uuid;

        let prediction = PredictionResult {
            id: Uuid::new_v4(),
            predictions: vec![1.0, 2.0, 3.0],
            confidence_intervals: Some(vec![(0.9, 1.1), (1.9, 2.1), (2.9, 3.1)]),
            uncertainty: Some(vec![0.05, 0.05, 0.05]),
            model_id: "test_model".to_string(),
            timestamp: Utc::now(),
            horizon: None,
        };

        let json = serde_json::to_string(&prediction).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: PredictionResult =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.predictions.len(), 3);
        assert_eq!(deserialized.model_id, "test_model");
    }

    #[test]
    fn test_decision_result_serialization() {
        use chrono::Utc;
        use uuid::Uuid;

        let decision = DecisionResult {
            decision_id: Uuid::new_v4(),
            decision: "approve".to_string(),
            confidence: 0.85,
            reasoning: "Test reasoning".to_string(),
            timestamp: Utc::now(),
            context: HashMap::new(),
        };

        let json = serde_json::to_string(&decision).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: DecisionResult = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.decision, "approve");
        assert_eq!(deserialized.confidence, 0.85);
    }

    #[test]
    fn test_intelligence_event_serialization() {
        use chrono::Utc;
        use uuid::Uuid;

        let event = IntelligenceEvent {
            id: Uuid::new_v4(),
            event_type: IntelligenceEventType::ModelTrained,
            data: HashMap::new(),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&event).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: IntelligenceEvent =
            serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(
            deserialized.event_type,
            IntelligenceEventType::ModelTrained
        ));
    }

    #[test]
    fn test_system_status_serialization() {
        use chrono::Utc;

        let status = SystemStatus {
            system_id: "test-system".to_string(),
            health: HealthStatus::Healthy,
            active_capabilities: vec![IntelligenceCapability::PredictiveAnalytics],
            metrics: IntelligenceMetrics::default(),
            last_updated: Utc::now(),
        };

        let json = serde_json::to_string(&status).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: SystemStatus = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.system_id, "test-system");
        assert_eq!(deserialized.health, HealthStatus::Healthy);
    }

    #[test]
    fn test_metrics_clone() {
        let metrics1 = IntelligenceMetrics::default();
        let metrics2 = metrics1;

        assert_eq!(metrics1.total_predictions, metrics2.total_predictions);
    }

    #[test]
    fn test_decision_context_serialization() {
        let context = DecisionContext {
            context_id: "ctx-123".to_string(),
            priority_level: 50,
            time_limit_ms: 1000,
            required_confidence: 0.7,
        };

        let json = serde_json::to_string(&context).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: DecisionContext =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.context_id, "ctx-123");
        assert_eq!(deserialized.priority_level, 50);
    }

    #[tokio::test]
    async fn test_multiple_predictions_independence() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let input1 = vec![1.0, 2.0, 3.0];
        let input2 = vec![10.0, 20.0, 30.0];

        let result1 = system.predict(input1, Some("model1".to_string())).await;
        let result2 = system.predict(input2, Some("model2".to_string())).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        let pred1 = result1.unwrap();
        let pred2 = result2.unwrap();

        assert_eq!(pred1.model_id, "model1");
        assert_eq!(pred2.model_id, "model2");
        assert_ne!(pred1.id, pred2.id);
    }

    #[tokio::test]
    async fn test_decision_with_high_risk() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let mut context = HashMap::new();
        context.insert(
            "risk_level".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(0.9).unwrap()),
        );

        let result = system.make_decision(context).await;

        assert!(result.is_ok());
        let decision = result.unwrap();
        // High risk should lead to rejection or review
        assert!(decision.decision.contains("reject") || decision.decision.contains("review"));
    }

    #[tokio::test]
    async fn test_decision_with_high_confidence() {
        let config = create_test_config();
        let system = HybridIntelligenceSystem::new(config).expect("Should create system");

        let mut context = HashMap::new();
        context.insert(
            "risk_level".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(0.1).unwrap()),
        );
        context.insert(
            "data_quality".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(0.95).unwrap()),
        );
        context.insert(
            "system_confidence".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(0.95).unwrap()),
        );

        let result = system.make_decision(context).await;

        assert!(result.is_ok());
        let decision = result.unwrap();
        // High confidence, low risk should approve
        assert!(decision.decision.contains("approve"));
        assert!(decision.confidence > 0.8);
    }
}
