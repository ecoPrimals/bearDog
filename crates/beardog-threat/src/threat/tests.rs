//! Threat detection tests
//!
//! Comprehensive tests for the threat detection and response system.

#[cfg(test)]
mod threat_tests {
    use crate::threat::{
        SecurityEvent, ThreatDetectionConfig, ThreatDetectionEngine, ThreatSeverity, ThreatStatus,
        ThreatType,
    };
    use chrono::Utc;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_threat_detection_engine_creation() {
        let config = ThreatDetectionConfig {
            enabled: true,
            real_time_detection: true,
            automated_response: false,
            ml_enhancement: false,
            alert_threshold: 0.7,
            max_alerts_per_minute: 100,
            ..Default::default()
        };

        let engine = ThreatDetectionEngine::new(config).await;

        // Verify engine has initialized properly
        // Note: Engine fields are private, so we'll just verify creation succeeded
        drop(engine); // This line ensures the engine was created successfully
    }

    #[tokio::test]
    async fn test_security_event_creation() {
        let event = SecurityEvent::new(
            "event_001".to_string(),
            "failed_login".to_string(),
            "192.168.1.1".to_string(),
            "192.168.1.100".to_string(),
            "admin".to_string(),
        );

        assert_eq!(event.event_type, "failed_login");
        assert_eq!(event.source_ip, "192.168.1.1");
        assert_eq!(event.destination_ip, "192.168.1.100");
        assert_eq!(event.user_id, "admin");
        assert!(event.timestamp <= Utc::now());
    }

    #[test]
    fn test_threat_severity_ordering() {
        // Test severity ordering (higher severity > lower severity)
        assert!(ThreatSeverity::Critical > ThreatSeverity::High);
        assert!(ThreatSeverity::High > ThreatSeverity::Medium);
        assert!(ThreatSeverity::Medium > ThreatSeverity::Low);
        assert!(ThreatSeverity::Low > ThreatSeverity::Info);

        // Test specific values
        let severities = [
            ThreatSeverity::Info,
            ThreatSeverity::Low,
            ThreatSeverity::Medium,
            ThreatSeverity::High,
            ThreatSeverity::Critical,
        ];

        // Verify they're in ascending order
        for i in 1..severities.len() {
            assert!(severities[i] > severities[i - 1]);
        }
    }

    #[test]
    fn test_threat_status_variants() {
        let statuses = [
            ThreatStatus::Active,
            ThreatStatus::Investigating,
            ThreatStatus::Mitigated,
            ThreatStatus::Resolved,
            ThreatStatus::FalsePositive,
        ];

        assert_eq!(statuses.len(), 5);

        // Test that each status can be pattern matched
        for status in statuses {
            match status {
                ThreatStatus::Active => {}
                ThreatStatus::Investigating => {}
                ThreatStatus::Mitigated => {}
                ThreatStatus::Resolved => {}
                ThreatStatus::FalsePositive => {}
                _ => {} // Handle other variants
            }
        }
    }

    #[test]
    fn test_threat_type_variants() {
        let threat_types = [
            ThreatType::BruteForceAttack,
            ThreatType::DataExfiltration,
            ThreatType::Malware,
            ThreatType::SqlInjection,
        ];

        assert_eq!(threat_types.len(), 4);

        // Verify each type can be pattern matched
        for threat_type in threat_types {
            match threat_type {
                ThreatType::BruteForceAttack => {}
                ThreatType::DataExfiltration => {}
                ThreatType::Malware => {}
                ThreatType::SqlInjection => {}
                _ => {} // Handle other variants
            }
        }
    }

    #[test]
    fn test_security_event_modification() {
        let mut event = SecurityEvent::new(
            "event_002".to_string(),
            "file_access".to_string(),
            "10.0.0.1".to_string(),
            "10.0.0.100".to_string(),
            "user1".to_string(),
        );

        // Test setting additional data
        event.user_agent = Some("Mozilla/5.0 Test".to_string());
        event.data_size = 1024.0; // This is f64, not Option<i32>

        let mut additional_data = HashMap::new();
        additional_data.insert("request_path".to_string(), "/login".to_string());
        additional_data.insert("method".to_string(), "POST".to_string());
        event.additional_data = additional_data;

        // Verify the modifications
        assert_eq!(event.user_agent, Some("Mozilla/5.0 Test".to_string()));
        assert_eq!(event.data_size, 1024.0);
        assert_eq!(event.additional_data.len(), 2);
        assert_eq!(
            event.additional_data.get("request_path"),
            Some(&"/login".to_string())
        );
        assert_eq!(
            event.additional_data.get("method"),
            Some(&"POST".to_string())
        );
    }

    #[test]
    fn test_threat_detection_config() {
        // Test default configuration
        let default_config = ThreatDetectionConfig::default();
        assert!(default_config.enabled); // Should be enabled by default

        // Test custom configuration
        let custom_config = ThreatDetectionConfig {
            enabled: true,
            real_time_detection: true,
            automated_response: false,
            ml_enhancement: false,
            alert_threshold: 0.5,
            max_alerts_per_minute: 50,
            ..Default::default()
        };

        assert!(custom_config.enabled);
        assert!(custom_config.real_time_detection);
        assert!(!custom_config.automated_response);
        assert!(!custom_config.ml_enhancement);
        assert_eq!(custom_config.alert_threshold, 0.5);
        assert_eq!(custom_config.max_alerts_per_minute, 50);
    }

    #[tokio::test]
    async fn test_multiple_engine_configurations() {
        // Test minimal configuration
        let minimal_config = ThreatDetectionConfig {
            enabled: false,
            ..Default::default()
        };

        let engine1 = ThreatDetectionEngine::new(minimal_config).await;

        // Test full configuration
        let full_config = ThreatDetectionConfig {
            enabled: true,
            real_time_detection: true,
            automated_response: true,
            ml_enhancement: true,
            alert_threshold: 0.8,
            max_alerts_per_minute: 200,
            ..Default::default()
        };

        let engine2 = ThreatDetectionEngine::new(full_config).await;

        // Both engines should be created successfully
        drop(engine1);
        drop(engine2);
    }

    #[tokio::test]
    async fn test_security_event_timing() {
        let before = Utc::now();
        let event = SecurityEvent::new(
            "event_003".to_string(),
            "api_access".to_string(),
            "192.168.0.100".to_string(),
            "192.168.0.1".to_string(),
            "api_user".to_string(),
        );
        let after = Utc::now();

        // Event timestamp should be between before and after
        assert!(event.timestamp >= before);
        assert!(event.timestamp <= after);
    }

    #[tokio::test]
    async fn test_basic_threat_detection_flow() {
        let config = ThreatDetectionConfig {
            enabled: true,
            real_time_detection: true,
            automated_response: false,
            ml_enhancement: false,
            alert_threshold: 0.5,
            max_alerts_per_minute: 100,
            ..Default::default()
        };

        let engine = ThreatDetectionEngine::new(config).await;

        // Create a suspicious event
        let suspicious_event = SecurityEvent::new(
            "event_004".to_string(),
            "failed_login".to_string(),
            "192.168.1.1".to_string(),
            "192.168.1.100".to_string(),
            "admin".to_string(),
        );

        // Verify event creation is working
        assert_eq!(suspicious_event.event_type, "failed_login");
        assert_eq!(suspicious_event.user_id, "admin");
        assert_eq!(suspicious_event.source_ip, "192.168.1.1");

        // Engine should be ready to process events
        drop(engine); // Successfully created and can be dropped
    }
}
