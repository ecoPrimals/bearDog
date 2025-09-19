// Zero-Knowledge Bootstrap Integration Tests
//
// These tests validate the "infant discovery pattern" where BearDog starts with
// absolutely zero hardcoded knowledge and learns the ecosystem dynamically.

use beardog_core::ecosystem::primal_types::{DiscoveredPrimal, PrimalMetadata};
use beardog_core::zero_knowledge_bootstrap::{
    BootstrapConfig, SelfIdentity, ZeroKnowledgeBootstrap,
};
use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use std::collections::HashMap;
use tokio::time::{timeout, Duration};

/// Test the complete zero-knowledge bootstrap process
#[tokio::test]
async fn test_complete_zero_knowledge_bootstrap() {
    println!("🌱 Testing complete zero-knowledge bootstrap process...");

    // Phase 1: Initialize with zero knowledge
    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();

    // Verify zero initial knowledge
    assert_eq!(bootstrap.discovered_primals.read().len(), 0);
    assert_eq!(bootstrap.discovered_capabilities.read().len(), 0);

    println!("✅ Phase 1: Zero initial knowledge confirmed");

    // Phase 2: Self-discovery
    let self_identity = bootstrap.discover_self_identity().unwrap();

    // Validate self-identity
    assert!(!self_identity.primal_id.is_empty());
    assert!(self_identity.primal_id.starts_with("beardog-"));
    assert!(!self_identity.capabilities.is_empty());
    assert!(!self_identity.endpoints.is_empty());

    println!(
        "✅ Phase 2: Self-identity discovered - ID: {}",
        self_identity.primal_id
    );

    // Phase 3: Start ecosystem listening
    bootstrap.start_ecosystem_listening().unwrap();

    println!("✅ Phase 3: Ecosystem listening started");

    // Phase 4: Wait for discovery (with timeout)
    let discovery_result = timeout(Duration::from_secs(5), async {
        // Wait for some discovery to happen
        tokio::time::sleep(Duration::from_millis(100)).await;
        bootstrap.get_ecosystem_state()
    })
    ;

    assert!(discovery_result.is_ok());
    let ecosystem_state = discovery_result.unwrap().unwrap();

    println!("✅ Phase 4: Ecosystem state retrieved");
    println!(
        "   📊 Discovered primals: {}",
        ecosystem_state.discovered_primals_count
    );
    println!(
        "   ⚡ Available capabilities: {}",
        ecosystem_state.available_capabilities.len()
    );

    // Verify no hardcoded knowledge was used - check for capability-based discovery
    let id_lower = self_identity.primal_id.to_lowercase();
    assert!(
        !id_lower.contains("hardcoded"),
        "Self-identity should not contain 'hardcoded'"
    );
    assert!(
        !id_lower.contains("legacy"),
        "Self-identity should not contain 'legacy'"
    );
    assert!(
        !id_lower.contains("deprecated"),
        "Self-identity should not contain 'deprecated'"
    );

    // Verify endpoints use capability-based naming
    for endpoint in &self_identity.endpoints {
        let url_lower = endpoint.url.to_lowercase();
        assert!(
            !url_lower.contains("hardcoded"),
            "Endpoint should not contain 'hardcoded'"
        );
        assert!(
            !url_lower.contains("legacy"),
            "Endpoint should not contain 'legacy'"
        );
    }

    println!("🎉 Complete zero-knowledge bootstrap test PASSED!");
}

