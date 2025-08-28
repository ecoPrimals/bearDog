#![allow(async_fn_in_trait)]

use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::{HsmKey, KeyMetadata, KeyType};
use beardog_types::canonical::workflow::WorkflowStatus;
use beardog_types::providers::{CryptoKeyPair, HashAlgorithm, KeyPairAlgorithm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Metrics collected from providers, mapping metric names to their values
pub type ProviderMetrics = HashMap<String, f64>;

/// Service health information for external services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub is_healthy: bool,
    pub response_time_ms: f64,
    pub error_message: Option<String>,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

use beardog_types::providers::{
    AuthenticationCredentials, AuthenticationResult, AuthorizationResult, CacheStats, ClientInfo,
    HsmKeyInfo, SecureSession,
};

use beardog_types::canonical::hsm::status::{HsmHealthStatus, HsmStatus, HsmTier};
use beardog_types::canonical::{HealthStatus, HsmCapabilities};
use beardog_types::providers::{ProviderConfig, ProviderStatus};
use beardog_types::SecurityAuditEvent;

type SecurityRequirements = std::collections::HashMap<String, String>;

/// Base trait that all BearDog providers must implement
///
/// This trait defines the fundamental operations that every provider
/// in the BearDog ecosystem must support, including health monitoring,
/// configuration management, and lifecycle operations.
pub trait BaseProvider: Send + Sync {
    /// Returns information about this provider
    fn provider_info(&self) -> ProviderInfo;

    /// Performs a health check on the provider
    async fn health_check(&self) -> Result<HealthStatus, BearDogError>;

    /// Returns the capabilities supported by this provider
    async fn capabilities(&self) -> Result<Vec<String>, BearDogError>;

    /// Initializes the provider with the given configuration
    async fn initialize(&self, config: &ProviderConfig) -> Result<(), BearDogError>;

    /// Shuts down the provider and cleans up resources
    async fn shutdown(&self) -> Result<(), BearDogError>;

    /// Returns current metrics for this provider
    async fn metrics(&self) -> Result<ProviderMetrics, BearDogError>;

    /// Validates the given configuration for this provider
    async fn validate_config(&self, config: &ProviderConfig) -> Result<bool, BearDogError>;

    /// Returns the current status of this provider
    async fn status(&self) -> Result<ProviderStatus, BearDogError>;

    async fn reload_config(&self, config: &ProviderConfig) -> Result<(), BearDogError>;

    fn version(&self) -> &str;

    fn id(&self) -> &str;
}

/// Security provider trait for authentication and authorization operations
pub trait SecurityProvider: BaseProvider {
    /// Authenticates a user with the provided credentials
    async fn authenticate(
        &self,
        credentials: AuthenticationCredentials,
    ) -> Result<AuthenticationResult, BearDogError>;

    /// Creates a new secure session for the given user
    async fn create_session(
        user_id: &str,
        client_info: ClientInfo,
    ) -> Result<SecureSession, BearDogError>;

    /// Validates an existing session and returns session details if valid
    async fn validate_session(
        &self,
        session_id: &str,
    ) -> Result<Option<SecureSession>, BearDogError>;

    /// Revokes an existing session
    async fn revoke_session(&self, session_id: &str) -> Result<(), BearDogError>;

    async fn authorize(
        subject: &str,
        resource: &str,
        action: &str,
    ) -> Result<AuthorizationResult, BearDogError>;

    async fn check_permissions(permissions: &[&str]) -> Result<Vec<bool>, BearDogError>;

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError>;

    async fn audit_event(&self, event: SecurityAuditEvent) -> Result<(), BearDogError>;

    async fn get_metrics(&self) -> Result<HashMap<String, f64>, BearDogError>;
}

pub trait HsmProvider: BaseProvider {
    fn provider_id(&self) -> &str;

    async fn initialize(&mut self, config: Option<&ProviderConfig>) -> Result<(), BearDogError>;

    async fn shutdown(&mut self) -> Result<(), BearDogError>;

    async fn get_capabilities(&self) -> Result<HsmCapabilities, BearDogError>;

    async fn health_check(&self) -> Result<HealthStatus, BearDogError>;

    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError>;

    async fn generate_key_pair(
        &self,
        key_type: KeyType,
        alias: &str,
    ) -> Result<String, BearDogError>;

    async fn import_key(
        &self,
        key_data: &[u8],
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError>;

    async fn derive_key(
        &self,
        master_key_id: &str,
        derivation_data: &[u8],
        derived_key_type: KeyType,
    ) -> Result<HsmKey, BearDogError>;

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError>;

    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, BearDogError>;

    async fn get_key_info(&self, key_id: &str) -> Result<HsmKeyInfo, BearDogError>;

    async fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, BearDogError>;

    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn collect_human_entropy(&self, _entropy_bits: u32) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::hsm(
            "Human entropy collection not supported by this HSM provider",
        ))
    }

    async fn get_attestation(&self, _challenge: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::hsm(
            "Hardware attestation not supported by this HSM provider",
        ))
    }

    async fn get_status(&self) -> Result<HsmStatus, BearDogError> {
        let health_status = BaseProvider::health_check(self).await?;

        let hsm_health_status = match health_status {
            HealthStatus::Healthy => {
                beardog_types::canonical::hsm::status::HsmHealthStatus::Healthy
            }
            HealthStatus::Degraded => {
                beardog_types::canonical::hsm::status::HsmHealthStatus::Degraded
            }
            HealthStatus::Unhealthy => {
                beardog_types::canonical::hsm::status::HsmHealthStatus::Unhealthy
            }
            _ => beardog_types::canonical::hsm::status::HsmHealthStatus::Unknown,
        };

        let _hsm_health = beardog_types::canonical::hsm::status::HsmHealth {
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

        // TODO: Use hsm_health in the returned HsmStatus when types are aligned
        Ok(HsmStatus {
            status: beardog_types::canonical::hsm::status::HsmStatusType::Healthy,
            last_check: chrono::Utc::now(),
            metrics: beardog_types::canonical::hsm::status::HealthMetrics::default(),
            config: beardog_types::canonical::hsm::status::HsmHealthCheckConfig::default(),
        })
    }
}

