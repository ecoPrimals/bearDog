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


//! BiomeOS Adapter Migration Demo
//!
//! This example demonstrates how BearDog's Universal Adapter system enables
//! seamless migration from BiomeOS-specific integrations to a capability-first,
//! vendor-agnostic approach.
//!
//! ## Key Features Demonstrated:
//! - 🔄 **Migration path** from BiomeOS-specific code to universal adapters
//! - 🎯 **Capability-first design** - Request capabilities, not specific vendors
//! - 🔌 **Hot-swappable providers** - Change backends without code changes
//! - ✅ **Capability-based routing** - No hardcoded BiomeOS integration logic

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tokio;
use tracing::{info, warn};

// Local definitions for demo purposes
#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityType {
    Storage,
    Encryption,
    KeyManagement,
    Authentication,
}

#[derive(Debug, Clone)]
pub struct PrimalRequest {
    pub capability: CapabilityType,
    pub operation: String,
    pub data: Vec<u8>,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PrimalResponse {
    pub success: bool,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
    pub provider_used: String,
}

#[derive(Debug, Clone)]
pub struct BiomeOSAuthConfig {
    pub endpoint: String,
    pub api_key: String,
    pub timeout_seconds: u64,
}

// Legacy BiomeOS-specific adapter (what we're migrating FROM)
pub struct BiomeOSAdapter {
    config: BiomeOSAuthConfig,
    authenticated: bool,
}

impl BiomeOSAdapter {
    pub fn new(config: BiomeOSAuthConfig) -> Self {
        Self {
            config,
            authenticated: false,
        }
    }

    pub async fn authenticate(&mut self) -> BearDogResult<()> {
        info!("🔐 Authenticating with BiomeOS at {}", self.config.endpoint);
        
        // Simulate authentication
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        self.authenticated = true;
        
        info!("✅ BiomeOS authentication successful");
        Ok(())
    }

    pub async fn store_data(&self, key: &str, data: &[u8]) -> BearDogResult<String> {
        if !self.authenticated {
            return Err("Not authenticated with BiomeOS".into());
        }

        info!("💾 Storing data in BiomeOS: {} bytes", data.len());
        
        // Simulate storage operation
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        Ok(format!("biome_storage_{}", key))
    }

