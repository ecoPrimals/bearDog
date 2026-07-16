// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::float_assert::near_f64;
use crate::threat::ThreatSeverity;
use chrono::Utc;

#[tokio::test]
async fn test_ml_engine_creation() {
    let engine = MlEngine::new();
    assert_eq!(engine.models.len(), 0);
    assert!(engine.universal_adapter.is_none());
}

#[tokio::test]
async fn test_risk_level_conversion() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let engine = MlEngine::new();

    assert_eq!(engine.score_to_risk_level(0.95), RiskLevel::Critical);
    assert_eq!(engine.score_to_risk_level(0.75), RiskLevel::High);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(engine.score_to_risk_level(0.55), RiskLevel::Medium);
    assert_eq!(engine.score_to_risk_level(0.35), RiskLevel::Low);
    assert_eq!(engine.score_to_risk_level(0.15), RiskLevel::Minimal);
}

#[tokio::test]
async fn test_threat_score_calculation() {
    let engine = MlEngine::new();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let event = SecurityEvent {
        id: "test-1".to_string(),
        event_type: "malware_detected".to_string(),
        severity: ThreatSeverity::Critical,
        timestamp: Utc::now().into(),
        source: "test_source".to_string(),
        description: "Malware detected".to_string(),
        data: std::collections::HashMap::new(),
    };

    let score = engine.calculate_threat_score(&event);
    assert!(score > 0.8); // Should be high for malware + critical
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_local_prediction() {
    let engine = MlEngine::new();

    let mut event_data = std::collections::HashMap::new();
    event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    event_data.insert("user_id".to_string(), "admin".to_string());

    let event = SecurityEvent {
        id: "test-2".to_string(),
        event_type: "authentication_failure".to_string(),
        severity: ThreatSeverity::Medium,
        timestamp: Utc::now().into(),
        source: "login_system".to_string(),
        description: "Authentication failure".to_string(),
        data: event_data,
    };

    let prediction = engine.predict_local(&event);
    assert!(prediction.confidence > 0.0);
    assert!(!prediction.reasoning.is_empty());
    assert_eq!(prediction.model_version, "beardog-local-v1.0");
}

#[test]
fn test_ml_engine_default() {
    let engine = MlEngine::default();
    assert_eq!(engine.models.len(), 0);
}

#[test]
fn test_add_model() {
    let mut engine = MlEngine::new();
    let model = MlModel {
        name: "test-model".to_string(),
        version: "1.0".to_string(),
        model_type: "neural_network".to_string(),
        accuracy: 0.95,
        last_updated: Utc::now(),
    };

    engine.add_model(model);
    assert_eq!(engine.models.len(), 1);
}

#[test]
fn test_add_multiple_models() {
    let mut engine = MlEngine::new();

    for i in 0..5 {
        let model = MlModel {
            name: format!("model-{i}"),
            version: "1.0".to_string(),
            model_type: "test".to_string(),
            accuracy: 0.9,
            last_updated: Utc::now(),
        };
        engine.add_model(model);
    }

    assert_eq!(engine.models.len(), 5);
}

#[tokio::test]
async fn test_predict_threat_with_caching() {
    let engine = MlEngine::new();
    let event = SecurityEvent {
        id: "test-cache".to_string(),
        event_type: "network_scan".to_string(),
        severity: ThreatSeverity::High,
        timestamp: Utc::now().into(),
        source: "scanner".to_string(),
        description: "Network scan detected".to_string(),
        data: std::collections::HashMap::new(),
    };

    let pred1 = engine
        .predict_threat(&event)
        .await
        .expect("predict_threat in test");
    let pred2 = engine
        .predict_threat(&event)
        .await
        .expect("predict_threat in test");

    near_f64(pred1.confidence, pred2.confidence);
    assert_eq!(pred1.risk_level, pred2.risk_level);
}

#[test]
fn test_risk_level_debug() {
    let risk = RiskLevel::Critical;
    let debug_str = format!("{risk:?}");
    assert!(debug_str.contains("Critical"));
}

#[test]
fn test_risk_level_equality() {
    assert_eq!(RiskLevel::Critical, RiskLevel::Critical);
    assert_ne!(RiskLevel::Critical, RiskLevel::High);
    assert_eq!(RiskLevel::Low, RiskLevel::Low);
}

#[test]
fn test_ml_prediction_clone() {
    let pred = MlPrediction {
        confidence: 0.95,
        risk_level: RiskLevel::High,
        reasoning: vec!["test".to_string()],
        model_version: "v1.0".to_string(),
        processing_time_ms: 100,
    };

    let cloned = pred.clone();
    near_f64(pred.confidence, cloned.confidence);
    assert_eq!(pred.risk_level, cloned.risk_level);
}

#[test]
fn test_ml_model_clone() {
    let model = MlModel {
        name: "test".to_string(),
        version: "1.0".to_string(),
        model_type: "nn".to_string(),
        accuracy: 0.9,
        last_updated: Utc::now(),
    };

    let cloned = model.clone();
    assert_eq!(model.name, cloned.name);
    assert_eq!(model.version, cloned.version);
}

#[test]
fn test_calculate_threat_score_various_events() {
    let engine = MlEngine::new();

    let test_cases = vec![
        ("authentication_failure", "critical", 0.8),
        ("network_scan", "high", 0.8),
        ("malware_detected", "critical", 1.0),
        ("data_exfiltration", "critical", 1.0),
        ("privilege_escalation", "high", 0.9),
        ("suspicious_process", "medium", 0.5),
        ("unknown_event", "low", 0.3),
    ];

    for (event_type, severity, min_score) in test_cases {
        let event = SecurityEvent {
            id: "test".to_string(),
            event_type: event_type.to_string(),
            severity: match severity {
                "critical" => ThreatSeverity::Critical,
                "high" => ThreatSeverity::High,
                "medium" => ThreatSeverity::Medium,
                _ => ThreatSeverity::Low,
            },
            timestamp: Utc::now().into(),
            source: "test".to_string(),
            description: "test".to_string(),
            data: std::collections::HashMap::new(),
        };

        let score = engine.calculate_threat_score(&event);
        assert!(
            score >= min_score,
            "Event {event_type} should have score >= {min_score}"
        );
    }
}

#[test]
fn test_score_to_risk_level_boundaries() {
    let engine = MlEngine::new();

    assert_eq!(engine.score_to_risk_level(1.0), RiskLevel::Critical);
    assert_eq!(engine.score_to_risk_level(0.9), RiskLevel::Critical);
    assert_eq!(engine.score_to_risk_level(0.89), RiskLevel::High);
    assert_eq!(engine.score_to_risk_level(0.7), RiskLevel::High);
    assert_eq!(engine.score_to_risk_level(0.69), RiskLevel::Medium);
    assert_eq!(engine.score_to_risk_level(0.5), RiskLevel::Medium);
    assert_eq!(engine.score_to_risk_level(0.49), RiskLevel::Low);
    assert_eq!(engine.score_to_risk_level(0.3), RiskLevel::Low);
    assert_eq!(engine.score_to_risk_level(0.29), RiskLevel::Minimal);
    assert_eq!(engine.score_to_risk_level(0.0), RiskLevel::Minimal);
}

#[tokio::test]
async fn test_predict_threat_various_severities() {
    let engine = MlEngine::new();

    for severity in &[
        ThreatSeverity::Critical,
        ThreatSeverity::High,
        ThreatSeverity::Medium,
        ThreatSeverity::Low,
    ] {
        let event = SecurityEvent {
            id: "test".to_string(),
            event_type: "test_event".to_string(),
            severity: severity.clone(),
            timestamp: Utc::now().into(),
            source: "test".to_string(),
            description: "test".to_string(),
            data: std::collections::HashMap::new(),
        };

        let result = engine.predict_threat(&event).await;
        assert!(result.is_ok());
    }
}

#[test]
fn test_ml_prediction_debug() {
    let pred = MlPrediction {
        confidence: 0.8,
        risk_level: RiskLevel::Medium,
        reasoning: vec!["test".to_string()],
        model_version: "v1".to_string(),
        processing_time_ms: 50,
    };

    let debug_str = format!("{pred:?}");
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("MlPrediction"));
}

