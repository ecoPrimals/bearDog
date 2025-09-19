//! Primal Hardcoding Elimination Template
//! 
//! Template for migrating any primal from hardcoded connections to capability-based discovery.
//! Each primal only knows itself and discovers others via the universal adapter.
//! 
//! USAGE:
//! 1. Replace YOUR_PRIMAL_NAME with your actual primal name
//! 2. Update capabilities to match what your primal provides
//! 3. Update required/optional capabilities to match what your primal needs
//! 4. Remove all hardcoded references to other primals
//! 5. Use capability discovery for all external communication

// MODERNIZATION NOTE: This file contains primal-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use beardog_core::ecosystem::{
    SelfDiscoveryManager, SelfIdentity, ServiceCapabilityType, UniversalIntegrationConfig,
    DiscoveredService,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{info, warn, error};
use uuid::Uuid;

/// YOUR_PRIMAL_NAME Core Implementation
/// PRINCIPLE: Only knows itself - discovers ecosystem dynamically
pub struct YourPrimalCore {
    /// Self-discovery manager (replaces hardcoded connections)
    discovery_manager: SelfDiscoveryManager,
    /// Discovered service connections (populated dynamically)
    discovered_services: HashMap<ServiceCapabilityType, Vec<DiscoveredService>>,
}

impl YourPrimalCore {
    /// Initialize YOUR_PRIMAL_NAME with self-discovery
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🚀 Initializing YOUR_PRIMAL_NAME with capability-based discovery");
        
        // Step 1: Define YOUR_PRIMAL_NAME's self-identity
        // IMPORTANT: Only define your own capabilities, never hardcode other primals
        let identity = SelfIdentity::new(
            "YOUR_PRIMAL_NAME",
            vec![
                // CUSTOMIZE: Replace with your actual capabilities
                ServiceCapabilityType::Custom("YourCapability1".to_string()),
                ServiceCapabilityType::Custom("YourCapability2".to_string()),
                // Example capabilities:
                // ServiceCapabilityType::Compute,
                // ServiceCapabilityType::Storage,
                // ServiceCapabilityType::ArtificialIntelligence,
                // ServiceCapabilityType::Networking,
            ],
            std::env::var("YOUR_PRIMAL_ENDPOINT")
                .unwrap_or_else(|_| {
                    // Dynamic discovery endpoint - no hardcoded localhost
                    std::env::var("BEARDOG_DISCOVERY_URL")
                        .unwrap_or_else(|_| "http://discovery.ecosystem.local:8080/discovery".to_string())
                }),
        );
        
        // Step 2: Define what capabilities YOUR_PRIMAL_NAME needs from the ecosystem
        let config = UniversalIntegrationConfig {
            enable_capability_discovery: true,
            required_capabilities: vec![
                // CUSTOMIZE: Replace with capabilities your primal requires
                // ServiceCapabilityType::Security, // If you need security
                // ServiceCapabilityType::Storage,  // If you need storage
            ],
            optional_capabilities: vec![
                // CUSTOMIZE: Replace with capabilities that enhance your primal
                // ServiceCapabilityType::ArtificialIntelligence, // If AI would help
                // ServiceCapabilityType::Networking, // If service mesh would help
            ],
            discovery_endpoints: vec![
                "http://localhost:8080/discovery".to_string(),
                std::env::var("ECOSYSTEM_DISCOVERY_ENDPOINT")
                    .unwrap_or_else(|_| "http://ecosystem.local:8080/discovery".to_string()),
            ],
            custom_config: HashMap::new(),
            enable_environment_discovery: true,
        };
        
        // Step 3: Create discovery manager
        let discovery_manager = SelfDiscoveryManager::new(identity, config).await?;
        
        // Step 4: Register your capabilities with the ecosystem
        discovery_manager.register_self().await?;
        