pub trait WorkflowProcessor: Send + Sync {
    fn processor_name(&self) -> &str;

    async fn process_workflow(
        &self,
        workflow: &WorkflowStatus,
    ) -> Result<ProviderMetrics, BearDogError>;

    fn can_process(&self, workflow_type: &WorkflowStatus) -> bool;

    fn can_handle(&self, workflow: &WorkflowStatus) -> bool {
        self.can_process(workflow)
    }

    async fn get_capabilities(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![])
    }

    async fn initialize(&mut self, _config: Option<&ProviderConfig>) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus::Healthy)
    }

    async fn get_metrics(&self) -> Result<ProviderMetrics, BearDogError> {
        Ok(ProviderMetrics::new())
    }
}

pub trait DiscoveryBackend: Send + Sync {
    fn backend_name(&self) -> &str;

    async fn discover_by_capability(&self, capability: &str) -> Result<Vec<String>, BearDogError>;

    async fn discover_by_type(&self, service_type: &str) -> Result<Vec<String>, BearDogError>;

    async fn get_service(&self, service_id: &str) -> Result<Option<String>, BearDogError>;

    async fn register_service(
        &self,
        service_id: &str,
        service_info: &str,
    ) -> Result<(), BearDogError>;

    async fn unregister_service(&self, service_id: &str) -> Result<(), BearDogError>;

    async fn can_discover(&self, capability: &str) -> Result<bool, BearDogError> {
        let services = self.discover_by_capability(capability).await?;
        Ok(!services.is_empty())
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus::Healthy)
    }

    async fn initialize(&mut self, _config: Option<&ProviderConfig>) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn get_metrics(&self) -> Result<ProviderMetrics, BearDogError> {
        Ok(ProviderMetrics::new())
    }
}

pub trait HsmCapabilityDetector: Send + Sync {
    async fn detect_capabilities(&self) -> Result<Vec<HsmCapabilities>, BearDogError>;

    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError>;

    async fn recommend_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> Result<HsmTier, BearDogError>;
}