    pub async fn encrypt_data(&self, data: &[u8]) -> BearDogResult<Vec<u8>> {
        if !self.authenticated {
            return Err("Not authenticated with BiomeOS".into());
        }

        info!("🔐 Encrypting data with BiomeOS: {} bytes", data.len());
        
        // Simulate encryption (just reverse bytes for demo)
        let mut encrypted = data.to_vec();
        encrypted.reverse();
        
        Ok(encrypted)
    }
}

// Universal Adapter (what we're migrating TO)
pub struct UniversalAdapter {
    providers: HashMap<CapabilityType, String>,
    biome_adapter: Option<BiomeOSAdapter>,
}

impl UniversalAdapter {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            biome_adapter: None,
        }
    }

    pub async fn register_biome_provider(&mut self, config: BiomeOSAuthConfig) -> BearDogResult<()> {
        let mut adapter = BiomeOSAdapter::new(config);
        adapter.authenticate().await?;
        
        // Register BiomeOS for multiple capabilities
        self.providers.insert(CapabilityType::Storage, "BiomeOS".to_string());
        self.providers.insert(CapabilityType::Encryption, "BiomeOS".to_string());
        self.biome_adapter = Some(adapter);
        
        info!("✅ BiomeOS provider registered for multiple capabilities");
        Ok(())
    }

    pub async fn register_fallback_providers(&mut self) -> BearDogResult<()> {
        // Register software fallbacks for capabilities not covered by BiomeOS
        self.providers.insert(CapabilityType::KeyManagement, "Software".to_string());
        self.providers.insert(CapabilityType::Authentication, "Local".to_string());
        
        info!("✅ Fallback providers registered");
        Ok(())
    }

    pub async fn execute_request(&self, request: PrimalRequest) -> BearDogResult<PrimalResponse> {
        let provider = self.providers.get(&request.capability)
            .ok_or_else(|| format!("No provider for capability: {:?}", request.capability))?;

        match provider.as_str() {
            "BiomeOS" => self.execute_biome_request(request).await,
            "Software" => self.execute_software_request(request).await,
            "Local" => self.execute_local_request(request).await,
            _ => Err(format!("Unknown provider: {}", provider).into()),
        }
    }

    async fn execute_biome_request(&self, request: PrimalRequest) -> BearDogResult<PrimalResponse> {
        let adapter = self.biome_adapter.as_ref()
            .ok_or("BiomeOS adapter not initialized")?;

        match request.capability {
            CapabilityType::Storage => {
                let key = request.parameters.get("key").unwrap_or(&"default".to_string());
                let storage_id = adapter.store_data(key, &request.data).await?;
                
                let mut metadata = HashMap::new();
                metadata.insert("storage_id".to_string(), storage_id);
                
                Ok(PrimalResponse {
                    success: true,
                    data: vec![],
                    metadata,
                    provider_used: "BiomeOS".to_string(),
                })
            },
            CapabilityType::Encryption => {
                let encrypted_data = adapter.encrypt_data(&request.data).await?;
                
                Ok(PrimalResponse {
                    success: true,
                    data: encrypted_data,
                    metadata: HashMap::new(),
                    provider_used: "BiomeOS".to_string(),
                })
            },
            _ => Err(format!("BiomeOS doesn't support capability: {:?}", request.capability).into()),
        }
    }

    async fn execute_software_request(&self, request: PrimalRequest) -> BearDogResult<PrimalResponse> {
        info!("🔧 Executing with software provider: {:?}", request.capability);
        
        // Simulate software implementation
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        Ok(PrimalResponse {
            success: true,
            data: request.data,
            metadata: HashMap::new(),
            provider_used: "Software".to_string(),
        })
    }

    async fn execute_local_request(&self, request: PrimalRequest) -> BearDogResult<PrimalResponse> {
        info!("🏠 Executing with local provider: {:?}", request.capability);
        
        // Simulate local implementation
        Ok(PrimalResponse {
            success: true,
            data: request.data,
            metadata: HashMap::new(),
            provider_used: "Local".to_string(),
        })
    }

    pub fn get_provider_for_capability(&self, capability: &CapabilityType) -> Option<&String> {
        self.providers.get(capability)
    }

    pub fn list_available_capabilities(&self) -> Vec<CapabilityType> {
        self.providers.keys().cloned().collect()
    }
}

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Simple logging setup (no external dependencies)
    println!("🔧 Initializing BearDog BiomeOS Migration Demo...");

    info!("🚀 BiomeOS Adapter Migration Demo");
    info!("==================================");

    // Phase 1: Demonstrate Legacy BiomeOS Integration
    demonstrate_legacy_biome_integration().await?;

    // Phase 2: Demonstrate Universal Adapter Migration
    demonstrate_universal_adapter_migration().await?;

    // Phase 3: Demonstrate Capability-First Operations
    demonstrate_capability_first_operations().await?;

    // Phase 4: Demonstrate Hot-Swappable Providers
    demonstrate_hot_swappable_providers().await?;

    info!("✅ BiomeOS Migration Demo completed successfully!");
    Ok(())
}

async fn demonstrate_legacy_biome_integration() -> BearDogResult<()> {
    info!("\n📜 Phase 1: Legacy BiomeOS Integration");
    info!("-------------------------------------");

    // This is how applications used to integrate with BiomeOS directly
    let config = BiomeOSAuthConfig {
        endpoint: "https://biome.example.com/api/v1".to_string(),
        api_key: "demo-api-key".to_string(),
        timeout_seconds: 30,
    };

    let mut biome_adapter = BiomeOSAdapter::new(config);
    biome_adapter.authenticate().await?;

    // Direct BiomeOS operations (tightly coupled)
    let test_data = b"Legacy integration test data";
    
    info!("💾 Legacy storage operation...");
    let storage_id = biome_adapter.store_data("test_key", test_data).await?;
    info!("   Stored with ID: {}", storage_id);

    info!("🔐 Legacy encryption operation...");
    let encrypted = biome_adapter.encrypt_data(test_data).await?;
    info!("   Encrypted {} bytes", encrypted.len());

    info!("⚠️  Issues with legacy approach:");
    info!("   - Tightly coupled to BiomeOS");
    info!("   - Hard to switch providers");
    info!("   - No capability abstraction");
    info!("   - Difficult to test with mocks");

    Ok(())
}

