use beardog_errors::BearDogError;


use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use beardog::{
    adapters::universal::traits::PrimalProvider,
    core::ecosystem_integration::{
        BearDogEcosystemProvider, EcosystemIntegration, EcosystemRequest, HealthStatus,
        HealthStatusType, ResourceUsage, SecurityContext, SecurityLevel,
    },
    BearDogConfig, BearDogCore, BearDogResult,
};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {

    tracing_subscriber::fmt::init();

    info!("🌐 Starting BearDog Ecosystem Integration Demo");
    info!("===============================================");

    let beardog_core = initialize_beardog_core().await?;

    let ecosystem_provider = create_ecosystem_provider(beardog_core.clone()).await?;

    demonstrate_songbird_registration(&ecosystem_provider).await?;

    demonstrate_ecosystem_requests(&ecosystem_provider).await?;

    demonstrate_cross_primal_integration(&ecosystem_provider).await?;

    demonstrate_security_capabilities(&ecosystem_provider).await?;

    info!("✅ BearDog Ecosystem Integration Demo Completed!");
    info!("===============================================");
    info!("🎯 BearDog is now integrated with the ecoPrimals ecosystem:");
    info!("   • Registered with Songbird for discovery");
    info!("   • Providing security services to all primals");
    info!("   • Handling authentication, encryption, and compliance");
    info!("   • Ready for production ecosystem deployment");

    Ok(())
}

async fn initialize_beardog_core() -> Result<Arc<BearDogCore, BearDogError>> {
    info!("🚀 Initializing BearDog Core for Ecosystem Integration");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    core.start().await?;

    info!("✅ BearDog Core initialized successfully");
    Ok(core)
}

async fn create_ecosystem_provider(
    core: Arc<BearDogCore>,
) -> Result<BearDogEcosystemProvider, BearDogError> {
    info!("🔧 Creating BearDog Ecosystem Provider");

    let instance_id = format_args!("beardog-security-{}", Uuid::new_v4().to_string());
    let provider = BearDogEcosystemProvider::new(core, instance_id);

    info!("✅ BearDog Ecosystem Provider created");
    Ok(provider)
}

async fn demonstrate_songbird_registration(
    provider: &BearDogEcosystemProvider,
) -> Result<(), BearDogError> {
    info!("🎼 Demonstrating Songbird Registration");
    info!("=====================================");

    match provider.register_with_songbird().await {
        Ok(service_id) => {
            info!("✅ Successfully registered with Songbird!");
            info!("   Service ID: {}", service_id);
            info!("   Capabilities: authentication, encryption, compliance, threat_detection");
            info!("   Status: Ready to receive ecosystem requests");
        }
        Err(e) => {
            warn!("⚠️ Songbird registration failed (expected in demo): {}", e);
            info!("   This is normal - Songbird is not running in demo mode");
            info!("   In production, BearDog would be discoverable via Songbird");
        }
    }

    Ok(())
}

async fn demonstrate_ecosystem_requests(provider: &BearDogEcosystemProvider) -> Result<(), BearDogError> {
    info!("📨 Demonstrating Ecosystem Request Handling");
    info!("===========================================");

    info!("🍄 Demo 1: ToadStool Authentication Request");
    let auth_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "toadstool-compute-001".to_string(),
        target_service: "beardog-security-001".to_string(),
        operation: "authenticate".to_string(),
        payload: json!({
            "username": "toadstool_user",
            "password": "secure_password_123"
        }),
        security_context: SecurityContext {
            auth_token: Some("toadstool-auth-token".to_string()),
            identity: "toadstool-compute".to_string(),
            permissions: vec!["compute.execute".to_string(), "storage.read".to_string()],
            security_level: SecurityLevel::Internal,
        },
        metadata: {
            let mut map = HashMap::with_capacity(16);
            map.insert("source_primal".to_string(), "toadstool".to_string());
            map.insert("request_type".to_string(), "authentication".to_string());
            map
        },
        timestamp: chrono::Utc::now(),
    };

    let auth_response = provider.handle_ecosystem_request(auth_request).await?;
    info!("✅ Authentication response: {:?}", auth_response.status);

    info!("🏠 Demo 2: NestGate Encryption Request");
    let encrypt_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "nestgate-storage-001".to_string(),
        target_service: "beardog-security-001".to_string(),
        operation: "encrypt".to_string(),
        payload: json!({
            "data": "sensitive_storage_data",
            "algorithm": "AES-256-GCM"
        }),
        security_context: SecurityContext {
            auth_token: Some("nestgate-auth-token".to_string()),
            identity: "nestgate-storage".to_string(),
            permissions: vec!["storage.encrypt".to_string(), "storage.decrypt".to_string()],
            security_level: SecurityLevel::Restricted,
        },
        metadata: {
            let mut map = HashMap::with_capacity(16);
            map.insert("source_primal".to_string(), "nestgate".to_string());
            map.insert("data_type".to_string(), "zfs_block".to_string());
            map
        },
        timestamp: chrono::Utc::now(),
    };

    let encrypt_response = provider.handle_ecosystem_request(encrypt_request).await?;
    info!("✅ Encryption response: {:?}", encrypt_response.status);

    info!("🐿️ Demo 3: Squirrel Compliance Check Request");
    let compliance_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "squirrel-ai-001".to_string(),
        target_service: "beardog-security-001".to_string(),
        operation: "compliance_check".to_string(),
        payload: json!({
            "data_type": "personal_data",
            "operation": "ai_processing",
            "standards": ["GDPR", "HIPAA"]
        }),
        security_context: SecurityContext {
            auth_token: Some("squirrel-auth-token".to_string()),
            identity: "squirrel-ai".to_string(),
            permissions: vec!["ai.process".to_string(), "data.analyze".to_string()],
            security_level: SecurityLevel::Confidential,
        },
        metadata: {
            let mut map = HashMap::with_capacity(16);
            map.insert("source_primal".to_string(), "squirrel".to_string());
            map.insert(
                "compliance_context".to_string(),
                "ai_processing".to_string(),
            );
            map
        },
        timestamp: chrono::Utc::now(),
    };

    let compliance_response = provider
        .handle_ecosystem_request(compliance_request)
        .await?;
    info!("✅ Compliance response: {:?}", compliance_response.status);

    Ok(())
}

