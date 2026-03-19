// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for universal adapter components

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::float_cmp, clippy::field_reassign_with_default)]
#![allow(clippy::useless_vec, clippy::print_literal)]

use super::*;
use std::collections::HashMap;

// ===== Events Tests =====

mod events_tests {
    use super::*;

    #[test]
    fn test_event_subscriber_new() {
        let subscriber = EventSubscriber::new(
            "test-subscriber".to_string(),
            vec!["event-type-1".to_string(), "event-type-2".to_string()],
        );
        assert_eq!(subscriber.name, "test-subscriber");
        assert_eq!(subscriber.event_types.len(), 2);
    }

    #[test]
    fn test_event_subscriber_empty_events() {
        let subscriber = EventSubscriber::new("empty-subscriber".to_string(), vec![]);
        assert_eq!(subscriber.name, "empty-subscriber");
        assert!(subscriber.event_types.is_empty());
    }

    #[test]
    fn test_event_subscriber_clone() {
        let subscriber =
            EventSubscriber::new("clone-test".to_string(), vec!["event-a".to_string()]);
        let cloned = subscriber.clone();
        assert_eq!(subscriber.name, cloned.name);
        assert_eq!(subscriber.event_types, cloned.event_types);
    }

    #[test]
    fn test_adapter_event_serialization() {
        let event = AdapterEvent {
            event_id: uuid::Uuid::new_v4(),
            event_type: "test-event".to_string(),
            timestamp: chrono::Utc::now(),
            source: "test-source".to_string(),
            payload: serde_json::json!({"key": "value"}),
            metadata: HashMap::new(),
        };
        let serialized = serde_json::to_string(&event).expect("serialize");
        assert!(serialized.contains("test-event"));
        assert!(serialized.contains("test-source"));

        let deserialized: AdapterEvent = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(event.event_type, deserialized.event_type);
    }

    #[test]
    fn test_adapter_event_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("correlation-id".to_string(), "abc-123".to_string());
        metadata.insert("priority".to_string(), "high".to_string());

        let event = AdapterEvent {
            event_id: uuid::Uuid::new_v4(),
            event_type: "with-metadata".to_string(),
            timestamp: chrono::Utc::now(),
            source: "metadata-source".to_string(),
            payload: serde_json::json!(null),
            metadata,
        };

        assert_eq!(event.metadata.len(), 2);
        assert_eq!(
            event.metadata.get("correlation-id"),
            Some(&"abc-123".to_string())
        );
    }
}

// ===== Metrics Tests =====

mod metrics_tests {
    use super::*;

    #[test]
    fn test_adapter_metrics_default() {
        let metrics = AdapterMetrics::default();
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.successful_requests, 0);
        assert_eq!(metrics.failed_requests, 0);
        assert_eq!(metrics.avg_response_time_ms, 0.0);
    }

    #[test]
    fn test_adapter_metrics_clone() {
        let metrics = AdapterMetrics {
            total_requests: 100,
            successful_requests: 95,
            failed_requests: 5,
            avg_response_time_ms: 50.5,
            connection_metrics: ConnectionMetrics::default(),
        };
        let cloned = metrics.clone();
        assert_eq!(metrics.total_requests, cloned.total_requests);
        assert_eq!(metrics.successful_requests, cloned.successful_requests);
    }

    #[test]
    fn test_adapter_metrics_serialization() {
        let metrics = AdapterMetrics {
            total_requests: 1000,
            successful_requests: 990,
            failed_requests: 10,
            avg_response_time_ms: 25.0,
            connection_metrics: ConnectionMetrics::default(),
        };
        let serialized = serde_json::to_string(&metrics).expect("serialize");
        assert!(serialized.contains("1000"));

        let deserialized: AdapterMetrics = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(metrics.total_requests, deserialized.total_requests);
    }

    #[test]
    fn test_connection_metrics_default() {
        let metrics = ConnectionMetrics::default();
        assert_eq!(metrics.total_connections, 0);
        assert_eq!(metrics.active_connections, 0);
        assert_eq!(metrics.failed_connections, 0);
        assert_eq!(metrics.avg_connection_time_ms, 0.0);
    }

    #[test]
    fn test_connection_metrics_clone() {
        let metrics = ConnectionMetrics {
            total_connections: 500,
            active_connections: 50,
            failed_connections: 10,
            avg_connection_time_ms: 100.0,
        };
        let cloned = metrics.clone();
        assert_eq!(metrics.total_connections, cloned.total_connections);
        assert_eq!(metrics.active_connections, cloned.active_connections);
    }

    #[test]
    fn test_connection_metrics_serialization() {
        let metrics = ConnectionMetrics {
            total_connections: 1000,
            active_connections: 100,
            failed_connections: 5,
            avg_connection_time_ms: 15.5,
        };
        let serialized = serde_json::to_string(&metrics).expect("serialize");

        let deserialized: ConnectionMetrics =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(metrics.active_connections, deserialized.active_connections);
    }
}

