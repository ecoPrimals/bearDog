// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Universal Service Mesh Migration Example
//!
//! **MIGRATION FROM SONGBIRD-SPECIFIC TO UNIVERSAL ADAPTER**
//!
//! This example demonstrates how to migrate from the deprecated SongBird-specific
//! client to the new universal service mesh client that follows the principle
//! "primals only know themselves."

use beardog_core::universal_service_mesh_client::{
    UniversalMeshConfig, UniversalServiceMeshClient, UniversalServiceRequest,
};
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    tracing_subscriber::init();

    info!("🔄 Universal Service Mesh Migration Example");
    info!("📚 Demonstrating migration from SongBird-specific to universal adapter");

    // OLD WAY (DEPRECATED): Direct SongBird coupling
    demonstrate_old_songbird_pattern().await?;

    // NEW WAY: Universal service mesh client
    demonstrate_universal_pattern().await?;

    // Migration comparison
    demonstrate_migration_benefits().await?;

    Ok(())
}

/// OLD PATTERN: Direct SongBird coupling (TECHNICAL DEBT)
async fn demonstrate_old_songbird_pattern() -> BearDogResult<()> {
    warn!("❌ OLD PATTERN: Direct SongBird coupling (DEPRECATED)");

    // This is what we used to do - hardcoded SongBird references
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

/// NEW PATTERN: Universal service mesh client (BEST PRACTICE)
async fn demonstrate_universal_pattern() -> BearDogResult<()> {
    info!("✅ NEW PATTERN: Universal service mesh client");

    // Create universal mesh client with configuration
    let config = UniversalMeshConfig {
        discovery_timeout: Duration::from_secs(10),
        health_check_interval: Duration::from_secs(30),
        max_concurrent_requests: 50,
        enable_failover: true,
        quality_threshold: 0.7,
    };

    let mesh_client = UniversalServiceMeshClient::new(config);

    // STEP 1: Discover available service mesh capabilities (any primal)
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

    // STEP 2: Register with ecosystem (primal-agnostic)
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

    // STEP 3: Send service request through universal mesh
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
            let mut meta = HashMap::new();
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

    // STEP 4: Discover available services
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

/// Demonstrate migration benefits
async fn demonstrate_migration_benefits() -> BearDogResult<()> {
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

/// Configuration example for universal mesh client
fn create_production_config() -> UniversalMeshConfig {
    UniversalMeshConfig {
        discovery_timeout: Duration::from_secs(30),
        health_check_interval: Duration::from_secs(60),
        max_concurrent_requests: 100,
        enable_failover: true,
        quality_threshold: 0.8, // Only use high-quality meshes in production
    }
}

/// Environment setup example
fn setup_environment_variables() {
    // Universal discovery configuration (no primal-specific variables)
    std::env::set_var("ECOSYSTEM_DISCOVERY_URL", "http://ecosystem-discovery:8080");
    std::env::set_var("ECOSYSTEM_HOST", "localhost");
    std::env::set_var("ECOSYSTEM_DISCOVERY_PORTS", "8080,8081,8082");

    // BearDog service configuration
    std::env::set_var("BEARDOG_ENDPOINT", "http://beardog:8080");
    std::env::set_var("DEPLOYMENT_ENV", "production");

    // No more SONGBIRD_PORT or other primal-specific variables!
}
