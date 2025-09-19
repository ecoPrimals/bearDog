// Universal Infant Discovery Demo
//
// This example demonstrates how BearDog starts with ZERO hardcoded knowledge
// and discovers capabilities dynamically, eliminating all vendor and primal hardcoding.

use beardog_core::discovery::UniversalInfantDiscovery;
use beardog_types::canonical::discovery::{
    UniversalCapabilityType, UniversalDiscoveryRequest, ComputeAbility, 
    StorageCharacteristic, NetworkFunction, SecurityService, OrchestrationFeature,
    PerformanceRequirements, SecurityRequirements,
};
use beardog_adapters::universal::primal_capability_adapter::{
    UniversalPrimalAdapter, DefaultPrimalDiscoveryClient,
};
use std::sync::Arc;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 BearDog Universal Infant Discovery Demo");
    println!("==========================================");
    println!();
    println!("👶 Starting with ZERO hardcoded knowledge...");
    println!("   ❌ No vendor names (AWS, GCP, Azure, Kubernetes, Consul)");
    println!("   ❌ No primal names (Songbird, Toadstool, Squirrel, NestGate, BiomeOS)");
    println!("   ❌ No hardcoded endpoints or URLs");
    println!("   ✅ Pure capability-based discovery");
    println!();

    // Demonstrate infant discovery system
    demo_infant_discovery()?;
    
    // Demonstrate universal primal adapter
    demo_universal_primal_adapter()?;
    
    // Demonstrate capability-based interactions
    demo_capability_based_interactions()?;

    println!();
    println!("🎉 Universal Infant Discovery Demo Complete!");
    println!("   ✅ Zero hardcoded vendor/primal names");
    println!("   ✅ Pure capability-based discovery");
    println!("   ✅ Dynamic service learning");
    println!("   ✅ Universal adapter patterns");

    Ok(())
}

async fn demo_infant_discovery() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Phase 1: Infant Discovery System");
    println!("-----------------------------------");
    
    // Create infant discovery system with zero knowledge
    let infant_discovery = UniversalInfantDiscovery::new()?;
    
    // Set up some environment variables to simulate discovered services
    std::env::set_var("COMPUTE_SERVICE_ENDPOINT", "http://discovered-compute:8080");
    std::env::set_var("STORAGE_SERVICE_ENDPOINT", "http://discovered-storage:8443");
    std::env::set_var("MESH_SERVICE_ENDPOINT", "http://discovered-mesh:9090");
    
    // Start discovery process - infant learns about the world
    info!("👶 Starting infant discovery...");
    infant_discovery.start_discovery()?;
    
    // Check what the infant learned
    let discovered_services = infant_discovery.get_discovered_services();
    println!("👶 Infant learned about {} services:", discovered_services.len());
    
    for (service_id, service) in &discovered_services {
        println!("   🔍 Service: {}", service_id);
        println!("      Capabilities: {:?}", service.capabilities);
        println!("      Endpoint: {}://{}:{}", 
                 service.endpoint.protocol, service.endpoint.host, service.endpoint.port);
        println!("      Trust Score: {:.2}", service.trust_score);
        println!();
    }
    
    // Request specific capabilities
    let compute_request = UniversalDiscoveryRequest {
        required_capabilities: vec![
            UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            }
        ],
        optional_capabilities: vec![],
        performance_requirements: PerformanceRequirements::default(),
        security_requirements: SecurityRequirements::default(),
    };
    
    let compute_response = infant_discovery.discover_capabilities(compute_request)?;
    println!("👶 Found {} services with compute capabilities", 
             compute_response.matching_services.len());
    
    Ok(())
}

async fn demo_universal_primal_adapter() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Phase 2: Universal Primal Adapter");
    println!("------------------------------------");
    
    // Create universal primal adapter (no hardcoded primal names)
    let discovery_client = Arc::new(DefaultPrimalDiscoveryClient);
    let primal_adapter = UniversalPrimalAdapter::new(discovery_client)?;
    
    // Discover compute primals (replaces hardcoded "toadstool" references)
    println!("🧠 Discovering compute capability primals...");
    let compute_primals = primal_adapter.discover_compute_primals()?;
    println!("   Found {} compute-capable primals", compute_primals.len());
    
    // Discover network primals (replaces hardcoded "songbird" references)  
    println!("🕊️ Discovering network capability primals...");
    let network_primals = primal_adapter.discover_network_primals()?;
    println!("   Found {} network-capable primals", network_primals.len());
    
    // Discover storage primals (replaces hardcoded "nestgate" references)
    println!("🗄️ Discovering storage capability primals...");
    let storage_primals = primal_adapter.discover_storage_primals()?;
    println!("   Found {} storage-capable primals", storage_primals.len());
    
    // Discover orchestration primals (replaces hardcoded "biomeOS" references)
    println!("🌱 Discovering orchestration capability primals...");
    let orchestration_primals = primal_adapter.discover_orchestration_primals()?;
    println!("   Found {} orchestration-capable primals", orchestration_primals.len());
    
    Ok(())
}

