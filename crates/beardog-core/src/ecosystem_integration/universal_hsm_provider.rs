

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::crypto::{HsmKey, HsmKeyInfo, KeyMetadata, KeyType};
use beardog_types::canonical::hsm::{HsmCapabilities, HsmHardwareStatus, HsmProvider};
use beardog_traits::canonical::{HsmProvider, CanonicalWorkflowEngine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHsmProvider {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub capabilities: HsmCapabilities,
    pub endpoint: String,
    pub priority: u32,
    pub health_status: ProviderHealthStatus,
    pub ecosystem_node: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderHealthStatus {
    Optimal,
    Healthy,
    Degraded,
    Unhealthy,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderSelectionStrategy {
    Priority,        // Select by priority
    LoadBalanced,    // Round-robin with health awareness
    Capability,      // Select best capability match
    Latency,         // Select lowest latency
    Ecosystem,       // Prefer ecosystem providers
}

#[derive(Debug)]
pub struct HsmFailoverManager {
    failover_enabled: bool,
    max_retries: u32,
    retry_delay_ms: u64,
    circuit_breaker_threshold: u32,
}

impl Default for HsmFailoverManager {
    fn default() -> Self {
        Self {
            failover_enabled: true,
            max_retries: 3,
            retry_delay_ms: 1000,
            circuit_breaker_threshold: 5,
        }
    }
}

#[derive(Debug)]
pub struct IntelligentProviderSelector {
    strategy: ProviderSelectionStrategy,
    health_check_interval_secs: u64,
    performance_metrics: HashMap<String, ProviderMetrics>,
}

#[derive(Debug, Clone)]
pub struct ProviderMetrics {
    pub avg_response_time_ms: f64,
    pub success_rate: f64,
    pub total_requests: u64,
    pub failed_requests: u64,
}

impl Default for IntelligentProviderSelector {
    fn default() -> Self {
        Self {
            strategy: ProviderSelectionStrategy::Priority,
            health_check_interval_secs: 30,
            performance_metrics: ahash::HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct UniversalHsmProvider<D: EcosystemServiceDiscovery = DefaultServiceDiscovery> {

    active_providers: Arc<RwLock<HashMap<String, Box<dyn HsmProvider + Send + Sync>>>>,

    ecosystem_providers: Arc<RwLock<HashMap<String, EcosystemHsmProvider>>>,

    provider_selector: IntelligentProviderSelector,

    failover_manager: HsmFailoverManager,

    service_discovery: Option<D>,
}

#[derive(Debug, Default)]
pub struct DefaultServiceDiscovery;

#[allow(async_fn_in_trait)]
impl EcosystemServiceDiscovery for DefaultServiceDiscovery {
    async fn discover_hsm_providers(&self) -> BearDogResult<Vec<EcosystemHsmProvider>> {
        Ok(Vec::new())
    }
    
    async fn register_hsm_provider(&self, _provider_info: &EcosystemHsmProvider) -> BearDogResult<()> {
        Ok(())
    }
    
    async fn health_check(&self) -> BearDogResult<bool> {
        Ok(true)
    }
}

#[allow(async_fn_in_trait)]
pub trait EcosystemServiceDiscovery {

    async fn discover_hsm_providers(&self) -> BearDogResult<Vec<EcosystemHsmProvider>>;

    async fn register_hsm_provider(&self, provider_info: &EcosystemHsmProvider) -> BearDogResult<()>;

    async fn health_check(&self) -> BearDogResult<bool>;
}

impl<D: EcosystemServiceDiscovery> UniversalHsmProvider<D> {

    pub fn new_with_discovery(service_discovery: D) -> Self {
        Self {
            active_providers: Arc::new(RwLock::new(ahash::HashMap::default())),
            ecosystem_providers: Arc::new(RwLock::new(ahash::HashMap::default())),
            provider_selector: IntelligentProviderSelector::default(),
            failover_manager: HsmFailoverManager::default(),
            service_discovery: Some(service_discovery),
        }
    }
}

impl UniversalHsmProvider<DefaultServiceDiscovery> {

    pub fn new() -> Self {
        Self {
            active_providers: Arc::new(RwLock::new(ahash::HashMap::default())),
            ecosystem_providers: Arc::new(RwLock::new(ahash::HashMap::default())),
            provider_selector: IntelligentProviderSelector::default(),
            failover_manager: HsmFailoverManager::default(),
            service_discovery: Some(DefaultServiceDiscovery),
        }
    }
}

    pub async fn register_provider(
        &self,
        provider_id: &str,
        provider: Box<dyn HsmProvider + Send + Sync>,
    ) -> BearDogResult<()> {
        let mut providers = self.active_providers.write().await;
        providers.insert(provider_id.clone(), provider);
        info!("🔐 Registered HSM provider: {}", provider_id);
        Ok(())
    }

    pub async fn discover_ecosystem_providers(&self) -> BearDogResult<Vec<EcosystemHsmProvider>> {
        if let Some(discovery) = &self.service_discovery {
            match discovery.discover_hsm_providers().await {
                Ok(providers) => {
                    let mut ecosystem_providers = self.ecosystem_providers.write().await;
                    for provider in &providers {
                        ecosystem_providers.insert(provider.id.clone(), provider.clone());
                        info!("🌐 Discovered ecosystem HSM provider: {} ({})", provider.name, provider.vendor);
                    }
                    Ok(providers)
                }
                Err(e) => {
                    warn!("Failed to discover ecosystem HSM providers: {}", e);
                    Ok(vec![])
                }
            }
        } else {
            debug!("No service discovery configured, skipping ecosystem provider discovery");
            Ok(vec![])
        }
    }

    async fn select_provider(&self, operation: &str, key_type: Option<KeyType>) -> BearDogResult<String> {
        let providers = self.active_providers.read().await;
        let ecosystem_providers = self.ecosystem_providers.read().await;

        if providers.is_empty() && ecosystem_providers.is_empty() {
            return Err(BearDogError::configuration(
                "No HSM providers available".to_string(),
            ));
        }

        match self.provider_selector.strategy {
            ProviderSelectionStrategy::Priority => {
                self.select_by_priority(&providers, &ecosystem_providers)
            }
            ProviderSelectionStrategy::LoadBalanced => {
                self.select_by_load_balance(&providers, &ecosystem_providers)
            }
            ProviderSelectionStrategy::Capability => {
                self.select_by_capability(&providers, &ecosystem_providers)
            }
            ProviderSelectionStrategy::Latency => {
                self.select_by_latency(&providers, &ecosystem_providers)
            }
            ProviderSelectionStrategy::Ecosystem => {
                self.select_by_ecosystem(&providers, &ecosystem_providers)
            }
        }.or_else(|_| {
            Err(BearDogError::configuration(
                "No suitable HSM provider found".to_string(),
            ))
        })
    }

    async fn execute_with_failover<F, T>(&self, operation: F) -> BearDogResult<T>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<T>> + Send>> + Send + Sync,
        T: Send,
    {
        let mut attempts = 0;
        let max_attempts = if self.failover_manager.failover_enabled {
            self.failover_manager.max_retries + 1
        } else {
            1
        };

        while attempts < max_attempts {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(e);
                    }
                    warn!("HSM operation failed (attempt {}/{}): {}", attempts, max_attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        self.failover_manager.retry_delay_ms,
                    )).await;
                }
            }
        }

        Err(BearDogError::system_error("Max retry attempts exceeded".to_string()))
    }

    pub async fn get_ecosystem_status(&self) -> BearDogResult<serde_json::Value> {
        let active_count = self.active_providers.read().await.len();
        let ecosystem_count = self.ecosystem_providers.read().await.len();
        
        let health_check_result = if let Some(discovery) = &self.service_discovery {
            discovery.health_check().await.unwrap_or(false)
        } else {
            false
        };

        Ok(serde_json::json!({
            "universal_hsm_status": "operational",
            "local_providers": active_count,
            "ecosystem_providers": ecosystem_count,
            "total_providers": active_count + ecosystem_count,
            "ecosystem_connectivity": health_check_result,
            "failover_enabled": self.failover_manager.failover_enabled,
            "provider_selection_strategy": format_args!("{:?}", self.provider_selector.strategy).to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

impl Default for UniversalHsmProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmProvider for UniversalHsmProvider {
    async fn generate_key(&self, key_type: KeyType, metadata: KeyMetadata) -> BearDogResult<HsmKey> {
        let provider_id = self.select_provider("generate_key", Some(key_type)).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_type = key_type;
            let metadata = metadata.clone();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.generate_key(key_type, metadata).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn import_key(&self, key_data: &[u8], key_type: KeyType, metadata: KeyMetadata) -> BearDogResult<HsmKey> {
        let provider_id = self.select_provider("import_key", Some(key_type)).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_data = key_data.to_vec();
            let key_type = key_type;
            let metadata = metadata.clone();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.import_key(&key_data, key_type, metadata).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn derive_key(&self, parent_key_id: &str, derivation_path: &str, derived_key_type: KeyType) -> BearDogResult<HsmKey> {
        let provider_id = self.select_provider("derive_key", Some(derived_key_type)).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let parent_key_id = parent_key_id.to_string();
            let derivation_path = derivation_path.to_string();
            let derived_key_type = derived_key_type;
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.derive_key(&parent_key_id, &derivation_path, derived_key_type).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        let provider_id = self.select_provider("delete_key", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_id = key_id.to_string();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.delete_key(&key_id).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>> {
        let provider_id = self.select_provider("list_keys", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.list_keys().await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn get_key_info(&self, key_id: &str) -> BearDogResult<HsmKeyInfo> {
        let provider_id = self.select_provider("get_key_info", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_id = key_id.to_string();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.get_key_info(&key_id).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        let provider_id = self.select_provider("sign_data", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_id = key_id.to_string();
            let data = data.to_vec();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.sign_data(&key_id, &data).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn verify_signature(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        let provider_id = self.select_provider("verify_signature", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_id = key_id.to_string();
            let data = data.to_vec();
            let signature = signature.to_vec();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.verify_signature(&key_id, &data, &signature).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn encrypt_with_key(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        let provider_id = self.select_provider("encrypt_with_key", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_id = key_id.to_string();
            let data = data.to_vec();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.encrypt_with_key(&key_id, &data).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn decrypt_with_key(&self, key_id: &str, encrypted_data: &[u8]) -> BearDogResult<Vec<u8>> {
        let provider_id = self.select_provider("decrypt_with_key", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            let key_id = key_id.to_string();
            let encrypted_data = encrypted_data.to_vec();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.decrypt_with_key(&key_id, &encrypted_data).await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn get_capabilities(&self) -> BearDogResult<HsmCapabilities> {
        let provider_id = self.select_provider("get_capabilities", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.get_capabilities().await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    async fn get_hardware_status(&self) -> BearDogResult<HsmHardwareStatus> {
        let provider_id = self.select_provider("get_hardware_status", None).await?;
        
        self.execute_with_failover(|| {
            let providers = self.active_providers.clone();
            let provider_id = provider_id.clone();
            
            Box::pin(async move {
                let providers = providers.read().await;
                if let Some(provider) = providers.get(&provider_id) {
                    provider.get_hardware_status().await
                } else {
                    Err(BearDogError::configuration(format!(
                        "HSM provider {} not found", provider_id
                    )))
                }
            })
        }).await
    }

    fn select_by_priority(
        &self,
        providers: &std::collections::HashMap<&str, Arc<dyn beardog_traits::HsmProvider>>,
        ecosystem_providers: &std::collections::HashMap<&str, EcosystemHsmProvider>
    ) -> BearDogResult<String> {

        for (id, _) in providers.iter() {
            if id.contains("strongbox") || id.contains("secure_enclave") || id.contains("pkcs11") {
                return Ok(id.clone());
            }
        }

        if let Some(provider_id) = providers.keys().next() {
            Ok(provider_id.clone())
        } else if let Some(ecosystem_provider) = ecosystem_providers.values().next() {
            Ok(ecosystem_provider.id.clone())
        } else {
            Err(BearDogError::configuration("No HSM providers available".to_string()))
        }
    }

    fn select_by_performance(
        &self,
        providers: &std::collections::HashMap<&str, Arc<dyn beardog_traits::HsmProvider>>,
        ecosystem_providers: &std::collections::HashMap<&str, EcosystemHsmProvider>
    ) -> BearDogResult<String> {

        for (id, _) in providers.iter() {
            if id.contains("strongbox") || id.contains("secure_enclave") {
                return Ok(id.clone());
            }
        }

        if let Some(provider_id) = providers.keys().next() {
            Ok(provider_id.clone())
        } else if let Some(ecosystem_provider) = ecosystem_providers.values().next() {
            Ok(ecosystem_provider.id.clone())
        } else {
            Err(BearDogError::configuration("No HSM providers available".to_string()))
        }
    }

    fn select_by_capability(
        &self,
        providers: &std::collections::HashMap<&str, Arc<dyn beardog_traits::HsmProvider>>,
        ecosystem_providers: &std::collections::HashMap<&str, EcosystemHsmProvider>
    ) -> BearDogResult<String> {

        if let Some(provider_id) = providers.keys().next() {
            Ok(provider_id.clone())
        } else if let Some(ecosystem_provider) = ecosystem_providers.values().next() {
            Ok(ecosystem_provider.id.clone())
        } else {
            Err(BearDogError::configuration("No HSM providers available".to_string()))
        }
    }

    fn select_by_latency(
        &self,
        providers: &std::collections::HashMap<&str, Arc<dyn beardog_traits::HsmProvider>>,
        ecosystem_providers: &std::collections::HashMap<&str, EcosystemHsmProvider>
    ) -> BearDogResult<String> {

        if let Some(provider_id) = providers.keys().next() {
            Ok(provider_id.clone())
        } else if let Some(ecosystem_provider) = ecosystem_providers.values().next() {
            Ok(ecosystem_provider.id.clone())
        } else {
            Err(BearDogError::configuration("No HSM providers available".to_string()))
        }
    }

    fn select_by_ecosystem(
        &self,
        providers: &std::collections::HashMap<&str, Arc<dyn beardog_traits::HsmProvider>>,
        ecosystem_providers: &std::collections::HashMap<&str, EcosystemHsmProvider>
    ) -> BearDogResult<String> {

        for (id, _) in ecosystem_providers.iter() {
            return Ok(id.clone());
        }

        if let Some(provider_id) = providers.keys().next() {
            Ok(provider_id.clone())
        } else {
            Err(BearDogError::configuration("No HSM providers available".to_string()))
        }
    }

    fn select_by_load_balance(
        &self,
        providers: &std::collections::HashMap<&str, Arc<dyn beardog_traits::HsmProvider>>,
        ecosystem_providers: &std::collections::HashMap<&str, EcosystemHsmProvider>
    ) -> BearDogResult<String> {

        let total_providers = providers.len() + ecosystem_providers.len();
        if total_providers == 0 {
            return Err(BearDogError::configuration("No HSM providers available".to_string()));
        }
        
        let selection_index = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize) % total_providers;
            
        if selection_index < providers.len() {
            let provider_id = providers.keys().nth(selection_index)
                .ok_or_else(|| BearDogError::system("Invalid provider selection index".to_string()))?;
            Ok(provider_id.clone())
        } else {
            let eco_index = selection_index - providers.len();
            let ecosystem_provider = ecosystem_providers.values().nth(eco_index)
                .ok_or_else(|| BearDogError::system("Invalid ecosystem provider selection index".to_string()))?;
            Ok(ecosystem_provider.id.clone())
        }
    }
} 