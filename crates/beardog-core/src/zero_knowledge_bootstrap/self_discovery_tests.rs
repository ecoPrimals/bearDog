// SPDX-License-Identifier: AGPL-3.0-only

// Tests for Self-Discovery Engine
use super::*;

#[tokio::test]
async fn test_self_discovery_engine() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;

    // Should create engine successfully
    assert!(engine.discovered_capabilities.is_empty());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Should discover self-identity
    let identity = engine.discover_self_identity()?;

    // Should have unique primal ID (format: type-hostname-uuid)
    assert!(!identity.primal_id.is_empty());
    assert!(
        identity.primal_id.contains('-'),
        "Primal ID should have UUID-based format"
    );

    // Primal ID should NOT hardcode specific primal names
    // It should be self-discovered from PRIMAL_TYPE or default to "primal"
    assert!(
        identity.primal_id.split('-').count() >= 3,
        "Primal ID should have format: type-hostname-uuid"
    );

    // Should have discovered capabilities
    assert!(!identity.capabilities.is_empty());

    // Should have at least one endpoint
    assert!(!identity.endpoints.is_empty());

    // Should have metadata
    assert!(!identity.metadata.version.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_zero_hardcoded_knowledge() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    let identity = engine.discover_self_identity()?;

    // Should not contain hardcoded primal names
    // Validate primal sovereignty - each primal only knows itself
    assert!(!identity.primal_id.is_empty(), "Must have self-identity");

    // ✅ SOVEREIGNTY COMPLIANCE: Primal type should be self-discovered or environment-configured
    // No hardcoded primal type enforcement - could be beardog, songbird, toadstool, or any primal
    let primal_type = PrimalIdEnvInputs::from_env()
        .primal_type
        .unwrap_or_else(|| "primal".to_string());

    assert!(
        identity
            .primal_id
            .to_lowercase()
            .contains(&primal_type.to_lowercase())
            || identity.primal_id.to_lowercase().contains("primal"),
        "Should identify with self-discovered type, got: {}",
        identity.primal_id
    );
    assert!(
        !identity.capabilities.is_empty(),
        "Must know own capabilities"
    );

    // Validate infant discovery - no hardcoded ecosystem knowledge
    for capability in &identity.capabilities {
        let cap_str = format!("{capability:?}");
        assert!(
            !cap_str.to_lowercase().contains("hardcoded"),
            "Capabilities should be discovered dynamically"
        );
    }
    // ✅ SOVEREIGNTY COMPLIANCE: No hardcoded primal assumptions
    assert!(
        !identity.primal_id.is_empty(),
        "Primal ID should be discovered dynamically"
    );

    // Should not have hardcoded endpoints
    for endpoint in &identity.endpoints {
        // Validate endpoint sovereignty - no hardcoded service assumptions
        assert!(!endpoint.url.is_empty(), "Must have endpoint URL");
        assert!(
            !endpoint.url.contains("hardcoded"),
            "URL should be discovered/configured dynamically"
        );

        // Validate proper endpoint format
        assert!(
            endpoint.url.starts_with("http://") || endpoint.url.starts_with("https://"),
            "Endpoint should be proper URL"
        );
    }

    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_capability_auto_detection() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let capabilities = engine.auto_detect_capabilities();

    // Should detect at least security capability
    assert!(
        capabilities
            .iter()
            .any(|c| matches!(c.capability_type, ServiceCapabilityType::Security))
    );

    // All capabilities should have reasonable confidence
    for cap in &capabilities {
        assert!(cap.confidence_score > 0.0);
        assert!(cap.confidence_score <= 1.0);
        assert!(!cap.evidence.is_empty());
    }

    Ok(())
}

// ========================================================================
// Additional Comprehensive Tests
// ========================================================================

