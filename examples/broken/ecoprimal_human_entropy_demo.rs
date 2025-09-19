// EcoPrimal Human Entropy Access Demonstration
//
// This example demonstrates the revolutionary capability for other ecoPrimals
// to access BearDog's human-owned entropy through the Universal Adapter system.
// This transforms randomness from a machine-only resource to a human-owned,
// ecosystem-wide sovereignty layer.
//
// ## Key Concepts Demonstrated:
//
// 1. **Cross-Primal Entropy Sharing**: Other primals accessing BearDog's human entropy
// 2. **Universal Adapter Integration**: Seamless entropy access through capability system
// 3. **Ownership Preservation**: Human ownership maintained across primal boundaries
// 4. **Audit Trail Continuity**: Complete tracking of entropy usage across ecosystem
// 5. **Sovereignty at Scale**: Human dignity preserved in distributed systems

use beardog_adapters::universal::{
    EntropyCapabilityConfig, EntropyRequest, EntropyResponse, UniversalCapabilityAdapter,
    UniversalEntropyCapabilityAdapter,
};
use beardog_core::migration::{SovereignEntropyMigrationConfig, SovereignEntropyMigrationManager};
use beardog_errors::BearDogError;
use beardog_genetics::genetics::entropy_hierarchy::EntropyHierarchyManager;
use beardog_types::canonical::capabilities::{
    CapabilityRequest, CapabilityResponse, ServiceCapabilityType,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🌐 EcoPrimal Human Entropy Access Demonstration");
    info!("==============================================");
    info!("Demonstrating ecosystem-wide human-owned randomness sharing");

    // Demo 1: BearDog Entropy Capability Setup
    demo_beardog_entropy_setup()?;

    // Demo 2: Songbird Accessing Human Entropy for Music Generation
    demo_songbird_music_entropy()?;

    // Demo 3: Toadstool Accessing Human Entropy for Blockchain Operations
    demo_toadstool_blockchain_entropy()?;

    // Demo 4: Squirrel Accessing Human Entropy for Data Processing
    demo_squirrel_data_entropy()?;

    // Demo 5: Cross-Primal Entropy Audit Trail
    demo_cross_primal_audit_trail()?;

    // Demo 6: Ownership Transfer and Authorization
    demo_entropy_ownership_management()?;

    info!("\n🎉 EcoPrimal Human Entropy Demonstration Complete!");
    info!("Key Achievement: Human sovereignty preserved across entire ecosystem");

    Ok(())
}

/// Demo 1: Set up BearDog's entropy capability for ecosystem sharing
async fn demo_beardog_entropy_setup() -> Result<(), BearDogError> {
    info!("\n🔧 Demo 1: BearDog Entropy Capability Setup");
    info!("==========================================");
    info!("Setting up human-owned entropy as universal ecosystem capability");

    // Create entropy hierarchy manager
    let entropy_manager = Arc::new(EntropyHierarchyManager::default());

    // Create migration manager for sovereign entropy
    let migration_config = SovereignEntropyMigrationConfig::default();
    let migration_manager = Arc::new(RwLock::new(
        SovereignEntropyMigrationManager::new(entropy_manager.clone(), migration_config)?,
    ));

    // Create entropy capability adapter
    let entropy_config = EntropyCapabilityConfig::default();
    let entropy_adapter =
        UniversalEntropyCapabilityAdapter::new(entropy_manager.clone(), entropy_config)?;

    // Create universal adapter and register entropy capability
    let universal_adapter = UniversalCapabilityAdapter::new()?;
    entropy_adapter
        .register_capability(&universal_adapter)
        ?;

    info!("✅ BearDog entropy capability registered with universal adapter");
    info!("   📊 Supported tiers: 1 (Machine), 2 (Supervised), 3 (Human)");
    info!("   🔒 Max entropy per request: 1MB");
    info!("   🌐 Cross-primal sharing: Enabled");
    info!("   📋 Audit trail: Comprehensive");

    Ok(())
}

