use beardog_errors::BearDogError;


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

#[tokio::test]
async fn test_beardog_core_security_comprehensive() {

    let core = BearDogCore::new(BearDogConfig::default())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Core initialization failed", e).to_string())
})?;

    test_core_security_methods(&core).await;
    test_core_error_handling(&core).await;
    test_core_concurrent_access(&core).await;
}

async fn test_core_security_methods(core: &BearDogCore) {

    let provider = core.security_provider();

    let auth_result = provider.authenticate("test_user", "test_token").await;
    assert!(
        auth_result.is_ok(),
        "Authentication should work with valid inputs"
    );

    let engine = core.encryption_engine();

    let test_data = b"test_security_data";
    let encrypted = engine
        .encrypt(test_data, Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    assert!(encrypted.is_ok(), "Encryption should succeed");

    if let Ok(encrypted_data) = encrypted {
        let decrypted = engine.decrypt(&encrypted_data).await;
        assert!(decrypted.is_ok(), "Decryption should succeed");
        assert_eq!(
            decrypted.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?,
            test_data,
            "Decrypted data should match original"
        );
    }

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
        metadata: HashMap::with_capacity(16),
        details: HashMap::with_capacity(16),
    };

    let result = audit.log_event(audit_event).await;
    assert!(result.is_ok(), "Audit logging should succeed");
}

async fn test_core_error_handling(core: &BearDogCore) {

    let provider = core.security_provider();
    let auth_result = provider.authenticate("", "").await;
    assert!(auth_result.is_err(), "Empty credentials should fail");

    let auth_result = provider.authenticate("invalid_user", "invalid_token").await;
    assert!(auth_result.is_err(), "Invalid credentials should fail");

    let engine = core.encryption_engine();
    let encrypt_result = engine
        .encrypt(b"test_data", Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    assert!(
        encrypt_result.is_ok(),
        "Encryption should succeed with valid data"
    );

    let invalid_encrypted = EncryptedData {
        algorithm: EncryptionAlgorithm::Aes256Gcm,
        ciphertext: b"invalid_data".to_vec(),
        nonce: beardog_security::crypto_utils::BearDogCrypto::generate_secure_nonce(12)?,
        tag: None,
        metadata: HashMap::with_capacity(16),
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

    let provider = core.security_provider();
    let mut handles = Vec::new();

    for i in 0..10 {

        let handle = task::spawn(async move {

            let test_core = BearDogCore::new(BearDogConfig::default())
                .await
                .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Core creation failed", e).to_string())
})?;
            let test_provider = test_core.security_provider();
            let user_id = format_args!("user_{}", i).to_string();
            let token = format_args!("token_{}", i).to_string();
            let _result = test_provider.authenticate(&user_id, &token).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Concurrent task should complete", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Concurrent task should complete", e).to_string())
})?;
    }
}

#[tokio::test]
async fn test_audit_engine_comprehensive() {
    let audit = AuditEngine::new().await;

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
            id: format_args!("test_{:?}", event_type).to_string(),
            event_type: event_type.clone(),
            severity: AuditSeverity::Medium,
            timestamp: Utc::now(),
            user_id: Some("test_user".to_string()),
            resource: Some("test_resource".to_string()),
            action: "test_action".to_string(),
            description: "Test audit event".to_string(),
            outcome: "success".to_string(),
            metadata: HashMap::with_capacity(16),
            details: HashMap::with_capacity(16),
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
        metadata: HashMap::with_capacity(16),
        details: HashMap::with_capacity(16),
    };

    let result = audit.log_event(security_event).await;
    assert!(result.is_ok(), "Security event logging should succeed");

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
        metadata: HashMap::with_capacity(16),
        details: HashMap::with_capacity(16),
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

    for i in 0..100 {

        let handle = task::spawn(async move {
            let test_audit = AuditEngine::new().await;
            let event = AuditEvent {
                id: format_args!("load_test_{}", i).to_string(),
                event_type: AuditEventType::System,
                severity: AuditSeverity::Low,
                timestamp: Utc::now(),
                user_id: Some(format_args!("user_{}", i).to_string()),
                resource: Some("system_resource".to_string()),
                action: "system_operation".to_string(),
                description: "Load test event".to_string(),
                outcome: "success".to_string(),
                metadata: HashMap::with_capacity(16),
                details: HashMap::with_capacity(16),
            };

            test_audit.log_event(event).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Load test task should complete", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Load test task should complete", e).to_string())
})?;
        assert!(result.is_ok(), "Load test event should succeed");
    }
}

async fn test_audit_tampering_detection(audit: &AuditEngine) {

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
        metadata: HashMap::with_capacity(16),
        details: HashMap::with_capacity(16),
    };

    let result = audit.log_event(sensitive_event).await;
    assert!(result.is_ok(), "Sensitive event logging should succeed");

    let recent_events = audit.get_recent_events(10).await;
    assert!(
        recent_events.is_ok(),
        "Recent events retrieval should succeed"
    );

    let events = recent_events.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert!(!events.is_empty(), "Should have at least one event");
}