#[tokio::test]
async fn test_multiple_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
    let engine1 = SelfDiscoveryEngine::new()?;
    let engine2 = SelfDiscoveryEngine::new()?;

    assert!(engine1.discovered_capabilities.is_empty());
    assert!(engine2.discovered_capabilities.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_primal_id_uniqueness() -> Result<(), Box<dyn std::error::Error>> {
    let id1 = SelfDiscoveryEngine::generate_primal_id_from_inputs(&PrimalIdEnvInputs::default());
    let id2 = SelfDiscoveryEngine::generate_primal_id_from_inputs(&PrimalIdEnvInputs::default());

    assert_ne!(id1, id2, "Primal IDs should be unique");
    Ok(())
}

#[tokio::test]
async fn test_primal_id_format() -> Result<(), Box<dyn std::error::Error>> {
    let primal_id =
        SelfDiscoveryEngine::generate_primal_id_from_inputs(&PrimalIdEnvInputs::default());

    assert!(!primal_id.is_empty(), "Primal ID should not be empty");
    assert!(
        primal_id.contains('-'),
        "Primal ID should have hyphen separators"
    );

    assert!(
        primal_id.split('-').count() >= 3,
        "Primal ID should have at least 3 parts"
    );
    Ok(())
}

#[tokio::test]
async fn test_endpoints_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let identity = engine.discover_self_identity()?;

    assert!(
        identity.endpoints.len() >= 3,
        "Should have multiple endpoints"
    );

    for endpoint in &identity.endpoints {
        assert!(!endpoint.url.is_empty(), "Endpoint URL should not be empty");
        assert!(
            !endpoint.protocols.is_empty(),
            "Endpoint should have protocols"
        );
        assert!(
            endpoint.url.starts_with("http://") || endpoint.url.starts_with("https://"),
            "Endpoint should have valid scheme"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_metadata_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let identity = engine.discover_self_identity()?;

    let metadata = &identity.metadata;
    assert!(!metadata.version.is_empty(), "Should have version");
    assert!(
        !metadata.protocol_versions.is_empty(),
        "Should have protocol versions"
    );
    assert_eq!(
        metadata.health_check_endpoint, "/health",
        "Should have health endpoint"
    );
    assert_eq!(
        metadata.metrics_endpoint, "/metrics",
        "Should have metrics endpoint"
    );

    // Check custom fields
    assert!(
        metadata.custom_fields.contains_key("bootstrap_method"),
        "Should have bootstrap method"
    );
    assert_eq!(
        metadata.custom_fields.get("bootstrap_method").unwrap(),
        "zero_knowledge",
        "Should use zero_knowledge bootstrap"
    );

    Ok(())
}

#[tokio::test]
async fn test_bootstrap_time_is_recent() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let identity = engine.discover_self_identity()?;

    let now = std::time::SystemTime::now();
    let duration = now.duration_since(identity.bootstrap_time)?;

    assert!(
        duration.as_secs() < 10,
        "Bootstrap time should be within last 10 seconds"
    );

    Ok(())
}

#[tokio::test]
async fn test_capabilities_not_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let identity = engine.discover_self_identity()?;

    assert!(
        !identity.capabilities.is_empty(),
        "Should detect at least one capability"
    );
    assert!(
        identity
            .capabilities
            .iter()
            .any(|c| matches!(c, ServiceCapabilityType::Security)),
        "Should detect security capability"
    );

    Ok(())
}

#[tokio::test]
async fn test_discovery_state_tracking() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;

    assert!(
        engine.discovered_capabilities.is_empty(),
        "Should start empty"
    );

    engine.discover_self_identity()?;

    assert!(
        !engine.discovered_capabilities.is_empty(),
        "Should track discovered capabilities"
    );

    Ok(())
}

#[tokio::test]
async fn test_idempotent_discovery() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine1 = SelfDiscoveryEngine::new()?;
    let identity1 = engine1.discover_self_identity()?;

    let mut engine2 = SelfDiscoveryEngine::new()?;
    let identity2 = engine2.discover_self_identity()?;

    // IDs should be different (UUID component)
    assert_ne!(identity1.primal_id, identity2.primal_id);

    // But capabilities should be the same
    assert_eq!(identity1.capabilities.len(), identity2.capabilities.len());

    Ok(())
}

#[tokio::test]
async fn test_capability_detection_functions() -> Result<(), Box<dyn std::error::Error>> {
    // Test individual capability detection functions
    let has_key_mgmt = SelfDiscoveryEngine::detect_key_management_capability();
    let has_hsm = SelfDiscoveryEngine::detect_hsm_capability();
    let has_auth = SelfDiscoveryEngine::detect_authentication_capability();
    let has_compliance = SelfDiscoveryEngine::detect_compliance_capability();
    let has_threat = SelfDiscoveryEngine::detect_threat_detection_capability();

    // Just verify they return boolean values (actual result depends on filesystem)
    // These capabilities may or may not be present - we just verify the detection runs
    let _ = (has_key_mgmt, has_hsm, has_auth, has_compliance, has_threat);

    Ok(())
}

