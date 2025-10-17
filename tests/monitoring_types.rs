// Monitoring Types Tests - Comprehensive Coverage
//
// Tests for monitoring enums and configuration types

use beardog_types::canonical::monitoring::{
    CustomMetricType, FilterOperator, MetricExporter, MonitoringEnvironment,
    NotificationChannelType,
};
use std::collections::HashMap;

// ============================================================================
// MonitoringEnvironment Tests
// ============================================================================

#[test]
fn test_monitoring_environment_development() {
    let env = MonitoringEnvironment::Development;
    assert_eq!(format!("{:?}", env), "Development");
}

#[test]
fn test_monitoring_environment_testing() {
    let env = MonitoringEnvironment::Testing;
    assert_eq!(format!("{:?}", env), "Testing");
}

#[test]
fn test_monitoring_environment_staging() {
    let env = MonitoringEnvironment::Staging;
    assert_eq!(format!("{:?}", env), "Staging");
}

#[test]
fn test_monitoring_environment_production() {
    let env = MonitoringEnvironment::Production;
    assert_eq!(format!("{:?}", env), "Production");
}

#[test]
fn test_monitoring_environment_clone() {
    let env1 = MonitoringEnvironment::Production;
    let env2 = env1.clone();
    assert_eq!(format!("{:?}", env1), format!("{:?}", env2));
}

#[test]
fn test_monitoring_environment_serialization() {
    let env = MonitoringEnvironment::Production;
    let serialized = serde_json::to_string(&env).unwrap();
    assert!(serialized.contains("Production"));

    let deserialized: MonitoringEnvironment = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", env), format!("{:?}", deserialized));
}

#[test]
fn test_all_monitoring_environments() {
    let envs = vec![
        MonitoringEnvironment::Development,
        MonitoringEnvironment::Testing,
        MonitoringEnvironment::Staging,
        MonitoringEnvironment::Production,
    ];

    assert_eq!(envs.len(), 4);
    for env in envs {
        let serialized = serde_json::to_string(&env).unwrap();
        let _deserialized: MonitoringEnvironment = serde_json::from_str(&serialized).unwrap();
    }
}

// ============================================================================
// MetricExporter Tests
// ============================================================================

#[test]
fn test_metric_exporter_prometheus() {
    let exporter = MetricExporter::Prometheus;
    assert_eq!(format!("{:?}", exporter), "Prometheus");
}

#[test]
fn test_metric_exporter_grafana() {
    let exporter = MetricExporter::Grafana;
    assert_eq!(format!("{:?}", exporter), "Grafana");
}

#[test]
fn test_metric_exporter_influxdb() {
    let exporter = MetricExporter::InfluxDB;
    assert_eq!(format!("{:?}", exporter), "InfluxDB");
}

#[test]
fn test_metric_exporter_cloudwatch() {
    let exporter = MetricExporter::CloudWatch;
    assert_eq!(format!("{:?}", exporter), "CloudWatch");
}

#[test]
fn test_metric_exporter_custom() {
    let mut config = HashMap::new();
    config.insert("endpoint".to_string(), serde_json::json!("http://custom"));

    let exporter = MetricExporter::Custom {
        name: "CustomBackend".to_string(),
        config,
    };

    let debug_str = format!("{:?}", exporter);
    assert!(debug_str.contains("Custom"));
    assert!(debug_str.contains("CustomBackend"));
}

#[test]
fn test_metric_exporter_clone() {
    let exporter1 = MetricExporter::Prometheus;
    let exporter2 = exporter1.clone();
    assert_eq!(format!("{:?}", exporter1), format!("{:?}", exporter2));
}

#[test]
fn test_metric_exporter_custom_clone() {
    let mut config = HashMap::new();
    config.insert("key".to_string(), serde_json::json!("value"));

    let exporter1 = MetricExporter::Custom {
        name: "Test".to_string(),
        config: config.clone(),
    };

    let exporter2 = exporter1.clone();
    assert_eq!(format!("{:?}", exporter1), format!("{:?}", exporter2));
}