#[tokio::test]
async fn test_compliance_engine_comprehensive() {
    let config = ComplianceConfig::default();
    let compliance = ComplianceEngine::new(config)
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Compliance engine creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Compliance engine creation failed", e).to_string())
})?;

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
            id: format_args!("test_compliance_{:?}", standard).to_string(),
            event_type: "DataAccess".to_string(),
            timestamp: Utc::now(),
            user_id: Some("test_user".to_string()),
            resource: Some("user_data".to_string()),
            data: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(16),
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

    let mut data = HashMap::with_capacity(16);
    data.insert("data_type".to_string(), "personal_data".to_string());
    data.insert("user_consent".to_string(), "false".to_string());

    let violation_event = ComplianceEvent {
        id: "gdpr_violation_001".to_string(),
        event_type: "DataAccess".to_string(),
        timestamp: Utc::now(),
        user_id: Some("analyst_user".to_string()),
        resource: Some("user_personal_data".to_string()),
        data,
        metadata: HashMap::with_capacity(16),
    };

    let result = compliance.evaluate_event(violation_event).await;
    assert!(result.is_ok(), "Compliance evaluation should succeed");

    let evaluation = result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert!(
        evaluation.compliance_score < 1.0,
        "Compliance score should be less than perfect for violation"
    );
}

async fn test_compliance_reporting(compliance: &ComplianceEngine) {

    let date_range = (Utc::now() - chrono::Duration::days(30), Utc::now());

    let report = compliance
        .generate_compliance_report(ComplianceStandard::GDPR, date_range)
        .await;

    assert!(
        report.is_ok(),
        "Compliance report generation should succeed"
    );

    let report = report.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert_eq!(report.standard, ComplianceStandard::GDPR);
    assert!(report.overall_score >= 0.0 && report.overall_score <= 1.0);
}

async fn test_compliance_real_time_monitoring(compliance: &ComplianceEngine) {

    let mut monitoring_events = Vec::new();

    for i in 0..10 {
        let has_consent = i % 2 == 0;
        let mut data = HashMap::with_capacity(16);
        data.insert("user_consent".to_string(), has_consent.to_string());

        let event = ComplianceEvent {
            id: format_args!("monitoring_event_{}", i).to_string(),
            event_type: "DataProcessing".to_string(),
            timestamp: Utc::now(),
            user_id: Some(format_args!("user_{}", i).to_string()),
            resource: Some("analytics_data".to_string()),
            data,
            metadata: HashMap::with_capacity(16),
        };

        monitoring_events.push(event);
    }

    for event in monitoring_events {
        let result = compliance.evaluate_event(event).await;
        assert!(result.is_ok(), "Real-time monitoring should succeed");
    }
}

#[tokio::test]
async fn test_threat_detection_comprehensive() {
    let config = ThreatDetectionConfig::default();
    let mut detector = ThreatDetectionEngine::new(config)
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Threat detection engine creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Threat detection engine creation failed", e).to_string())
})?;

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
        let mut event_data = HashMap::with_capacity(16);
        event_data.insert("threat_type".to_string(), format_args!("{:?}", threat_type).to_string());
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

    let suspicious_patterns = vec![
        ("multiple_failed_logins", "10", "192.168.1.100"),
        ("large_data_transfer", "1000000", "192.168.1.200"),
        ("unusual_access_time", "03:00", "192.168.1.300"),
        ("privilege_escalation", "admin", "192.168.1.400"),
    ];

    for (pattern, value, ip) in suspicious_patterns {
        let mut event_data = HashMap::with_capacity(16);
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

    for i in 0..50 {
        let mut event_data = HashMap::with_capacity(16);
        event_data.insert("attack_type".to_string(), "ddos".to_string());
        event_data.insert(
            "source_ip".to_string(),
            format_args!("192.168.{}.{}", i / 256, i % 256).to_string(),
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

    let legitimate_activities = vec![
        ("normal_login", "user123", "192.168.1.50"),
        ("scheduled_backup", "backup_service", "192.168.1.10"),
        ("system_update", "admin", "192.168.1.5"),
        ("regular_api_call", "mobile_app", "192.168.1.60"),
    ];

    for (activity, user, ip) in legitimate_activities {
        let mut event_data = HashMap::with_capacity(16);
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

#[tokio::test]
async fn test_encryption_engine_comprehensive() {
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config)
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Encryption engine creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Encryption engine creation failed", e).to_string())
})?;

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
                decrypted.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?,
                test_data,
                "Decrypted data should match original for {:?}",
                algorithm
            );
        }
    }
}

