

use super::{
    IntegrationEngine, 
    UniversalHsmProvider, 
    SongbirdServiceDiscoveryFactory,
    EcosystemHsmProvider,
    ProviderHealthStatus,
};
use beardog_errors::BearDogResult;
use beardog_types::canonical::crypto::{KeyType, KeyMetadata};
use beardog_types::canonical::hsm::{HsmCapabilities, HsmProvider};
use beardog_traits::canonical::CanonicalHsmProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

pub async fn example_universal_hsm_setup() -> BearDogResult<()> {
    info!("🚀 Example: Universal HSM Architecture Setup");

    let mut integration_engine = IntegrationEngine::new();

    integration_engine.initialize_universal_hsm().await?;
    info!("✅ Universal HSM Architecture initialized");

    integration_engine.integrate_with_ecosystem().await?;
    info!("✅ Ecosystem integration completed");

    if let Some(universal_hsm) = integration_engine.get_universal_hsm() {
        info!("🔐 Universal HSM Provider available");

        let discovered_providers = universal_hsm.discover_ecosystem_providers().await?;
        info!("🌐 Discovered {} ecosystem HSM providers", discovered_providers.len());

        let status = universal_hsm.get_ecosystem_status().await?;
        info!("📊 Universal HSM Status: {}", serde_json::to_string_pretty(&status)?);
    }

    let health_status = integration_engine.get_service_health().await?;
    info!("🏥 Service Health: {}", serde_json::to_string_pretty(&health_status)?);

    info!("🎉 Universal HSM Architecture setup completed successfully!");
    Ok(())
}

pub async fn example_vendor_agnostic_operations() -> BearDogResult<()> {
    info!("🔐 Example: Vendor-Agnostic HSM Operations");

    let universal_hsm = UniversalHsmProvider::new();

    let mock_provider = MockHsmProvider::new();
    universal_hsm.register_provider(
        "mock_hsm_1".to_string(),
        Box::new(mock_provider),
    ).await?;

    info!("🔑 Generating key through Universal HSM");
    let key_metadata = KeyMetadata {
        usage: vec!["signing".to_string(), "verification".to_string()],
        algorithm: "ECDSA-P256".to_string(),
        extractable: false,
        key_size: Some(256),
        created_at: chrono::Utc::now(),
        expires_at: None,
        tags: ahash::HashMap::default(),
    };

    let hsm_key = universal_hsm.generate_key(KeyType::EcdsaP256, key_metadata).await?;
    info!("✅ Generated key: {}", hsm_key.id);

    let keys = universal_hsm.list_keys().await?;
    info!("📋 Total keys in Universal HSM: {}", keys.len());

    let test_data = b"Hello, Universal HSM Architecture!";
    let signature = universal_hsm.sign_data(&hsm_key.id, test_data).await?;
    info!("✍️ Signed data, signature length: {} bytes", signature.len());

    let is_valid = universal_hsm.verify_signature(&hsm_key.id, test_data, &signature).await?;
    info!("✅ Signature verification: {}", if is_valid { "VALID" } else { "INVALID" });

    info!("🎉 Vendor-agnostic operations completed successfully!");
    Ok(())
}

pub async fn example_ecosystem_provider_registration() -> BearDogResult<()> {
    info!("🌐 Example: Ecosystem HSM Provider Registration");

    let mut songbird_discovery = SongbirdServiceDiscoveryFactory::create_for_development();

    songbird_discovery.register_beardog_service().await?;
    info!("✅ BearDog registered as ecosystem HSM provider");

    let custom_provider = EcosystemHsmProvider {
        id: format_args!("custom-hsm-{}", Uuid::new_v4().to_string()),
        name: "Custom Hardware HSM".to_string(),
        vendor: "Example Corp".to_string(),
        capabilities: HsmCapabilities {
            supported_key_types: vec![KeyType::EcdsaP256, KeyType::Rsa2048],
            max_keys: 5000,
            hardware_backed: true,
            fips_140_2_level: Some(3),
            common_criteria_level: Some(5),
            supports_key_derivation: true,
            supports_bulk_operations: true,
        },
        endpoint: "https://custom-hsm.example.com".to_string(),
        priority: 50,
        health_status: ProviderHealthStatus::Healthy,
        ecosystem_node: "node-1".to_string(),
    };

    songbird_discovery.register_hsm_provider(&custom_provider).await?;
    info!("✅ Custom HSM provider registered");

    let discovered_providers = songbird_discovery.discover_hsm_providers().await?;
    info!("🔍 Discovered {} HSM providers in ecosystem:", discovered_providers.len());
    
    for provider in &discovered_providers {
        info!("  - {} ({}) - Priority: {}, Status: {:?}", 
              provider.name, provider.vendor, provider.priority, provider.health_status);
    }

    let is_connected = songbird_discovery.health_check().await?;
    info!("🏥 Ecosystem connectivity: {}", if is_connected { "HEALTHY" } else { "DEGRADED" });

    info!("🎉 Ecosystem provider registration completed successfully!");
    Ok(())
}