#[test]
fn test_metric_exporter_serialization() {
    let exporter = MetricExporter::Prometheus;
    let serialized = serde_json::to_string(&exporter).unwrap();
    assert!(serialized.contains("Prometheus"));

    let deserialized: MetricExporter = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", exporter), format!("{:?}", deserialized));
}

#[test]
fn test_metric_exporter_custom_serialization() {
    let mut config = HashMap::new();
    config.insert("url".to_string(), serde_json::json!("http://test"));

    let exporter = MetricExporter::Custom {
        name: "TestExporter".to_string(),
        config,
    };

    let serialized = serde_json::to_string(&exporter).unwrap();
    let deserialized: MetricExporter = serde_json::from_str(&serialized).unwrap();

    match deserialized {
        MetricExporter::Custom { name, .. } => {
            assert_eq!(name, "TestExporter");
        }
        _ => panic!("Expected Custom variant"),
    }
}

// ============================================================================
// CustomMetricType Tests
// ============================================================================

#[test]
fn test_custom_metric_type_counter() {
    let metric_type = CustomMetricType::Counter;
    assert_eq!(format!("{:?}", metric_type), "Counter");
}

#[test]
fn test_custom_metric_type_gauge() {
    let metric_type = CustomMetricType::Gauge;
    assert_eq!(format!("{:?}", metric_type), "Gauge");
}

#[test]
fn test_custom_metric_type_histogram() {
    let metric_type = CustomMetricType::Histogram;
    assert_eq!(format!("{:?}", metric_type), "Histogram");
}

#[test]
fn test_custom_metric_type_summary() {
    let metric_type = CustomMetricType::Summary;
    assert_eq!(format!("{:?}", metric_type), "Summary");
}

#[test]
fn test_custom_metric_type_clone() {
    let metric1 = CustomMetricType::Counter;
    let metric2 = metric1.clone();
    assert_eq!(format!("{:?}", metric1), format!("{:?}", metric2));
}

#[test]
fn test_custom_metric_type_serialization() {
    let metric_type = CustomMetricType::Histogram;
    let serialized = serde_json::to_string(&metric_type).unwrap();
    assert!(serialized.contains("Histogram"));

    let deserialized: CustomMetricType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", metric_type), format!("{:?}", deserialized));
}

#[test]
fn test_all_custom_metric_types() {
    let types = vec![
        CustomMetricType::Counter,
        CustomMetricType::Gauge,
        CustomMetricType::Histogram,
        CustomMetricType::Summary,
    ];

    assert_eq!(types.len(), 4);
    for metric_type in types {
        let serialized = serde_json::to_string(&metric_type).unwrap();
        let _deserialized: CustomMetricType = serde_json::from_str(&serialized).unwrap();
    }
}

// ============================================================================
// NotificationChannelType Tests
// ============================================================================

#[test]
fn test_notification_channel_type_email() {
    let channel = NotificationChannelType::Email;
    assert_eq!(format!("{:?}", channel), "Email");
}

#[test]
fn test_notification_channel_type_slack() {
    let channel = NotificationChannelType::Slack;
    assert_eq!(format!("{:?}", channel), "Slack");
}

#[test]
fn test_notification_channel_type_discord() {
    let channel = NotificationChannelType::Discord;
    assert_eq!(format!("{:?}", channel), "Discord");
}

#[test]
fn test_notification_channel_type_webhook() {
    let channel = NotificationChannelType::Webhook;
    assert_eq!(format!("{:?}", channel), "Webhook");
}

#[test]
fn test_notification_channel_type_custom() {
    let channel = NotificationChannelType::Custom("PagerDuty".to_string());
    let debug_str = format!("{:?}", channel);
    assert!(debug_str.contains("Custom"));
    assert!(debug_str.contains("PagerDuty"));
}

#[test]
fn test_notification_channel_type_clone() {
    let channel1 = NotificationChannelType::Slack;
    let channel2 = channel1.clone();
    assert_eq!(format!("{:?}", channel1), format!("{:?}", channel2));
}

#[test]
fn test_notification_channel_type_serialization() {
    let channel = NotificationChannelType::Discord;
    let serialized = serde_json::to_string(&channel).unwrap();
    assert!(serialized.contains("Discord"));

    let deserialized: NotificationChannelType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", channel), format!("{:?}", deserialized));
}

