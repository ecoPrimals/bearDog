//! # Universal Storage Adapter
//!
//! This module provides a universal adapter for storage services,
//! enabling capability-based discovery and unified storage operations.

use beardog_errors::BearDogError;
use beardog_types::{
    canonical::providers::{ProviderConfig, ProviderHealthStatus},
    capabilities::CapabilityType,
    providers::{BaseProvider, PrimalRequest, PrimalResponse},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

// ============================================================
// Configuration Types
// ============================================================

/// Storage adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalStorageConfig {
    /// Discovery timeout in milliseconds
    pub discovery_timeout_ms: u64,

    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,

    /// Authentication configuration
    pub auth_config: UniversalStorageAuthConfig,

    /// Connection configuration
    pub connection_config: UniversalStorageConnectionConfig,

    /// Required capabilities
    pub required_capabilities: Vec<CapabilityType>,
}

impl Default for UniversalStorageConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: 10000,
            request_timeout_ms: 30000,
            auth_config: UniversalStorageAuthConfig::default(),
            connection_config: UniversalStorageConnectionConfig::default(),
            required_capabilities: vec![
                CapabilityType::StorageServices,
                CapabilityType::DataPersistence,
                CapabilityType::BackupRecovery,
            ],
        }
    }
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalStorageAuthConfig {
    /// Authentication type
    pub auth_type: String,

    /// Credentials
    pub credentials: HashMap<String, String>,

    /// Token refresh interval
    #[serde(with = "humantime_serde")]
    pub token_refresh_interval: Duration,
}

impl Default for UniversalStorageAuthConfig {
    fn default() -> Self {
        Self {
            auth_type: std::env::var("STORAGE_AUTH_TYPE")
                .unwrap_or_else(|_| "bearer_token".to_string()),
            credentials: HashMap::new(),
            token_refresh_interval: Duration::from_secs(3600),
        }
    }
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalStorageConnectionConfig {
    /// Connection timeout in seconds
    pub timeout_seconds: u32,

    /// Maximum retries
    pub max_retries: u32,

    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
}

impl Default for UniversalStorageConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            health_check_interval_seconds: 60,
        }
    }
}

// ============================================================
// Provider Types
// ============================================================

/// Provider health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderHealth {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self::Healthy
    }
}

/// Storage type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    /// Object storage
    Object,
    /// Block storage
    Block,
    /// File storage
    File,
    /// Archive storage
    Archive,
}

/// Storage performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformance {
    /// Read throughput in Mbps
    pub read_throughput_mbps: f64,

    /// Write throughput in Mbps
    pub write_throughput_mbps: f64,

    /// Average latency in ms
    pub average_latency_ms: f64,

    /// IOPS
    pub iops: u64,
}

impl Default for StoragePerformance {
    fn default() -> Self {
        Self {
            read_throughput_mbps: 100.0,
            write_throughput_mbps: 100.0,
            average_latency_ms: 5.0,
            iops: 1000,
        }
    }
}

/// Storage info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    /// Total capacity in GB
    pub total_capacity_gb: u64,

    /// Used capacity in GB
    pub used_capacity_gb: u64,

    /// Supported storage types
    pub supported_types: Vec<StorageType>,

    /// Performance metrics
    pub performance: StoragePerformance,
}

impl Default for StorageInfo {
    fn default() -> Self {
        Self {
            total_capacity_gb: 1000,
            used_capacity_gb: 0,
            supported_types: vec![StorageType::Object, StorageType::File],
            performance: StoragePerformance::default(),
        }
    }
}

/// Storage provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProvider {
    /// Provider ID
    pub provider_id: String,

    /// Endpoint URL
    pub endpoint: String,

    /// Capabilities
    pub capabilities: Vec<CapabilityType>,

    /// Health status
    pub health_status: ProviderHealth,

    /// Storage info
    pub storage_info: StorageInfo,

    /// Last health check
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

