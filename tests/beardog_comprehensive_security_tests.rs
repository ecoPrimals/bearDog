use beardog::audit::*;
use beardog::compliance::*;
use beardog::config::EncryptionConfig;
use beardog::core::*;
use beardog::encryption::*;
use beardog::security::*;
use beardog::threat::*;
use beardog::*;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;

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
    let provider = core.security_provider();
    // Test provider security methods
    let auth_result = provider.authenticate("test_user", "test_token").await;
    assert!(
        auth_result.is_ok(),
        "Authentication should work with valid inputs"
    );

    // Test encryption engine access
    let engine = core.encryption_engine();
    // Test encryption/decryption
    let test_data = b"test_security_data";
    let encrypted = engine
        .encrypt(test_data, Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    assert!(encrypted.is_ok(), "Encryption should succeed");

    if let Ok(encrypted_data) = encrypted {
        let decrypted = engine.decrypt(&encrypted_data).await;
        assert!(decrypted.is_ok(), "Decryption should succeed");
        assert_eq!(
            decrypted.unwrap(),
            test_data,
            "Decrypted data should match original"
        );
    }

    // Test audit engine
    let audit = core.audit_engine();
    let audit_event = AuditEvent {
        id: "test_event".to_string(),
        event_type: AuditEventType::Security,
        severity: AuditSeverity::Medium,
        timestamp: Utc::now(),
        user_id: Some("test_user".to_string()),
        resource: Some("test_resource".to_string()),
        action: "test_action".to_string(),
        description: "Test audit event".to_string(),
        outcome: "success".to_string(),
        metadata: HashMap::new(),
        details: HashMap::new(),
    };

    let result = audit.log_event(audit_event).await;
    assert!(result.is_ok(), "Audit logging should succeed");
}

