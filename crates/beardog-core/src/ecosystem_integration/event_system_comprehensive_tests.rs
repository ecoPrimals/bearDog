//! Comprehensive tests for Event System
//!
//! Added December 8, 2025 for 90% coverage milestone
//! Targets: `AdapterEvent`, `EventSubscriber`, `EcosystemEvent`, `EventPriority`, `EventStatus`

#[cfg(test)]
mod tests {
    use crate::ecosystem_integration::types::{
        EcosystemEvent, EventPriority, EventStatus, NodeType,
    };
    use crate::ecosystem_integration::universal_adapter::events::{AdapterEvent, EventSubscriber};
    use chrono::Utc;
    use std::collections::HashMap;
    use uuid::Uuid;

    // ============================================================================
    // EventSubscriber Tests
    // ============================================================================

    #[test]
    fn test_event_subscriber_new() {
        let subscriber = EventSubscriber::new(
            "test-subscriber".to_string(),
            vec!["event.type.1".to_string(), "event.type.2".to_string()],
        );

        assert_eq!(subscriber.name, "test-subscriber");
        assert_eq!(subscriber.event_types.len(), 2);
        assert!(subscriber.event_types.contains(&"event.type.1".to_string()));
        assert!(subscriber.event_types.contains(&"event.type.2".to_string()));
    }

    #[test]
    fn test_event_subscriber_unique_ids() {
        let sub1 = EventSubscriber::new("sub1".to_string(), vec![]);
        let sub2 = EventSubscriber::new("sub2".to_string(), vec![]);

        assert_ne!(sub1.id, sub2.id);
    }

    #[test]
    fn test_event_subscriber_clone() {
        let sub1 = EventSubscriber::new("test".to_string(), vec!["type1".to_string()]);
        let sub2 = sub1.clone();

        assert_eq!(sub1.id, sub2.id);
        assert_eq!(sub1.name, sub2.name);
        assert_eq!(sub1.event_types, sub2.event_types);
    }

    #[test]
    fn test_event_subscriber_empty_event_types() {
        let subscriber = EventSubscriber::new("test".to_string(), vec![]);

        assert!(subscriber.event_types.is_empty());
    }

    #[test]
    fn test_event_subscriber_multiple_event_types() {
        let types = vec![
            "service.started".to_string(),
            "service.stopped".to_string(),
            "service.error".to_string(),
            "service.health".to_string(),
        ];
        let subscriber = EventSubscriber::new("monitor".to_string(), types.clone());

        assert_eq!(subscriber.event_types.len(), 4);
        assert_eq!(subscriber.event_types, types);
    }

    // ============================================================================
    // AdapterEvent Tests
    // ============================================================================

    #[test]
    fn test_adapter_event_creation() {
        let event = AdapterEvent {
            event_id: Uuid::new_v4(),
            event_type: "test.event".to_string(),
            timestamp: Utc::now(),
            source: "test-source".to_string(),
            payload: serde_json::json!({"key": "value"}),
            metadata: HashMap::new(),
        };

        assert_eq!(event.event_type, "test.event");
        assert_eq!(event.source, "test-source");
        assert!(event.metadata.is_empty());
    }