/// Demo 2: Songbird accessing human entropy for music generation
async fn demo_songbird_music_entropy() -> Result<(), BearDogError> {
    info!("\n🎵 Demo 2: Songbird Music Generation with Human Entropy");
    info!("====================================================");
    info!("Songbird primal accessing Alice's entropy for personalized music");

    // Simulate Songbird's entropy request
    let songbird_request = CapabilityRequest {
        capability: "human_owned_entropy".to_string(),
        operation: "generate_entropy".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("requesting_primal".to_string(), "songbird".to_string());
            params.insert(
                "human_identity_id".to_string(),
                "alice_music_lover".to_string(),
            );
            params.insert("required_tier".to_string(), "3".to_string()); // Human lived experience
            params.insert("entropy_bytes".to_string(), "1024".to_string()); // 1KB for music generation
            params.insert(
                "use_case".to_string(),
                "personalized_music_generation".to_string(),
            );
            params.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            params
        },
    };

    info!("🎶 Songbird Request Details:");
    info!("   Primal: Songbird");
    info!("   Human: alice_music_lover");
    info!("   Entropy Tier: 3 (Human Lived Experience)");
    info!("   Use Case: Personalized music generation");
    info!("   Entropy Size: 1KB");

    // Simulate successful entropy generation
    let mock_entropy_response = simulate_entropy_response(
        "songbird",
        "alice_music_lover",
        3,
        1024,
        "personalized_music_generation",
    )
    ?;

    info!("✅ Entropy Generated Successfully:");
    info!(
        "   📊 Quality Score: {:.2}",
        mock_entropy_response.quality_score
    );
    info!(
        "   🎯 Provided Tier: {}",
        mock_entropy_response.provided_tier
    );
    info!("   📋 Audit ID: {}", mock_entropy_response.audit_id);
    info!("   🎵 Result: Alice's unique musical preferences encoded in entropy");
    info!("   🔒 Sovereignty: Alice owns the randomness driving her music generation");

    Ok(())
}

/// Demo 3: Toadstool accessing human entropy for blockchain operations
async fn demo_toadstool_blockchain_entropy() -> Result<(), BearDogError> {
    info!("\n🍄 Demo 3: Toadstool Blockchain Operations with Human Entropy");
    info!("==========================================================");
    info!("Toadstool primal accessing Bob's entropy for sovereign blockchain operations");

    // Simulate Toadstool's entropy request for blockchain operations
    let toadstool_request = CapabilityRequest {
        capability: "human_owned_entropy".to_string(),
        operation: "generate_entropy".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("requesting_primal".to_string(), "toadstool".to_string());
            params.insert(
                "human_identity_id".to_string(),
                "bob_crypto_trader".to_string(),
            );
            params.insert("required_tier".to_string(), "3".to_string()); // Human lived experience
            params.insert("entropy_bytes".to_string(), "2048".to_string()); // 2KB for blockchain ops
            params.insert(
                "use_case".to_string(),
                "sovereign_blockchain_operations".to_string(),
            );
            params.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            params
        },
    };

    info!("🔗 Toadstool Request Details:");
    info!("   Primal: Toadstool");
    info!("   Human: bob_crypto_trader");
    info!("   Entropy Tier: 3 (Human Lived Experience)");
    info!("   Use Case: Sovereign blockchain operations");
    info!("   Entropy Size: 2KB");

    let mock_entropy_response = simulate_entropy_response(
        "toadstool",
        "bob_crypto_trader",
        3,
        2048,
        "sovereign_blockchain_operations",
    )
    ?;

    info!("✅ Blockchain Entropy Generated:");
    info!(
        "   📊 Quality Score: {:.2}",
        mock_entropy_response.quality_score
    );
    info!("   🔐 Cryptographic Grade: Maximum security");
    info!("   📋 Audit ID: {}", mock_entropy_response.audit_id);
    info!("   🍄 Result: Bob's sovereign blockchain keys generated");
    info!("   🔒 Sovereignty: Bob owns his blockchain randomness completely");

    Ok(())
}

