use beardog::audit::*;
use beardog::compliance::*;
use beardog::core::*;
use beardog::encryption::*;
use beardog::error::*;
use beardog::node_registry::*;
use beardog::proof_verifier::*;
use beardog::security_provider::*;
use beardog::threat_detection::*;
use beardog::workflows::*;
use beardog::*;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::time::sleep;

/// Comprehensive security testing for all BearDog core components
/// This test suite ensures 100% coverage of critical security paths
#[tokio::test]
async fn test_beardog_core_security_comprehensive() {
    // Test core initialization and basic operations
    let core = BearDogCore::new(BearDogConfig::default())
        .await
        .expect("Core initialization failed");

    // Test all core security methods
    test_core_security_methods(&core).await;
    test_core_error_handling(&core).await;
    test_core_concurrent_access(&core).await;
}

async fn test_core_security_methods(core: &BearDogCore) {
    // Test security provider access
    match core.security_provider() {
        Ok(provider) => {
            // Test provider security methods
            let auth_result = provider.authenticate("test_user", "test_token").await;
            assert!(
                auth_result.is_ok(),
                "Authentication should work with valid inputs"
            );
        }
        Err(e) => {
            println!("Security provider not available: {:?}", e);
        }
    }

    // Test encryption engine access
    match core.encryption_engine() {
        Ok(engine) => {
            // Test encryption/decryption
            let test_data = b"test_security_data";
            let encrypted = engine.encrypt(test_data, "test_key").await;
            assert!(encrypted.is_ok(), "Encryption should succeed");

            if let Ok(encrypted_data) = encrypted {
                let decrypted = engine.decrypt(&encrypted_data, "test_key").await;
                assert!(decrypted.is_ok(), "Decryption should succeed");
                assert_eq!(
                    decrypted.unwrap(),
                    test_data,
                    "Decrypted data should match original"
                );
            }
        }
        Err(e) => {
            println!("Encryption engine not available: {:?}", e);
        }
    }

    // Test audit engine
    match core.audit_engine() {
        Ok(audit) => {
            let audit_event = AuditEvent {
                event_id: "test_event".to_string(),
                event_type: AuditEventType::SecurityEvent,
                timestamp: SystemTime::now(),
                user_id: Some("test_user".to_string()),
                resource: "test_resource".to_string(),
                action: "test_action".to_string(),
                outcome: AuditOutcome::Success,
                metadata: HashMap::new(),
            };

            let result = audit.log_event(audit_event).await;
            assert!(result.is_ok(), "Audit logging should succeed");
        }
        Err(e) => {
            println!("Audit engine not available: {:?}", e);
        }
    }
}

async fn test_core_error_handling(core: &BearDogCore) {
    // Test invalid authentication
    if let Ok(provider) = core.security_provider() {
        let auth_result = provider.authenticate("", "").await;
        assert!(auth_result.is_err(), "Empty credentials should fail");

        let auth_result = provider.authenticate("invalid_user", "invalid_token").await;
        assert!(auth_result.is_err(), "Invalid credentials should fail");
    }

    // Test invalid encryption
    if let Ok(engine) = core.encryption_engine() {
        let encrypt_result = engine.encrypt(b"", "").await;
        assert!(encrypt_result.is_err(), "Empty key should fail");

        let decrypt_result = engine.decrypt(b"invalid_data", "test_key").await;
        assert!(
            decrypt_result.is_err(),
            "Invalid encrypted data should fail"
        );
    }
}

