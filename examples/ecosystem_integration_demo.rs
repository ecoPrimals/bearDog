//! BearDog Ecosystem Integration Demo
//!
//! This example demonstrates how BearDog integrates with the ecoPrimals ecosystem
//! using the standardized ecosystem API patterns.

use beardog::{
    ecosystem_integration::{
        BearDogEcosystemFactory, BearDogEcosystemProvider, EcosystemIntegration, EcosystemRequest,
        PrimalType, SecurityContext, SecurityLevel,
    },
    BearDogConfig, BearDogCore,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🌌 BearDog Ecosystem Integration Demo");
    println!("=====================================\n");

    // 1. Initialize BearDog Core
    println!("1. Initializing BearDog Core...");
    let config = BearDogConfig::from_env()?;
    let core = Arc::new(BearDogCore::new(config).await?);
    println!("   ✅ BearDog Core initialized\n");

    // 2. Create ecosystem provider
    println!("2. Creating ecosystem provider...");
    let provider = BearDogEcosystemFactory::create_provider(core.clone());
    println!("   ✅ Ecosystem provider created\n");

    // 3. Register with Songbird service mesh
    println!("3. Registering with Songbird service mesh...");
    let registration_id = provider.register_with_songbird().await?;
    println!("   ✅ Registered with Songbird: {}\n", registration_id);

    // 4. Demonstrate ecosystem request handling
    println!("4. Demonstrating ecosystem request handling...");

    // Create sample authentication request
    let auth_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "toadstool-compute-01".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "authenticate".to_string(),
        payload: serde_json::json!({
            "username": "demo_user",
            "password": "demo_password"
        }),
        security_context: SecurityContext {
            auth_token: Some("demo_token".to_string()),
            identity: "toadstool-service".to_string(),
            permissions: vec!["security.auth".to_string()],
            security_level: SecurityLevel::Internal,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let auth_response = provider.handle_ecosystem_request(auth_request).await?;
    println!(
        "   🔐 Authentication response: {:?}\n",
        auth_response.status
    );

    // Create sample encryption request
    let encrypt_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "nestgate-storage-01".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "encrypt".to_string(),
        payload: serde_json::json!({
            "data": "sensitive_data_to_encrypt",
            "algorithm": "aes-256-gcm"
        }),
        security_context: SecurityContext {
            auth_token: Some("demo_token".to_string()),
            identity: "nestgate-service".to_string(),
            permissions: vec!["security.encrypt".to_string()],
            security_level: SecurityLevel::Confidential,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let encrypt_response = provider.handle_ecosystem_request(encrypt_request).await?;
    println!("   🔒 Encryption response: {:?}\n", encrypt_response.status);

    // Create sample compliance check request
    let compliance_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "biomeos-orchestrator-01".to_string(),
        target_service: "beardog-security-01".to_string(),
        operation: "compliance_check".to_string(),
        payload: serde_json::json!({
            "framework": "gdpr",
            "data_type": "personal_data"
        }),
        security_context: SecurityContext {
            auth_token: Some("demo_token".to_string()),
            identity: "biomeos-service".to_string(),
            permissions: vec!["security.compliance".to_string()],
            security_level: SecurityLevel::Restricted,
        },
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    let compliance_response = provider
        .handle_ecosystem_request(compliance_request)
        .await?;
    println!(
        "   📋 Compliance response: {:?}\n",
        compliance_response.status
    );

    // 5. Report health status
    println!("5. Reporting health status...");
    let health_status = provider.get_health_status().await?;
    provider.report_health(health_status).await?;
    println!("   ✅ Health status reported\n");

    // 6. Demonstrate graceful shutdown
    println!("6. Demonstrating graceful shutdown...");
    provider.deregister().await?;
    println!("   ✅ Deregistered from ecosystem\n");

    println!("🎉 Demo completed successfully!");
    println!("\nKey capabilities demonstrated:");
    println!("  • 🔐 Authentication services");
    println!("  • 🔒 Encryption services");
    println!("  • 📋 Compliance checking");
    println!("  • 🏥 Health monitoring");
    println!("  • 🎼 Songbird service mesh integration");
    println!("  • 🌐 Standardized ecosystem communication");

    println!("\nBearDog is now ready to serve security requests from:");
    println!("  • 🍄 ToadStool (compute services)");
    println!("  • 🏠 NestGate (storage services)");
    println!("  • 🐿️ Squirrel (AI services)");
    println!("  • 🌱 biomeOS (orchestration)");

    Ok(())
}
