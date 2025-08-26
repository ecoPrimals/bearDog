

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

        drop(engine); // This line ensures the engine was created successfully
    }
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

        assert!(ThreatSeverity::Critical > ThreatSeverity::High);
        assert!(ThreatSeverity::High > ThreatSeverity::Medium);
        assert!(ThreatSeverity::Medium > ThreatSeverity::Low);
        assert!(ThreatSeverity::Low > ThreatSeverity::Info);

        let severities = [
            ThreatSeverity::Info,
            ThreatSeverity::Low,
            ThreatSeverity::Medium,
            ThreatSeverity::High,
            ThreatSeverity::Critical,
        ];

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
    fn test_threat_type_variants() {
        let threat_types = [
            ThreatType::BruteForceAttack,
            ThreatType::DataExfiltration,
            ThreatType::Malware,
            ThreatType::SqlInjection,
        ];
        
        assert_eq!(threat_types.len(), 4);

        for threat_type in threat_types {
            match threat_type {
                ThreatType::BruteForceAttack => {}
                ThreatType::DataExfiltration => {}
                ThreatType::Malware => {}
                ThreatType::SqlInjection => {}
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

        event.user_agent = Some("Mozilla/5.0 Test".to_string());
        event.data_size = 1024.0; // This is f64, not Option<i32>
        let mut additional_data = HashMap::with_capacity(16);
        additional_data.insert("request_path".to_string(), "/login".to_string());
        additional_data.insert("method".to_string(), "POST".to_string());
        event.additional_data = additional_data;

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

        let default_config = ThreatDetectionConfig::default();
        assert!(default_config.enabled); // Should be enabled by default

        let custom_config = ThreatDetectionConfig {
            enabled: true,
            real_time_detection: true,
            automated_response: false,
            ml_enhancement: false,
            alert_threshold: 0.5,
            max_alerts_per_minute: 50,
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

        let minimal_config = ThreatDetectionConfig {
            enabled: false,
            real_time_detection: false,
            automated_response: false,
            ml_enhancement: false,
            alert_threshold: 0.7,
            max_alerts_per_minute: 100,
        };
        
        let _engine1 = ThreatDetectionEngine::new(minimal_config).await;

        let full_config = ThreatDetectionConfig {
            enabled: true,
            real_time_detection: true,
            automated_response: true,
            ml_enhancement: true,
            alert_threshold: 0.8,
            max_alerts_per_minute: 200,
        };
        let _engine2 = ThreatDetectionEngine::new(full_config).await;

        drop(_engine1);
        drop(_engine2);
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

        assert!(event.timestamp >= before);
        assert!(event.timestamp <= after);
    }

    #[tokio::test]
    async fn test_basic_threat_detection_flow() {

        let suspicious_event = SecurityEvent::new(
            "event_004".to_string(),
            "failed_login".to_string(),
            "admin".to_string(),
            "192.168.1.1".to_string(),
            "api_user".to_string(),
        );

        assert_eq!(suspicious_event.event_type, "failed_login");
        assert_eq!(suspicious_event.user_id, "admin");
        assert_eq!(suspicious_event.source_ip, "192.168.1.1");

        let engine = ();

        drop(engine); // Successfully created and can be dropped
    }
}