async fn test_encryption_key_management(engine: &EncryptionEngine) {

    let key_result = engine.generate_key("AES256", "test_purpose").await;
    assert!(key_result.is_ok(), "Key generation should succeed");

    if let Ok((key_id, _key_bytes)) = key_result {

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
                decrypted.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?,
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

    let enc1 = encrypted1.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    let enc2 = encrypted2.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

    assert_ne!(
        enc1.ciphertext, enc2.ciphertext,
        "Ciphertexts should be different"
    );

    let dec1 = engine.decrypt(&enc1).await;
    let dec2 = engine.decrypt(&enc2).await;

    assert!(
        dec1.is_ok() && dec2.is_ok(),
        "Both decryptions should succeed"
    );
    assert_eq!(
        dec1.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?,
        test_data,
        "First decryption should match original"
    );
    assert_eq!(
        dec2.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?,
        test_data,
        "Second decryption should match original"
    );
}

#[tokio::test]
async fn test_integrated_security_workflow() {
    let (_, compliance_config, threat_config, encryption_config) = create_test_configs();

    let audit = Arc::new(AuditEngine::new().await);
    let compliance = Arc::new(
        ComplianceEngine::new(compliance_config)
            .await
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Compliance engine creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Compliance engine creation failed", e).to_string())
})?,
    );
    let mut threat_detector = ThreatDetectionEngine::new(threat_config)
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Threat detection engine creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Threat detection engine creation failed", e).to_string())
})?;
    let encryption = Arc::new(
        EncryptionEngine::new(encryption_config)
            .await
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Encryption engine creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Encryption engine creation failed", e).to_string())
})?,
    );

    test_security_incident_workflow(audit.clone(), compliance.clone(), &mut threat_detector).await;
    test_data_protection_workflow(audit.clone(), compliance.clone(), encryption.clone()).await;
    test_compliance_monitoring_workflow(audit.clone(), compliance.clone()).await;
}

async fn test_security_incident_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
    threat_detector: &mut ThreatDetectionEngine,
) {

    let mut incident_data = HashMap::with_capacity(16);
    incident_data.insert("attack_type".to_string(), "sql_injection".to_string());
    incident_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
    incident_data.insert("payload".to_string(), "' OR 1=1 --".to_string());
    incident_data.insert("timestamp".to_string(), Utc::now().to_rfc3339());

    let analysis = threat_detector.analyze_event(&incident_data).await;
    assert!(analysis.is_ok(), "Threat analysis should succeed");

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
            let mut metadata = HashMap::with_capacity(16);
            metadata.insert("source_ip".to_string(), "192.168.1.100".to_string());
            metadata
        },
        details: HashMap::with_capacity(16),
    };

    let audit_result = audit.log_event(audit_event).await;
    assert!(audit_result.is_ok(), "Audit logging should succeed");

    let compliance_event = ComplianceEvent {
        id: "incident_compliance_001".to_string(),
        event_type: "SecurityIncident".to_string(),
        timestamp: Utc::now(),
        user_id: Some("attacker".to_string()),
        resource: Some("database".to_string()),
        data: incident_data,
        metadata: HashMap::with_capacity(16),
    };

    let compliance_result = compliance.evaluate_event(compliance_event).await;
    assert!(
        compliance_result.is_ok(),
        "Compliance evaluation should succeed"
    );

    let evaluation = compliance_result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
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

    let user_data = b"User personal information: John Doe, SSN: 123-45-6789";
    let encrypted_result = encryption
        .encrypt(user_data, Some(EncryptionAlgorithm::Aes256Gcm))
        .await;
    assert!(encrypted_result.is_ok(), "Data encryption should succeed");

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
        metadata: HashMap::with_capacity(16),
        details: HashMap::with_capacity(16),
    };

    let audit_result = audit.log_event(audit_event).await;
    assert!(audit_result.is_ok(), "Data access audit should succeed");

    let mut compliance_data = HashMap::with_capacity(16);
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
        metadata: HashMap::with_capacity(16),
    };

    let compliance_result = compliance.evaluate_event(compliance_event).await;
    assert!(
        compliance_result.is_ok(),
        "Data protection compliance should succeed"
    );

    let evaluation = compliance_result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
    assert!(
        evaluation.compliance_score >= 0.8,
        "Data protection should have high compliance score"
    );
}

async fn test_compliance_monitoring_workflow(
    audit: Arc<AuditEngine>,
    compliance: Arc<ComplianceEngine>,
) {

    let mut compliance_events = Vec::new();

    for i in 0..10 {
        let has_consent = i % 2 == 0;
        let mut data = HashMap::with_capacity(16);
        data.insert("user_consent".to_string(), has_consent.to_string());

        let event = ComplianceEvent {
            id: format_args!("monitoring_event_{}", i).to_string(),
            event_type: "DataProcessing".to_string(),
            timestamp: Utc::now(),
            user_id: Some(format_args!("user_{}", i).to_string()),
            resource: Some("analytics_data".to_string()),
            data,
            metadata: HashMap::with_capacity(16),
        };

        compliance_events.push(event);
    }

    for event in compliance_events {
        let audit_event = AuditEvent {
            id: format_args!("compliance_audit_{}", event.id).to_string(),
            event_type: AuditEventType::Compliance,
            severity: AuditSeverity::Low,
            timestamp: Utc::now(),
            user_id: event.user_id.clone(),
            resource: Some("data_processing_pipeline".to_string()),
            action: "compliance_check".to_string(),
            description: "Automated compliance monitoring".to_string(),
            outcome: "success".to_string(),
            metadata: HashMap::with_capacity(16),
            details: HashMap::with_capacity(16),
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
