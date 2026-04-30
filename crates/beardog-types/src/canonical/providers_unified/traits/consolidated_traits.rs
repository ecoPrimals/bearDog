// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    clippy::wildcard_imports,
    reason = "consolidated types re-export all items from sibling module"
)]
#![allow(
    clippy::missing_errors_doc,
    reason = "trait error docs are on the concrete impls, not the trait definitions"
)]
#![allow(
    async_fn_in_trait,
    reason = "native async fn in traits; all impls are Send + Sync"
)]

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::consolidated_types::*;

/// **CONSOLIDATED PROVIDER TRAIT** - Foundation of all providers
///
/// This is the core trait that all providers in the `BearDog` ecosystem must implement.
/// It provides the fundamental interface for provider lifecycle, health monitoring,
/// capabilities discovery, and metrics collection.
///
/// **Modernization Note** (November 2025): Now using native async/await (no `async_trait`)
/// for 5-15% performance improvement and zero-cost abstractions.
pub trait ConsolidatedProvider: Send + Sync + 'static {
    /// Error type for this provider
    type Error: std::error::Error + Send + Sync + 'static;

    /// Configuration type for this provider
    type Config: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Provider-specific data type
    type Data: Send + Sync + Clone;

    /// Get provider identification information
    fn provider_info(&self) -> ProviderInfo;

    /// Get provider version string
    fn provider_version(&self) -> &str;

    /// Get list of capabilities this provider supports
    fn capabilities(&self) -> Vec<ProviderCapability>;

    /// Initialize the provider with configuration
    async fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error>;

    /// Perform health check and return current status
    async fn health_check(&self) -> Result<ProviderHealth, Self::Error>;

    /// Collect and return current performance metrics
    async fn metrics(&self) -> Result<ProviderMetrics, Self::Error>;

    /// Gracefully shutdown the provider
    async fn shutdown(&mut self) -> Result<(), Self::Error>;

    /// Get provider configuration (if available)
    fn get_config(&self) -> Option<&Self::Config> {
        None
    }

    /// Validate provider configuration
    fn validate_config(config: &Self::Config) -> Result<(), Self::Error>;

    /// Check if provider is ready to serve requests
    async fn is_ready(&self) -> bool {
        true
    }
}

/// **SECURITY PROVIDER** - Unified security operations
pub trait SecurityProvider: ConsolidatedProvider {
    /// Encrypt data using the provider's security capabilities
    fn encrypt(&self, data: &[u8], key: &str) -> Result<Vec<u8>, BearDogError>;

    /// Decrypt data using the provider's security capabilities  
    fn decrypt(&self, encrypted_data: &[u8], key: &str) -> Result<Vec<u8>, BearDogError>;

    /// Generate a secure key
    fn generate_key(&self, algorithm: &str, key_size: usize) -> Result<String, BearDogError>;

    /// Validate security credentials
    fn validate_credentials(
        &self,
        credentials: &HashMap<String, String>,
    ) -> Result<bool, BearDogError>;
}

/// **HSM PROVIDER** - Hardware Security Module operations
pub trait HsmProvider: SecurityProvider {
    /// Generate a key in the HSM
    fn hsm_generate_key(&self, algorithm: &str, key_size: usize) -> Result<String, BearDogError>;

    /// Sign data using HSM
    fn hsm_sign(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>, BearDogError>;

    /// Verify signature using HSM
    fn hsm_verify(&self, data: &[u8], signature: &[u8], key_id: &str)
    -> Result<bool, BearDogError>;

    /// Get HSM status
    fn hsm_status(&self) -> std::result::Result<HsmStatus, BearDogError>;
}

/// **MONITORING PROVIDER** - System monitoring and observability
pub trait MonitoringProvider: ConsolidatedProvider {
    /// Collect system metrics
    fn collect_metrics(&self) -> std::result::Result<SystemMetrics, BearDogError>;

    /// Send alert
    fn send_alert(&self, alert: &Alert) -> Result<(), BearDogError>;

    /// Get service health
    fn get_health(&self) -> std::result::Result<ServiceHealth, BearDogError>;

