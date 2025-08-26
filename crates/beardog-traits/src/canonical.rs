

#![allow(async_fn_in_trait)]

use beardog_errors::BearDogResult;
use beardog_types::providers::{CryptoKeyPair, HashAlgorithm, KeyPairAlgorithm};
use beardog_types::canonical::hsm::{HsmKey, KeyMetadata, KeyType};
use beardog_types::canonical::workflow::{WorkflowStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub type ProviderMetrics = HashMap<String, f64>;

use beardog_types::providers::{
    AuthenticationCredentials, AuthenticationResult, AuthorizationResult, CacheStats, ClientInfo,
    HsmKeyInfo, SecureSession,
};

use beardog_types::canonical::{HealthStatus, HsmCapabilities};
use beardog_types::providers::{ProviderConfig, ProviderStatus};
use beardog_types::SecurityAuditEvent;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::status::{HsmTier, HsmHealthStatus, HsmStatus};

type SecurityRequirements = std::collections::HashMap<String, String>;

pub trait BaseProvider: Send + Sync {

    fn provider_info(&self) -> ProviderInfo;

    async fn health_check(&self) -> BearDogResult<HealthStatus>;

    async fn capabilities(&self) -> BearDogResult<Vec<String>>;

    async fn initialize(&self, config: &ProviderConfig) -> BearDogResult<()>;

    async fn shutdown(&self) -> BearDogResult<()>;

    async fn metrics(&self) -> BearDogResult<ProviderMetrics>;

    async fn validate_config(&self, config: &ProviderConfig) -> BearDogResult<bool>;

    async fn status(&self) -> BearDogResult<ProviderStatus>;

    async fn reload_config(&self, config: &ProviderConfig) -> BearDogResult<()>;

    fn version(&self) -> &str;

    fn id(&self) -> &str;
}

pub trait SecurityProvider: BaseProvider {

    async fn authenticate(
        &self,
        credentials: AuthenticationCredentials,
    ) -> BearDogResult<AuthenticationResult>;

    async fn create_session(
        user_id: &str,
        client_info: ClientInfo,
    ) -> BearDogResult<SecureSession>;

    async fn validate_session(&self, session_id: &str) -> BearDogResult<Option<SecureSession>>;

    async fn revoke_session(&self, session_id: &str) -> BearDogResult<()>;

    async fn authorize(
        subject: &str,
        resource: &str,
        action: &str,
    ) -> BearDogResult<AuthorizationResult>;

    async fn check_permissions(
        permissions: &[&str],
    ) -> BearDogResult<Vec<bool>>;

    async fn encrypt(&self, data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn decrypt(&self, encrypted_data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn sign(&self, data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn verify(&self, data: &[u8], signature: &[u8]) -> BearDogResult<bool>;

    async fn audit_event(&self, event: SecurityAuditEvent) -> BearDogResult<()>;

    async fn get_metrics(&self) -> BearDogResult<HashMap<String, f64>>;
}

pub trait HsmProvider: BaseProvider {

    fn provider_id(&self) -> &str;

    async fn initialize(&mut self, config: Option<&ProviderConfig>) -> BearDogResult<()>;

    async fn shutdown(&mut self) -> BearDogResult<()>;

    async fn get_capabilities(&self) -> BearDogResult<HsmCapabilities>;

    async fn health_check(&self) -> BearDogResult<HealthStatus>;

    async fn generate_key(&self, key_type: KeyType, metadata: KeyMetadata) -> BearDogResult<HsmKey>;

    async fn generate_key_pair(&self, key_type: KeyType, alias: &str) -> BearDogResult<String>;

    async fn import_key(&self, key_data: &[u8], key_type: KeyType, metadata: KeyMetadata) -> BearDogResult<HsmKey>;

    async fn derive_key(&self, master_key_id: &str, derivation_data: &[u8], derived_key_type: KeyType) -> BearDogResult<HsmKey>;

    async fn delete_key(&self, key_id: &str) -> BearDogResult<()>;

    async fn list_keys(&self) -> BearDogResult<Vec<KeyMetadata>>;

    async fn get_key_info(&self, key_id: &str) -> BearDogResult<HsmKeyInfo>;

    async fn get_key_metadata(&self, key_id: &str) -> BearDogResult<KeyMetadata>;

    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool>;

    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn collect_human_entropy(&self, _entropy_bits: u32) -> BearDogResult<Vec<u8>> {

        Err(BearDogError::hsm("Human entropy collection not supported by this HSM provider"))
    }

    async fn get_attestation(&self, _challenge: &[u8]) -> BearDogResult<Vec<u8>> {

        Err(BearDogError::hsm("Hardware attestation not supported by this HSM provider"))
    }

    async fn get_status(&self) -> BearDogResult<HsmStatus> {

        let health_status = BaseProvider::health_check(self).await?;

        let hsm_health_status = match health_status {
            HealthStatus::Healthy => beardog_types::canonical::hsm::status::HsmHealthStatus::Healthy,
            HealthStatus::Degraded => beardog_types::canonical::hsm::status::HsmHealthStatus::Degraded,
            HealthStatus::Unhealthy => beardog_types::canonical::hsm::status::HsmHealthStatus::Unhealthy,
            _ => beardog_types::canonical::hsm::status::HsmHealthStatus::Unknown,
        };
        
        let hsm_health = beardog_types::canonical::hsm::status::HsmHealth {
            status: hsm_health_status,
            last_check: chrono::Utc::now(),
            details: std::collections::HashMap::with_capacity(16),
            performance: beardog_types::canonical::hsm::status::HealthMetrics {
                ops_per_second: 0.0,
                avg_response_time_ms: 0.0,
                error_rate_percent: 0.0,
                memory_usage_percent: 0.0,
                cpu_usage_percent: 0.0,
                active_connections: 0,
            },
            errors: Vec::new(),
        };
        
        Ok(HsmStatus {
            hsm_id: self.provider_id().to_string(),
            health: hsm_health,
            connected: true,
            last_updated: chrono::Utc::now(),
            metadata: std::collections::HashMap::with_capacity(16),
        })
    }
}

pub trait WorkflowProcessor: Send + Sync {

    fn processor_name(&self) -> &str;

    async fn process_workflow(
        &self,
        workflow: &WorkflowStatus,
    ) -> BearDogResult<ProviderMetrics>;

    fn can_process(&self, workflow_type: &WorkflowStatus) -> bool;

    fn can_handle(&self, workflow: &WorkflowStatus) -> bool {
        self.can_process(workflow)
    }

    async fn get_capabilities(&self) -> BearDogResult<Vec<String>> {

        Ok(vec![])
    }

    async fn initialize(&mut self, _config: Option<&ProviderConfig>) -> BearDogResult<()> {

        Ok(())
    }

    async fn shutdown(&mut self) -> BearDogResult<()> {

        Ok(())
    }

    async fn health_check(&self) -> BearDogResult<HealthStatus> {

        Ok(HealthStatus::Healthy)
    }

    async fn get_metrics(&self) -> BearDogResult<ProviderMetrics> {

        Ok(ProviderMetrics::new())
    }
}

pub trait DiscoveryBackend: Send + Sync {

    fn backend_name(&self) -> &str;

    async fn discover_by_capability(&self, capability: &str) -> BearDogResult<Vec<String>>;

    async fn discover_by_type(&self, service_type: &str) -> BearDogResult<Vec<String>>;

    async fn get_service(&self, service_id: &str) -> BearDogResult<Option<String>>;

    async fn register_service(&self, service_id: &str, service_info: &str) -> BearDogResult<()>;

    async fn unregister_service(&self, service_id: &str) -> BearDogResult<()>;

    async fn can_discover(&self, capability: &str) -> BearDogResult<bool> {

        let services = self.discover_by_capability(capability).await?;
        Ok(!services.is_empty())
    }

    async fn health_check(&self) -> BearDogResult<HealthStatus> {

        Ok(HealthStatus::Healthy)
    }

    async fn initialize(&mut self, _config: Option<&ProviderConfig>) -> BearDogResult<()> {

        Ok(())
    }

    async fn shutdown(&mut self) -> BearDogResult<()> {

        Ok(())
    }

    async fn get_metrics(&self) -> BearDogResult<ProviderMetrics> {

        Ok(ProviderMetrics::new())
    }
}

pub trait HsmCapabilityDetector: Send + Sync {

    async fn detect_capabilities(&self) -> BearDogResult<Vec<HsmCapabilities>>;

    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> BearDogResult<bool>;

    async fn recommend_hsm_tier(&self, requirements: &SecurityRequirements) -> BearDogResult<HsmTier>;
}

pub trait HsmHealthMonitor: Send + Sync {

    fn start_monitoring(&self, providers: Vec<&str>) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    async fn get_health_status(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>>;

    async fn filter_healthy_providers(&self, providers: Vec<&str>) -> BearDogResult<Vec<String>>;
}

pub trait HsmFailoverManager: Send + Sync {

    async fn handle_provider_failure(&self, provider_id: &str, error: &BearDogError) -> BearDogResult<()>;

    async fn get_failover_provider(&self, failed_provider_id: &str) -> BearDogResult<String>;

    async fn perform_with_failover<T>(&self, provider_id: &str) -> BearDogResult<T>
    where
        T: Send + 'static;
}

pub trait CacheProvider: BaseProvider {

    async fn get<T>(&self, key: &str) -> BearDogResult<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de> + Send;

    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> BearDogResult<()>
    where
        T: serde::Serialize + Send + Sync;

    async fn remove(&self, key: &str) -> BearDogResult<bool>;

    async fn exists(&self, key: &str) -> BearDogResult<bool>;

    async fn clear(&self) -> BearDogResult<()>;

    async fn get_many(&self, keys: &[&str]) -> BearDogResult<HashMap<String, String>>;

    async fn set_many(
        &self,
        entries: HashMap<&str, &str>,
        ttl: Option<Duration>,
    ) -> BearDogResult<()>;

    async fn remove_many(&self, keys: &[&str]) -> BearDogResult<u64>;

    async fn get_stats(&self) -> BearDogResult<CacheStats>;

    async fn expire(&self, key: &str, ttl: Duration) -> BearDogResult<bool>;

    async fn get_ttl(&self, key: &str) -> BearDogResult<Option<Duration>>;
}

pub trait CryptoProvider: BaseProvider {

    async fn generate_random(&self, length: usize) -> BearDogResult<Vec<u8>>;

    async fn generate_key_pair(&self, algorithm: KeyPairAlgorithm) -> BearDogResult<CryptoKeyPair>;

    async fn hash_data(&self, data: &[u8], algorithm: HashAlgorithm) -> BearDogResult<Vec<u8>>;

    async fn derive_key_pbkdf2(
        &self,
        password: &str,
        salt: &[u8],
        iterations: u32,
    ) -> BearDogResult<Vec<u8>>;

    async fn encrypt_symmetric(&self, key: &[u8], plaintext: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn decrypt_symmetric(&self, key: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn sign_with_key(&self, private_key: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn verify_with_key(
        &self,
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool>;

    async fn encrypt_asymmetric(
        &self,
        public_key: &[u8],
        plaintext: &[u8],
    ) -> BearDogResult<Vec<u8>>;

    async fn decrypt_asymmetric(
        &self,
        private_key: &[u8],
        ciphertext: &[u8],
    ) -> BearDogResult<Vec<u8>>;
}

pub trait WorkflowProvider: BaseProvider {

    async fn execute_workflow(
        &self,
        workflow_id: &str,
        input: HashMap<&str, &str>,
    ) -> BearDogResult<String>;

    async fn get_workflow_status(&self, execution_id: &str) -> BearDogResult<WorkflowStatus>;

    async fn cancel_workflow(&self, execution_id: &str) -> BearDogResult<()>;

    async fn create_workflow(&self, definition: &str) -> BearDogResult<String>;

    async fn update_workflow(&self, workflow_id: &str, definition: &str) -> BearDogResult<()>;

    async fn delete_workflow(&self, workflow_id: &str) -> BearDogResult<()>;

    async fn list_workflows(&self) -> BearDogResult<Vec<String>>;

    async fn get_workflow_metrics(&self, workflow_id: &str) -> BearDogResult<HashMap<String, f64>>;

    async fn get_execution_history(
        &self,
        limit: Option<usize>,
    ) -> BearDogResult<Vec<String>>;
}

pub trait GeneticsProvider: BaseProvider {

    async fn spawn_organism(
        &self,
        dna: Vec<u8>,
        environment: HashMap<&str, &str>,
    ) -> BearDogResult<String>;

    async fn evolve_population(
        &self,
        population: &[&str],
        generations: u32,
    ) -> BearDogResult<Vec<String>>;

    async fn crossover(&self, parent1: &str, parent2: &str) -> BearDogResult<String>;

    async fn mutate(&self, organism: &str, mutation_rate: f64) -> BearDogResult<String>;

    async fn analyze_fitness(
        &self,
        organism: &str,
        environment: &HashMap<&str, &str>,
    ) -> BearDogResult<f64>;

    async fn get_genetic_diversity(&self, population: &[&str]) -> BearDogResult<f64>;

    async fn get_population_metrics(
        &self,
    ) -> BearDogResult<HashMap<String, f64>>;
}

pub trait MonitoringProvider: BaseProvider {

    async fn record_metric(
        &self,
        name: &str,
        value: f64,
        tags: HashMap<&str, &str>,
    ) -> BearDogResult<()>;

    async fn record_counter(
        &self,
        name: &str,
        value: u64,
    ) -> BearDogResult<()>;

    async fn record_gauge(
        &self,
        name: &str,
        value: f64,
    ) -> BearDogResult<()>;

    async fn record_histogram(
        &self,
        name: &str,
        value: f64,
    ) -> BearDogResult<()>;

    async fn query_metrics(&self, query: &str) -> BearDogResult<Vec<HashMap<String, String>>>;

    async fn get_metric_names(&self) -> BearDogResult<Vec<String>>;

    async fn get_metric_tags(&self, metric_name: &str) -> BearDogResult<Vec<String>>;

    async fn create_alert(&self, alert: HashMap<&str, &str>) -> BearDogResult<String>;

    async fn update_alert(
        &self,
        alert_id: &str,
        alert: HashMap<&str, &str>,
    ) -> BearDogResult<()>;

    async fn delete_alert(&self, alert_id: &str) -> BearDogResult<()>;

    async fn get_active_alerts(&self) -> BearDogResult<Vec<HashMap<String, String>>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {

    pub name: String,

    pub version: String,

    pub provider_type: String,

    pub capabilities: Vec<String>,
}

pub struct CanonicalTraitRegistry;

impl CanonicalTraitRegistry {

    pub fn trait_names() -> Vec<&'static str> {
        vec![
            "BaseProvider",
            "SecurityProvider",
            "HsmProvider",
            "CacheProvider",
            "CryptoProvider",
            "WorkflowProvider",
            "GeneticsProvider",
            "MonitoringProvider",
            // New consolidated traits
            "UniversalProvider",
            "PlatformProvider",
        ]
    }

    pub fn is_canonical_trait(name: &str) -> bool {
        Self::trait_names().contains(&name)
    }

    /// Returns deprecated trait names that should be migrated
    pub fn deprecated_traits() -> Vec<&'static str> {
        vec![
            "UniversalPrimalProvider",
            "ExternalSystemProvider", 
            "UniversalServiceProvider",
            "SafeHardwareProvider",
            "SafeIOSProvider",
            "GamingSecurityProvider",
            "HybridAISecurityProvider",
            "SimpleCacheProvider",
        ]
    }
}

/// Consolidated universal provider trait that unifies external system integration
/// Replaces: UniversalProvider, ExternalSystemProvider, UniversalServiceProvider
pub trait UniversalProvider: BaseProvider {
    /// Provider type identifier (e.g., "songbird", "nestgate", "toadstool")
    fn provider_type(&self) -> &str;
    
    /// Discover available capabilities
    async fn discover_capabilities(&self) -> BearDogResult<Vec<String>>;
    
    /// Execute operation with universal parameters
    async fn execute_operation(
        &self,
        operation: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> BearDogResult<serde_json::Value>;
    
    /// Get connection status and health
    async fn connection_status(&self) -> BearDogResult<ConnectionStatus>;
    
    /// Validate compatibility with target system
    async fn validate_compatibility(&self, target_version: &str) -> BearDogResult<bool>;
}

/// Consolidated platform-specific provider trait 
/// Replaces: PlatformProvider, SafeIOSProvider, GamingSecurityProvider
pub trait PlatformProvider: BaseProvider {
    /// Platform identifier (e.g., "ios", "android", "gaming", "desktop")
    fn platform_type(&self) -> PlatformType;
    
    /// Get platform-specific security features
    async fn get_security_features(&self) -> BearDogResult<Vec<SecurityFeature>>;
    
    /// Execute platform-specific secure operation
    async fn secure_execute(
        &self,
        operation: SecureOperation,
        context: SecurityContext,
    ) -> BearDogResult<SecureResult>;
    
    /// Get platform attestation if available
    async fn get_platform_attestation(&self) -> BearDogResult<Option<AttestationData>>;
    
    /// Check if platform supports specific security level
    async fn supports_security_level(&self, level: SecurityLevel) -> BearDogResult<bool>;
}

/// Enhanced cache provider that consolidates caching interfaces
/// Replaces: EnhancedCacheProvider and extends CacheProvider
pub trait EnhancedCacheProvider: CacheProvider {
    /// Batch operations for improved performance
    async fn batch_get(&self, keys: &[&str]) -> BearDogResult<HashMap<String, serde_json::Value>>;
    
    async fn batch_set(
        &self, 
        entries: HashMap<&str, serde_json::Value>,
        ttl: Option<Duration>
    ) -> BearDogResult<()>;
    
    /// Cache invalidation patterns
    async fn invalidate_pattern(&self, pattern: &str) -> BearDogResult<u64>;
    
    /// Cache warming and preloading
    async fn warm_cache(&self, keys: &[&str]) -> BearDogResult<()>;
    
    /// Advanced cache statistics
    async fn get_advanced_stats(&self) -> BearDogResult<AdvancedCacheStats>;
}

// Supporting types for consolidated traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformType {
    Ios,
    Android,
    Gaming,
    Desktop,
    Server,
    Embedded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFeature {
    pub name: String,
    pub capability: String,
    pub security_level: SecurityLevel,
    pub hardware_backed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureOperation {
    pub operation_type: String,
    pub data: Vec<u8>,
    pub security_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub permissions: Vec<String>,
    pub security_level: SecurityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureResult {
    pub data: Vec<u8>,
    pub attestation: Option<AttestationData>,
    pub security_metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationData {
    pub platform: String,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub certificate_chain: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub connected: bool,
    pub latency_ms: Option<u64>,
    pub last_successful_operation: Option<chrono::DateTime<chrono::Utc>>,
    pub error_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    Enhanced,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCacheStats {
    pub basic_stats: CacheStats,
    pub memory_usage_bytes: u64,
    pub eviction_count: u64,
    pub hit_rate_percent: f64,
    pub average_ttl_seconds: f64,
    pub hot_keys: Vec<String>,
}

/// Migration helper for deprecated traits
pub struct TraitMigrationHelper;

impl TraitMigrationHelper {
    /// Get migration path for deprecated trait
    pub fn get_migration_path(deprecated_trait: &str) -> Option<&'static str> {
        match deprecated_trait {
            "UniversalPrimalProvider" => Some("UniversalProvider"),
            "ExternalSystemProvider" => Some("UniversalProvider"),
            "UniversalServiceProvider" => Some("UniversalProvider"),
            "SafeHardwareProvider" => Some("PlatformProvider"),
            "SafeIOSProvider" => Some("PlatformProvider"),
            "GamingSecurityProvider" => Some("PlatformProvider"),
            "SimpleCacheProvider" => Some("EnhancedCacheProvider"),
            _ => None,
        }
    }
    
    /// Check if trait is deprecated
    pub fn is_deprecated(trait_name: &str) -> bool {
        CanonicalTraitRegistry::deprecated_traits().contains(&trait_name)
    }
}