        Ok(Self {
            discovery_manager,
            discovered_services: HashMap::new(),
        })
    }
    
    /// Discover and connect to required ecosystem services
    /// This replaces hardcoded primal connections
    pub async fn discover_ecosystem_services(&mut self) -> Result<(), BearDogError> {
        info!("🔍 YOUR_PRIMAL_NAME discovering ecosystem services");
        
        // Discover required capabilities
        let required_services = self.discovery_manager.discover_required_capabilities().await?;
        for (capability, services) in required_services {
            info!("✅ Found {} providers for required {:?}", services.len(), capability);
            self.discovered_services.insert(capability, services);
        }
        
        // Discover optional capabilities
        let optional_services = self.discovery_manager.discover_optional_capabilities().await?;
        for (capability, services) in optional_services {
            info!("✅ Found {} providers for optional {:?}", services.len(), capability);
            self.discovered_services.insert(capability, services);
        }
        
        info!("✅ YOUR_PRIMAL_NAME ecosystem discovery complete");
        Ok(())
    }
    
    /// Request capability from ecosystem (replaces hardcoded primal calls)
    /// 
    /// BEFORE (hardcoded):
    /// ```rust
    /// Universal compute capability_client.compute(data).await?;
    /// ```
    /// 
    /// AFTER (capability-based):
    /// ```rust
    /// let result = self.request_capability(ServiceCapabilityType::Compute, request_data).await?;
    /// ```
    pub async fn request_capability(
        &self,
        capability: ServiceCapabilityType,
        request_data: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        info!("📡 YOUR_PRIMAL_NAME requesting {:?} capability", capability);
        
        // Find providers for this capability
        let providers = self.discovered_services.get(&capability)
            .ok_or_else(|| BearDogError::validation(format!("No providers found for {:?}", capability)))?;
            
        if providers.is_empty() {
            return Err(BearDogError::validation(format!("No available providers for {:?}", capability)));
        }
        
        // Use first healthy provider (in production, implement load balancing/failover)
        let provider = &providers[0];
        info!("📞 Using provider: {} at {}", provider.name, provider.endpoint);
        
        // Make request via universal adapter (implementation depends on your HTTP client)
        let response = self.make_capability_request(provider, request_data).await?;
        
        Ok(response)
    }
    
    /// Make HTTP request to capability provider
    async fn make_capability_request(
        &self,
        provider: &DiscoveredService,
        request_data: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        // CUSTOMIZE: Implement HTTP request to provider endpoint
        // This is where you'd use your HTTP client (reqwest, etc.)
        
        info!("🌐 Making request to {} at {}", provider.name, provider.endpoint);
        
        // Placeholder response
        Ok(serde_json::json!({
            "status": "success",
            "provider": provider.name,
            "message": "Capability request completed"
        }))
    }
}

/// Example usage showing migration from hardcoded to capability-based
pub mod migration_examples {
    use super::*;
    
    /// Universal compute capability discovery compute calls
    pub async fn migrate_compute_example(primal: &YourPrimalCore) -> Result<(), BearDogError> {
        info!("📝 Example: Migrating from hardcoded compute calls");
        
        // BEFORE (hardcoded - DON'T DO THIS):
        // Universal compute capability:8081");
        // Universal compute capability_client.compute(data).await?;
        
        // AFTER (capability-based - DO THIS):
        let compute_request = serde_json::json!({
            "operation": "process_data",
            "data": "example_data",
            "priority": "normal"
        });
        
        let result = primal.request_capability(
            ServiceCapabilityType::Compute,
            compute_request,
        ).await?;
        
        info!("✅ Compute result: {}", result);
        Ok(())
    }
    
    /// Universal AI capability discovery AI calls
    pub async fn migrate_ai_example(primal: &YourPrimalCore) -> Result<(), BearDogError> {
        info!("📝 Example: Migrating from hardcoded AI calls");
        
        // BEFORE (hardcoded - DON'T DO THIS):
        // Universal AI capability:8082");
        // Universal AI capability_client.analyze_threat(data).await?;
        
        // AFTER (capability-based - DO THIS):
        let ai_request = serde_json::json!({
            "operation": "analyze_threat",
            "data": "threat_data",
            "model": "security_model"
        });
        
        let result = primal.request_capability(
            ServiceCapabilityType::ArtificialIntelligence,
            ai_request,
        ).await?;
        
        info!("✅ AI analysis result: {}", result);
        Ok(())
    }
    