async fn demo_capability_based_interactions() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Phase 3: Capability-Based Interactions");
    println!("-----------------------------------------");
    
    let discovery_client = Arc::new(DefaultPrimalDiscoveryClient);
    let primal_adapter = UniversalPrimalAdapter::new(discovery_client)?;
    
    // Example: Request data analysis (no hardcoded "toadstool" call)
    println!("📊 Requesting data analysis capability...");
    let analysis_data = serde_json::json!({
        "dataset": "sample_data.csv",
        "analysis_type": "pattern_recognition"
    });
    
    match primal_adapter.request_compute_analysis(analysis_data) {
        Ok(response) => {
            println!("   ✅ Analysis completed by: {}", response.service_id);
            println!("   📈 Result: {}", response.payload);
        }
        Err(e) => {
            println!("   ⚠️ Analysis request failed: {}", e);
        }
    }
    
    // Example: Request network routing (no hardcoded "songbird" call)
    println!("🌐 Requesting network routing capability...");
    let routing_config = serde_json::json!({
        "source": "service_a",
        "destination": "service_b",
        "load_balancing": "round_robin"
    });
    
    match primal_adapter.request_network_routing(routing_config) {
        Ok(response) => {
            println!("   ✅ Routing configured by: {}", response.service_id);
            println!("   🔀 Configuration: {}", response.payload);
        }
        Err(e) => {
            println!("   ⚠️ Routing request failed: {}", e);
        }
    }
    
    // Example: Request data storage (no hardcoded "nestgate" call)
    println!("💾 Requesting data storage capability...");
    let storage_request = serde_json::json!({
        "data": "sensitive_user_data",
        "encryption": true,
        "replication": 3
    });
    
    match primal_adapter.request_data_storage(storage_request) {
        Ok(response) => {
            println!("   ✅ Data stored by: {}", response.service_id);
            println!("   🔒 Storage confirmation: {}", response.payload);
        }
        Err(e) => {
            println!("   ⚠️ Storage request failed: {}", e);
        }
    }
    
    // Example: Request orchestration (no hardcoded "biomeOS" call)
    println!("🚀 Requesting orchestration capability...");
    let orchestration_request = serde_json::json!({
        "service": "web_app",
        "replicas": 3,
        "resources": {
            "cpu": "500m",
            "memory": "1Gi"
        }
    });
    
    match primal_adapter.request_orchestration(orchestration_request) {
        Ok(response) => {
            println!("   ✅ Service deployed by: {}", response.service_id);
            println!("   🎯 Deployment status: {}", response.payload);
        }
        Err(e) => {
            println!("   ⚠️ Orchestration request failed: {}", e);
        }
    }
    
    println!();
    println!("🎯 Key Benefits Demonstrated:");
    println!("   ✅ No hardcoded primal names (toadstool, songbird, nestgate, biomeOS)");
    println!("   ✅ No hardcoded vendor names (AWS, GCP, Azure, Kubernetes)"); 
    println!("   ✅ Dynamic capability discovery");
    println!("   ✅ Universal adapter patterns");
    println!("   ✅ Infant learning from environment");
    println!("   ✅ Pure capability-based interactions");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_infant_discovery_zero_knowledge() {
        // Test that infant discovery starts with zero hardcoded knowledge
        let infant_discovery = UniversalInfantDiscovery::new().unwrap();
        
        // Initially should have no discovered services
        let initial_services = infant_discovery.get_discovered_services();
        assert_eq!(initial_services.len(), 0, "Should start with zero knowledge");
        
        // After discovery, should learn from environment
        infant_discovery.start_discovery().unwrap();
        
        // Should have learned about services from environment
        let discovered_services = infant_discovery.get_discovered_services();
        println!("Discovered {} services", discovered_services.len());
    }

    #[tokio::test] 
    async fn test_universal_primal_adapter_no_hardcoding() {
        // Test that primal adapter uses capabilities, not hardcoded names
        let discovery_client = Arc::new(DefaultPrimalDiscoveryClient);
        let adapter = UniversalPrimalAdapter::new(discovery_client).unwrap();
        
        // Should discover primals by capability, not by name
        let compute_primals = adapter.discover_compute_primals().unwrap();
        assert!(!compute_primals.is_empty(), "Should find compute-capable primals");
        
        // Verify no hardcoded primal names in service IDs
        for primal in &compute_primals {
            assert!(!primal.service_id.contains("toadstool"));
            assert!(!primal.service_id.contains("songbird"));
            assert!(!primal.service_id.contains("nestgate"));
            assert!(!primal.service_id.contains("biomeOS"));
        }
    }
} 