/// Demo 4: Squirrel accessing human entropy for data processing
async fn demo_squirrel_data_entropy() -> Result<(), BearDogError> {
    info!("\n🐿️  Demo 4: Squirrel Data Processing with Human Entropy");
    info!("===================================================");
    info!("Squirrel primal accessing Carol's entropy for privacy-preserving data analysis");

    // Simulate Squirrel's entropy request for data processing
    let squirrel_request = CapabilityRequest {
        capability: "human_owned_entropy".to_string(),
        operation: "generate_entropy".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("requesting_primal".to_string(), "squirrel".to_string());
            params.insert(
                "human_identity_id".to_string(),
                "carol_data_scientist".to_string(),
            );
            params.insert("required_tier".to_string(), "2".to_string()); // Human supervised
            params.insert("entropy_bytes".to_string(), "512".to_string()); // 512B for data processing
            params.insert(
                "use_case".to_string(),
                "privacy_preserving_data_analysis".to_string(),
            );
            params.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            params
        },
    };

    info!("📊 Squirrel Request Details:");
    info!("   Primal: Squirrel");
    info!("   Human: carol_data_scientist");
    info!("   Entropy Tier: 2 (Human Supervised)");
    info!("   Use Case: Privacy-preserving data analysis");
    info!("   Entropy Size: 512B");

    let mock_entropy_response = simulate_entropy_response(
        "squirrel",
        "carol_data_scientist",
        2,
        512,
        "privacy_preserving_data_analysis",
    )
    ?;

    info!("✅ Data Processing Entropy Generated:");
    info!(
        "   📊 Quality Score: {:.2}",
        mock_entropy_response.quality_score
    );
    info!("   🔍 Privacy Grade: Human-supervised");
    info!("   📋 Audit ID: {}", mock_entropy_response.audit_id);
    info!("   🐿️  Result: Carol's data analysis with human-owned randomness");
    info!("   🔒 Sovereignty: Carol controls randomness in her data processing");

    Ok(())
}

/// Demo 5: Cross-primal entropy audit trail
async fn demo_cross_primal_audit_trail() -> Result<(), BearDogError> {
    info!("\n📋 Demo 5: Cross-Primal Entropy Audit Trail");
    info!("==========================================");
    info!("Demonstrating comprehensive audit trail across ecosystem");

    // Simulate audit trail aggregation across primals
    let audit_events = vec![
        json!({
            "timestamp": chrono::Utc::now(),
            "primal": "songbird",
            "human_identity": "alice_music_lover",
            "entropy_tier": 3,
            "bytes_generated": 1024,
            "use_case": "personalized_music_generation",
            "sovereignty_preserved": true,
        }),
        json!({
            "timestamp": chrono::Utc::now(),
            "primal": "toadstool",
            "human_identity": "bob_crypto_trader",
            "entropy_tier": 3,
            "bytes_generated": 2048,
            "use_case": "sovereign_blockchain_operations",
            "sovereignty_preserved": true,
        }),
        json!({
            "timestamp": chrono::Utc::now(),
            "primal": "squirrel",
            "human_identity": "carol_data_scientist",
            "entropy_tier": 2,
            "bytes_generated": 512,
            "use_case": "privacy_preserving_data_analysis",
            "sovereignty_preserved": true,
        }),
    ];

    info!("📊 Ecosystem Entropy Usage Summary:");
    info!("   🎵 Songbird: 1KB Tier-3 entropy for Alice's music");
    info!("   🍄 Toadstool: 2KB Tier-3 entropy for Bob's blockchain");
    info!("   🐿️  Squirrel: 512B Tier-2 entropy for Carol's data");
    info!("   📈 Total: 3.5KB human-owned entropy across ecosystem");
    info!("   🔒 Sovereignty: 100% preserved across all primals");

    // Simulate audit compliance check
    let compliance_score = calculate_sovereignty_compliance(&audit_events);
    info!("✅ Sovereignty Compliance Score: {:.1}%", compliance_score);

    if compliance_score >= 95.0 {
        info!("🏆 EXCELLENT: Ecosystem maintains human sovereignty standards");
    }

    Ok(())
}