impl Default for StorageProvider {
    fn default() -> Self {
        Self {
            provider_id: Uuid::new_v4().to_string(),
            endpoint: String::new(),
            capabilities: Vec::new(),
            health_status: ProviderHealth::Healthy,
            storage_info: StorageInfo::default(),
            last_health_check: chrono::Utc::now(),
        }
    }
}

// ============================================================
// Universal Storage Adapter
// ============================================================

/// Universal storage adapter
pub struct UniversalStorageAdapter {
    /// Configuration
    pub config: UniversalStorageConfig,

    /// HTTP client
    http_client: reqwest::Client,

    /// Discovered providers
    discovered_providers: RwLock<Vec<StorageProvider>>,
}

impl UniversalStorageAdapter {
    /// Create a new adapter
    pub fn new(config: UniversalStorageConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
            discovered_providers: RwLock::new(Vec::new()),
        }
    }

    /// Discover storage providers
    ///
    /// # Errors
    /// Returns an error if discovery fails.
    pub async fn discover_storage_providers(&self) -> Result<Vec<StorageProvider>, BearDogError> {
        info!("🔍 Discovering storage providers by capability...");

        let discovery_endpoint = std::env::var("STORAGE_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| "https://capability-discovery.ecosystem.internal".to_string());

        match self.discover_from_endpoint(&discovery_endpoint).await {
            Ok(providers) => {
                let suitable_providers: Vec<_> = providers
                    .into_iter()
                    .filter(|provider| {
                        provider.health_status == ProviderHealth::Healthy
                            && self
                                .config
                                .required_capabilities
                                .iter()
                                .any(|cap| provider.capabilities.contains(cap))
                    })
                    .collect();

                info!("✅ Discovered {} suitable storage providers", suitable_providers.len());

                // Update cache
                let mut cache = self.discovered_providers.write().await;
                *cache = suitable_providers.clone();

                Ok(suitable_providers)
            }
            Err(e) => {
                warn!("⚠️ Discovery failed: {}", e);
                // Return cached providers if available
                let cache = self.discovered_providers.read().await;
                if cache.is_empty() {
                    Err(e)
                } else {
                    Ok(cache.clone())
                }
            }
        }
    }

    async fn discover_from_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Vec<StorageProvider>, BearDogError> {
        let discovery_url = format!("{}/api/v1/capabilities/storage", endpoint);

        let response = tokio::time::timeout(
            Duration::from_millis(self.config.discovery_timeout_ms),
            self.http_client.get(&discovery_url).send(),
        )
        .await
        .map_err(|_| BearDogError::timeout("Storage provider discovery timeout"))?
        .map_err(|e| BearDogError::network(format!("Discovery request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BearDogError::network(format!(
                "Discovery failed with status: {}",
                response.status()
            )));
        }

        let providers: Vec<StorageProvider> = response.json().await.map_err(|e| {
            BearDogError::api(
                format!("Failed to parse discovery response: {}", e),
                beardog_errors::ApiErrorCategory::General,
            )
        })?;

        Ok(providers)
    }

    /// Handle storage request
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn handle_storage_request(
        &self,
        request: PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        let providers = self.discover_storage_providers().await?;

        if let Some(provider) = providers.first() {
            match request.capability {
                CapabilityType::StorageServices => {
                    info!("💾 Processing storage services request");
                    self.handle_storage_operation(&provider.endpoint, &request)
                        .await
                }
                CapabilityType::DataPersistence => {
                    info!("🔐 Processing data persistence request");
                    self.handle_persistence_operation(&provider.endpoint, &request)
                        .await
                }
                CapabilityType::BackupRecovery => {
                    info!("🔄 Processing backup recovery request");
                    self.handle_backup_operation(&provider.endpoint, &request)
                        .await
                }
                _ => Err(BearDogError::system(format!(
                    "Unsupported capability: {:?}",
                    request.capability
                ))),
            }
        } else {
            Err(BearDogError::system(
                "No suitable storage provider available".to_string(),
            ))
        }
    }

    async fn handle_storage_operation(
        &self,
        endpoint: &str,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        let operation_url = format!("{}/api/v1/storage/operate", endpoint);

        let response = self
            .http_client
            .post(&operation_url)
            .json(&request.parameters)
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("Storage operation failed: {}", e)))?;

        if response.status().is_success() {
            Ok(PrimalResponse {
                request_id: request.request_id.clone(),
                success: true,
                data: serde_json::to_vec(&serde_json::json!({
                    "status": "success",
                    "details": "Storage operation completed successfully"
                }))
                .unwrap_or_default(),
                metadata: HashMap::new(),
            })
        } else {
            Err(BearDogError::network(format!(
                "Storage operation failed with status: {}",
                response.status()
            )))
        }
    }

    async fn handle_persistence_operation(
        &self,
        endpoint: &str,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        let persistence_url = format!("{}/api/v1/storage/persist", endpoint);

        let response = self
            .http_client
            .post(&persistence_url)
            .json(&request.parameters)
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("Persistence operation failed: {}", e)))?;

        if response.status().is_success() {
            Ok(PrimalResponse {
                request_id: request.request_id.clone(),
                success: true,
                data: serde_json::to_vec(&serde_json::json!({
                    "status": "success",
                    "details": "Data persistence completed successfully"
                }))
                .unwrap_or_default(),
                metadata: HashMap::new(),
            })
        } else {
            Err(BearDogError::network(format!(
                "Persistence operation failed with status: {}",
                response.status()
            )))
        }
    }

    async fn handle_backup_operation(
        &self,
        endpoint: &str,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        let backup_url = format!("{}/api/v1/storage/backup", endpoint);

        let response = self
            .http_client
            .post(&backup_url)
            .json(&request.parameters)
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("Backup operation failed: {}", e)))?;

        if response.status().is_success() {
            Ok(PrimalResponse {
                request_id: request.request_id.clone(),
                success: true,
                data: serde_json::to_vec(&serde_json::json!({
                    "status": "success",
                    "details": "Backup operation completed successfully"
                }))
                .unwrap_or_default(),
                metadata: HashMap::new(),
            })
        } else {
            Err(BearDogError::network(format!(
                "Backup operation failed with status: {}",
                response.status()
            )))
        }
    }
}

