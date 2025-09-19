// async_trait no longer needed - using native fn
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
// Canonical base types for trait system
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ConnectionStatus {
    /// State indicating connected
    Connected,
    /// State indicating disconnected
    Disconnected,
    /// Currently connecting
    Connecting,
    /// Error or failure state
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceHealth {
    /// Current status of the component
    pub status: String,
    pub uptime: u64,
    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
}
use beardog_types::canonical::providers_unified::{
    CanonicalProviderConfig as ProviderConfig, ProviderStatus,
};
use std::collections::HashMap;

pub type ProviderMetrics = HashMap<String, f64>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthStatus {
    /// Last health check timestamp
    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Resource usage metrics
    /// Mapping of resource usage
    pub resource_usage: HashMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderInfo {
    /// Provider name
    /// Name of the item
    pub name: String,
    /// Provider version
    /// The version value
    pub version: String,
    /// Provider description
    /// The description value
    pub description: String,
    /// Type of provider
    pub provider_type: String,
    /// Available capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

pub trait PlatformProvider: Send + Sync {
    fn platform_info(
        &self,
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, BearDogError>> + Send;

    fn initialize_platform(
        &self,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn platform_capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;
}

pub trait BaseProvider: Send + Sync {
    fn provider_id(&self) -> &str;

    /// Gets info
    fn get_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "BearDog Provider".to_string(),
            version: "3.0.0".to_string(),
            description: "Canonical BearDog Provider".to_string(),
            provider_type: "generic".to_string(),
            capabilities: vec![],
            metadata: HashMap::with_capacity(16),
        }
    }

    fn provider_info(&self) -> ProviderInfo {
        self.get_info()
    }

    fn id(&self) -> &str {
        self.provider_id()
    }

    /// Initializes componentialize
    fn initialize(
        &mut self,
        config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<HealthStatus, BearDogError>> + Send;

    fn capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;

    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderMetrics, BearDogError>> + Send;

    fn connection_status(&self) -> ConnectionStatus {
        ConnectionStatus::Connected
    }

    fn service_health(&self) -> HealthStatus {
        HealthStatus {
            last_check: chrono::Utc::now(),
            uptime_seconds: 0,
            resource_usage: HashMap::with_capacity(16),
        }
    }

    /// Validates config
    fn validate_config(
        &self,
        config: &ProviderConfig,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    fn status(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderStatus, BearDogError>> + Send;

    fn supports_capability(&self, capability: &str) -> bool {
        self.get_info()
            .capabilities
            .contains(&capability.to_string())
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    fn version(&self) -> &'static str {
        "3.0.0"
    }

    fn reload_config(
        &self,
        config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}