pub trait HsmHealthMonitor: Send + Sync {
    fn start_monitoring(
        &self,
        providers: Vec<&str>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    async fn get_health_status(&self) -> Result<HashMap<String, HsmHealthStatus>, BearDogError>;

    async fn filter_healthy_providers(
        &self,
        providers: Vec<&str>,
    ) -> Result<Vec<String>, BearDogError>;
}

pub trait HsmFailoverManager: Send + Sync {
    async fn handle_provider_failure(
        &self,
        provider_id: &str,
        error: &BearDogError,
    ) -> Result<(), BearDogError>;

    async fn get_failover_provider(&self, failed_provider_id: &str)
        -> Result<String, BearDogError>;

    async fn perform_with_failover<T>(&self, provider_id: &str) -> Result<T, BearDogError>
    where
        T: Send + 'static;
}

pub trait CacheProvider: BaseProvider {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, BearDogError>
    where
        T: for<'de> serde::Deserialize<'de> + Send;

    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), BearDogError>
    where
        T: serde::Serialize + Send + Sync;

    async fn remove(&self, key: &str) -> Result<bool, BearDogError>;

    async fn exists(&self, key: &str) -> Result<bool, BearDogError>;

    async fn clear(&self) -> Result<(), BearDogError>;

    async fn get_many(&self, keys: &[&str]) -> Result<HashMap<String, String>, BearDogError>;

    async fn set_many(
        &self,
        entries: HashMap<&str, &str>,
        ttl: Option<Duration>,
    ) -> Result<(), BearDogError>;

    async fn remove_many(&self, keys: &[&str]) -> Result<u64, BearDogError>;

    async fn get_stats(&self) -> Result<CacheStats, BearDogError>;

    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool, BearDogError>;

    async fn get_ttl(&self, key: &str) -> Result<Option<Duration>, BearDogError>;
}

pub trait CryptoProvider: BaseProvider {
    async fn generate_random(&self, length: usize) -> Result<Vec<u8>, BearDogError>;

    async fn generate_key_pair(
        &self,
        algorithm: KeyPairAlgorithm,
    ) -> Result<CryptoKeyPair, BearDogError>;