#[tokio::test]
async fn test_endpoint_discovery_functions() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = SelfDiscoveryEnvInputs::default();
    let local = SelfDiscoveryEngine::discover_local_endpoint(&inputs);
    let network = SelfDiscoveryEngine::discover_network_endpoint(&inputs);
    let mesh = SelfDiscoveryEngine::discover_mesh_endpoint(&inputs);

    assert!(local.url.starts_with("http://"), "Local should use HTTP");
    assert!(
        network.url.starts_with("http://"),
        "Network should use HTTP"
    );
    assert!(mesh.url.starts_with("https://"), "Mesh should use HTTPS");

    assert!(local.protocols.contains(&"HTTP".to_string()));
    assert!(network.protocols.contains(&"gRPC".to_string()));
    assert!(mesh.protocols.contains(&"WebSocket".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_self_knowledge_validation() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let capabilities = engine.auto_detect_capabilities();

    let validation = SelfDiscoveryEngine::validate_self_knowledge(&capabilities);
    assert!(validation.is_ok(), "Self-knowledge validation should pass");

    // Test with empty capabilities
    let empty: Vec<SelfCapabilityDetection> = vec![];
    let validation_empty = SelfDiscoveryEngine::validate_self_knowledge(&empty);
    assert!(
        validation_empty.is_err(),
        "Should fail with empty capabilities"
    );

    Ok(())
}

#[tokio::test]
async fn test_capability_evidence_quality() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let capabilities = engine.auto_detect_capabilities();

    for cap in &capabilities {
        assert!(cap.confidence_score > 0.0 && cap.confidence_score <= 1.0);
        assert!(
            cap.auto_detected,
            "All capabilities should be auto-detected"
        );
        assert!(!cap.evidence.is_empty(), "Should have evidence");

        for evidence in &cap.evidence {
            assert!(!evidence.is_empty(), "Evidence should not be empty");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_metadata_building() -> Result<(), Box<dyn std::error::Error>> {
    let caps = vec![SelfCapabilityDetection {
        capability_type: ServiceCapabilityType::Security,
        confidence_score: 1.0,
        evidence: vec!["test".to_string()],
        auto_detected: true,
    }];

    let metadata = SelfDiscoveryEngine::build_self_metadata(
        "test-id",
        &caps,
        &SelfDiscoveryEnvInputs::default(),
    );

    assert!(!metadata.version.is_empty());
    assert_eq!(
        metadata.custom_fields.get("capabilities_count").unwrap(),
        "1"
    );

    Ok(())
}

#[tokio::test]
async fn test_custom_primal_type() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = PrimalIdEnvInputs {
        primal_type: Some("test-primal".to_string()),
        ..Default::default()
    };
    let primal_id = SelfDiscoveryEngine::generate_primal_id_from_inputs(&inputs);
    assert!(
        primal_id.starts_with("test-primal"),
        "Should use custom primal type"
    );
    Ok(())
}

#[tokio::test]
async fn test_custom_hostname() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = PrimalIdEnvInputs {
        hostname: Some("test-host-123".to_string()),
        ..Default::default()
    };
    let primal_id = SelfDiscoveryEngine::generate_primal_id_from_inputs(&inputs);
    assert!(
        primal_id.contains("test-hos") || primal_id.contains("test-host"),
        "Should include hostname"
    );
    Ok(())
}

#[tokio::test]
async fn test_no_vendor_hardcoding() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let identity = engine.discover_self_identity()?;

    // Check primal ID
    let id_lower = identity.primal_id.to_lowercase();
    assert!(!id_lower.contains("aws"), "Should not hardcode AWS");
    assert!(!id_lower.contains("azure"), "Should not hardcode Azure");
    assert!(!id_lower.contains("gcp"), "Should not hardcode GCP");

    // Check endpoints
    for endpoint in &identity.endpoints {
        let url_lower = endpoint.url.to_lowercase();
        assert!(
            !url_lower.contains("aws"),
            "Should not hardcode AWS endpoints"
        );
        assert!(
            !url_lower.contains("azure"),
            "Should not hardcode Azure endpoints"
        );
        assert!(
            !url_lower.contains("gcp"),
            "Should not hardcode GCP endpoints"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_sovereignty_compliance() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SelfDiscoveryEngine::new()?;
    let identity = engine.discover_self_identity()?;

    // Should only know itself
    assert!(!identity.primal_id.is_empty(), "Should know own ID");
    assert!(
        !identity.capabilities.is_empty(),
        "Should know own capabilities"
    );
    assert!(!identity.endpoints.is_empty(), "Should know own endpoints");

    // Should not have hardcoded knowledge of other primals
    for cap in &identity.capabilities {
        let cap_str = format!("{cap:?}");
        assert!(
            !cap_str.contains("songbird") && !cap_str.contains("toadstool"),
            "Should not hardcode other primal names"
        );
    }

    Ok(())
}