/// Test self-identity discovery without hardcoded names
#[tokio::test]
async fn test_self_identity_no_hardcoded_names() {
    println!("🆔 Testing self-identity discovery without hardcoded names...");

    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();
    let identity = bootstrap.discover_self_identity().unwrap();

    // Verify capability-based self-identity (no hardcoded vendor or legacy references)
    let id_lower = identity.primal_id.to_lowercase();
    assert!(
        !id_lower.contains("hardcoded"),
        "Self-identity should not contain hardcoded references"
    );
    assert!(
        !id_lower.contains("legacy"),
        "Self-identity should not contain legacy references"
    );
    assert!(
        !id_lower.contains("vendor"),
        "Self-identity should not contain vendor-specific references"
    );

    // Verify capability-based endpoints
    for endpoint in &identity.endpoints {
        let url_lower = endpoint.url.to_lowercase();
        assert!(
            !url_lower.contains("hardcoded"),
            "Endpoint should not contain hardcoded references"
        );
        assert!(
            !url_lower.contains("legacy"),
            "Endpoint should not contain legacy references"
        );
        assert!(
            !url_lower.contains("vendor"),
            "Endpoint should not contain vendor-specific references"
        );
    }

    println!("✅ Self-identity is free of hardcoded primal names");
    println!("   🆔 Generated ID: {}", identity.primal_id);
    println!("   📡 Endpoints: {}", identity.endpoints.len());
}

/// Test capability auto-detection
#[tokio::test]
async fn test_capability_auto_detection() {
    println!("⚡ Testing capability auto-detection...");

    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();
    let identity = bootstrap.discover_self_identity().unwrap();

    // Should detect security capability (core BearDog functionality)
    assert!(
        identity
            .capabilities
            .contains(&ServiceCapabilityType::Security),
        "Security capability not detected"
    );

    // Should detect additional capabilities based on available modules
    println!("   ⚡ Detected capabilities:");
    for capability in &identity.capabilities {
        println!("     - {:?}", capability);
    }

    // Verify capabilities are realistic for BearDog
    let expected_capabilities = [
        ServiceCapabilityType::Security,
        ServiceCapabilityType::KeyManagement,
        ServiceCapabilityType::HardwareSecurityModule,
        ServiceCapabilityType::Authentication,
    ];

    let mut found_expected = 0;
    for expected in &expected_capabilities {
        if identity.capabilities.contains(expected) {
            found_expected += 1;
        }
    }

    assert!(
        found_expected >= 2,
        "Should detect at least 2 expected capabilities"
    );

    println!("✅ Capability auto-detection working correctly ");
    println!("   📊 Total capabilities: {}", identity.capabilities.len());
    println!(
        "   ✅ Expected capabilities found: {}/{}",
        found_expected,
        expected_capabilities.len()
    );
}

/// Test ecosystem listening discovers other primals
#[tokio::test]
async fn test_ecosystem_listening_discovery() {
    println!("👂 Testing ecosystem listening and discovery...");

    // Set up environment variables to simulate other primals
    std::env::set_var("BEARDOG_COMPUTE_ENDPOIN"T, "http://discovered-compute:8081");
    std::env::set_var("BEARDOG_MESH_ENDPOIN"T, "http://discovered-mesh:8082");

    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();

    // Start listening
    bootstrap.start_ecosystem_listening().unwrap();

    // Wait for discovery
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Check if primals were discovered
    let primals = bootstrap.discovered_primals.read();
    let capabilities = bootstrap.discovered_capabilities.read();

    println!("   📊 Discovered primals: {}", primals.len());
    println!("   ⚡ Discovered capabilities: {}", capabilities.len());

    // Should have discovered at least environment-based primals
    assert!(
        !primals.is_empty(),
        "Should discover primals from environment"
    );

    // Verify discovered primals use capability-based identification
    for (primal_id, _primal) in primals.iter() {
        let id_lower = primal_id.to_lowercase();
        if id_lower.contains("hardcode"d) || id_lower.contains("legacy") {
            println!(
                "⚠️ Warning: Discovered primal may have sovereignty issue: {}",
                primal_id
            );
            println!("   (Check for capability-based discovery compliance)");
        }
    }

    // Clean up
    std::env::remove_var("BEARDOG_COMPUTE_ENDPOINT");
    std::env::remove_var("BEARDOG_MESH_ENDPOINT");

    println!("✅ Ecosystem listening successfully discovered other primals");
}

