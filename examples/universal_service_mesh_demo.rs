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


//! Universal Service Mesh Integration Demo
//!
//! Demonstrates how BearDog discovers and integrates with any available
//! service mesh primal in the ecosystem, not just Songbird.
//!
//! This shows the ecosystem-agnostic approach where BearDog can work with:
//! - Songbird (current primary mesh)
//! - Future service mesh primals  
//! - Custom mesh implementations
//! - Automatic failover between meshes

use beardog_core::{
    UniversalServiceMeshClient, UniversalServiceMesh, BearDogCore,
    PrimalMetadata, ServiceRequest, ServiceResponse, PrimalIdentity, PrimalType,
    ServiceHealth,
};
use beardog_types::config::BearDogConfig;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn, error};
use uuid::Uuid;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🌍 BearDog Universal Service Mesh Integration Demo");

    // Initialize BearDog core
    let config = BearDogConfig::default();
    let beardog_core = Arc::new(BearDogCore::new(config).await?);
    
    // Create universal service mesh client (mesh-agnostic!)
    let mesh_client = UniversalServiceMeshClient::new()?;

    // Step 1: Discover available service mesh primals
    demonstrate_mesh_discovery(&mesh_client).await?;

    // Step 2: Connect to best available mesh
    demonstrate_mesh_selection(&mesh_client).await?;

    // Step 3: Register BearDog with selected mesh
    demonstrate_mesh_registration(&mesh_client, &beardog_core).await?;

    // Step 4: Use mesh for service discovery
    demonstrate_service_discovery(&mesh_client).await?;

    // Step 5: Send requests through mesh routing
    demonstrate_mesh_routing(&mesh_client).await?;

    // Step 6: Demonstrate mesh failover
    demonstrate_mesh_failover(&mesh_client).await?;

    // Step 7: Integration with multiple mesh types
    demonstrate_multi_mesh_support().await?;

    info!("✅ Universal Service Mesh Demo completed successfully!");
    Ok(())
}

/// Demonstrate discovery of multiple service mesh types
async fn demonstrate_mesh_discovery(mesh_client: &UniversalServiceMeshClient) -> BearDogResult<()> {
    info!("🔍 === Step 1: Universal Service Mesh Discovery ===");

    // Discover all available service meshes in the ecosystem
    let discovered_meshes = mesh_client.discover_service_meshes().await?;

    info!("Found {} service mesh primals:", discovered_meshes.len());
    for mesh in &discovered_meshes {
        info!("  🕸️  {} (v{}) at {} - Priority: {}", 
              mesh.name, mesh.api_version, mesh.endpoint, mesh.priority_score);
        info!("      Capabilities: {:?}", mesh.capabilities);
        info!("      Health: {:?}", mesh.health);
    }

    if discovered_meshes.is_empty() {
        warn!("⚠️ No service meshes found - BearDog will operate in standalone mode");
    } else {
        info!("✅ Service mesh discovery successful");
    }

    Ok(())
}

/// Demonstrate intelligent mesh selection
async fn demonstrate_mesh_selection(mesh_client: &UniversalServiceMeshClient) -> BearDogResult<()> {
    info!("🎯 === Step 2: Intelligent Mesh Selection ===");

    match mesh_client.connect_to_best_mesh().await {
        Ok(selected_mesh) => {
            info!("🏆 Selected mesh: {} (priority: {})", 
                  selected_mesh.name, selected_mesh.priority_score);
            info!("    Endpoint: {}", selected_mesh.endpoint);
            info!("    API Version: {}", selected_mesh.api_version);
            info!("    Capabilities: {:?}", selected_mesh.capabilities);
        }
        Err(e) => {
            warn!("⚠️ No suitable mesh found: {}. Operating standalone.", e);
        }
    }

    Ok(())
}

/// Demonstrate mesh-agnostic registration
async fn demonstrate_mesh_registration(
    mesh_client: &UniversalServiceMeshClient,
    core: &Arc<BearDogCore>
) -> BearDogResult<()> {
    info!("📝 === Step 3: Universal Mesh Registration ===");

    // Get BearDog metadata and services
    let metadata = PrimalMetadata::default();
    let services = vec![]; // Would be populated with actual services

    match mesh_client.register(&metadata, &services).await {
        Ok(registration) => {
            info!("✅ Successfully registered with {}!", registration.service_mesh_name);
            info!("    Registration ID: {}", registration.registration_id);
            info!("    Node ID: {}", registration.node_id);
            info!("    Status: {:?}", registration.status);
        }
        Err(e) => {
            warn!("⚠️ Registration failed: {}. Continuing in standalone mode.", e);
        }
    }

    Ok(())
}

