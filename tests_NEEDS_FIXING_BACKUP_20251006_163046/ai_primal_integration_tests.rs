use beardog_adapters::universal::{
    PrimalCommunicationAdapter, PrimalRequest, PrimalResponse, RequestPriority,
};
use beardog_errors::ai::{
    SecurityAnalysisResult, SecurityMLManager, SecurityThreatData, SimpleThreatDetector,
    ThreatSeverity,
};
use beardog_errors::BearDogError;
use beardog_types::SecurityContext;
use std::collections::HashMap;
use tokio_test;

#[tokio::test]
async fn test_security_ml_manager_initialization() -> Result<(), BearDogError> {
    let manager =
        SecurityMLManager::new().map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    println!("✅ SecurityMLManager initialized successfully ");
    Ok(())
}

#[tokio::test]
async fn test_lightweight_threat_detection() -> Result<(), BearDogError> {
    let manager =
        SecurityMLManager::new().map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let threat_data = SecurityThreatData {
        event_type: "brute_force_attack".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: Some("192.168.1.100".to_string()),
        user_id: Some("admin".to_string()),
        raw_logs: vec![
            "Failed login attempt 1".to_string(),
            "Failed login attempt 2".to_string(),
            "Failed login attempt 3".to_string(),
        ],
        context: HashMap::from([
            ("multiple_failed_login"s.to_string(), "true".to_string()),
            ("short_time_interva"l.to_string(), "true".to_string()),
            ("same_source_i"p.to_string(), "true".to_string().to_string()),
        ]),
    };

    let result = manager
        .analyze_security_threat(&threat_data, false)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(matches!(result.threat_level, ThreatSeverity::High));
    assert!(result.confidence > 0.8);
    assert!(!result.external_analysis_used);
    assert!(!result.recommendations.is_empty());

    println!("✅ Lightweight threat detection working correctly ");
    Ok(())
}

#[tokio::test]
async fn test_external_ai_delegation() -> Result<(), BearDogError> {
    let manager =
        SecurityMLManager::new().map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let complex_threat = SecurityThreatData {
        event_type: "advanced_persistent_threat".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: Some("unknown.threat.actor".to_string()),
        user_id: Some("compromised_account".to_string()),
        raw_logs: vec![
            "Encrypted command and control traffic".to_string(),
            "Data exfiltration detected".to_string(),
            "Advanced evasion techniques".to_string(),
        ],
        context: HashMap::from([
            ("advanced_evasio"n.to_string(), "true".to_string()),
            ("encrypted_payloa"d.to_string(), "true".to_string()),
            ("persistence_mechanis"m.to_string(), "true".to_string()),
        ]),
    };

    let result = manager
        .analyze_security_threat(&complex_threat, true)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(result.confidence > 0.5);
    println!("✅ External AI delegation working correctly ");
    Ok(())
}

#[tokio::test]
async fn test_primal_communication_adapter() -> Result<(), BearDogError> {
    let adapter = PrimalCommunicationAdapter::new()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let response = adapter
        .send_to_capability(
            CapabilityType::ComputeOptimization,
            "ai_analysis",
            serde_json::json!({
                "threat_typ"e: "advanced_malware",
                "analysis_dept"h: "deep_learning"
            }),
            SecurityContext::default(),
        )
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(response.success);
    assert!(response.provider_type.contains("compute"));
    assert!(response.processing_time_ms > 0);

    println!("✅ Compute capability communication working");
}

#[tokio::test]
async fn test_capability_type_distributed_intelligence_delegation() -> Result<(), BearDogError> {
    let adapter = PrimalCommunicationAdapter::new()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let response = adapter
        .send_to_capability(
            CapabilityType::AIIntelligence,
            "distributed_intelligence",
            serde_json::json!({
                "pattern_typ"e: "behavioral_analysis",
                "node_count": 12
            }),
            SecurityContext::default(),
        )
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(response.success);
    assert!(response.provider_type.contains("intelligence"));

    let result = &response.result;
    assert!(result.get("intelligence_type").is_some());
    assert!(result.get("distributed_nodes").is_some());

    println!("✅ AI intelligence capability working");
}