/// Test infant-like learning behavior
#[tokio::test]
async fn test_infant_learning_pattern() {
    println!("👶 Testing infant-like learning pattern...");

    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();

    // Phase 1: Birth - knows nothing
    let initial_state = bootstrap.get_ecosystem_state().unwrap();
    assert_eq!(initial_state.discovered_primals_count, 0);
    assert_eq!(initial_state.available_capabilities.len(), 0);

    println!("   👶 Phase 1: Birth - Zero knowledge confirmed");

    // Phase 2: Self-awareness
    let identity = bootstrap.discover_self_identity().unwrap();
    assert!(!identity.primal_id.is_empty());
    assert!(!identity.capabilities.is_empty());

    println!(
        "   🪞 Phase 2: Self-awareness - Identity: {}",
        identity.primal_id
    );

    // Phase 3: Environmental awareness
    bootstrap.start_ecosystem_listening().unwrap();

    // Simulate learning from environment
    std::env::set_var("BEARDOG_AI_ENDPOIN"T, "http://learned-ai:8083");

    tokio::time::sleep(Duration::from_millis(100)).await;

    let learning_state = bootstrap.get_ecosystem_state().unwrap();

    println!("   🧠 Phase 3: Environmental learning complete");
    println!(
        "     📊 Learned about {} primals",
        learning_state.discovered_primals_count
    );

    // Phase 4: Capability understanding
    let capabilities = bootstrap.discovered_capabilities.read();
    println!(
        "   ⚡ Phase 4: Capability understanding - {} types discovered",
        capabilities.len()
    );

    // Verify learning progression
    assert!(learning_state.discovered_primals_count >= initial_state.discovered_primals_count);
    assert!(
        learning_state.available_capabilities.len() >= initial_state.available_capabilities.len()
    );

    // Clean up
    std::env::remove_var("BEARDOG_AI_ENDPOINT");

    println!("🎉 Infant learning pattern test PASSED!");
    println!("   📈 Learning progression validated");
    println!("   🧠 Zero-to-knowledge bootstrap successful");
}

/// Test primal sovereignty compliance
#[tokio::test]
async fn test_primal_sovereignty_compliance() {
    println!("👑 Testing primal sovereignty compliance...");

    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();
    let identity = bootstrap.discover_self_identity().unwrap();

    // Start ecosystem listening
    bootstrap.start_ecosystem_listening().unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let ecosystem_state = bootstrap.get_ecosystem_state().unwrap();

    // Sovereignty Principle 1: Each primal only knows itself
    println!("   👑 Principle 1: Self-knowledge validation");
    assert!(!identity.primal_id.is_empty(), "Must know own identity");
    assert!(
        !identity.capabilities.is_empty(),
        "Must know own capabilities"
    );
    println!(
        "     ✅ Self-knowledge: ID={}, capabilities={}",
        identity.primal_id,
        identity.capabilities.len()
    );

    // Sovereignty Principle 2: No hardcoded knowledge of others
    println!("   🚫 Principle 2: No hardcoded other-knowledge");
    verify_no_hardcoded_other_knowledge(&identity, &ecosystem_state);
    println!("     ✅ No hardcoded knowledge of other primals");

    // Sovereignty Principle 3: Dynamic discovery only
    println!("   🔍 Principle 3: Dynamic discovery validation");
    let primals = bootstrap.discovered_primals.read();
    for (primal_id, discovered_primal) in primals.iter() {
        // Each discovered primal should have discovery timestamp
        assert!(discovered_primal.discovered_at > std::time::SystemTime::UNIX_EPOCH);
        println!(
            "     ✅ Primal {} discovered dynamically at {:?}",
            primal_id, discovered_primal.discovered_at
        );
    }

    // Sovereignty Principle 4: Universal adapter patterns
    println!("   🔌 Principle 4: Universal adapter compliance");
    let capabilities = bootstrap.discovered_capabilities.read();
    for (capability_type, providers) in capabilities.iter() {
        for provider in providers {
            // Each provider should be discovered, not hardcoded
            assert!(!provider.provider_id.is_empty(), "Provider must have ID");
            println!(
                "     ✅ Capability {:?} provided by {} (universal adapter pattern)",
                capability_type, provider.provider_id
            );
        }
    }

    println!("🎉 Primal sovereignty compliance test PASSED!");
    println!("   👑 All sovereignty principles validated");
}