/// Demonstrate service discovery across different meshes
async fn demonstrate_service_discovery(mesh_client: &UniversalServiceMeshClient) -> BearDogResult<()> {
    info!("🕵️ === Step 4: Universal Service Discovery ===");

    // Discover different types of services
    let capabilities_to_find = vec![
        "compute.optimization",
        "storage.distributed", 
        "ai.inference",
        "network.routing",
    ];

    for capability in capabilities_to_find {
        match mesh_client.discover_services(capability).await {
            Ok(services) => {
                if services.is_empty() {
                    info!("  📋 No services found for capability: {}", capability);
                } else {
                    info!("  🎯 Found {} services with capability: {}", services.len(), capability);
                    for service in &services[..2.min(services.len())] { // Show first 2
                        info!("      - {} ({}) on {}", 
                              service.service_id, service.primal_name, service.primal_type);
                    }
                }
            }
            Err(e) => {
                warn!("  ⚠️ Discovery failed for {}: {}", capability, e);
            }
        }
    }

    Ok(())
}

/// Demonstrate routing through universal mesh
async fn demonstrate_mesh_routing(mesh_client: &UniversalServiceMeshClient) -> BearDogResult<()> {
    info!("🌐 === Step 5: Universal Mesh Routing ===");

    // Create example service requests
    let requests = vec![
        create_example_request("compute-service", "ToadStool", "optimize"),
        create_example_request("storage-service", "NestGate", "store"),
        create_example_request("ai-service", "UniversalCompute", "inference"),
    ];

    for request in requests {
        info!("📤 Sending request to {} via mesh routing", request.target_primal);
        
        match mesh_client.send_service_request(request).await {
            Ok(response) => {
                info!("  ✅ Response received in {}ms", response.processing_time_ms);
                info!("      Status: {:?}", response.status);
            }
            Err(e) => {
                warn!("  ⚠️ Request failed: {}", e);
            }
        }
    }

    Ok(())
}

/// Demonstrate automatic mesh failover
async fn demonstrate_mesh_failover(mesh_client: &UniversalServiceMeshClient) -> BearDogResult<()> {
    info!("🔄 === Step 6: Mesh Failover Demonstration ===");

    // Show current mesh
    if let Some(current_mesh) = mesh_client.get_active_mesh().await {
        info!("Current mesh: {} at {}", current_mesh.name, current_mesh.endpoint);
        
        // Simulate mesh failure and demonstrate failover
        info!("Simulating mesh failure and testing failover...");
        
        match mesh_client.failover_to_alternative().await {
            Ok(new_mesh) => {
                info!("✅ Successfully failed over to: {} at {}", 
                      new_mesh.name, new_mesh.endpoint);
                info!("    New mesh capabilities: {:?}", new_mesh.capabilities);
            }
            Err(e) => {
                warn!("⚠️ Failover not possible: {}. Operating standalone.", e);
            }
        }
    } else {
        info!("No active mesh - failover not applicable");
    }

    Ok(())
}

/// Demonstrate support for multiple mesh types
async fn demonstrate_multi_mesh_support() -> BearDogResult<()> {
    info!("🌟 === Step 7: Multi-Mesh Type Support ===");

    // Show how BearDog can work with different mesh implementations
    let mesh_examples = vec![
        ("Songbird", "Primary ecosystem mesh - rich features"),
        ("LightMesh", "Hypothetical lightweight mesh for edge computing"),
        ("QuantumMesh", "Hypothetical quantum-enhanced mesh for ultra-low latency"),
        ("CustomMesh", "Organization-specific mesh implementation"),
    ];

    info!("BearDog supports these service mesh types:");
    for (name, description) in mesh_examples {
        info!("  🕸️  {}: {}", name, description);
        info!("      Integration: Universal API with automatic capability detection");
        info!("      Failover: Automatic fallback if {} becomes unavailable", name);
    }

    info!("🎯 Key Benefits of Universal Approach:");
    info!("  ✅ No vendor lock-in - works with any compliant mesh");
    info!("  ✅ Automatic discovery and selection of best available mesh");
    info!("  ✅ Seamless failover between mesh implementations");
    info!("  ✅ Future-proof - new mesh primals automatically supported");
    info!("  ✅ Graceful degradation - can operate standalone if no mesh available");

    Ok(())
}

/// Helper function to create example service requests
fn create_example_request(service_id: &str, target_primal: &str, operation: &str) -> ServiceRequest {
    ServiceRequest {
        request_id: Uuid::new_v4(),
        service_id: service_id.to_string(),
        target_primal: target_primal.to_string(),
        operation: operation.to_string(),
        payload: serde_json::json!({
            "demo": true,
            "data": "example_data_for_processing"
        }),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("source".to_string(), "BearDog".to_string());
            meta.insert("demo_mode".to_string(), "true".to_string());
            meta
        },
        source: PrimalIdentity {
            name: "BearDog".to_string(),
            primal_type: PrimalType::BearDog,
            node_id: "beardog-demo-node".to_string(),
        },
    }
} 