pub async fn example_failover_and_high_availability() -> BearDogResult<()> {
    info!("🛡️ Example: Failover and High Availability");

    let universal_hsm = UniversalHsmProvider::new();

    let primary_provider = MockHsmProvider::new_with_reliability(0.7); // 70% success rate
    universal_hsm.register_provider(
        "primary_hsm".to_string(),
        Box::new(primary_provider),
    ).await?;

    let backup_provider = MockHsmProvider::new_with_reliability(0.9); // 90% success rate
    universal_hsm.register_provider(
        "backup_hsm".to_string(),
        Box::new(backup_provider),
    ).await?;

    info!("✅ Registered primary and backup HSM providers");

    let key_metadata = KeyMetadata {
        usage: vec!["signing".to_string()],
        algorithm: "ECDSA-P256".to_string(),
        extractable: false,
        key_size: Some(256),
        created_at: chrono::Utc::now(),
        expires_at: None,
        tags: ahash::HashMap::default(),
    };

    let mut successful_operations = 0;
    let total_operations = 10;

    for i in 1..=total_operations {
        match universal_hsm.generate_key(KeyType::EcdsaP256, key_metadata.clone()).await {
            Ok(key) => {
                successful_operations += 1;
                info!("✅ Operation {}: Generated key {}", i, key.id);
            }
            Err(e) => {
                warn!("❌ Operation {} failed: {}", i, e);
            }
        }
    }

    let success_rate = (successful_operations as f64 / total_operations as f64) * 100.0;
    info!("📊 Failover Results: {}/{} operations successful ({:.1}% success rate)", 
          successful_operations, total_operations, success_rate);

    let status = universal_hsm.get_ecosystem_status().await?;
    info!("🏥 Final ecosystem status: {}", serde_json::to_string_pretty(&status)?);

    info!("🎉 Failover and high availability demonstration completed!");
    Ok(())
}

#[derive(Debug)]
pub struct MockHsmProvider {
    keys: Arc<tokio::sync::RwLock<HashMap<String, beardog_types::canonical::crypto::HsmKey>>>,
    reliability: f64, // Success rate (0.0 to 1.0)
}

impl MockHsmProvider {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(tokio::sync::RwLock::new(ahash::HashMap::default())),
            reliability: 1.0, // 100% reliable by default
        }
    }

    pub fn new_with_reliability(reliability: f64) -> Self {
        Self {
            keys: Arc::new(tokio::sync::RwLock::new(ahash::HashMap::default())),
            reliability: reliability.clamp(0.0, 1.0),
        }
    }

    fn should_succeed(&self) -> bool {
        use rand::Rng;
        rand::thread_rng().gen::<f64>() < self.reliability
    }
}