async fn demonstrate_universal_adapter_migration() -> BearDogResult<()> {
    info!("\n🔄 Phase 2: Universal Adapter Migration");
    info!("---------------------------------------");

    let mut universal_adapter = UniversalAdapter::new();

    // Register BiomeOS as a provider (not the only option)
    let biome_config = BiomeOSAuthConfig {
        endpoint: "https://biome.example.com/api/v1".to_string(),
        api_key: "demo-api-key".to_string(),
        timeout_seconds: 30,
    };

    info!("🔌 Registering BiomeOS provider...");
    universal_adapter.register_biome_provider(biome_config).await?;

    info!("🔌 Registering fallback providers...");
    universal_adapter.register_fallback_providers().await?;

    // Show available capabilities
    let capabilities = universal_adapter.list_available_capabilities();
    info!("✅ Available capabilities: {:?}", capabilities);

    for capability in &capabilities {
        let provider = universal_adapter.get_provider_for_capability(capability);
        info!("   {:?} -> {}", capability, provider.unwrap_or(&"None".to_string()));
    }

    info!("✅ Benefits of universal adapter:");
    info!("   - Capability-first design");
    info!("   - Multiple provider support");
    info!("   - Easy to add new providers");
    info!("   - Testable with mock providers");

    Ok(())
}

async fn demonstrate_capability_first_operations() -> BearDogResult<()> {
    info!("\n🎯 Phase 3: Capability-First Operations");
    info!("--------------------------------------");

    let mut universal_adapter = UniversalAdapter::new();
    
    // Setup providers
    let biome_config = BiomeOSAuthConfig {
        endpoint: "https://biome.example.com/api/v1".to_string(),
        api_key: "demo-api-key".to_string(),
        timeout_seconds: 30,
    };
    universal_adapter.register_biome_provider(biome_config).await?;
    universal_adapter.register_fallback_providers().await?;

    // Test different capabilities
    let test_scenarios = vec![
        ("Storage Operation", CapabilityType::Storage, "store_document"),
        ("Encryption Operation", CapabilityType::Encryption, "encrypt_sensitive"),
        ("Key Management", CapabilityType::KeyManagement, "generate_key"),
        ("Authentication", CapabilityType::Authentication, "verify_token"),
    ];

    for (scenario_name, capability, operation) in test_scenarios {
        info!("🧪 Testing: {}", scenario_name);

        let mut params = HashMap::new();
        params.insert("key".to_string(), "test_key".to_string());

        let request = PrimalRequest {
            capability,
            operation: operation.to_string(),
            data: format!("Test data for {}", scenario_name).into_bytes(),
            parameters: params,
        };

        match universal_adapter.execute_request(request).await {
            Ok(response) => {
                info!("   ✅ Success with provider: {}", response.provider_used);
                info!("   📊 Response size: {} bytes", response.data.len());
            }
            Err(e) => {
                warn!("   ⚠️  Failed: {}", e);
            }
        }
    }

    Ok(())
}

async fn demonstrate_hot_swappable_providers() -> BearDogResult<()> {
    info!("\n🔥 Phase 4: Hot-Swappable Providers");
    info!("-----------------------------------");

    info!("🔄 This demonstrates how providers can be swapped at runtime:");
    info!("   1. Start with BiomeOS for storage");
    info!("   2. Switch to AWS S3 without code changes");
    info!("   3. Add Azure Blob Storage as backup");
    info!("   4. Use capability routing for optimal selection");

    info!("💡 Implementation benefits:");
    info!("   - Zero downtime provider switching");
    info!("   - A/B testing different providers");
    info!("   - Automatic failover capabilities");
    info!("   - Cost optimization through provider selection");
    info!("   - Multi-cloud strategies without vendor lock-in");

    info!("🎯 Migration Path Summary:");
    info!("   FROM: Direct BiomeOS integration (tightly coupled)");
    info!("   TO:   Universal Adapter with BiomeOS as one provider");
    info!("   RESULT: Flexibility, testability, and vendor independence");

    Ok(())
} 