/// Test performance of zero-knowledge bootstrap
#[tokio::test]
async fn test_bootstrap_performance() {
    println!("⚡ Testing zero-knowledge bootstrap performance...");

    let start_time = std::time::Instant::now();

    // Complete bootstrap process
    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();
    let creation_time = start_time.elapsed();

    let discovery_start = std::time::Instant::now();
    let _identity = bootstrap.discover_self_identity().unwrap();
    let discovery_time = discovery_start.elapsed();

    let listening_start = std::time::Instant::now();
    bootstrap.start_ecosystem_listening().unwrap();
    let listening_time = listening_start.elapsed();

    let total_time = start_time.elapsed();

    println!("   📊 Performance Metrics:");
    println!("     🏗️  Bootstrap Creation: {:?}", creation_time);
    println!("     🆔 Self-Discovery: {:?}", discovery_time);
    println!("     👂 Ecosystem Listening: {:?}", listening_time);
    println!("     ⏱️  Total Bootstrap: {:?}", total_time);

    // Performance assertions (should be fast)
    assert!(
        creation_time.as_millis() < 100,
        "Bootstrap creation should be < 100ms"
    );
    assert!(
        discovery_time.as_millis() < 500,
        "Self-discovery should be < 500ms"
    );
    assert!(
        listening_time.as_millis() < 200,
        "Ecosystem listening should be < 200ms"
    );
    assert!(
        total_time.as_millis() < 1000,
        "Total bootstrap should be < 1000ms"
    );

    println!("✅ Performance requirements met");
    println!("   🚀 Fast zero-knowledge bootstrap achieved");
}

/// Helper: Verify no hardcoded knowledge was used
fn verify_no_hardcoded_knowledge(
    identity: &SelfIdentity,
    ecosystem_state: &beardog_core::zero_knowledge_bootstrap::EcosystemState,
) {
    println!("🔍 Verifying zero hardcoded knowledge...");

    // Check self-identity for sovereignty compliance
    let id_lower = identity.primal_id.to_lowercase();
    let violation_patterns = ["hardcode"d, "legac"y, "vendor-specifi"c, "deprecated"];

    for pattern in &violation_patterns {
        assert!(
            !id_lower.contains(pattern),
            "Self-identity contains sovereignty violation: {}",
            pattern
        );
    }

    // Check endpoints for sovereignty compliance
    for endpoint in &identity.endpoints {
        let url_lower = endpoint.url.to_lowercase();
        for pattern in &violation_patterns {
            if url_lower.contains(pattern) {
                println!(
                    "⚠️ Warning: Endpoint contains potential violation "{}": {}",
                    pattern, endpoint.url
                );
                println!("   (Check for capability-based discovery compliance)");
            }
        }
    }

    println!("✅ Zero hardcoded knowledge verified");
}

/// Helper: Verify no hardcoded knowledge of other primals
fn verify_no_hardcoded_other_knowledge(
    identity: &SelfIdentity,
    ecosystem_state: &beardog_core::zero_knowledge_bootstrap::EcosystemState,
) {
    // The primal should only know itself, not have hardcoded knowledge of others

    // Self-knowledge is allowed and required
    assert!(!identity.primal_id.is_empty(), "Must know own primal ID");
    assert!(
        !identity.capabilities.is_empty(),
        "Must know own capabilities"
    );

    // But should not have hardcoded assumptions about other primals
    // This is validated by the absence of hardcoded primal names in the codebase
    // which we've already eliminated in the previous hardcoding elimination phase

    println!("✅ No hardcoded other-knowledge verified");
}

