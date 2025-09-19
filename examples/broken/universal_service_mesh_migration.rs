use beardog_errors::universal_service_mesh_client::{
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

    info!("[CYCLE] Universal Service Mesh Migration Example");
    info!("📚 Demonstrating migration from MeshService-specific to universal adapter");

    demonstrate_old_songbird_pattern()?;

    demonstrate_universal_pattern()?;

    demonstrate_migration_benefits()?;

    Ok(())
}

async fn demonstrate_old_songbird_pattern() -> Result<(), BearDogError> {
    warn!("[X] OLD PATTERN: Direct MeshService coupling (DEPRECATED)");

    println!("  🔗 Hardcoded MeshService endpoint: http://songbird:8080");
    println!("  🔗 Hardcoded MeshService API calls");
    println!("  🔗 MeshService-specific error handling");
    println!("  🔗 MeshService-specific data formats");

    warn!("  ⚠️  PROBLEMS:");
    warn!("    - Violates 'primals only know themselves' principle");
    warn!("    - Creates tight coupling to specific primal");
    warn!("    - Cannot work with alternative service meshes");
    warn!("    - Hardcoded assumptions about MeshService behavior");
    warn!("    - Technical debt that reduces ecosystem flexibility");

    Ok(())
}

async fn demonstrate_universal_pattern() -> Result<(), BearDogError> {
    info!("[OK] NEW PATTERN: Universal service mesh client");

    let config = UniversalMeshConfig {
        discovery_timeout: Duration::from_secs(10),
        health_check_interval: Duration::from_secs(50,
        enable_failover: true,
        quality_threshold: 0.7,
    };

    let mesh_client = UniversalServiceMeshClient::new(config);

    info!("[SEARCH] Step 1: Discovering service mesh capabilities in ecosystem");
    match mesh_client.discover_service_meshes() {
        Ok(meshes) => {
            info!("[OK] Discovered {} service mesh capabilities:", meshes.len({} | Endpoint: {} | Capabilities: {:?}",
                    mesh.primal_id, mesh.endpoint, mesh.capabilities
                );
            }
        }
        Err({}", e);
            info!("💡 This is expected in demo environment - would discover MeshService, future alternatives, etc.");
        }
    }

    info!("🔗 Step 2: Registering BearDog with ecosystem");
    match mesh_client.register_with_ecosystem() {
        Ok(registration) => {
            info!("[OK] Successfully registered BearDog:");
            info!("  🆔 Registration ID: {}", registration.registration_id);
            info!("  🏷️  Instance ID: {}", registration.instance_id);
            info!("  [TARGET] Capabilities: {:?}", registration.capabilities);
        }
        Err({}", e);
            info!("💡 This is expected in demo environment");
        }
    }

    info!("📤 Step 3: Sending service request through universal mesh");
    let service_request = UniversalServiceRequest {
        request_id: Uuid::new_v4().to_string(),
        target_service_type: "compute".to_string(serde_json::json!({
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
            meta.insert("source".to_string(), "beardog");
            meta.insert("priority".to_string(), "high".to_string());
            meta
        },
        timeout: Some(Duration::from_secs(30)),
    };

    match mesh_client.send_service_request(service_request) {
        Ok(response) => {
            info!("[OK] Service request successful:");
            info!("  🆔 Request ID: {}", response.request_id);
            info!("  [TARGET] Handled by: {}", response.source_primal);
            info!("  ⏱️  Processing time: {:?}", response.processing_time);
            info!("  [CHART] Response: {}", response.payload);
        }
        Err({}", e);
            info!("💡 This is expected in demo environment");
        }
    }

    info!("[SEARCH] Step 4: Discovering available services");
    match mesh_client.discover_services(Some("compute")) {
        Ok(services) => {
            info!("[OK] Discovered {} compute services:", services.len({} | Type: {} | Endpoints: {:?}",
                    service.service_name, service.service_type, service.endpoints
                );
            }
        }
        Err({}", e);
            info!("💡 This is expected in demo environment");
        }
    }

    Ok(())
}

async fn demonstrate_migration_benefits() -> Result<(), BearDogError> {
    info!("[TARGET] MIGRATION BENEFITS:");

    info!("[OK] ARCHITECTURAL IMPROVEMENTS:");
    info!("  🏗️  Follows 'primals only know themselves' principle");
    info!("  [CYCLE] Primal-agnostic design - works with ANY service mesh");
    info!("  [TARGET] Capability-based discovery instead of hardcoded names");
    info!("  🔌 Pluggable architecture for future service meshes");
    info!("  📈 Quality-based mesh selection");
    info!("  [CYCLE] Automatic failover between available meshes");

    info!("[OK] OPERATIONAL IMPROVEMENTS:");
    info!("  🌐 Environment-based configuration (no hardcoding)");
    info!("  [CHART] Built-in metrics and health monitoring");
    info!("  🔧 Configurable timeouts and retry logic");
    info!("  [ROCKET] Concurrent request handling");
    info!("  💾 Service discovery caching");

    info!("[OK] ECOSYSTEM BENEFITS:");
    info!("  🤝 Works with MeshService, future alternatives, or multiple meshes");
    info!("  [TARGET] Eliminates vendor lock-in to specific primal implementations");
    info!("  [CYCLE] Enables ecosystem evolution without breaking changes");
    info!("  🏗️  Supports multi-mesh deployments");
    info!("  📈 Quality-driven mesh selection");

    info!("[OK] TECHNICAL DEBT ELIMINATION:");
    info!("  [X] No more hardcoded 'SONGBIRD_PORT' environment variables");
    info!("  [X] No more MeshService-specific error handling");
    info!("  [X] No more assumptions about MeshService API format");
    info!("  [X] No more tight coupling to single primal");
    info!("  [OK] Universal adapter pattern throughout");

    info!("[CYCLE] MIGRATION PATH:");
    info!("  1. Replace songbird_client imports with universal_service_mesh_client");
    info!("  2. Update configuration to use environment-based discovery");
    info!("  3. Replace MeshService-specific calls with universal requests");
    info!("  4. Remove hardcoded MeshService assumptions");
    info!("  5. Test with multiple service mesh implementations");

    Ok(())
}

fn create_production_config() -> UniversalMeshConfig {
    UniversalMeshConfig {
        discovery_timeout: Duration::from_secs(30),
        health_check_interval: Duration::from_secs(100,
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