// ===== Config Tests =====

mod config_tests {
    use super::*;

    #[test]
    fn test_adapter_config_default() {
        let config = config::UniversalAdapterConfig::default();
        assert!(!config.adapter_id.is_empty());
    }

    #[test]
    fn test_adapter_config_clone() {
        let config = config::UniversalAdapterConfig::default();
        let cloned = config.clone();
        assert_eq!(config.adapter_id, cloned.adapter_id);
    }

    #[test]
    fn test_adapter_config_timeout_values() {
        let config = config::UniversalAdapterConfig::default();
        assert!(config.connection_timeout_ms > 0);
        assert!(config.request_timeout_ms > 0);
    }

    #[test]
    fn test_production_config_default() {
        let config = config::ProductionConfig::default();
        assert!(config.production_mode());
        assert!(config.enhanced_logging());
        assert!(config.metrics_enabled());
        assert!(config.circuit_breaker_enabled());
        assert!(config.rate_limiting_enabled());
    }

    #[test]
    fn test_production_config_clone() {
        let config = config::ProductionConfig::default();
        let cloned = config.clone();
        assert_eq!(config.production_mode(), cloned.production_mode());
    }
}

// ===== Types Tests =====

mod types_tests {
    use super::*;

    #[test]
    fn test_protocol_type_variants() {
        assert_ne!(ProtocolType::Http, ProtocolType::WebSocket);
        assert_ne!(ProtocolType::Grpc, ProtocolType::Mqtt);
        assert_ne!(ProtocolType::Tcp, ProtocolType::Udp);
    }

    #[test]
    fn test_protocol_type_custom() {
        let custom = ProtocolType::Custom("my-protocol".to_string());
        let debug_str = format!("{custom:?}");
        assert!(debug_str.contains("my-protocol"));
    }

    #[test]
    fn test_adapter_operation_variants() {
        assert_ne!(AdapterOperation::Connect, AdapterOperation::Disconnect);
        assert_ne!(AdapterOperation::Request, AdapterOperation::Subscribe);
        assert_ne!(AdapterOperation::HealthCheck, AdapterOperation::Unsubscribe);
    }

    #[test]
    fn test_adapter_operation_custom() {
        let custom = AdapterOperation::Custom("special-op".to_string());
        let debug_str = format!("{custom:?}");
        assert!(debug_str.contains("special-op"));
    }

    #[test]
    fn test_adapter_request_new() {
        let request = AdapterRequest::new(
            AdapterOperation::Connect,
            "http://localhost:8080".to_string(),
        );
        assert_eq!(request.endpoint, "http://localhost:8080");
        assert!(request.headers.is_empty());
        assert!(request.payload.is_none());
        assert!(request.timeout_ms.is_none());
    }

    #[test]
    fn test_adapter_request_builder() {
        let request = AdapterRequest::new(
            AdapterOperation::Request,
            "http://api.example.com".to_string(),
        )
        .with_header("Authorization".to_string(), "Bearer token".to_string())
        .with_payload(serde_json::json!({"data": "value"}))
        .with_metadata("trace-id".to_string(), "123".to_string())
        .with_timeout(5000);

        assert_eq!(
            request.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert!(request.payload.is_some());
        assert_eq!(request.metadata.get("trace-id"), Some(&"123".to_string()));
        assert_eq!(request.timeout_ms, Some(5000));
    }

    #[test]
    fn test_response_status_variants() {
        assert_ne!(ResponseStatus::Success, ResponseStatus::Failed);
        assert_ne!(ResponseStatus::Timeout, ResponseStatus::ServiceUnavailable);
        assert_ne!(
            ResponseStatus::AuthenticationFailed,
            ResponseStatus::AuthorizationFailed
        );
    }

    #[test]
    fn test_connection_status_variants() {
        assert_ne!(ConnectionStatus::Active, ConnectionStatus::Idle);
        assert_ne!(ConnectionStatus::Connecting, ConnectionStatus::Closing);
        assert_ne!(ConnectionStatus::Closed, ConnectionStatus::Failed);
        assert_ne!(ConnectionStatus::Error, ConnectionStatus::Active);
    }

    #[test]
    fn test_connection_info_creation() {
        let conn = ConnectionInfo {
            connection_id: uuid::Uuid::new_v4(),
            endpoint: "http://localhost:8080".to_string(),
            protocol: ProtocolType::Http,
            status: ConnectionStatus::Active,
            established_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        assert_eq!(conn.status, ConnectionStatus::Active);
        assert_eq!(conn.protocol, ProtocolType::Http);
    }

    #[test]
    fn test_service_endpoint_creation() {
        let endpoint = ServiceEndpoint {
            endpoint_id: "svc-001".to_string(),
            url: "http://service.internal:8080".to_string(),
            protocol: ProtocolType::Http,
            health_status: beardog_types::canonical::HealthStatus::Healthy,
            last_health_check: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        assert_eq!(endpoint.endpoint_id, "svc-001");
    }
}