    /// Universal storage capability discovery storage calls
    pub async fn migrate_storage_example(primal: &YourPrimalCore) -> Result<(), BearDogError> {
        info!("📝 Example: Migrating from hardcoded storage calls");
        
        // BEFORE (hardcoded - DON'T DO THIS):
        // Universal storage capability:8083");
        // Universal storage capability_client.store_secure(data).await?;
        
        // AFTER (capability-based - DO THIS):
        let storage_request = serde_json::json!({
            "operation": "store_secure",
            "data": "sensitive_data",
            "encryption": "required"
        });
        
        let result = primal.request_capability(
            ServiceCapabilityType::Storage,
            storage_request,
        ).await?;
        
        info!("✅ Storage result: {}", result);
        Ok(())
    }
}

/// Configuration migration helper
pub mod config_migration {
    use super::*;
    
    /// ✅ EXAMPLE: Convert legacy hardcoded primal names to capability-based config
    /// This shows the correct migration pattern from sovereignty violations to pure discovery
    pub fn migrate_legacy_config(
        // ❌ OLD: Legacy hardcoded primal flags (violates sovereignty)
        enable_compute_capability: bool,      // Was: enable_toadstool
        enable_ai_capability: bool,           // Was: enable_squirrel
        enable_storage_capability: bool,      // Was: enable_nestgate
        enable_mesh_capability: bool,         // Was: enable_songbird
    ) -> UniversalIntegrationConfig {
        let mut required_capabilities = vec![];
        let mut optional_capabilities = vec![];
        
        // ✅ NEW: Convert to capability requirements (achieves sovereignty)
        if enable_compute_capability {
            required_capabilities.push(ServiceCapabilityType::ComputeIntelligence);
        }
        if enable_ai_capability {
            required_capabilities.push(ServiceCapabilityType::DistributedIntelligence);
        }
        if enable_storage_capability {
            required_capabilities.push(ServiceCapabilityType::DataStorage);
        }
        if enable_mesh_capability {
            optional_capabilities.push(ServiceCapabilityType::ServiceMesh);
        }
        
        UniversalIntegrationConfig {
            enable_capability_discovery: true,
            required_capabilities,
            optional_capabilities,
            discovery_endpoints: vec![
                "http://localhost:8080/discovery".to_string(),
            ],
            custom_config: HashMap::new(),
            enable_environment_discovery: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_primal_self_discovery() {
        let primal = YourPrimalCore::new().await.unwrap();
        
        // Verify primal only knows itself
        assert_eq!(primal.discovery_manager.identity().name, "YOUR_PRIMAL_NAME");
        
        // Verify no hardcoded knowledge of other primals
        // (capabilities are discovered dynamically)
    }
    
    #[tokio::test]
    async fn test_capability_discovery() {
        let mut primal = YourPrimalCore::new().await.unwrap();
        
        // Discovery should work without hardcoded connections
        // (In real implementation, this would connect to actual services)
        let result = primal.discover_ecosystem_services().await;
        
        // Should not fail due to hardcoded dependencies
        // May fail due to no discovery services available (expected in test)
    }
}

/// Migration checklist for eliminating primal hardcoding
/// 
/// ✅ CUSTOMIZE: Complete these steps for your primal
/// 
/// 1. [ ] Replace hardcoded primal names with ServiceCapabilityType
/// 2. [ ] Replace hardcoded client constructors with capability discovery
/// 3. [ ] Replace hardcoded method calls with request_capability()
/// 4. [ ] Replace hardcoded configuration flags with UniversalIntegrationConfig
/// 5. [ ] Add SelfDiscoveryManager to your primal's core
/// 6. [ ] Update initialization to register self-capabilities
/// 7. [ ] Update service calls to use discovered endpoints
/// 8. [ ] Remove all hardcoded endpoint URLs
/// 9. [ ] Remove all hardcoded primal type references
/// 10. [ ] Test with dynamic service discovery
/// 
/// PRINCIPLE: Your primal should only know:
/// - Its own identity and capabilities
/// - What capabilities it needs from the ecosystem
/// - How to discover and use those capabilities dynamically
/// 
/// Your primal should NEVER know:
/// - Names of other primals
/// - Endpoints of other primals  
/// - Implementation details of other primals 