/// Demo 6: Entropy ownership management and authorization
async fn demo_entropy_ownership_management() -> Result<(), BearDogError> {
    info!("\n👤 Demo 6: Entropy Ownership Management");
    info!("=====================================");
    info!("Demonstrating ownership transfer and authorization across primals");

    // Simulate ownership authorization scenario
    info!("🔐 Ownership Scenario: Alice authorizes Songbird and Toadstool");
    info!("   Owner: alice_music_lover");
    info!("   Authorized Primals: [songbird, toadstool]");
    info!("   Unauthorized Primal: squirrel (requires explicit consent)");

    // Simulate successful authorization
    let authorization_request = json!({
        "human_identity": "alice_music_lover",
        "requesting_primal": "songbird",
        "operation": "generate_entropy",
        "authorization_status": "granted"
    });

    info!("✅ Authorization Granted:");
    info!("   Human: alice_music_lover");
    info!("   Primal: songbird");
    info!("   Status: Authorized for entropy access");

    // Simulate unauthorized access attempt
    let unauthorized_request = json!({
        "human_identity": "alice_music_lover",
        "requesting_primal": "unauthorized_primal",
        "operation": "generate_entropy",
        "authorization_status": "denied"
    });

    info!("❌ Authorization Denied:");
    info!("   Human: alice_music_lover");
    info!("   Primal: unauthorized_primal");
    info!("   Status: Not authorized - explicit consent required");
    info!("   🔒 Sovereignty Protection: Alice's entropy remains secure");

    // Demonstrate ownership transfer
    info!("🔄 Ownership Transfer Demonstration:");
    info!("   Original Owner: alice_music_lover");
    info!("   Transfer To: alice_music_lover_v2 (account upgrade)");
    info!("   Authorized Primals: Preserved during transfer");
    info!("   🔒 Continuity: Entropy ownership smoothly transferred");

    Ok(())
}

/// Simulate entropy response for demonstration
async fn simulate_entropy_response(
    requesting_primal: &str,
    human_identity: &str,
    entropy_tier: u8,
    entropy_bytes: usize,
    use_case: &str,
) -> Result<EntropyResponse, BearDogError> {
    // Simulate entropy generation with mock data
    let mock_entropy = vec![0u8; entropy_bytes]; // In real implementation, this would be actual entropy

    let response = EntropyResponse {
        request_id: uuid::Uuid::new_v4().to_string(),
        entropy_bytes: Some(mock_entropy),
        provided_tier: entropy_tier,
        quality_score: match entropy_tier {
            3 => 0.95, // Tier 3: High quality human entropy
            2 => 0.88, // Tier 2: Good quality supervised entropy
            1 => 0.75, // Tier 1: Standard quality machine entropy
            _ => 0.50,
        },
        generated_at: chrono::Utc::now(),
        audit_id: format!(
            "audit_{}_{}",
            requesting_primal,
            chrono::Utc::now().timestamp()
        ),
    };

    // Log the entropy generation for audit
    let audit_record = json!({
        "timestamp": chrono::Utc::now(),
        "requesting_primal": requesting_primal,
        "human_identity": human_identity,
        "entropy_tier": entropy_tier,
        "entropy_bytes": entropy_bytes,
        "use_case": use_case,
        "quality_score": response.quality_score,
        "sovereignty_preserved": true,
        "cross_primal_sharing": true,
    });

    info!("📋 Entropy Audit Record: {}", audit_record);

    Ok(response)
}

/// Calculate sovereignty compliance score across audit events
fn calculate_sovereignty_compliance(audit_events: &[serde_json::Value]) -> f64 {
    if audit_events.is_empty() {
        return 0.0;
    }

    let total_events = audit_events.len() as f64;
    let compliant_events = audit_events
        .iter()
        .filter(|event| {
            event
                .get("sovereignty_preserved")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        })
        .count() as f64;

    (compliant_events / total_events) * 100.0
}