    #[test]
    fn test_adapter_event_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());
        metadata.insert("environment".to_string(), "production".to_string());

        let event = AdapterEvent {
            event_id: Uuid::new_v4(),
            event_type: "deployment".to_string(),
            timestamp: Utc::now(),
            source: "ci-cd".to_string(),
            payload: serde_json::json!({}),
            metadata: metadata.clone(),
        };

        assert_eq!(event.metadata.len(), 2);
        assert_eq!(event.metadata.get("version"), Some(&"1.0".to_string()));
        assert_eq!(
            event.metadata.get("environment"),
            Some(&"production".to_string())
        );
    }

    #[test]
    fn test_adapter_event_serialization() {
        let event = AdapterEvent {
            event_id: Uuid::new_v4(),
            event_type: "test".to_string(),
            timestamp: Utc::now(),
            source: "source".to_string(),
            payload: serde_json::json!({"data": "test"}),
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string(&event).expect("Should serialize");
        let deserialized: AdapterEvent = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.source, deserialized.source);
    }

    #[test]
    fn test_adapter_event_clone() {
        let event1 = AdapterEvent {
            event_id: Uuid::new_v4(),
            event_type: "test".to_string(),
            timestamp: Utc::now(),
            source: "source".to_string(),
            payload: serde_json::json!({}),
            metadata: HashMap::new(),
        };

        let event2 = event1.clone();

        assert_eq!(event1.event_id, event2.event_id);
        assert_eq!(event1.event_type, event2.event_type);
    }

    #[test]
    fn test_adapter_event_complex_payload() {
        let complex_payload = serde_json::json!({
            "user": {
                "id": 123,
                "name": "Test User",
                "roles": ["admin", "user"]
            },
            "action": "login",
            "timestamp": 1_234_567_890,
            "metadata": {
                "ip": "192.168.1.1",
                "user_agent": "Test Browser"
            }
        });

        let event = AdapterEvent {
            event_id: Uuid::new_v4(),
            event_type: "user.login".to_string(),
            timestamp: Utc::now(),
            source: "auth-service".to_string(),
            payload: complex_payload,
            metadata: HashMap::new(),
        };

        assert!(event.payload.get("user").is_some());
        assert!(event.payload["user"].get("roles").is_some());
    }

    // ============================================================================
    // EventPriority Tests
    // ============================================================================

    #[test]
    fn test_event_priority_ordering() {
        assert!(EventPriority::Low < EventPriority::Medium);
        assert!(EventPriority::Medium < EventPriority::High);
        assert!(EventPriority::High < EventPriority::Critical);
    }

    #[test]
    fn test_event_priority_equality() {
        assert_eq!(EventPriority::Low, EventPriority::Low);
        assert_eq!(EventPriority::Medium, EventPriority::Medium);
        assert_eq!(EventPriority::High, EventPriority::High);
        assert_eq!(EventPriority::Critical, EventPriority::Critical);
    }

    #[test]
    fn test_event_priority_serialization() {
        let low = EventPriority::Low;
        let json = serde_json::to_string(&low).expect("Should serialize");
        let deserialized: EventPriority = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(low, deserialized);
    }

    #[test]
    fn test_event_priority_clone_copy() {
        let p1 = EventPriority::High;
        let p2 = p1; // Copy
        let p3 = p1; // Copy (no need to clone Copy types)

        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
    }

    // ============================================================================
    // EventStatus Tests
    // ============================================================================

    #[test]
    fn test_event_status_all_variants() {
        let statuses = vec![
            EventStatus::Pending,
            EventStatus::Processing,
            EventStatus::Completed,
            EventStatus::Failed,
            EventStatus::Cancelled,
        ];

        assert_eq!(statuses.len(), 5);
    }

    #[test]
    fn test_event_status_equality() {
        assert_eq!(EventStatus::Pending, EventStatus::Pending);
        assert_ne!(EventStatus::Pending, EventStatus::Processing);
        assert_ne!(EventStatus::Completed, EventStatus::Failed);
    }

    #[test]
    fn test_event_status_serialization() {
        let status = EventStatus::Processing;
        let json = serde_json::to_string(&status).expect("Should serialize");
        let deserialized: EventStatus = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_event_status_clone_copy() {
        let s1 = EventStatus::Completed;
        let s2 = s1; // Copy
        let s3 = s1; // Copy (no need to clone Copy types)

        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    // ============================================================================
    // EcosystemEvent Tests
    // ============================================================================

    #[test]
    fn test_ecosystem_event_creation() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "service.started".to_string(),
            source: "beardog-api".to_string(),
            target: None,
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Pending,
        };

        assert_eq!(event.event_type, "service.started");
        assert_eq!(event.source, "beardog-api");
        assert!(event.target.is_none());
        assert_eq!(event.priority, EventPriority::Medium);
        assert_eq!(event.status, EventStatus::Pending);
    }

    #[test]
    fn test_ecosystem_event_with_target() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "task.assigned".to_string(),
            source: "orchestrator".to_string(),
            target: Some("worker-node-1".to_string()),
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::High,
            status: EventStatus::Pending,
        };

        assert_eq!(event.target, Some("worker-node-1".to_string()));
        assert_eq!(event.priority, EventPriority::High);
    }

    #[test]
    fn test_ecosystem_event_with_data() {
        let mut data = HashMap::new();
        data.insert("task_id".to_string(), serde_json::json!("task-123"));
        data.insert("duration".to_string(), serde_json::json!(300));
        data.insert("result".to_string(), serde_json::json!("success"));

        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "task.completed".to_string(),
            source: "worker-1".to_string(),
            target: None,
            data: data.clone(),
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Completed,
        };

        assert_eq!(event.data.len(), 3);
        assert!(event.data.contains_key("task_id"));
        assert!(event.data.contains_key("duration"));
        assert!(event.data.contains_key("result"));
    }

    #[test]
    fn test_ecosystem_event_serialization() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "test".to_string(),
            source: "source".to_string(),
            target: Some("target".to_string()),
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Low,
            status: EventStatus::Pending,
        };

        let json = serde_json::to_string(&event).expect("Should serialize");
        let deserialized: EcosystemEvent = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(event.id, deserialized.id);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.source, deserialized.source);
        assert_eq!(event.target, deserialized.target);
        assert_eq!(event.priority, deserialized.priority);
        assert_eq!(event.status, deserialized.status);
    }

    #[test]
    fn test_ecosystem_event_clone() {
        let event1 = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "test".to_string(),
            source: "source".to_string(),
            target: None,
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Processing,
        };

        let event2 = event1.clone();

        assert_eq!(event1.id, event2.id);
        assert_eq!(event1.event_type, event2.event_type);
        assert_eq!(event1.priority, event2.priority);
        assert_eq!(event1.status, event2.status);
    }

    // ============================================================================
    // Priority-Based Event Scenarios
    // ============================================================================

    #[test]
    fn test_critical_priority_event() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "system.failure".to_string(),
            source: "monitoring".to_string(),
            target: Some("admin-console".to_string()),
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Critical,
            status: EventStatus::Pending,
        };

        assert_eq!(event.priority, EventPriority::Critical);
        assert_eq!(event.event_type, "system.failure");
    }

    #[test]
    fn test_low_priority_event() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "metrics.reported".to_string(),
            source: "metrics-collector".to_string(),
            target: None,
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Low,
            status: EventStatus::Completed,
        };

        assert_eq!(event.priority, EventPriority::Low);
        assert_eq!(event.status, EventStatus::Completed);
    }

    // ============================================================================
    // Event Status Transitions
    // ============================================================================

    #[test]
    fn test_event_status_transitions() {
        let mut event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "task.processing".to_string(),
            source: "worker".to_string(),
            target: None,
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Pending,
        };

        // Pending -> Processing
        assert_eq!(event.status, EventStatus::Pending);
        event.status = EventStatus::Processing;
        assert_eq!(event.status, EventStatus::Processing);

        // Processing -> Completed
        event.status = EventStatus::Completed;
        assert_eq!(event.status, EventStatus::Completed);
    }

    #[test]
    fn test_event_failed_status() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "task.failed".to_string(),
            source: "worker".to_string(),
            target: None,
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::High,
            status: EventStatus::Failed,
        };

        assert_eq!(event.status, EventStatus::Failed);
        assert_eq!(event.priority, EventPriority::High);
    }

    #[test]
    fn test_event_cancelled_status() {
        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "task.cancelled".to_string(),
            source: "orchestrator".to_string(),
            target: Some("worker".to_string()),
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Cancelled,
        };

        assert_eq!(event.status, EventStatus::Cancelled);
    }

    // ============================================================================
    // NodeType Tests (from types.rs)
    // ============================================================================

    #[test]
    fn test_node_type_service() {
        let node_type = NodeType::Service;
        let json = serde_json::to_string(&node_type).expect("Should serialize");
        let deserialized: NodeType = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(node_type, deserialized);
    }

    #[test]
    fn test_node_type_compute() {
        let node_type = NodeType::Compute;
        assert_eq!(node_type, NodeType::Compute);
    }

    // ============================================================================
    // Realistic Event Scenarios
    // ============================================================================

    #[test]
    fn test_service_startup_event() {
        let mut data = HashMap::new();
        data.insert("service_name".to_string(), serde_json::json!("beardog-api"));
        data.insert("version".to_string(), serde_json::json!("1.0.0"));
        data.insert("port".to_string(), serde_json::json!(8080));

        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "service.started".to_string(),
            source: "beardog-api".to_string(),
            target: None,
            data,
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Completed,
        };

        assert_eq!(event.data.len(), 3);
        assert_eq!(event.status, EventStatus::Completed);
    }

    #[test]
    fn test_health_check_event() {
        let mut data = HashMap::new();
        data.insert("healthy".to_string(), serde_json::json!(true));
        data.insert("response_time_ms".to_string(), serde_json::json!(45));

        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "health.check".to_string(),
            source: "health-monitor".to_string(),
            target: Some("beardog-core".to_string()),
            data,
            timestamp: Utc::now(),
            priority: EventPriority::Low,
            status: EventStatus::Completed,
        };

        assert_eq!(event.priority, EventPriority::Low);
        assert!(event.data.contains_key("healthy"));
    }

    #[test]
    fn test_error_event() {
        let mut data = HashMap::new();
        data.insert(
            "error_type".to_string(),
            serde_json::json!("ConnectionTimeout"),
        );
        data.insert(
            "message".to_string(),
            serde_json::json!("Failed to connect to service"),
        );

        let event = EcosystemEvent {
            id: Uuid::new_v4(),
            event_type: "error.occurred".to_string(),
            source: "beardog-tunnel".to_string(),
            target: Some("monitoring".to_string()),
            data,
            timestamp: Utc::now(),
            priority: EventPriority::High,
            status: EventStatus::Failed,
        };

        assert_eq!(event.priority, EventPriority::High);
        assert_eq!(event.status, EventStatus::Failed);
        assert!(event.data.contains_key("error_type"));
    }
}
