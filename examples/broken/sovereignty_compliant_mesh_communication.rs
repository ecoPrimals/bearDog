// # Sovereignty-Compliant Mesh Communication Example
//
// This example demonstrates the CORRECT way to communicate with mesh services
// using BearDog's capability-based discovery system, replacing hardcoded
// songbird references with proper sovereignty-compliant architecture.

use beardog_core::ecosystem_integration::universal_adapter::{
    UniversalAdapter, UniversalAdapterConfig, AdapterRequest, AdapterOperation, ProtocolType
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize logging
    tracing_subscriber::init();

    info!("[TARGET] Sovereignty-Compliant Mesh Communication Example");
    info!("🚫 NO hardcoded service names - using capability discovery");

    // Step 1: Create universal adapter (sovereignty-compliant)
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalAdapter::new(config);

    // Step 2: Discover mesh service by capability (CORRECT approach)
    info!("[SEARCH] Discovering mesh communication service...");
    let mesh_service = adapter.discover_service_by_capability("mesh-communication")?;
    
    info!("[OK] Discovered mesh service:");
    info!("   Service ID: {}", mesh_service.service_id);
    info!("   Endpoint: {}", mesh_service.endpoint);
    info!("   Capabilities: {:?}", mesh_service.capabilities);

    // Step 3: Connect to discovered service
    let connection_id = adapter.connect(
        mesh_service.endpoint.clone(),
        mesh_service.protocol
    )?;
    
    info!("🔗 Connected to mesh service with ID: {}", connection_id);

    // Step 4: Send mesh communication request
    let request = AdapterRequest::new(
        AdapterOperation::Request,
        mesh_service.endpoint
    )
    .with_header("Content-Type".to_string(), "application/json".to_string())
    .with_header("X-Service-Capability".to_string(), "mesh-communication".to_string());

    let response = adapter.send_request(request)?;
    
    info!("📨 Mesh communication response:");
    info!("   Status: {:?}", response.status);
    info!("   Duration: {}ms", response.duration_ms);

    // Step 5: Demonstrate other capability discoveries
    demonstrate_other_capabilities(&adapter)?;

    // Step 6: Clean up
    adapter.disconnect(connection_id)?;
    info!("🧹 Cleaned up connections");

    info!("[PARTY] Sovereignty-compliant mesh communication complete!");
    Ok(())
}

/// Demonstrates discovering other services by capability
async fn demonstrate_other_capabilities(adapter: &UniversalAdapter) -> Result<(), BearDogError> {
    info!("[SEARCH] Demonstrating other capability discoveries...");

    // Discover compute orchestration service
    match adapter.discover_service_by_capability("compute-orchestration") {
        Ok(service) => {
            info!("[OK] Found compute service: {} at {}", service.service_id, service.endpoint);
        }
        Err(e) => {
            warn!("⚠️  Compute service not available: {}", e);
        }
    }

    // Discover AI coordination service
    match adapter.discover_service_by_capability("ai-coordination") {
        Ok(service) => {
            info!("[OK] Found AI service: {} at {}", service.service_id, service.endpoint);
        }
        Err(e) => {
            warn!("⚠️  AI service not available: {}", e);
        }
    }

    // Discover custom capability
    match adapter.discover_service_by_capability("data-processing") {
        Ok(service) => {
            info!("[OK] Found data processing service: {} at {}", service.service_id, service.endpoint);
        }
        Err(e) => {
            warn!("⚠️  Data processing service not available: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capability_based_discovery() {
        let adapter = UniversalAdapter::new(UniversalAdapterConfig::default());
        
        // Test mesh service discovery
        let result = adapter.discover_service_by_capability("mesh-communication");
        assert!(result.is_ok(), "Mesh service discovery should succeed");
        
        let service = result.unwrap();
        assert_eq!(service.service_id, "mesh-service");
        assert!(service.capabilities.contains(&"mesh-communication".to_string()));
    }

    #[tokio::test]
    async fn test_environment_based_endpoints() {
        // Test that environment variables are used for service discovery
        std::env::set_var("MESH_SERVICE_ENDPOINT", "http://test-mesh:9090");
        
        let adapter = UniversalAdapter::new(UniversalAdapterConfig::default());
        let service = adapter.discover_service_by_capability("mesh-communication").unwrap();
        
        assert_eq!(service.endpoint, "http://test-mesh:9090");
        
        std::env::remove_var("MESH_SERVICE_ENDPOINT");
    }

    #[test]
    fn test_no_hardcoded_references() {
        // This test ensures no hardcoded service names exist in the example
        let source = include_str!("sovereignty_compliant_mesh_communication.rs");
        
        // Should NOT contain hardcoded service names
        assert!(!source.contains("songbird"), "Should not contain hardcoded 'songbird'");
        assert!(!source.contains("Songbird"), "Should not contain hardcoded 'Songbird'");
        
        // Should contain capability-based discovery
        assert!(source.contains("capability"), "Should use capability-based discovery");
        assert!(source.contains("discover_service_by_capability"), "Should use discovery method");
    }
} 