async fn demonstrate_cross_primal_integration(
    provider: &BearDogEcosystemProvider,
) -> Result<(), BearDogError> {
    info!("🤝 Demonstrating Cross-Primal Integration");
    info!("=========================================");

    info!("🔗 BearDog's Role in the Ecosystem:");
    info!("   • Security Provider: Handles all encryption, authentication, compliance");
    info!("   • Not a Standalone Server: Integrates with other primals via Songbird");
    info!("   • Discovery via Songbird: Other primals find BearDog through service mesh");
    info!("   • Request Routing: Songbird routes security requests to BearDog");

    info!("🍄 ToadStool Integration:");
    info!("   • Compute Security: BearDog secures ToadStool's compute operations");
    info!("   • Genetic Spawning: BearDog + ToadStool create secure compute hybrids");
    info!("   • Resource Authorization: Cryptographic proof for compute access");

    info!("🏠 NestGate Integration:");
    info!("   • Storage Encryption: BearDog encrypts all NestGate storage operations");
    info!("   • Key Management: BearDog manages ZFS encryption keys");
    info!("   • Access Control: BearDog authorizes storage operations");

    info!("🐿️ Squirrel Integration:");
    info!("   • AI Data Security: BearDog secures AI model training data");
    info!("   • Plugin Security: BearDog validates and secures plugin operations");
    info!("   • Compliance Monitoring: BearDog ensures AI operations meet regulations");

    info!("🌱 biomeOS Integration:");
    info!("   • System Security: BearDog provides OS-level security services");
    info!("   • Multi-tenant Security: BearDog isolates different biome instances");
    info!("   • Configuration Security: BearDog secures biome configuration data");

    Ok(())
}

async fn demonstrate_security_capabilities(
    provider: &BearDogEcosystemProvider,
) -> Result<(), BearDogError> {
    info!("🛡️ Demonstrating Security Capabilities");
    info!("======================================");

    info!("🔍 Threat Detection Capabilities:");
    let threat_request = EcosystemRequest {
        request_id: Uuid::new_v4(),
        source_service: "songbird-monitor-001".to_string(),
        target_service: "beardog-security-001".to_string(),
        operation: "threat_scan".to_string(),
        payload: json!({
            "target": "ecosystem_traffic",
            "scan_type": "comprehensive"
        }),
        security_context: SecurityContext {
            auth_token: Some("songbird-monitor-token".to_string()),
            identity: "songbird-monitor".to_string(),
            permissions: vec!["monitor.scan".to_string(), "threat.detect".to_string()],
            security_level: SecurityLevel::Internal,
        },
        metadata: HashMap::with_capacity(16),
        timestamp: chrono::Utc::now(),
    };

    let threat_response = provider.handle_ecosystem_request(threat_request).await?;
    info!("✅ Threat scan completed: {:?}", threat_response.status);

    info!("📋 Available Security Capabilities:");
    info!("   Core Capabilities:");
    info!("     • Authentication: Multi-factor, biometric, cryptographic");
    info!("     • Encryption: AES-256-GCM, ChaCha20-Poly1305, post-quantum ready");
    info!("     • Key Management: Automated rotation, HSM integration");
    info!("     • Threat Detection: ML-powered, real-time analysis");
    info!("     • Compliance: GDPR, HIPAA, SOX, PCI DSS monitoring");

    info!("   Extended Capabilities:");
    info!("     • Audit Logging: Immutable compliance trails");
    info!("     • ML Threat Detection: Behavioral analysis, anomaly detection");
    info!("     • Genetic Spawning: Evolutionary security adaptation");
    info!("     • HSM Support: Hardware security modules, Android StrongBox");

    info!("   Ecosystem Integrations:");
    info!("     • Songbird: Service mesh registration and discovery");
    info!("     • ToadStool: Compute security and genetic spawning");
    info!("     • NestGate: Storage encryption and key management");
    info!("     • Squirrel: AI data security and plugin validation");
    info!("     • biomeOS: System-level security and configuration");

    Ok(())
}

async fn demonstrate_health_monitoring(provider: &BearDogEcosystemProvider) -> Result<(), BearDogError> {
    info!("🏥 Health Monitoring Capabilities");
    info!("================================");

    let health_status = HealthStatus {
        status: HealthStatusType::Healthy,
        version: "1.0.0".to_string(),
        uptime_seconds: 3600,
        resource_usage: ResourceUsage {
            cpu_percent: 25.0,
            memory_bytes: 1024 * 1024 * 100,    // 100MB
            disk_bytes: 1024 * 1024 * 1024,     // 1GB
            network_bytes_per_sec: 1024 * 1024, // 1MB/s
        },
        capabilities_online: vec![
            "encryption".to_string(),
            "authentication".to_string(),
            "audit".to_string(),
        ],
        last_check: chrono::Utc::now(),
    };
    match provider.report_health(health_status).await {
        Ok(_) => info!("✅ Health status reported to Songbird"),
        Err(e) => warn!("⚠️ Health reporting failed (expected in demo): {}", e),
    }

    Ok(())
}