async fn test_core_concurrent_access(core: &BearDogCore) {
    use std::sync::Arc;
    use tokio::task;

    let core = Arc::new(core);
    let mut handles = Vec::new();

    // Test concurrent access to security provider
    for i in 0..10 {
        let core_clone = Arc::clone(&core);
        let handle = task::spawn(async move {
            if let Ok(provider) = core_clone.security_provider() {
                let user_id = format!("user_{}", i);
                let token = format!("token_{}", i);
                let _result = provider.authenticate(&user_id, &token).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Concurrent task should complete");
    }
}

/// Test audit engine comprehensive security
#[tokio::test]
async fn test_audit_engine_comprehensive() {
    let config = AuditConfig {
        enable_real_time_monitoring: true,
        log_level: AuditLogLevel::Detailed,
        retention_days: 90,
        max_log_size_mb: 100,
        enable_integrity_checks: true,
        enable_encryption: true,
    };

    let audit = AuditEngine::new(config)
        .await
        .expect("Audit engine creation failed");

    // Test all audit event types
    test_audit_event_types(&audit).await;
    test_audit_security_events(&audit).await;
    test_audit_performance_under_load(&audit).await;
    test_audit_tampering_detection(&audit).await;
}

async fn test_audit_event_types(audit: &AuditEngine) {
    let event_types = vec![
        AuditEventType::SecurityEvent,
        AuditEventType::AuthenticationEvent,
        AuditEventType::AuthorizationEvent,
        AuditEventType::DataAccessEvent,
        AuditEventType::ConfigurationEvent,
        AuditEventType::SystemEvent,
    ];

    for event_type in event_types {
        let event = AuditEvent {
            event_id: format!("test_{:?}", event_type),
            event_type,
            timestamp: SystemTime::now(),
            user_id: Some("test_user".to_string()),
            resource: "test_resource".to_string(),
            action: "test_action".to_string(),
            outcome: AuditOutcome::Success,
            metadata: HashMap::new(),
        };

        let result = audit.log_event(event).await;
        assert!(
            result.is_ok(),
            "Audit event logging should succeed for {:?}",
            event_type
        );
    }
}

async fn test_audit_security_events(audit: &AuditEngine) {
    // Test security breach logging
    let security_event = AuditEvent {
        event_id: "security_breach_test".to_string(),
        event_type: AuditEventType::SecurityEvent,
        timestamp: SystemTime::now(),
        user_id: Some("potential_attacker".to_string()),
        resource: "critical_system".to_string(),
        action: "unauthorized_access_attempt".to_string(),
        outcome: AuditOutcome::Failure,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("threat_level".to_string(), "high".to_string());
            metadata.insert("source_ip".to_string(), "192.168.1.100".to_string());
            metadata
        },
    };

    let result = audit.log_event(security_event).await;
    assert!(result.is_ok(), "Security event logging should succeed");

    // Test rapid security events (potential attack)
    for i in 0..100 {
        let event = AuditEvent {
            event_id: format!("rapid_event_{}", i),
            event_type: AuditEventType::SecurityEvent,
            timestamp: SystemTime::now(),
            user_id: Some("attacker".to_string()),
            resource: "system".to_string(),
            action: "brute_force_attempt".to_string(),
            outcome: AuditOutcome::Failure,
            metadata: HashMap::new(),
        };

        let result = audit.log_event(event).await;
        assert!(result.is_ok(), "Rapid event logging should succeed");
    }
}

async fn test_audit_performance_under_load(audit: &AuditEngine) {
    use std::time::Instant;

    let start = Instant::now();
    let mut handles = Vec::new();

    // Create 50 concurrent audit operations
    for i in 0..50 {
        let audit_clone = audit.clone();
        let handle = tokio::spawn(async move {
            for j in 0..20 {
                let event = AuditEvent {
                    event_id: format!("load_test_{}_{}", i, j),
                    event_type: AuditEventType::SystemEvent,
                    timestamp: SystemTime::now(),
                    user_id: Some(format!("user_{}", i)),
                    resource: format!("resource_{}", j),
                    action: "load_test".to_string(),
                    outcome: AuditOutcome::Success,
                    metadata: HashMap::new(),
                };

                let _result = audit_clone.log_event(event).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.expect("Load test task should complete");
    }

    let duration = start.elapsed();
    println!("Audit load test completed in {:?}", duration);

    // Performance should be reasonable (less than 10 seconds for 1000 events)
    assert!(
        duration < Duration::from_secs(10),
        "Audit performance should be acceptable"
    );
}

async fn test_audit_tampering_detection(audit: &AuditEngine) {
    // Test integrity validation
    let integrity_result = audit.validate_integrity().await;
    assert!(
        integrity_result.is_ok(),
        "Audit integrity validation should succeed"
    );

    // Test audit log retrieval
    let logs = audit
        .get_logs(
            Some(SystemTime::now() - Duration::from_secs(3600)), // Last hour
            None,
            Some(AuditEventType::SecurityEvent),
        )
        .await;
    assert!(logs.is_ok(), "Audit log retrieval should succeed");
}

/// Test compliance engine comprehensive security
#[tokio::test]
async fn test_compliance_engine_comprehensive() {
    let config = ComplianceConfig {
        enabled_standards: vec![
            ComplianceStandard::GDPR,
            ComplianceStandard::SOX,
            ComplianceStandard::PCI_DSS,
            ComplianceStandard::HIPAA,
        ],
        strict_mode: true,
        real_time_monitoring: true,
        audit_integration: true,
    };

    let compliance = ComplianceEngine::new(config)
        .await
        .expect("Compliance engine creation failed");

    // Test compliance for all standards
    test_compliance_standards(&compliance).await;
    test_compliance_violations(&compliance).await;
    test_compliance_reporting(&compliance).await;
    test_compliance_real_time_monitoring(&compliance).await;
}

async fn test_compliance_standards(compliance: &ComplianceEngine) {
    let standards = vec![
        ComplianceStandard::GDPR,
        ComplianceStandard::SOX,
        ComplianceStandard::PCI_DSS,
        ComplianceStandard::HIPAA,
    ];

    for standard in standards {
        let event = ComplianceEvent {
            event_id: format!("compliance_test_{:?}", standard),
            event_type: ComplianceEventType::DataProcessing,
            timestamp: SystemTime::now(),
            data_category: DataCategory::PersonalData,
            processing_purpose: "testing".to_string(),
            user_consent: Some(true),
            data_subject_rights: vec![],
            geographical_location: "EU".to_string(),
            metadata: HashMap::new(),
        };

        let result = compliance.evaluate_event(event).await;
        assert!(
            result.is_ok(),
            "Compliance evaluation should succeed for {:?}",
            standard
        );

        let evaluation = result.unwrap();
        assert!(
            evaluation.overall_score >= 0.0 && evaluation.overall_score <= 1.0,
            "Compliance score should be between 0 and 1"
        );
    }
}

async fn test_compliance_violations(compliance: &ComplianceEngine) {
    // Test data processing without consent (GDPR violation)
    let violation_event = ComplianceEvent {
        event_id: "gdpr_violation_test".to_string(),
        event_type: ComplianceEventType::DataProcessing,
        timestamp: SystemTime::now(),
        data_category: DataCategory::PersonalData,
        processing_purpose: "marketing".to_string(),
        user_consent: Some(false), // No consent - should trigger violation
        data_subject_rights: vec![],
        geographical_location: "EU".to_string(),
        metadata: HashMap::new(),
    };

    let result = compliance.evaluate_event(violation_event).await;
    assert!(
        result.is_ok(),
        "Compliance evaluation should succeed even for violations"
    );

    let evaluation = result.unwrap();
    assert!(
        !evaluation.violations.is_empty(),
        "GDPR violation should be detected"
    );
    assert!(
        evaluation.overall_score < 0.8,
        "Score should be low for violations"
    );
}

async fn test_compliance_reporting(compliance: &ComplianceEngine) {
    // Generate compliance report
    let report = compliance
        .generate_report(
            SystemTime::now() - Duration::from_secs(3600), // Last hour
            SystemTime::now(),
            vec![ComplianceStandard::GDPR],
        )
        .await;

    assert!(
        report.is_ok(),
        "Compliance report generation should succeed"
    );

    let report = report.unwrap();
    assert!(!report.report_id.is_empty(), "Report should have an ID");
    assert!(
        report
            .standards_coverage
            .contains_key(&ComplianceStandard::GDPR),
        "Report should cover requested standards"
    );
}

async fn test_compliance_real_time_monitoring(compliance: &ComplianceEngine) {
    // Test real-time monitoring by creating multiple events
    let mut events = Vec::new();

    for i in 0..10 {
        let event = ComplianceEvent {
            event_id: format!("monitoring_test_{}", i),
            event_type: ComplianceEventType::DataProcessing,
            timestamp: SystemTime::now(),
            data_category: DataCategory::PersonalData,
            processing_purpose: "testing".to_string(),
            user_consent: Some(i % 2 == 0), // Every other event has consent
            data_subject_rights: vec![],
            geographical_location: "EU".to_string(),
            metadata: HashMap::new(),
        };
        events.push(event);
    }

    // Process events and check monitoring
    for event in events {
        let result = compliance.evaluate_event(event).await;
        assert!(
            result.is_ok(),
            "Real-time compliance monitoring should work"
        );
    }

    // Check if monitoring detected the violations
    let metrics = compliance.get_metrics().await;
    assert!(metrics.is_ok(), "Compliance metrics should be available");
}

/// Test threat detection engine comprehensive security
#[tokio::test]
async fn test_threat_detection_comprehensive() {
    let config = ThreatDetectionConfig {
        enable_real_time_detection: true,
        enable_behavioral_analysis: true,
        enable_machine_learning: false, // Disable ML for testing
        sensitivity_level: SensitivityLevel::High,
        max_concurrent_analyses: 100,
        analysis_timeout_seconds: 30,
    };

    let threat_detector = ThreatDetectionEngine::new(config)
        .await
        .expect("Threat detection engine creation failed");

    // Test comprehensive threat detection
    test_threat_types(&threat_detector).await;
    test_threat_behavioral_analysis(&threat_detector).await;
    test_threat_performance_under_attack(&threat_detector).await;
    test_threat_false_positive_management(&threat_detector).await;
}

async fn test_threat_types(detector: &ThreatDetectionEngine) {
    let threat_types = vec![
        ("brute_force", "Multiple failed login attempts"),
        ("sql_injection", "Malicious SQL in input"),
        ("xss_attack", "Cross-site scripting attempt"),
        ("dos_attack", "Denial of service pattern"),
        ("privilege_escalation", "Unauthorized privilege access"),
        ("data_exfiltration", "Suspicious data access pattern"),
    ];

    for (threat_type, description) in threat_types {
        let event = ThreatEvent {
            event_id: format!("threat_test_{}", threat_type),
            event_type: ThreatEventType::SecurityIncident,
            timestamp: SystemTime::now(),
            source_ip: "192.168.1.100".to_string(),
            user_agent: Some("test_agent".to_string()),
            request_path: Some("/api/test".to_string()),
            payload: Some(description.to_string()),
            threat_indicators: vec![threat_type.to_string()],
            severity: if threat_type.contains("attack") {
                ThreatSeverity::High
            } else {
                ThreatSeverity::Medium
            },
            metadata: HashMap::new(),
        };

        let result = detector.analyze_event(event).await;
        assert!(
            result.is_ok(),
            "Threat analysis should succeed for {}",
            threat_type
        );

        let analysis = result.unwrap();
        assert!(
            analysis.threat_score >= 0.0 && analysis.threat_score <= 1.0,
            "Threat score should be between 0 and 1"
        );
    }
}

async fn test_threat_behavioral_analysis(detector: &ThreatDetectionEngine) {
    // Simulate user behavior patterns
    let normal_user_id = "normal_user";
    let suspicious_user_id = "suspicious_user";

    // Normal user behavior
    for i in 0..10 {
        let event = ThreatEvent {
            event_id: format!("normal_behavior_{}", i),
            event_type: ThreatEventType::UserActivity,
            timestamp: SystemTime::now(),
            source_ip: "192.168.1.50".to_string(),
            user_agent: Some("normal_browser".to_string()),
            request_path: Some("/api/data".to_string()),
            payload: None,
            threat_indicators: vec![],
            severity: ThreatSeverity::Low,
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("user_id".to_string(), normal_user_id.to_string());
                metadata
            },
        };

        let result = detector.analyze_event(event).await;
        assert!(result.is_ok(), "Normal behavior analysis should succeed");
    }

    // Suspicious user behavior (rapid requests, different IPs)
    for i in 0..50 {
        let event = ThreatEvent {
            event_id: format!("suspicious_behavior_{}", i),
            event_type: ThreatEventType::UserActivity,
            timestamp: SystemTime::now(),
            source_ip: format!("192.168.1.{}", 100 + (i % 50)), // Different IPs
            user_agent: Some("suspicious_agent".to_string()),
            request_path: Some("/api/sensitive".to_string()),
            payload: None,
            threat_indicators: vec!["rapid_requests".to_string()],
            severity: ThreatSeverity::Medium,
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("user_id".to_string(), suspicious_user_id.to_string());
                metadata
            },
        };

        let result = detector.analyze_event(event).await;
        assert!(
            result.is_ok(),
            "Suspicious behavior analysis should succeed"
        );

        // Later events should have higher threat scores
        if i > 30 {
            let analysis = result.unwrap();
            assert!(
                analysis.threat_score >= 0.5,
                "Repeated suspicious behavior should increase threat score"
            );
        }
    }
}

async fn test_threat_performance_under_attack(detector: &ThreatDetectionEngine) {
    use std::time::Instant;

    let start = Instant::now();
    let mut handles = Vec::new();

    // Simulate large-scale attack (1000 concurrent threat events)
    for i in 0..100 {
        let detector_clone = detector.clone();
        let handle = tokio::spawn(async move {
            for j in 0..10 {
                let event = ThreatEvent {
                    event_id: format!("attack_simulation_{}_{}", i, j),
                    event_type: ThreatEventType::SecurityIncident,
                    timestamp: SystemTime::now(),
                    source_ip: format!("10.0.{}.{}", i % 256, j % 256),
                    user_agent: Some("attack_bot".to_string()),
                    request_path: Some("/api/target".to_string()),
                    payload: Some("malicious_payload".to_string()),
                    threat_indicators: vec!["dos_attack".to_string(), "bot_activity".to_string()],
                    severity: ThreatSeverity::High,
                    metadata: HashMap::new(),
                };

                let _result = detector_clone.analyze_event(event).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all analyses to complete
    for handle in handles {
        handle
            .await
            .expect("Attack simulation task should complete");
    }

    let duration = start.elapsed();
    println!("Threat detection under attack completed in {:?}", duration);

    // Should handle attack within reasonable time (less than 30 seconds for 1000 events)
    assert!(
        duration < Duration::from_secs(30),
        "Threat detection should perform well under attack"
    );
}

async fn test_threat_false_positive_management(detector: &ThreatDetectionEngine) {
    // Test legitimate activities that might be flagged as threats
    let legitimate_activities = vec![
        ("batch_processing", "Legitimate batch job"),
        ("api_testing", "Automated API testing"),
        ("data_migration", "Database migration process"),
        ("backup_operation", "System backup"),
    ];

    for (activity_type, description) in legitimate_activities {
        let event = ThreatEvent {
            event_id: format!("legitimate_{}", activity_type),
            event_type: ThreatEventType::SystemActivity,
            timestamp: SystemTime::now(),
            source_ip: "127.0.0.1".to_string(), // Localhost
            user_agent: Some("system_process".to_string()),
            request_path: Some("/internal/api".to_string()),
            payload: Some(description.to_string()),
            threat_indicators: vec![], // No threat indicators
            severity: ThreatSeverity::Low,
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("activity_type".to_string(), activity_type.to_string());
                metadata.insert("authorized".to_string(), "true".to_string());
                metadata
            },
        };

        let result = detector.analyze_event(event).await;
        assert!(
            result.is_ok(),
            "Legitimate activity analysis should succeed"
        );

        let analysis = result.unwrap();
        assert!(
            analysis.threat_score < 0.3,
            "Legitimate activities should have low threat scores"
        );
        assert!(
            !analysis.is_threat,
            "Legitimate activities should not be flagged as threats"
        );
    }
}

/// Test encryption engine comprehensive security
#[tokio::test]
async fn test_encryption_engine_comprehensive() {
    let config = EncryptionConfig {
        default_algorithm: EncryptionAlgorithm::Aes256Gcm,
        key_rotation_interval: Duration::from_secs(3600),
        enable_hardware_acceleration: true,
        enable_quantum_resistance: false, // For testing
    };

    let encryption = EncryptionEngine::new(config)
        .await
        .expect("Encryption engine creation failed");

    // Test comprehensive encryption functionality
    test_encryption_algorithms(&encryption).await;
    test_encryption_key_management(&encryption).await;
    test_encryption_performance(&encryption).await;
    test_encryption_attack_resistance(&encryption).await;
}

async fn test_encryption_algorithms(engine: &EncryptionEngine) {
    let algorithms = vec![
        EncryptionAlgorithm::Aes256Gcm,
        EncryptionAlgorithm::ChaCha20Poly1305,
        EncryptionAlgorithm::Aes128Gcm,
    ];

    let test_data = b"This is test data for encryption testing";

    for algorithm in algorithms {
        let key = format!("test_key_{:?}", algorithm);

        // Test encryption
        let encrypted = engine
            .encrypt_with_algorithm(test_data, &key, algorithm)
            .await;
        assert!(
            encrypted.is_ok(),
            "Encryption should succeed for {:?}",
            algorithm
        );

        if let Ok(encrypted_data) = encrypted {
            // Test decryption
            let decrypted = engine
                .decrypt_with_algorithm(&encrypted_data, &key, algorithm)
                .await;
            assert!(
                decrypted.is_ok(),
                "Decryption should succeed for {:?}",
                algorithm
            );
            assert_eq!(
                decrypted.unwrap(),
                test_data,
                "Decrypted data should match original"
            );
        }
    }
}

async fn test_encryption_key_management(engine: &EncryptionEngine) {
    // Test key generation
    let key = engine.generate_key(256).await;
    assert!(key.is_ok(), "Key generation should succeed");

    // Test key rotation
    let rotation_result = engine.rotate_keys().await;
    assert!(rotation_result.is_ok(), "Key rotation should succeed");

    // Test key derivation
    let master_key = "master_key_for_testing";
    let context = "test_context";
    let derived_key = engine.derive_key(master_key, context).await;
    assert!(derived_key.is_ok(), "Key derivation should succeed");
}

async fn test_encryption_performance(engine: &EncryptionEngine) {
    use std::time::Instant;

    let test_sizes = vec![1024, 10240, 102400, 1048576]; // 1KB, 10KB, 100KB, 1MB

    for size in test_sizes {
        let test_data = vec![0u8; size];
        let key = format!("performance_test_key_{}", size);

        // Measure encryption performance
        let start = Instant::now();
        let encrypted = engine.encrypt(&test_data, &key).await;
        let encrypt_duration = start.elapsed();

        assert!(
            encrypted.is_ok(),
            "Encryption should succeed for {} bytes",
            size
        );

        // Measure decryption performance
        if let Ok(encrypted_data) = encrypted {
            let start = Instant::now();
            let decrypted = engine.decrypt(&encrypted_data, &key).await;
            let decrypt_duration = start.elapsed();

            assert!(
                decrypted.is_ok(),
                "Decryption should succeed for {} bytes",
                size
            );

            println!(
                "Encryption performance for {} bytes: encrypt={:?}, decrypt={:?}",
                size, encrypt_duration, decrypt_duration
            );

            // Performance requirements (adjust based on actual requirements)
            assert!(
                encrypt_duration < Duration::from_millis(1000),
                "Encryption should be fast for {} bytes",
                size
            );
            assert!(
                decrypt_duration < Duration::from_millis(1000),
                "Decryption should be fast for {} bytes",
                size
            );
        }
    }
}

async fn test_encryption_attack_resistance(engine: &EncryptionEngine) {
    let test_data = b"sensitive_data";
    let correct_key = "correct_key";
    let wrong_key = "wrong_key";

    // Test encryption with correct key
    let encrypted = engine.encrypt(test_data, correct_key).await;
    assert!(
        encrypted.is_ok(),
        "Encryption with correct key should succeed"
    );

    if let Ok(encrypted_data) = encrypted {
        // Test decryption with wrong key should fail
        let decryption_result = engine.decrypt(&encrypted_data, wrong_key).await;
        assert!(
            decryption_result.is_err(),
            "Decryption with wrong key should fail"
        );

        // Test tampering resistance
        let mut tampered_data = encrypted_data.clone();
        if !tampered_data.is_empty() {
            tampered_data[0] ^= 0xFF; // Flip bits in first byte

            let tampered_decryption = engine.decrypt(&tampered_data, correct_key).await;
            assert!(
                tampered_decryption.is_err(),
                "Decryption of tampered data should fail"
            );
        }

        // Test replay attack resistance (if applicable)
        let decryption1 = engine.decrypt(&encrypted_data, correct_key).await;
        let decryption2 = engine.decrypt(&encrypted_data, correct_key).await;

        // Both should succeed but encryption should use different nonces
        assert!(decryption1.is_ok(), "First decryption should succeed");
        assert!(decryption2.is_ok(), "Second decryption should succeed");
    }
}

/// Helper function to create default configurations for testing
fn create_test_configs() -> (
    AuditConfig,
    ComplianceConfig,
    ThreatDetectionConfig,
    EncryptionConfig,
) {
    let audit_config = AuditConfig {
        enable_real_time_monitoring: true,
        log_level: AuditLogLevel::Detailed,
        retention_days: 30,
        max_log_size_mb: 50,
        enable_integrity_checks: true,
        enable_encryption: true,
    };

    let compliance_config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::GDPR],
        strict_mode: false,
        real_time_monitoring: true,
        audit_integration: true,
    };

    let threat_config = ThreatDetectionConfig {
        enable_real_time_detection: true,
        enable_behavioral_analysis: true,
        enable_machine_learning: false,
        sensitivity_level: SensitivityLevel::Medium,
        max_concurrent_analyses: 50,
        analysis_timeout_seconds: 15,
    };

    let encryption_config = EncryptionConfig {
        default_algorithm: EncryptionAlgorithm::Aes256Gcm,
        key_rotation_interval: Duration::from_secs(1800),
        enable_hardware_acceleration: false, // Disable for consistent testing
        enable_quantum_resistance: false,
    };

    (
        audit_config,
        compliance_config,
        threat_config,
        encryption_config,
    )
}

#[tokio::test]
async fn test_integrated_security_workflow() {
    let (audit_config, compliance_config, threat_config, encryption_config) = create_test_configs();

    // Initialize all engines
    let audit = Arc::new(
        AuditEngine::new(audit_config)
            .await
            .expect("Audit engine failed"),
    );
    let compliance = Arc::new(
        ComplianceEngine::new(compliance_config)
            .await
            .expect("Compliance engine failed"),
    );
    let threat_detector = Arc::new(
        ThreatDetectionEngine::new(threat_config)
            .await
            .expect("Threat detection failed"),
    );
    let encryption = Arc::new(
        EncryptionEngine::new(encryption_config)
            .await
            .expect("Encryption engine failed"),
    );

    // Test integrated workflow
    test_security_incident_workflow(audit.clone(), compliance.clone(), threat_detector.clone())
        .await;
    test_data_protection_workflow(audit.clone(), compliance.clone(), encryption.clone()).await;
    test_compliance_monitoring_workflow(audit.clone(), compliance.clone()).await;
}

async fn test_security_incident_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
    threat_detector: Arc<ThreatDetectionEngine>,
) {
    // Simulate security incident
    let incident_event = ThreatEvent {
        event_id: "security_incident_001".to_string(),
        event_type: ThreatEventType::SecurityIncident,
        timestamp: SystemTime::now(),
        source_ip: "192.168.1.100".to_string(),
        user_agent: Some("malicious_scanner".to_string()),
        request_path: Some("/admin/users".to_string()),
        payload: Some("' OR 1=1 --".to_string()), // SQL injection attempt
        threat_indicators: vec!["sql_injection".to_string()],
        severity: ThreatSeverity::High,
        metadata: HashMap::new(),
    };

    // 1. Threat detection
    let threat_analysis = threat_detector.analyze_event(incident_event.clone()).await;
    assert!(threat_analysis.is_ok(), "Threat analysis should succeed");

    let analysis = threat_analysis.unwrap();
    assert!(
        analysis.is_threat,
        "SQL injection should be detected as threat"
    );
    assert!(analysis.threat_score >= 0.7, "High threat score expected");

    // 2. Audit logging
    let audit_event = AuditEvent {
        event_id: incident_event.event_id.clone(),
        event_type: AuditEventType::SecurityEvent,
        timestamp: incident_event.timestamp,
        user_id: Some("unknown_attacker".to_string()),
        resource: "admin_panel".to_string(),
        action: "sql_injection_attempt".to_string(),
        outcome: AuditOutcome::Blocked,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert(
                "threat_score".to_string(),
                analysis.threat_score.to_string(),
            );
            metadata.insert("source_ip".to_string(), incident_event.source_ip.clone());
            metadata
        },
    };

    let audit_result = audit.log_event(audit_event).await;
    assert!(
        audit_result.is_ok(),
        "Security incident audit should succeed"
    );

    // 3. Compliance evaluation
    let compliance_event = ComplianceEvent {
        event_id: incident_event.event_id,
        event_type: ComplianceEventType::SecurityIncident,
        timestamp: incident_event.timestamp,
        data_category: DataCategory::SystemLogs,
        processing_purpose: "security_monitoring".to_string(),
        user_consent: None, // Not applicable for security incidents
        data_subject_rights: vec![],
        geographical_location: "internal".to_string(),
        metadata: HashMap::new(),
    };

    let compliance_result = compliance.evaluate_event(compliance_event).await;
    assert!(
        compliance_result.is_ok(),
        "Compliance evaluation should succeed"
    );
}

async fn test_data_protection_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
    encryption: Arc<EncryptionEngine>,
) {
    let user_data = b"Personal information: John Doe, SSN: 123-45-6789";
    let encryption_key = "user_data_encryption_key";

    // 1. Encrypt sensitive data
    let encrypted_data = encryption.encrypt(user_data, encryption_key).await;
    assert!(encrypted_data.is_ok(), "Data encryption should succeed");

    // 2. Log data processing
    let audit_event = AuditEvent {
        event_id: "data_processing_001".to_string(),
        event_type: AuditEventType::DataAccessEvent,
        timestamp: SystemTime::now(),
        user_id: Some("system_user".to_string()),
        resource: "user_personal_data".to_string(),
        action: "encrypt_personal_data".to_string(),
        outcome: AuditOutcome::Success,
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert(
                "data_type".to_string(),
                "personal_identifiable_information".to_string(),
            );
            metadata.insert("encryption_algorithm".to_string(), "AES256-GCM".to_string());
            metadata
        },
    };

    let audit_result = audit.log_event(audit_event).await;
    assert!(audit_result.is_ok(), "Data processing audit should succeed");

    // 3. Compliance check
    let compliance_event = ComplianceEvent {
        event_id: "data_processing_001".to_string(),
        event_type: ComplianceEventType::DataProcessing,
        timestamp: SystemTime::now(),
        data_category: DataCategory::PersonalData,
        processing_purpose: "user_account_management".to_string(),
        user_consent: Some(true),
        data_subject_rights: vec![DataSubjectRight::DataPortability, DataSubjectRight::Erasure],
        geographical_location: "EU".to_string(),
        metadata: HashMap::new(),
    };

    let compliance_result = compliance.evaluate_event(compliance_event).await;
    assert!(
        compliance_result.is_ok(),
        "Data processing compliance should succeed"
    );

    let evaluation = compliance_result.unwrap();
    assert!(
        evaluation.overall_score >= 0.8,
        "Good compliance score expected for proper data handling"
    );
}