#[test]
fn test_ml_model_debug() {
    let model = MlModel {
        name: "test".to_string(),
        version: "1.0".to_string(),
        model_type: "nn".to_string(),
        accuracy: 0.9,
        last_updated: Utc::now(),
    };

    let debug_str = format!("{model:?}");
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("MlModel"));
}

#[tokio::test]
async fn test_predict_with_data_exfiltration() {
    let engine = MlEngine::new();
    let event = SecurityEvent {
        id: "exfil-1".to_string(),
        event_type: "data_exfiltration".to_string(),
        severity: ThreatSeverity::Critical,
        timestamp: Utc::now().into(),
        source: "network_monitor".to_string(),
        description: "Suspicious data transfer".to_string(),
        data: std::collections::HashMap::new(),
    };

    let pred = engine
        .predict_threat(&event)
        .await
        .expect("predict_threat in test");
    assert!(matches!(
        pred.risk_level,
        RiskLevel::Critical | RiskLevel::High
    ));
}

#[tokio::test]
async fn test_predict_with_privilege_escalation() {
    let engine = MlEngine::new();
    let event = SecurityEvent {
        id: "priv-1".to_string(),
        event_type: "privilege_escalation".to_string(),
        severity: ThreatSeverity::High,
        timestamp: Utc::now().into(),
        source: "access_monitor".to_string(),
        description: "Unauthorized privilege change".to_string(),
        data: std::collections::HashMap::new(),
    };

    let pred = engine
        .predict_threat(&event)
        .await
        .expect("predict_threat in test");
    assert!(matches!(
        pred.risk_level,
        RiskLevel::High | RiskLevel::Critical
    ));
}

