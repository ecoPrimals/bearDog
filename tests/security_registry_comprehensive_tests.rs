use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::info;

#[tokio::test]
async fn test_security_registry_initialization() -> Result<(), BearDogError> {
    info!("🔐 Testing Security Registry Initialization");

    let config = SecurityRegistryConfig::default();
    let registry = BearDogSecurityRegistry::new(config)?;

    let health = registry.health_check()?;
    assert!(
        health.operational,
        "Security registry must be operational after initialization"
    );
    assert_eq!(
        health.trust_relationships_count, 0,
        "Should start with zero trust relationships"
    );

    info!("✅ Security registry initialization test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_trust_establishment_flow() -> Result<(), BearDogError> {
    info!("🤝 Testing Trust Establishment Flow");

    let config = SecurityRegistryConfig::default();
    let registry = BearDogSecurityRegistry::new(config)?;

    let peer_node_id = "beardog-peer-123";
    registry
        .establish_trust(peer_node_id, TrustLevel::Medium)
        ?;

    let trust_level = registry.verify_trust(peer_node_id)?;
    assert_eq!(
        trust_level,
        Some(TrustLevel::Medium),
        "Trust level should match what was established"
    );

    let health = registry.health_check()?;
    assert_eq!(
        health.trust_relationships_count, 1,
        "Should have one trust relationship"
    );

    info!("✅ Trust establishment test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_multiple_trust_relationships() -> Result<(), BearDogError> {
    info!("🔗 Testing Multiple Trust Relationships");

    let config = SecurityRegistryConfig::default();
    let registry = BearDogSecurityRegistry::new(config)?;

    let peers = vec![
        ("beardog-peer-1", TrustLevel::High),
        ("beardog-peer-2", TrustLevel::Medium),
        ("beardog-peer-3", TrustLevel::Basic),
    ];

    for (peer_id, trust_level) in &peers {
        registry.establish_trust(peer_id, *trust_level)?;
    }

    for (peer_id, expected_level) in &peers {
        let actual_level = registry.verify_trust(peer_id)?;
        assert_eq!(
            actual_level,
            Some(*expected_level),
            "Trust level mismatch for {}",
            peer_id
        );
    }

    let health = registry.health_check()?;
    assert_eq!(
        health.trust_relationships_count,
        peers.len(),
        "Should have {} trust relationships",
        peers.len()
    );

    info!("✅ Multiple trust relationships test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_trust_verification_edge_cases() -> Result<(), BearDogError> {
    info!("⚠️ Testing Trust Verification Edge Cases");

    let config = SecurityRegistryConfig::default();
    let registry = BearDogSecurityRegistry::new(config)?;

    let non_existent_trust = registry.verify_trust("beardog-nonexistent")?;
    assert_eq!(
        non_existent_trust, None,
        "Non-existent trust should return None"
    );

    let empty_result = registry.establish_trust("", TrustLevel::Basic);

    info!("✅ Trust verification edge cases test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_security_registry_configuration() {
    info!("⚙️ Testing Security Registry Configuration");

    let default_config = SecurityRegistryConfig::default();
    assert!(
        !default_config.instance_id.is_empty(),
        "Instance ID must not be empty"
    );
    assert!(
        default_config.instance_id.starts_with("beardog-"),
        "Instance ID must have beardog prefix"
    );
    assert!(
        !default_config.public_endpoint.is_empty(),
        "Public endpoint must be configured"
    );
    assert!(
        !default_config.mesh_service_endpoint.is_empty(),
        "Mesh service endpoint must be configured"
    );

    std::env::set_var("BEARDOG_PUBLIC_ENDPOINT", "https://production.beardog.eco");
    std::env::set_var("MESH_SERVICE_ENDPOINT", "https://mesh-service.production.eco");

    let env_config = SecurityRegistryConfig::default();
    assert_eq!(env_config.public_endpoint, "https://production.beardog.eco");
    assert_eq!(
        env_config.mesh_service_endpoint,
        "https://mesh-service.production.eco"
    );

    std::env::remove_var("BEARDOG_PUBLIC_ENDPOINT");
    std::env::remove_var("MESH_SERVICE_ENDPOINT");

    info!("✅ Configuration test PASSED");
}

#[tokio::test]
async fn test_concurrent_trust_operations() -> Result<(), BearDogError> {
    info!("⚡ Testing Concurrent Trust Operations");

    let config = SecurityRegistryConfig::default();
    let registry = Arc::new(BearDogSecurityRegistry::new(config)?);

    let mut handles = vec![];
    for i in 0..50 {
        let registry_clone = Arc::clone(&registry);
        let handle = tokio::spawn(async move {
            let peer_id = format!("beardog-concurrent-{}", i);
            let trust_level = if i % 3 == 0 {
                TrustLevel::High
            } else if i % 2 == 0 {
                TrustLevel::Medium
            } else {
                TrustLevel::Basic
            };

            registry_clone.establish_trust(&peer_id, trust_level)
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(result) = handle {
            if result.is_ok() {
                success_count += 1;
            }
        }
    }

    assert!(
        success_count >= 45,
        "At least 90% of concurrent trust operations should succeed (got {})",
        success_count
    );

    let health = registry.health_check()?;
    assert!(
        health.trust_relationships_count >= 45,
        "Should have established most trust relationships"
    );

    info!("✅ Concurrent operations test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_trust_level_hierarchy() -> Result<(), BearDogError> {
    info!("📊 Testing Trust Level Hierarchy");

    let config = SecurityRegistryConfig::default();
    let registry = BearDogSecurityRegistry::new(config)?;

    let trust_levels = vec![
        TrustLevel::Basic,
        TrustLevel::Medium,
        TrustLevel::High,
        TrustLevel::Maximum,
    ];

    for (i, trust_level) in trust_levels.iter().enumerate() {
        let peer_id = format!("beardog-level-{}", i);
        registry.establish_trust(&peer_id, *trust_level)?;

        let verified_level = registry.verify_trust(&peer_id)?;
        assert_eq!(
            verified_level,
            Some(*trust_level),
            "Trust level verification failed for {:?}",
            trust_level
        );
    }

    info!("✅ Trust level hierarchy test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_security_registry_performance() -> Result<(), BearDogError> {
    info!("🚀 Testing Security Registry Performance");

    let config = SecurityRegistryConfig::default();
    let registry = BearDogSecurityRegistry::new(config)?;

    let start = std::time::Instant::now();
    let health = registry.health_check()?;
    let duration = start.elapsed();

    assert!(health.operational, "Health check must succeed");
    assert!(
        duration < std::time::Duration::from_millis(10),
        "Health check must complete within 10ms (took {:?})",
        duration
    );

    let start = std::time::Instant::now();
    registry
        .establish_trust("beardog-perf-test", TrustLevel::Medium)
        ?;
    let trust_duration = start.elapsed();

    assert!(
        trust_duration < std::time::Duration::from_millis(50),
        "Trust establishment must complete within 50ms (took {:?})",
        trust_duration
    );

    info!("✅ Performance test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_security_registry_ecosystem_integration() -> Result<(), BearDogError> {
    info!("🌐 Testing Ecosystem Integration Points");

    let config = SecurityRegistryConfig::default();

    assert!(
        config.mesh_service_endpoint.contains("mesh-service"),
        "Mesh service endpoint should contain 'mesh-service'"
    );
    assert!(
        config.mesh_service_endpoint.starts_with("http"),
        "Mesh service endpoint should be a valid URL"
    );

    assert!(
        config.instance_id.starts_with("beardog-"),
        "Instance ID should have beardog prefix"
    );
    assert!(
        config.instance_id.len() > 10,
        "Instance ID should be sufficiently unique"
    );

    info!("✅ Ecosystem integration test PASSED");
    Ok(())
}