impl HsmProvider for MockHsmProvider {
    async fn generate_key(&self, key_type: KeyType, metadata: KeyMetadata) -> BearDogResult<beardog_types::canonical::crypto::HsmKey> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM operation failed".to_string()));
        }

        let key_id = format_args!("mock-key-{}", Uuid::new_v4().to_string());
        let hsm_key = beardog_types::canonical::crypto::HsmKey {
            id: key_id.clone(),
            key_type,
            metadata,
            public_key: Some(b"mock_public_key".to_vec()),
            created_at: chrono::Utc::now(),
            provider_id: "mock_hsm".to_string(),
        };

        let mut keys = self.keys.write().await;
        keys.insert(key_id, hsm_key.clone());

        Ok(hsm_key)
    }

    async fn import_key(&self, _key_data: &[u8], key_type: KeyType, metadata: KeyMetadata) -> BearDogResult<beardog_types::canonical::crypto::HsmKey> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM import failed".to_string()));
        }

        self.generate_key(key_type, metadata).await
    }

    async fn derive_key(&self, _parent_key_id: &str, _derivation_path: &str, derived_key_type: KeyType) -> BearDogResult<beardog_types::canonical::crypto::HsmKey> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM derivation failed".to_string()));
        }

        let metadata = KeyMetadata {
            usage: vec!["derived".to_string()],
            algorithm: "ECDSA-P256".to_string(),
            extractable: false,
            key_size: Some(256),
            created_at: chrono::Utc::now(),
            expires_at: None,
            tags: ahash::HashMap::default(),
        };

        self.generate_key(derived_key_type, metadata).await
    }

    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM deletion failed".to_string()));
        }

        let mut keys = self.keys.write().await;
        keys.remove(key_id);
        Ok(())
    }

    async fn list_keys(&self) -> BearDogResult<Vec<beardog_types::canonical::crypto::HsmKeyInfo>> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM listing failed".to_string()));
        }

        let keys = self.keys.read().await;
        let key_infos = keys.values().map(|key| {
            beardog_types::canonical::crypto::HsmKeyInfo {
                id: key.id.clone(),
                key_type: key.key_type,
                algorithm: key.metadata.algorithm.clone(),
                key_size: key.metadata.key_size,
                created_at: key.created_at,
                usage: key.metadata.usage.clone(),
                extractable: key.metadata.extractable,
            }
        }).collect();

        Ok(key_infos)
    }

    async fn get_key_info(&self, key_id: &str) -> BearDogResult<beardog_types::canonical::crypto::HsmKeyInfo> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM key info failed".to_string()));
        }

        let keys = self.keys.read().await;
        if let Some(key) = keys.get(key_id) {
            Ok(beardog_types::canonical::crypto::HsmKeyInfo {
                id: key.id.clone(),
                key_type: key.key_type,
                algorithm: key.metadata.algorithm.clone(),
                key_size: key.metadata.key_size,
                created_at: key.created_at,
                usage: key.metadata.usage.clone(),
                extractable: key.metadata.extractable,
            })
        } else {
            Err(beardog_errors::BearDogError::not_found(format_args!("Key {} not found", key_id).to_string()))
        }
    }

    async fn sign_data(&self, _key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM signing failed".to_string()));
        }

        let mut signature = b"mock_signature_".to_vec();
        signature.extend_from_slice(&data[..std::cmp::min(32, data.len())]);
        Ok(signature)
    }

    async fn verify_signature(&self, _key_id: &str, _data: &[u8], _signature: &[u8]) -> BearDogResult<bool> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM verification failed".to_string()));
        }

        Ok(true)
    }

    async fn encrypt_with_key(&self, _key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM encryption failed".to_string()));
        }

        let mut encrypted = b"MOCK_ENCRYPTED:".to_vec();
        encrypted.extend_from_slice(data);
        Ok(encrypted)
    }

    async fn decrypt_with_key(&self, _key_id: &str, encrypted_data: &[u8]) -> BearDogResult<Vec<u8>> {
        if !self.should_succeed() {
            return Err(beardog_errors::BearDogError::hsm_error("Mock HSM decryption failed".to_string()));
        }

        if encrypted_data.starts_with(b"MOCK_ENCRYPTED:") {
            Ok(encrypted_data[15..].to_vec())
        } else {
            Err(beardog_errors::BearDogError::hsm_error("Invalid encrypted data format".to_string()))
        }
    }

    async fn get_capabilities(&self) -> BearDogResult<beardog_types::canonical::hsm::HsmCapabilities> {
        Ok(beardog_types::canonical::hsm::HsmCapabilities {
            supported_key_types: vec![KeyType::EcdsaP256, KeyType::Rsa2048],
            max_keys: 1000,
            hardware_backed: false, // Mock provider is software-based
            fips_140_2_level: None,
            common_criteria_level: None,
            supports_key_derivation: true,
            supports_bulk_operations: false,
        })
    }

    async fn get_hardware_status(&self) -> BearDogResult<beardog_types::canonical::hsm::HsmHardwareStatus> {
        Ok(beardog_types::canonical::hsm::HsmHardwareStatus {
            is_available: true,
            temperature_celsius: None,
            uptime_seconds: Some(3600), // 1 hour uptime
            firmware_version: Some("Mock-1.0.0".to_string()),
            hardware_version: Some("Mock-Hardware-v1".to_string()),
            total_memory_bytes: Some(1024 * 1024), // 1MB
            available_memory_bytes: Some(512 * 1024), // 512KB
            authentication_attempts: 0,
            max_authentication_attempts: 3,
            is_authenticated: true,
        })
    }
}

pub async fn run_all_examples() -> BearDogResult<()> {
    info!("🚀 Running all Universal HSM Architecture examples");

    example_universal_hsm_setup().await?;
    println!();

    example_vendor_agnostic_operations().await?;
    println!();

    example_ecosystem_provider_registration().await?;
    println!();

    example_failover_and_high_availability().await?;
    println!();

    info!("🎉 All Universal HSM Architecture examples completed successfully!");
    Ok(())
} 