#[test]
fn test_all_notification_channel_types() {
    let channels = vec![
        NotificationChannelType::Email,
        NotificationChannelType::Slack,
        NotificationChannelType::Discord,
        NotificationChannelType::Webhook,
        NotificationChannelType::Custom("Custom1".to_string()),
    ];

    assert_eq!(channels.len(), 5);
    for channel in channels {
        let serialized = serde_json::to_string(&channel).unwrap();
        let _deserialized: NotificationChannelType = serde_json::from_str(&serialized).unwrap();
    }
}

// ============================================================================
// FilterOperator Tests
// ============================================================================

#[test]
fn test_filter_operator_equals() {
    let op = FilterOperator::Equals;
    assert_eq!(format!("{:?}", op), "Equals");
}

#[test]
fn test_filter_operator_not_equals() {
    let op = FilterOperator::NotEquals;
    assert_eq!(format!("{:?}", op), "NotEquals");
}

#[test]
fn test_filter_operator_contains() {
    let op = FilterOperator::Contains;
    assert_eq!(format!("{:?}", op), "Contains");
}

#[test]
fn test_filter_operator_greater_than() {
    let op = FilterOperator::GreaterThan;
    assert_eq!(format!("{:?}", op), "GreaterThan");
}

#[test]
fn test_filter_operator_less_than() {
    let op = FilterOperator::LessThan;
    assert_eq!(format!("{:?}", op), "LessThan");
}

#[test]
fn test_filter_operator_clone() {
    let op1 = FilterOperator::Equals;
    let op2 = op1.clone();
    assert_eq!(format!("{:?}", op1), format!("{:?}", op2));
}

#[test]
fn test_filter_operator_serialization() {
    let op = FilterOperator::GreaterThan;
    let serialized = serde_json::to_string(&op).unwrap();
    assert!(serialized.contains("GreaterThan"));

    let deserialized: FilterOperator = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", op), format!("{:?}", deserialized));
}

#[test]
fn test_all_filter_operators() {
    let ops = vec![
        FilterOperator::Equals,
        FilterOperator::NotEquals,
        FilterOperator::Contains,
        FilterOperator::GreaterThan,
        FilterOperator::LessThan,
    ];

    assert_eq!(ops.len(), 5);
    for op in ops {
        let serialized = serde_json::to_string(&op).unwrap();
        let _deserialized: FilterOperator = serde_json::from_str(&serialized).unwrap();
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_monitoring_types_together() {
    let env = MonitoringEnvironment::Production;
    let exporter = MetricExporter::Prometheus;
    let metric_type = CustomMetricType::Counter;
    let channel = NotificationChannelType::Slack;
    let operator = FilterOperator::Equals;

    // All should be cloneable
    let _env2 = env.clone();
    let _exporter2 = exporter.clone();
    let _metric2 = metric_type.clone();
    let _channel2 = channel.clone();
    let _operator2 = operator.clone();

    // All should serialize
    let _ = serde_json::to_string(&env).unwrap();
    let _ = serde_json::to_string(&exporter).unwrap();
    let _ = serde_json::to_string(&metric_type).unwrap();
    let _ = serde_json::to_string(&channel).unwrap();
    let _ = serde_json::to_string(&operator).unwrap();
}

#[test]
fn test_metric_exporters_in_vec() {
    let exporters = vec![
        MetricExporter::Prometheus,
        MetricExporter::Grafana,
        MetricExporter::InfluxDB,
        MetricExporter::CloudWatch,
    ];

    assert_eq!(exporters.len(), 4);

    for exporter in &exporters {
        let _ = format!("{:?}", exporter);
    }
}

#[test]
fn test_notification_channels_mixed() {
    let channels = vec![
        NotificationChannelType::Email,
        NotificationChannelType::Slack,
        NotificationChannelType::Discord,
    ];

    for channel in channels {
        let serialized = serde_json::to_string(&channel).unwrap();
        let deserialized: NotificationChannelType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(format!("{:?}", channel), format!("{:?}", deserialized));
    }
}