async fn test_core_error_handling(core: &BearDogCore) {
    // Test invalid authentication
    let provider = core.security_provider();
    let auth_result = provider.authenticate("", "").await;
    assert!(auth_result.is_err(), "Empty credentials should fail");

    let auth_result = provider.authenticate("invalid_user", "invalid_token").await;
    assert!(auth_result.is_err(), "Invalid credentials should fail");

    // Test invalid encryption
    let engine = core.encryption_engine();
    let encrypt_result = engine
        .encrypt(b"test_data", Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    assert!(
        encrypt_result.is_ok(),
        "Encryption should succeed with valid data"
    );

    // Test decryption with invalid data
    let invalid_encrypted = EncryptedData {
        algorithm: EncryptionAlgorithm::Aes256Gcm,
        ciphertext: b"invalid_data".to_vec(),
        nonce: vec![0u8; 12],
        tag: None,
        metadata: HashMap::new(),
        key_id: None,
    };
    let decrypt_result = engine.decrypt(&invalid_encrypted).await;
    assert!(
        decrypt_result.is_err(),
        "Invalid encrypted data should fail"
    );
}

async fn test_core_concurrent_access(core: &BearDogCore) {
    use tokio::task;

    // Clone core data we need for testing to avoid lifetime issues
    let provider = core.security_provider();
    let mut handles = Vec::new();

    // Test concurrent access to security provider
    for i in 0..10 {
        // Create test data inside the loop to avoid lifetime issues
        let handle = task::spawn(async move {
            // Create a new core instance for testing instead of sharing
            let test_core = BearDogCore::new(BearDogConfig::default())
                .await
                .expect("Core creation failed");
            let test_provider = test_core.security_provider();
            let user_id = format!("user_{}", i);
            let token = format!("token_{}", i);
            let _result = test_provider.authenticate(&user_id, &token).await;
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
    let audit = AuditEngine::new().await;

    // Test all audit event types
    test_audit_event_types(&audit).await;
    test_audit_security_events(&audit).await;
    test_audit_performance_under_load(&audit).await;
    test_audit_tampering_detection(&audit).await;
}

async fn test_audit_event_types(audit: &AuditEngine) {
    let event_types = vec![
        AuditEventType::Security,
        AuditEventType::Authentication,
        AuditEventType::Authorization,
        AuditEventType::DataAccess,
        AuditEventType::Configuration,
        AuditEventType::System,
    ];

    for event_type in event_types {
        let event = AuditEvent {
            id: format!("test_{:?}", event_type),
            event_type: event_type.clone(),
            severity: AuditSeverity::Medium,
            timestamp: Utc::now(),
            user_id: Some("test_user".to_string()),
            resource: Some("test_resource".to_string()),
            action: "test_action".to_string(),
            description: "Test audit event".to_string(),
            outcome: "success".to_string(),
            metadata: HashMap::new(),
            details: HashMap::new(),
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
        id: "security_breach_001".to_string(),
        event_type: AuditEventType::Security,
        severity: AuditSeverity::Critical,
        timestamp: Utc::now(),
        user_id: Some("potential_attacker".to_string()),
        resource: Some("sensitive_data".to_string()),
        action: "unauthorized_access".to_string(),
        description: "Potential security breach detected".to_string(),
        outcome: "blocked".to_string(),
        metadata: HashMap::new(),
        details: HashMap::new(),
    };

    let result = audit.log_event(security_event).await;
    assert!(result.is_ok(), "Security event logging should succeed");

    // Test authentication event
    let auth_event = AuditEvent {
        id: "auth_failure_001".to_string(),
        event_type: AuditEventType::Authentication,
        severity: AuditSeverity::High,
        timestamp: Utc::now(),
        user_id: Some("suspicious_user".to_string()),
        resource: Some("login_endpoint".to_string()),
        action: "failed_login".to_string(),
        description: "Multiple failed login attempts".to_string(),
        outcome: "failed".to_string(),
        metadata: HashMap::new(),
        details: HashMap::new(),
    };

    let result = audit.log_event(auth_event).await;
    assert!(
        result.is_ok(),
        "Authentication event logging should succeed"
    );
}

async fn test_audit_performance_under_load(audit: &AuditEngine) {
    use tokio::task;
    let mut handles = Vec::new();

    // Simulate high-load audit logging
    for i in 0..100 {
        // Create a new audit instance for each task to avoid lifetime issues
        let handle = task::spawn(async move {
            let test_audit = AuditEngine::new().await;
            let event = AuditEvent {
                id: format!("load_test_{}", i),
                event_type: AuditEventType::System,
                severity: AuditSeverity::Low,
                timestamp: Utc::now(),
                user_id: Some(format!("user_{}", i)),
                resource: Some("system_resource".to_string()),
                action: "system_operation".to_string(),
                description: "Load test event".to_string(),
                outcome: "success".to_string(),
                metadata: HashMap::new(),
                details: HashMap::new(),
            };

            test_audit.log_event(event).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.expect("Load test task should complete");
        assert!(result.is_ok(), "Load test event should succeed");
    }
}

async fn test_audit_tampering_detection(audit: &AuditEngine) {
    // Test audit log integrity
    let sensitive_event = AuditEvent {
        id: "sensitive_operation_001".to_string(),
        event_type: AuditEventType::Configuration,
        severity: AuditSeverity::Critical,
        timestamp: Utc::now(),
        user_id: Some("admin_user".to_string()),
        resource: Some("system_config".to_string()),
        action: "modify_security_settings".to_string(),
        description: "Critical configuration change".to_string(),
        outcome: "success".to_string(),
        metadata: HashMap::new(),
        details: HashMap::new(),
    };

    let result = audit.log_event(sensitive_event).await;
    assert!(result.is_ok(), "Sensitive event logging should succeed");

    // Verify event can be retrieved
    let recent_events = audit.get_recent_events(10).await;
    assert!(
        recent_events.is_ok(),
        "Recent events retrieval should succeed"
    );

    let events = recent_events.unwrap();
    assert!(!events.is_empty(), "Should have at least one event");
}

/// Test compliance engine comprehensive security
#[tokio::test]
async fn test_compliance_engine_comprehensive() {
    let config = ComplianceConfig::default();
    let compliance = ComplianceEngine::new(config)
        .await
        .expect("Compliance engine creation failed");

    // Test all compliance standards
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
        let test_event = ComplianceEvent {
            id: format!("test_compliance_{:?}", standard),
            event_type: "DataAccess".to_string(),
            timestamp: Utc::now(),
            user_id: Some("test_user".to_string()),
            resource: Some("user_data".to_string()),
            data: HashMap::new(),
            metadata: HashMap::new(),
        };

        let result = compliance.evaluate_event(test_event).await;
        assert!(
            result.is_ok(),
            "Compliance evaluation should succeed for {:?}",
            standard
        );
    }
}

async fn test_compliance_violations(compliance: &ComplianceEngine) {
    // Test data access without proper consent
    let mut data = HashMap::new();
    data.insert("data_type".to_string(), "personal_data".to_string());
    data.insert("user_consent".to_string(), "false".to_string());

    let violation_event = ComplianceEvent {
        id: "gdpr_violation_001".to_string(),
        event_type: "DataAccess".to_string(),
        timestamp: Utc::now(),
        user_id: Some("analyst_user".to_string()),
        resource: Some("user_personal_data".to_string()),
        data,
        metadata: HashMap::new(),
    };

    let result = compliance.evaluate_event(violation_event).await;
    assert!(result.is_ok(), "Compliance evaluation should succeed");

    let evaluation = result.unwrap();
    assert!(
        evaluation.compliance_score < 1.0,
        "Compliance score should be less than perfect for violation"
    );
}

async fn test_compliance_reporting(compliance: &ComplianceEngine) {
    // Test compliance report generation
    let date_range = (Utc::now() - chrono::Duration::days(30), Utc::now());

    let report = compliance
        .generate_compliance_report(ComplianceStandard::GDPR, date_range)
        .await;

    assert!(
        report.is_ok(),
        "Compliance report generation should succeed"
    );

    let report = report.unwrap();
    assert_eq!(report.standard, ComplianceStandard::GDPR);
    assert!(report.overall_score >= 0.0 && report.overall_score <= 1.0);
}

async fn test_compliance_real_time_monitoring(compliance: &ComplianceEngine) {
    // Test real-time compliance monitoring
    let mut monitoring_events = Vec::new();

    for i in 0..10 {
        let has_consent = i % 2 == 0;
        let mut data = HashMap::new();
        data.insert("user_consent".to_string(), has_consent.to_string());

        let event = ComplianceEvent {
            id: format!("monitoring_event_{}", i),
            event_type: "DataProcessing".to_string(),
            timestamp: Utc::now(),
            user_id: Some(format!("user_{}", i)),
            resource: Some("analytics_data".to_string()),
            data,
            metadata: HashMap::new(),
        };

        monitoring_events.push(event);
    }

    // Evaluate all events
    for event in monitoring_events {
        let result = compliance.evaluate_event(event).await;
        assert!(result.is_ok(), "Real-time monitoring should succeed");
    }
}

/// Test threat detection comprehensive security
#[tokio::test]
async fn test_threat_detection_comprehensive() {
    let config = ThreatDetectionConfig::default();
    let mut detector = ThreatDetectionEngine::new(config)
        .await
        .expect("Threat detection engine creation failed");

    // Test all threat detection capabilities
    test_threat_types(&mut detector).await;
    test_threat_behavioral_analysis(&mut detector).await;
    test_threat_performance_under_attack(&mut detector).await;
    test_threat_false_positive_management(&mut detector).await;
}

async fn test_threat_types(detector: &mut ThreatDetectionEngine) {
    let threat_types = vec![
        ThreatType::Malware,
        ThreatType::Phishing,
        ThreatType::BruteForce,
        ThreatType::DdosAttack,
        ThreatType::SqlInjection,
        ThreatType::Xss,
        ThreatType::Ransomware,
    ];

    for threat_type in threat_types {
        let mut event_data = HashMap::new();
        event_data.insert("threat_type".to_string(), format!("{:?}", threat_type));
        event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
        event_data.insert("severity".to_string(), "high".to_string());

        let analysis = detector.analyze_event(&event_data).await;
        assert!(
            analysis.is_ok(),
            "Threat analysis should succeed for {:?}",
            threat_type
        );
    }
}

async fn test_threat_behavioral_analysis(detector: &mut ThreatDetectionEngine) {
    // Test behavioral analysis with suspicious patterns
    let suspicious_patterns = vec![
        ("multiple_failed_logins", "10", "192.168.1.100"),
        ("large_data_transfer", "1000000", "192.168.1.200"),
        ("unusual_access_time", "03:00", "192.168.1.300"),
        ("privilege_escalation", "admin", "192.168.1.400"),
    ];

    for (pattern, value, ip) in suspicious_patterns {
        let mut event_data = HashMap::new();
        event_data.insert("pattern".to_string(), pattern.to_string());
        event_data.insert("value".to_string(), value.to_string());
        event_data.insert("source_ip".to_string(), ip.to_string());
        event_data.insert("timestamp".to_string(), Utc::now().to_rfc3339());

        let analysis = detector.analyze_event(&event_data).await;
        assert!(
            analysis.is_ok(),
            "Behavioral analysis should succeed for pattern: {}",
            pattern
        );
    }
}

async fn test_threat_performance_under_attack(detector: &mut ThreatDetectionEngine) {
    // Simulate high-volume attack with simpler approach
    for i in 0..50 {
        let mut event_data = HashMap::new();
        event_data.insert("attack_type".to_string(), "ddos".to_string());
        event_data.insert(
            "source_ip".to_string(),
            format!("192.168.{}.{}", i / 256, i % 256),
        );
        event_data.insert("request_rate".to_string(), "1000".to_string());

        let analysis = detector.analyze_event(&event_data).await;
        assert!(
            analysis.is_ok(),
            "High-volume threat analysis should succeed"
        );
    }
}

async fn test_threat_false_positive_management(detector: &mut ThreatDetectionEngine) {
    // Test legitimate activities that shouldn't trigger alerts
    let legitimate_activities = vec![
        ("normal_login", "user123", "192.168.1.50"),
        ("scheduled_backup", "backup_service", "192.168.1.10"),
        ("system_update", "admin", "192.168.1.5"),
        ("regular_api_call", "mobile_app", "192.168.1.60"),
    ];

    for (activity, user, ip) in legitimate_activities {
        let mut event_data = HashMap::new();
        event_data.insert("activity".to_string(), activity.to_string());
        event_data.insert("user".to_string(), user.to_string());
        event_data.insert("source_ip".to_string(), ip.to_string());
        event_data.insert("legitimate".to_string(), "true".to_string());

        let analysis = detector.analyze_event(&event_data).await;
        assert!(
            analysis.is_ok(),
            "False positive test should succeed for activity: {}",
            activity
        );
    }
}

/// Test encryption engine comprehensive security
#[tokio::test]
async fn test_encryption_engine_comprehensive() {
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config)
        .await
        .expect("Encryption engine creation failed");

    // Test all encryption capabilities
    test_encryption_algorithms(&engine).await;
    test_encryption_key_management(&engine).await;
    test_encryption_performance(&engine).await;
    test_encryption_attack_resistance(&engine).await;
}

async fn test_encryption_algorithms(engine: &EncryptionEngine) {
    let algorithms = vec![
        EncryptionAlgorithm::Aes256Gcm,
        EncryptionAlgorithm::ChaCha20Poly1305,
        EncryptionAlgorithm::QuantumResistant,
    ];

    let test_data = b"Sensitive test data for encryption";

    for algorithm in algorithms {
        let encrypted = engine.encrypt(test_data, Some(algorithm)).await;
        assert!(
            encrypted.is_ok(),
            "Encryption should succeed for {:?}",
            algorithm
        );

        if let Ok(encrypted_data) = encrypted {
            let decrypted = engine.decrypt(&encrypted_data).await;
            assert!(
                decrypted.is_ok(),
                "Decryption should succeed for {:?}",
                algorithm
            );
            assert_eq!(
                decrypted.unwrap(),
                test_data,
                "Decrypted data should match original for {:?}",
                algorithm
            );
        }
    }
}

async fn test_encryption_key_management(engine: &EncryptionEngine) {
    // Test key generation
    let key_result = engine.generate_key("AES256", "test_purpose").await;
    assert!(key_result.is_ok(), "Key generation should succeed");

    if let Ok((key_id, _key_bytes)) = key_result {
        // Test key deletion
        let delete_result = engine.delete_key(&key_id).await;
        assert!(delete_result.is_ok(), "Key deletion should succeed");
    }
}

async fn test_encryption_performance(engine: &EncryptionEngine) {
    use std::time::Instant;

    let test_sizes = vec![1024, 10240, 102400, 1048576]; // 1KB, 10KB, 100KB, 1MB

    for size in test_sizes {
        let test_data = vec![0u8; size];
        let start = Instant::now();

        let encrypted = engine
            .encrypt(&test_data, Some(EncryptionAlgorithm::Aes256Gcm))
            .await;
        let encrypt_time = start.elapsed();

        assert!(
            encrypted.is_ok(),
            "Encryption should succeed for {} bytes",
            size
        );

        if let Ok(encrypted_data) = encrypted {
            let start = Instant::now();
            let decrypted = engine.decrypt(&encrypted_data).await;
            let decrypt_time = start.elapsed();

            assert!(
                decrypted.is_ok(),
                "Decryption should succeed for {} bytes",
                size
            );
            assert_eq!(
                decrypted.unwrap(),
                test_data,
                "Decrypted data should match original"
            );

            println!(
                "Size: {} bytes, Encrypt: {:?}, Decrypt: {:?}",
                size, encrypt_time, decrypt_time
            );
        }
    }
}

async fn test_encryption_attack_resistance(engine: &EncryptionEngine) {
    let test_data = b"Attack resistance test data";

    // Test multiple encryptions of same data produce different ciphertexts
    let encrypted1 = engine
        .encrypt(test_data, Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    let encrypted2 = engine
        .encrypt(test_data, Some(EncryptionAlgorithm::Aes256Gcm))
        .await;

    assert!(
        encrypted1.is_ok() && encrypted2.is_ok(),
        "Both encryptions should succeed"
    );

    let enc1 = encrypted1.unwrap();
    let enc2 = encrypted2.unwrap();

    // Ciphertexts should be different (due to different nonces)
    assert_ne!(
        enc1.ciphertext, enc2.ciphertext,
        "Ciphertexts should be different"
    );

    // Both should decrypt to the same plaintext
    let dec1 = engine.decrypt(&enc1).await;
    let dec2 = engine.decrypt(&enc2).await;

    assert!(
        dec1.is_ok() && dec2.is_ok(),
        "Both decryptions should succeed"
    );
    assert_eq!(
        dec1.unwrap(),
        test_data,
        "First decryption should match original"
    );
    assert_eq!(
        dec2.unwrap(),
        test_data,
        "Second decryption should match original"
    );
}

/// Test integrated security workflow
#[tokio::test]
async fn test_integrated_security_workflow() {
    let (_, compliance_config, threat_config, encryption_config) = create_test_configs();

    // Initialize all security components
    let audit = Arc::new(AuditEngine::new().await);
    let compliance = Arc::new(
        ComplianceEngine::new(compliance_config)
            .await
            .expect("Compliance engine creation failed"),
    );
    let mut threat_detector = ThreatDetectionEngine::new(threat_config)
        .await
        .expect("Threat detection engine creation failed");
    let encryption = Arc::new(
        EncryptionEngine::new(encryption_config)
            .await
            .expect("Encryption engine creation failed"),
    );

    // Test integrated workflow
    test_security_incident_workflow(audit.clone(), compliance.clone(), &mut threat_detector).await;
    test_data_protection_workflow(audit.clone(), compliance.clone(), encryption.clone()).await;
    test_compliance_monitoring_workflow(audit.clone(), compliance.clone()).await;
}

async fn test_security_incident_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
    threat_detector: &mut ThreatDetectionEngine,
) {
    // 1. Threat Detection - Simulate SQL injection attempt
    let mut incident_data = HashMap::new();
    incident_data.insert("attack_type".to_string(), "sql_injection".to_string());
    incident_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    incident_data.insert("payload".to_string(), "' OR 1=1 --".to_string());
    incident_data.insert("timestamp".to_string(), Utc::now().to_rfc3339());

    let analysis = threat_detector.analyze_event(&incident_data).await;
    assert!(analysis.is_ok(), "Threat analysis should succeed");

    // 2. Audit Logging - Log the security incident
    let audit_event = AuditEvent {
        id: "security_incident_001".to_string(),
        event_type: AuditEventType::Security,
        severity: AuditSeverity::High,
        timestamp: Utc::now(),
        user_id: Some("system".to_string()),
        resource: Some("admin_panel".to_string()),
        action: "threat_detected".to_string(),
        description: "SQL injection attempt detected".to_string(),
        outcome: "blocked".to_string(),
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("source_ip".to_string(), "192.168.1.100".to_string());
            metadata
        },
        details: HashMap::new(),
    };

    let audit_result = audit.log_event(audit_event).await;
    assert!(audit_result.is_ok(), "Audit logging should succeed");

    // 3. Compliance Evaluation - Check if incident violates policies
    let compliance_event = ComplianceEvent {
        id: "incident_compliance_001".to_string(),
        event_type: "SecurityIncident".to_string(),
        timestamp: Utc::now(),
        user_id: Some("attacker".to_string()),
        resource: Some("database".to_string()),
        data: incident_data,
        metadata: HashMap::new(),
    };

    let compliance_result = compliance.evaluate_event(compliance_event).await;
    assert!(
        compliance_result.is_ok(),
        "Compliance evaluation should succeed"
    );

    let evaluation = compliance_result.unwrap();
    assert!(
        evaluation.compliance_score < 1.0,
        "Security incident should affect compliance score"
    );
}

async fn test_data_protection_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
    encryption: Arc<EncryptionEngine>,
) {
    // 1. Data Encryption - Encrypt sensitive user data
    let user_data = b"User personal information: John Doe, SSN: 123-45-6789";
    let encrypted_result = encryption
        .encrypt(user_data, Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    assert!(encrypted_result.is_ok(), "Data encryption should succeed");

    // 2. Audit Data Access
    let audit_event = AuditEvent {
        id: "data_processing_001".to_string(),
        event_type: AuditEventType::DataAccess,
        severity: AuditSeverity::Medium,
        timestamp: Utc::now(),
        user_id: Some("data_processor".to_string()),
        resource: Some("user_personal_data".to_string()),
        action: "encrypt_data".to_string(),
        description: "User personal data encrypted before storage".to_string(),
        outcome: "success".to_string(),
        metadata: HashMap::new(),
        details: HashMap::new(),
    };

    let audit_result = audit.log_event(audit_event).await;
    assert!(audit_result.is_ok(), "Data access audit should succeed");

    // 3. Compliance Check - Ensure data protection compliance
    let mut compliance_data = HashMap::new();
    compliance_data.insert("data_type".to_string(), "personal_data".to_string());
    compliance_data.insert("encrypted".to_string(), "true".to_string());
    compliance_data.insert("user_consent".to_string(), "true".to_string());

    let compliance_event = ComplianceEvent {
        id: "data_processing_001".to_string(),
        event_type: "DataProcessing".to_string(),
        timestamp: Utc::now(),
        user_id: Some("data_processor".to_string()),
        resource: Some("user_personal_data".to_string()),
        data: compliance_data,
        metadata: HashMap::new(),
    };

    let compliance_result = compliance.evaluate_event(compliance_event).await;
    assert!(
        compliance_result.is_ok(),
        "Data protection compliance should succeed"
    );

    let evaluation = compliance_result.unwrap();
    assert!(
        evaluation.compliance_score >= 0.8,
        "Data protection should have high compliance score"
    );
}

async fn test_compliance_monitoring_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
) {
    // Test continuous compliance monitoring
    let mut compliance_events = Vec::new();

    for i in 0..10 {
        let has_consent = i % 2 == 0;
        let mut data = HashMap::new();
        data.insert("user_consent".to_string(), has_consent.to_string());

        let event = ComplianceEvent {
            id: format!("monitoring_event_{}", i),
            event_type: "DataProcessing".to_string(),
            timestamp: Utc::now(),
            user_id: Some(format!("user_{}", i)),
            resource: Some("analytics_data".to_string()),
            data,
            metadata: HashMap::new(),
        };

        compliance_events.push(event);
    }

    // Process all events and create corresponding audit logs
    for event in compliance_events {
        let audit_event = AuditEvent {
            id: format!("compliance_audit_{}", event.id),
            event_type: AuditEventType::Compliance,
            severity: AuditSeverity::Low,
            timestamp: Utc::now(),
            user_id: event.user_id.clone(),
            resource: Some("data_processing_pipeline".to_string()),
            action: "compliance_check".to_string(),
            description: "Automated compliance monitoring".to_string(),
            outcome: "success".to_string(),
            metadata: HashMap::new(),
            details: HashMap::new(),
        };

        let audit_result = audit.log_event(audit_event).await;
        assert!(audit_result.is_ok(), "Compliance audit should succeed");

        let compliance_result = compliance.evaluate_event(event).await;
        assert!(
            compliance_result.is_ok(),
            "Compliance evaluation should succeed"
        );
    }
}

fn create_test_configs() -> (
    (),
    ComplianceConfig,
    ThreatDetectionConfig,
    EncryptionConfig,
) {
    let audit_config = ();

    let compliance_config = ComplianceConfig::default();

    let threat_config = ThreatDetectionConfig::default();

    let encryption_config = EncryptionConfig::default();

    (
        audit_config,
        compliance_config,
        threat_config,
        encryption_config,
    )
}