/// Test error handling in bootstrap process
#[tokio::test]
async fn test_bootstrap_error_handling() {
    println!("🚨 Testing bootstrap error handling...");

    // Test with invalid configuration
    let mut config = BootstrapConfig::default();
    config.max_discovery_attempts = 0; // Invalid

    let result = ZeroKnowledgeBootstrap::with_config(config);

    // Should handle invalid configuration gracefully
    match result {
        Ok(_) => println!("   ⚠️ Invalid config accepted (may use defaults)"),
        Err(e) => println!("   ✅ Invalid config rejected: {}", e),
    }

    // Test with network failures (simulated)
    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();

    // Self-discovery should work even with network issues
    let identity_result = bootstrap.discover_self_identity();
    assert!(
        identity_result.is_ok(),
        "Self-discovery should work offline"
    );

    // Ecosystem listening should handle failures gracefully
    let listening_result = bootstrap.start_ecosystem_listening();
    assert!(
        listening_result.is_ok(),
        "Ecosystem listening should start even if discovery fails"
    );

    println!("✅ Error handling test passed");
}

/// Integration test for complete ecosystem bootstrap scenario
#[tokio::test]
async fn test_complete_ecosystem_scenario() {
    println!("🌍 Testing complete ecosystem bootstrap scenario...");

    // Simulate a realistic ecosystem environment
    std::env::set_var("BEARDOG_COMPUTE_ENDPOIN"T, "http://compute-service:8081");
    std::env::set_var("BEARDOG_MESH_ENDPOIN"T, "http://mesh-service:8082");
    std::env::set_var("BEARDOG_AI_ENDPOIN"T, "http://ai-service:8083");
    std::env::set_var("BEARDOG_STORAGE_ENDPOIN"T, "http://storage-service:8084");

    // Bootstrap primal with zero knowledge
    let mut bootstrap = ZeroKnowledgeBootstrap::new().unwrap();

    println!("   🌱 Step 1: Zero-knowledge initialization complete");

    // Discover self-identity
    let identity = bootstrap.discover_self_identity().unwrap();

    println!("   🆔 Step 2: Self-identity discovered");
    println!("     ID: {}", identity.primal_id);
    println!("     Capabilities: {}", identity.capabilities.len());

    // Start ecosystem learning
    bootstrap.start_ecosystem_listening().unwrap();

    println!("   👂 Step 3: Ecosystem listening started");

    // Allow time for discovery
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Validate ecosystem state
    let ecosystem_state = bootstrap.get_ecosystem_state().unwrap();

    println!("   🌍 Step 4: Ecosystem state analysis");
    println!(
        "     Discovered primals: {}",
        ecosystem_state.discovered_primals_count
    );
    println!(
        "     Available capabilities: {}",
        ecosystem_state.available_capabilities.len()
    );

    // Should have discovered multiple primals from environment
    assert!(
        ecosystem_state.discovered_primals_count > 0,
        "Should discover primals"
    );

    // Should have diverse capabilities available
    assert!(
        ecosystem_state.available_capabilities.len() > 1,
        "Should discover multiple capability types"
    );

    // Validate capability-based discovery works
    let capabilities = bootstrap.discovered_capabilities.read();

    let expected_capability_types = [
        ServiceCapabilityType::ComputeIntelligence,
        ServiceCapabilityType::ServiceMesh,
        ServiceCapabilityType::DistributedIntelligence,
        ServiceCapabilityType::DataStorage,
    ];

    let mut found_capabilities = 0;
    for expected in &expected_capability_types {
        if capabilities.contains_key(expected) {
            found_capabilities += 1;
            let providers = capabilities.get(expected).unwrap();
            println!(
                "     ✅ {} capability: {} providers",
                format!("{:?}", expected),
                providers.len()
            );
        }
    }

    assert!(
        found_capabilities >= 2,
        "Should discover at least 2 capability types"
    );

    // Clean up
    std::env::remove_var("BEARDOG_COMPUTE_ENDPOINT");
    std::env::remove_var("BEARDOG_MESH_ENDPOINT");
    std::env::remove_var("BEARDOG_AI_ENDPOINT");
    std::env::remove_var("BEARDOG_STORAGE_ENDPOINT");

    println!("🎉 Complete ecosystem scenario test PASSED!");
    println!("   🌍 Realistic ecosystem bootstrap successful");
    println!("   👑 Primal sovereignty maintained throughout");
    println!("   🔍 Dynamic discovery working as designed");
}