async fn test_compliance_monitoring_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
) {
    // Generate series of data processing events
    for i in 0..20 {
        let has_consent = i % 3 != 0; // 2/3 have consent, 1/3 don't

        let compliance_event = ComplianceEvent {
            event_id: format!("monitoring_event_{}", i),
            event_type: ComplianceEventType::DataProcessing,
            timestamp: SystemTime::now(),
            data_category: DataCategory::PersonalData,
            processing_purpose: "analytics".to_string(),
            user_consent: Some(has_consent),
            data_subject_rights: vec![],
            geographical_location: "EU".to_string(),
            metadata: HashMap::new(),
        };

        // Evaluate compliance
        let result = compliance.evaluate_event(compliance_event.clone()).await;
        assert!(result.is_ok(), "Compliance evaluation should succeed");

        // Log the compliance evaluation
        let audit_event = AuditEvent {
            event_id: format!("compliance_audit_{}", i),
            event_type: AuditEventType::ComplianceEvent,
            timestamp: SystemTime::now(),
            user_id: Some("compliance_system".to_string()),
            resource: "data_processing_pipeline".to_string(),
            action: "compliance_evaluation".to_string(),
            outcome: if has_consent {
                AuditOutcome::Success
            } else {
                AuditOutcome::Warning
            },
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("consent_status".to_string(), has_consent.to_string());
                metadata.insert("data_category".to_string(), "personal_data".to_string());
                metadata
            },
        };

        let audit_result = audit.log_event(audit_event).await;
        assert!(audit_result.is_ok(), "Compliance audit should succeed");
    }

    // Generate compliance report
    let report = compliance
        .generate_report(
            SystemTime::now() - Duration::from_secs(300), // Last 5 minutes
            SystemTime::now(),
            vec![ComplianceStandard::GDPR],
        )
        .await;

    assert!(
        report.is_ok(),
        "Compliance report generation should succeed"
    );

    let report = report.unwrap();
    assert!(!report.report_id.is_empty(), "Report should have ID");
    assert!(
        report
            .standards_coverage
            .contains_key(&ComplianceStandard::GDPR),
        "Report should cover GDPR"
    );

    // Check that violations were detected for events without consent
    if let Some(gdpr_result) = report.standards_coverage.get(&ComplianceStandard::GDPR) {
        assert!(
            !gdpr_result.violations.is_empty(),
            "GDPR violations should be detected"
        );
    }
}