#[test]
fn test_calculate_threat_score_caps_at_one() {
    let engine = MlEngine::new();
    let event = SecurityEvent {
        id: "max-test".to_string(),
        event_type: "data_exfiltration".to_string(),
        severity: ThreatSeverity::Critical,
        timestamp: Utc::now().into(),
        source: "test".to_string(),
        description: "Maximum threat".to_string(),
        data: std::collections::HashMap::new(),
    };

    let score = engine.calculate_threat_score(&event);
    assert!(score <= 1.0);
}

#[tokio::test]
async fn test_prediction_includes_reasoning() {
    let engine = MlEngine::new();
    let event = SecurityEvent {
        id: "reason-test".to_string(),
        event_type: "malware_detected".to_string(),
        severity: ThreatSeverity::Critical,
        timestamp: Utc::now().into(),
        source: "av_scanner".to_string(),
        description: "Malware found".to_string(),
        data: std::collections::HashMap::new(),
    };

    let pred = engine
        .predict_threat(&event)
        .await
        .expect("predict_threat in test");
    assert!(!pred.reasoning.is_empty());
    assert!(pred.reasoning[0].contains("Local heuristic analysis"));
}

#[tokio::test]
async fn test_prediction_has_processing_time() {
    let engine = MlEngine::new();
    let event = SecurityEvent {
        id: "time-test".to_string(),
        event_type: "test".to_string(),
        severity: ThreatSeverity::Medium,
        timestamp: Utc::now().into(),
        source: "test".to_string(),
        description: "test".to_string(),
        data: std::collections::HashMap::new(),
    };

    let pred = engine
        .predict_threat(&event)
        .await
        .expect("predict_threat in test");
    assert!(pred.processing_time_ms >= 0);
}

#[test]
fn test_add_model_replaces_existing() {
    let mut engine = MlEngine::new();

    let model1 = MlModel {
        name: "same-name".to_string(),
        version: "1.0".to_string(),
        model_type: "old".to_string(),
        accuracy: 0.8,
        last_updated: Utc::now(),
    };

    let model2 = MlModel {
        name: "same-name".to_string(),
        version: "2.0".to_string(),
        model_type: "new".to_string(),
        accuracy: 0.9,
        last_updated: Utc::now(),
    };

    engine.add_model(model1);
    engine.add_model(model2);

    assert_eq!(engine.models.len(), 1);
}
