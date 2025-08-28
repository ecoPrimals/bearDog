

use beardog_core::universal_service_mesh_client::{
    UniversalMeshConfig, UniversalServiceMeshClient, UniversalServiceRequest,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::init();

    info!("🔄 Universal Service Mesh Migration Example");
    info!("📚 Demonstrating migration from SongBird-specific to universal adapter");

    demonstrate_old_songbird_pattern().await?;

    demonstrate_universal_pattern().await?;

    demonstrate_migration_benefits().await?;

    Ok(())
}

async fn demonstrate_old_songbird_pattern() -> Result<(), BearDogError> {
    warn!("❌ OLD PATTERN: Direct SongBird coupling (DEPRECATED)");

    println!("  🔗 Hardcoded SongBird endpoint: http://songbird:8080");
    println!("  🔗 Hardcoded SongBird API calls");
    println!("  🔗 SongBird-specific error handling");
    println!("  🔗 SongBird-specific data formats");

    warn!("  ⚠️  PROBLEMS:");
    warn!("    - Violates 'primals only know themselves' principle");
    warn!("    - Creates tight coupling to specific primal");
    warn!("    - Cannot work with alternative service meshes");
    warn!("    - Hardcoded assumptions about SongBird behavior");
    warn!("    - Technical debt that reduces ecosystem flexibility");

    Ok(())
}

async fn demonstrate_universal_pattern() -> Result<(), BearDogError> {
    info!("✅ NEW PATTERN: Universal service mesh client");

    let config = UniversalMeshConfig {
        discovery_timeout: Duration::from_secs(10),
        health_check_interval: Duration::from_secs(30),
        max_concurrent_requests: 50,
        enable_failover: true,
        quality_threshold: 0.7,
    };

    let mesh_client = UniversalServiceMeshClient::new(config);

    info!("🔍 Step 1: Discovering service mesh capabilities in ecosystem");
    match mesh_client.discover_service_meshes().await {
        Ok(meshes) => {
            info!("✅ Discovered {} service mesh capabilities:", meshes.len());
            for mesh in &meshes {
                info!(
                    "  📡 Primal: {} | Endpoint: {} | Capabilities: {:?}",
                    mesh.primal_id, mesh.endpoint, mesh.capabilities
                );
            }
        }
        Err(e) => {
            warn!("⚠️ No service meshes discovered: {}", e);
            info!("💡 This is expected in demo environment - would discover SongBird, future alternatives, etc.");
        }
    }

    info!("🔗 Step 2: Registering BearDog with ecosystem");
    match mesh_client.register_with_ecosystem().await {
        Ok(registration) => {
            info!("✅ Successfully registered BearDog:");
            info!("  🆔 Registration ID: {}", registration.registration_id);
            info!("  🏷️  Instance ID: {}", registration.instance_id);
            info!("  🎯 Capabilities: {:?}", registration.capabilities);
        }
        Err(e) => {
            warn!("⚠️ Registration failed: {}", e);
            info!("💡 This is expected in demo environment");
        }
    }

    info!("📤 Step 3: Sending service request through universal mesh");
    let service_request = UniversalServiceRequest {
        request_id: Uuid::new_v4().to_string(),
        target_service_type: "compute".to_string(),
        payload: serde_json::json!({
            "operation": "create_instance",
            "instance_type": "secure_compute",
            "security_requirements": {
                "encryption": "required",
                "hsm_backed": true,
                "zero_trust": true
            }
        }),
        metadata: {
            let mut meta = HashMap::with_capacity(16);
            meta.insert("source".to_string(), "beardog".to_string());
            meta.insert("priority".to_string(), "high".to_string());
            meta
        },
        timeout: Some(Duration::from_secs(30)),
    };

    match mesh_client.send_service_request(service_request).await {
        Ok(response) => {
            info!("✅ Service request successful:");
            info!("  🆔 Request ID: {}", response.request_id);
            info!("  🎯 Handled by: {}", response.source_primal);
            info!("  ⏱️  Processing time: {:?}", response.processing_time);
            info!("  📊 Response: {}", response.payload);
        }
        Err(e) => {
            warn!("⚠️ Service request failed: {}", e);
            info!("💡 This is expected in demo environment");
        }
    }

    info!("🔍 Step 4: Discovering available services");
    match mesh_client.discover_services(Some("compute")).await {
        Ok(services) => {
            info!("✅ Discovered {} compute services:", services.len());
            for service in &services {
                info!(
                    "  🏷️  Service: {} | Type: {} | Endpoints: {:?}",
                    service.service_name, service.service_type, service.endpoints
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Service discovery failed: {}", e);
            info!("💡 This is expected in demo environment");
        }
    }

    Ok(())
}

async fn demonstrate_migration_benefits() -> Result<(), BearDogError> {
    info!("🎯 MIGRATION BENEFITS:");

    info!("✅ ARCHITECTURAL IMPROVEMENTS:");
    info!("  🏗️  Follows 'primals only know themselves' principle");
    info!("  🔄 Primal-agnostic design - works with ANY service mesh");
    info!("  🎯 Capability-based discovery instead of hardcoded names");
    info!("  🔌 Pluggable architecture for future service meshes");
    info!("  📈 Quality-based mesh selection");
    info!("  🔄 Automatic failover between available meshes");

    info!("✅ OPERATIONAL IMPROVEMENTS:");
    info!("  🌐 Environment-based configuration (no hardcoding)");
    info!("  📊 Built-in metrics and health monitoring");
    info!("  🔧 Configurable timeouts and retry logic");
    info!("  🚀 Concurrent request handling");
    info!("  💾 Service discovery caching");

    info!("✅ ECOSYSTEM BENEFITS:");
    info!("  🤝 Works with SongBird, future alternatives, or multiple meshes");
    info!("  🎯 Eliminates vendor lock-in to specific primal implementations");
    info!("  🔄 Enables ecosystem evolution without breaking changes");
    info!("  🏗️  Supports multi-mesh deployments");
    info!("  📈 Quality-driven mesh selection");

    info!("✅ TECHNICAL DEBT ELIMINATION:");
    info!("  ❌ No more hardcoded 'SONGBIRD_PORT' environment variables");
    info!("  ❌ No more SongBird-specific error handling");
    info!("  ❌ No more assumptions about SongBird API format");
    info!("  ❌ No more tight coupling to single primal");
    info!("  ✅ Universal adapter pattern throughout");

    info!("🔄 MIGRATION PATH:");
    info!("  1. Replace songbird_client imports with universal_service_mesh_client");
    info!("  2. Update configuration to use environment-based discovery");
    info!("  3. Replace SongBird-specific calls with universal requests");
    info!("  4. Remove hardcoded SongBird assumptions");
    info!("  5. Test with multiple service mesh implementations");

    Ok(())
}

fn create_production_config() -> UniversalMeshConfig {
    UniversalMeshConfig {
        discovery_timeout: Duration::from_secs(30),
        health_check_interval: Duration::from_secs(60),
        max_concurrent_requests: 100,
        enable_failover: true,
        quality_threshold: 0.8, // Only use high-quality meshes in production
    }
}

fn setup_environment_variables() {

    std::env::set_var("ECOSYSTEM_DISCOVERY_URL", "http://ecosystem-discovery:8080");
    std::env::set_var("ECOSYSTEM_HOST", "localhost");
    std::env::set_var("ECOSYSTEM_DISCOVERY_PORTS", "8080,8081,8082");

    std::env::set_var("BEARDOG_ENDPOINT", "http://beardog:8080");
    std::env::set_var("DEPLOYMENT_ENV", "production");

}