impl BaseProvider for UniversalStorageAdapter {
    fn provider_id(&self) -> &str {
        "universal_storage_adapter"
    }

    fn get_capabilities(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![
            "storage_services".to_string(),
            "data_persistence".to_string(),
            "backup_recovery".to_string(),
            "object_storage".to_string(),
            "file_management".to_string(),
        ])
    }

    fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        info!("🚀 Initializing Universal Storage Adapter");
        Ok(())
    }

    fn health_check(&self) -> Result<ProviderHealthStatus, BearDogError> {
        Ok(ProviderHealthStatus {
            healthy: true,
            last_check: chrono::Utc::now(),
            details: Some("Storage adapter healthy".to_string()),
            response_time_ms: Some(10),
        })
    }
}

// ============================================================
// Factory
// ============================================================

/// Factory for creating storage adapters
pub struct UniversalStorageAdapterFactory;

impl UniversalStorageAdapterFactory {
    /// Create with default configuration
    pub fn create_default() -> UniversalStorageAdapter {
        UniversalStorageAdapter::new(UniversalStorageConfig::default())
    }

    /// Create with custom configuration
    pub fn create_with_config(config: UniversalStorageConfig) -> UniversalStorageAdapter {
        UniversalStorageAdapter::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = UniversalStorageConfig::default();
        assert_eq!(config.discovery_timeout_ms, 10000);
        assert_eq!(config.request_timeout_ms, 30000);
    }

    #[test]
    fn test_connection_config_default() {
        let config = UniversalStorageConnectionConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_provider_health_default() {
        assert_eq!(ProviderHealth::default(), ProviderHealth::Healthy);
    }

    #[test]
    fn test_storage_performance_default() {
        let perf = StoragePerformance::default();
        assert_eq!(perf.read_throughput_mbps, 100.0);
    }

    #[test]
    fn test_factory() {
        let adapter = UniversalStorageAdapterFactory::create_default();
        assert_eq!(adapter.provider_id(), "universal_storage_adapter");
    }
}