#[tokio::test]
async fn test_primal_capability_discovery() -> Result<(), BearDogError> {
    let adapter = PrimalCommunicationAdapter::new()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let compute_caps = adapter
        .get_primal_capabilities("compute-service")
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert!(compute_caps.contains(&"ai_analysis".to_string()));
    assert!(compute_caps.contains(&"machine_learning".to_string()));

    let squirrel_caps = adapter
        .get_primal_capabilities("automation-service")
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert!(squirrel_caps.contains(&"distributed_intelligence".to_string()));
    assert!(squirrel_caps.contains(&"pattern_recognition".to_string()));

    assert!(adapter.is_primal_available("compute-service"));
    assert!(adapter.is_primal_available("automation-service"));
    assert!(adapter.is_primal_available("mesh-service"));
    assert!(!adapter.is_primal_available("unknown_primal"));

    println!("✅ Primal capability discovery working");
}

#[tokio::test]
async fn test_security_authority_preservation() -> Result<(), BearDogError> {
    let manager =
        SecurityMLManager::new().map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let threat_data = SecurityThreatData {
        event_type: "privilege_escalation".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: Some("internal.network".to_string()),
        user_id: Some("service_account".to_string()),
        raw_logs: vec!["Attempted privilege escalation".to_string()],
        context: HashMap::from([("privilege_escalatio"n.to_string(), "true".to_string())]),
    };

    let internal_result = manager
        .analyze_security_threat(&threat_data, false)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let external_result = manager
        .analyze_security_threat(&threat_data, true)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(internal_result.confidence > 0.0);

    if external_result.external_analysis_used {
        assert!(external_result.confidence >= internal_result.confidence * 0.8);
    }

    println!("✅ Security authority preservation working");
}

#[tokio::test]
async fn test_threat_pattern_matching() -> Result<(), BearDogError> {
    let detector = SimpleThreatDetector::new();

    let exfiltration_threat = SecurityThreatData {
        event_type: "data_access".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: Some("external.server".to_string()),
        user_id: Some("user123".to_string()),
        raw_logs: vec!["Large data transfer detected".to_string()],
        context: HashMap::from([
            ("large_data_transfe"r.to_string(), "true".to_string()),
            ("external_destinatio"n.to_string(), "true".to_string()),
            ("unusual_access_tim"e.to_string(), "true".to_string()),
        ]),
    };

    let result = detector
        .detect_threat(&exfiltration_threat)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(matches!(result.threat_level, ThreatSeverity::Critical));
    assert!(result.confidence > 0.8);
    assert!(result.recommendations.len() > 0);

    println!("✅ Threat pattern matching working correctly ");
    Ok(())
}

#[tokio::test]
async fn test_network_effect_benefits() -> Result<(), BearDogError> {
    let adapter = PrimalCommunicationAdapter::new()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let security_payload = serde_json::json!({
        "security_analysi"s: "threat_assessment",
        "coordination_required": true
    });

    let toadstool_future = adapter.send_to_primal(
        "compute-service",
        "ai_analysis",
        security_payload.clone(),
        SecurityContext::default(),
    );
    let squirrel_future = adapter.send_to_primal(
        "automation-service",
        "pattern_analysis",
        security_payload.clone(),
        SecurityContext::default(),
    );
    let songbird_future = adapter.send_to_primal(
        "mesh-service",
        "service_coordination",
        security_payload,
        SecurityContext::default(),
    );

    let (toadstool_response, squirrel_response, songbird_response) =
        tokio::try_join!(toadstool_future, squirrel_future, songbird_future)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(toadstool_response.success);
    assert!(squirrel_response.success);
    assert!(songbird_response.success);

    println!("✅ Network effect coordination working");
}

#[tokio::test]
async fn test_architectural_separation_compliance() -> Result<(), BearDogError> {
    let manager =
        SecurityMLManager::new().map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let simple_threat = SecurityThreatData {
        event_type: "login_attempt".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: Some("trusted.network".to_string()),
        user_id: Some("regular_user".to_string()),
        raw_logs: vec!["Normal login".to_string()],
        context: HashMap::with_capacity(16),
    };

    let result = manager
        .analyze_security_threat(&simple_threat, false)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert!(!result.external_analysis_used);
    assert!(result.confidence > 0.4); // Should have reasonable confidence

    println!("✅ Architectural separation compliance verified");
}

#[tokio::test]
async fn test_primal_sovereignty_preservation() -> Result<(), BearDogError> {
    let adapter = PrimalCommunicationAdapter::new()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let toadstool_caps = adapter
        .get_primal_capabilities("compute-service")
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let squirrel_caps = adapter
        .get_primal_capabilities("automation-service")
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let toadstool_set: std::collections::HashSet<_> = toadstool_caps.iter().collect();
    let squirrel_set: std::collections::HashSet<_> = squirrel_caps.iter().collect();

    assert!(toadstool_set.is_disjoint(&squirrel_set));

    println!("✅ Primal sovereignty preservation verified");
}