    async fn hash_data(
        &self,
        data: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<Vec<u8>, BearDogError>;

    async fn derive_key_pbkdf2(
        &self,
        password: &str,
        salt: &[u8],
        iterations: u32,
    ) -> Result<Vec<u8>, BearDogError>;

    async fn encrypt_symmetric(
        &self,
        key: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError>;

    async fn decrypt_symmetric(
        &self,
        key: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError>;

    async fn sign_with_key(&self, private_key: &[u8], data: &[u8])
        -> Result<Vec<u8>, BearDogError>;

    async fn verify_with_key(
        &self,
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    async fn encrypt_asymmetric(
        &self,
        public_key: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError>;

    async fn decrypt_asymmetric(
        &self,
        private_key: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError>;
}

pub trait WorkflowProvider: BaseProvider {
    async fn execute_workflow(
        &self,
        workflow_id: &str,
        input: HashMap<&str, &str>,
    ) -> Result<String, BearDogError>;

    async fn get_workflow_status(&self, execution_id: &str)
        -> Result<WorkflowStatus, BearDogError>;

    async fn cancel_workflow(&self, execution_id: &str) -> Result<(), BearDogError>;

    async fn create_workflow(&self, definition: &str) -> Result<String, BearDogError>;

    async fn update_workflow(
        &self,
        workflow_id: &str,
        definition: &str,
    ) -> Result<(), BearDogError>;

    async fn delete_workflow(&self, workflow_id: &str) -> Result<(), BearDogError>;

    async fn list_workflows(&self) -> Result<Vec<String>, BearDogError>;

    async fn get_workflow_metrics(
        &self,
        workflow_id: &str,
    ) -> Result<HashMap<String, f64>, BearDogError>;

    async fn get_execution_history(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<String>, BearDogError>;
}

pub trait GeneticsProvider: BaseProvider {
    async fn spawn_organism(
        &self,
        dna: Vec<u8>,
        environment: HashMap<&str, &str>,
    ) -> Result<String, BearDogError>;

    async fn evolve_population(
        &self,
        population: &[&str],
        generations: u32,
    ) -> Result<Vec<String>, BearDogError>;

    async fn crossover(&self, parent1: &str, parent2: &str) -> Result<String, BearDogError>;

    async fn mutate(&self, organism: &str, mutation_rate: f64) -> Result<String, BearDogError>;

    async fn analyze_fitness(
        &self,
        organism: &str,
        environment: &HashMap<&str, &str>,
    ) -> Result<f64, BearDogError>;

    async fn get_genetic_diversity(&self, population: &[&str]) -> Result<f64, BearDogError>;

    async fn get_population_metrics(&self) -> Result<HashMap<String, f64>, BearDogError>;
}

pub trait MonitoringProvider: BaseProvider {
    async fn record_metric(
        &self,
        name: &str,
        value: f64,
        tags: HashMap<&str, &str>,
    ) -> Result<(), BearDogError>;

    async fn record_counter(&self, name: &str, value: u64) -> Result<(), BearDogError>;

    async fn record_gauge(&self, name: &str, value: f64) -> Result<(), BearDogError>;

    async fn record_histogram(&self, name: &str, value: f64) -> Result<(), BearDogError>;

    async fn query_metrics(
        &self,
        query: &str,
    ) -> Result<Vec<HashMap<String, String>>, BearDogError>;

    async fn get_metric_names(&self) -> Result<Vec<String>, BearDogError>;

    async fn get_metric_tags(&self, metric_name: &str) -> Result<Vec<String>, BearDogError>;

    async fn create_alert(&self, alert: HashMap<&str, &str>) -> Result<String, BearDogError>;

    async fn update_alert(
        &self,
        alert_id: &str,
        alert: HashMap<&str, &str>,
    ) -> Result<(), BearDogError>;

    async fn delete_alert(&self, alert_id: &str) -> Result<(), BearDogError>;

    async fn get_active_alerts(&self) -> Result<Vec<HashMap<String, String>>, BearDogError>;
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
    async fn discover_capabilities(&self) -> Result<Vec<String>, BearDogError>;

    /// Execute operation with universal parameters
    async fn execute_operation(
        &self,
        operation: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, BearDogError>;

    /// Get connection status and health
    async fn connection_status(&self) -> Result<ConnectionStatus, BearDogError>;

    /// Validate compatibility with target system
    async fn validate_compatibility(&self, target_version: &str) -> Result<bool, BearDogError>;
}

/// Consolidated platform-specific provider trait
/// Replaces: PlatformProvider, SafeIOSProvider, GamingSecurityProvider
pub trait PlatformProvider: BaseProvider {
    /// Platform identifier (e.g., "ios", "android", "gaming", "desktop")
    fn platform_type(&self) -> PlatformType;

    /// Get platform-specific security features
    async fn get_security_features(&self) -> Result<Vec<SecurityFeature>, BearDogError>;

    /// Execute platform-specific secure operation
    async fn secure_execute(
        &self,
        operation: SecureOperation,
        context: SecurityContext,
    ) -> Result<SecureResult, BearDogError>;

    /// Get platform attestation if available
    async fn get_platform_attestation(&self) -> Result<Option<AttestationData>, BearDogError>;

    /// Check if platform supports specific security level
    async fn supports_security_level(&self, level: SecurityLevel) -> Result<bool, BearDogError>;
}

/// Enhanced cache provider that consolidates caching interfaces
/// Replaces: EnhancedCacheProvider and extends CacheProvider
pub trait EnhancedCacheProvider: CacheProvider {
    /// Batch operations for improved performance
    async fn batch_get(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, serde_json::Value>, BearDogError>;

    async fn batch_set(
        &self,
        entries: HashMap<&str, serde_json::Value>,
        ttl: Option<Duration>,
    ) -> Result<(), BearDogError>;

    /// Cache invalidation patterns
    async fn invalidate_pattern(&self, pattern: &str) -> Result<u64, BearDogError>;

    /// Cache warming and preloading
    async fn warm_cache(&self, keys: &[&str]) -> Result<(), BearDogError>;

    /// Advanced cache statistics
    async fn get_advanced_stats(&self) -> Result<AdvancedCacheStats, BearDogError>;
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