    /// Record event
    fn record_event(&self, event: &SystemEvent) -> Result<(), BearDogError>;
}

/// **CRYPTO PROVIDER** - Cryptographic operations
pub trait CryptoProvider: SecurityProvider {
    /// Hash data
    fn hash(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>, BearDogError>;

    /// Generate random bytes
    fn generate_random(&self, length: usize) -> Result<Vec<u8>, BearDogError>;

    /// Create digital signature
    fn sign(&self, data: &[u8], private_key: &str) -> Result<Vec<u8>, BearDogError>;

    /// Verify digital signature
    fn verify(&self, data: &[u8], signature: &[u8], public_key: &str)
    -> Result<bool, BearDogError>;
}

/// **STORAGE PROVIDER** - Data storage operations (replaces `DatabaseProvider`)
pub trait StorageProvider: ConsolidatedProvider {
    /// Store data
    fn store(&self, key: &str, data: &[u8]) -> Result<(), BearDogError>;

    /// Retrieve data
    fn retrieve(&self, key: &str) -> Result<Vec<u8>, BearDogError>;

    /// Delete data
    fn delete(&self, key: &str) -> Result<(), BearDogError>;

    /// List keys
    fn list_keys(&self, prefix: &str) -> Result<Vec<String>, BearDogError>;

    /// Execute query
    fn execute_query(&self, query: &str) -> std::result::Result<DatabaseResult, BearDogError>;
}

/// **NETWORK PROVIDER** - Network operations (replaces `CacheProvider`)
pub trait NetworkProvider: ConsolidatedProvider {
    /// Send network request
    fn send_request(
        &self,
        request: &HttpRequest,
    ) -> std::result::Result<HttpResponse, BearDogError>;

    /// Cache data
    fn cache_set(&self, key: &str, value: &[u8], ttl: Option<u64>) -> Result<(), BearDogError>;

    /// Retrieve cached data
    fn cache_get(&self, key: &str) -> std::result::Result<Option<Vec<u8>>, BearDogError>;

    /// Remove from cache
    fn cache_delete(&self, key: &str) -> Result<(), BearDogError>;

    /// Get network status
    fn network_status(&self) -> std::result::Result<NetworkStatus, BearDogError>;
}

/// **WORKFLOW PROVIDER** - Workflow orchestration
pub trait WorkflowProvider: ConsolidatedProvider {
    /// Execute workflow
    fn execute_workflow(
        &self,
        workflow: &WorkflowDefinition,
    ) -> std::result::Result<WorkflowResult, BearDogError>;

    /// Get workflow status
    fn get_workflow_status(
        &self,
        workflow_id: &str,
    ) -> std::result::Result<WorkflowStatus, BearDogError>;

    /// Cancel workflow
    fn cancel_workflow(&self, workflow_id: &str) -> Result<(), BearDogError>;

    /// List active workflows
    fn list_workflows(&self) -> Result<Vec<WorkflowInstance>, BearDogError>;
}

/// **ADAPTER PROVIDER** - Universal adapter operations
pub trait AdapterProvider: ConsolidatedProvider {
    /// Discover available adapters
    fn discover_adapters(&self) -> Result<Vec<AdapterInfo>, BearDogError>;

    /// Connect to adapter
    fn connect_adapter(
        &self,
        adapter_id: &str,
    ) -> std::result::Result<AdapterConnection, BearDogError>;

    /// Execute adapter operation
    fn execute_operation(
        &self,
        operation: &AdapterRequest,
    ) -> std::result::Result<AdapterResponse, BearDogError>;

    /// Get adapter capabilities
    fn get_capabilities(
        &self,
        adapter_id: &str,
    ) -> std::result::Result<AdapterCapabilities, BearDogError>;
}

/// **GENETICS PROVIDER** - Genetic algorithm operations
pub trait GeneticsProvider: ConsolidatedProvider {
    /// Initialize population
    fn initialize_population(&self, size: usize) -> std::result::Result<Population, BearDogError>;

    /// Execute evolution step
    fn evolve(&self, population: &Population) -> std::result::Result<Population, BearDogError>;

    /// Evaluate fitness
    fn evaluate_fitness(&self, individual: &Individual) -> std::result::Result<f64, BearDogError>;

    /// Get evolution statistics
    fn get_statistics(&self) -> std::result::Result<EvolutionProgress, BearDogError>;
}
