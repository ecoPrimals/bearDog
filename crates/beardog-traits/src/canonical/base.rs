// SPDX-License-Identifier: AGPL-3.0-only

//! Legacy provider connection state, health snapshots, and the [`BaseProvider`] contract.

use beardog_errors::BearDogError;
/// Transport or RPC connection state for a canonical provider.
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

/// Compact health tuple for lightweight probes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceHealth {
    /// Current status of the component
    pub status: String,
    /// Seconds the service has been running since last restart.
    pub uptime: u64,
    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
}
use beardog_types::canonical::providers_unified::{
    CanonicalProviderConfig as ProviderConfig, ProviderStatus,
};
use std::collections::HashMap;

/// Simple scalar metrics keyed by name (latency, error rate, queue depth, …).
pub type ProviderMetrics = HashMap<String, f64>;

/// Result of a synchronous health probe against a provider instance.
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

/// Static marketing and capability metadata returned to registries.
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
    /// Logical class (`security`, `crypto`, `storage`, …).
    pub provider_type: String,
    /// Available capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Host OS and hardware introspection for installers and attestation.
pub trait PlatformProvider: Send + Sync {
    /// Returns CPU arch, kernel version, and similar facts.
    fn platform_info(
        &self,
    ) -> impl std::future::Future<Output = Result<HashMap<String, String>, BearDogError>> + Send;

    /// One-time setup hooks (directories, permissions, drivers).
    fn initialize_platform(
        &self,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Lists optional platform features the runtime may exploit.
    fn platform_capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;
}

/// Minimal provider surface used across the legacy canonical stack.
pub trait BaseProvider: Send + Sync {
    /// Stable id for metrics, logging, and configuration keys.
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

    /// Alias for [`Self::get_info`] retained for older call sites.
    fn provider_info(&self) -> ProviderInfo {
        self.get_info()
    }

    /// Same as [`Self::provider_id`] for [`BaseProvider`] implementors.
    fn id(&self) -> &str {
        self.provider_id()
    }

    /// Initializes componentialize
    fn initialize(
        &mut self,
        config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Lightweight liveness/readiness probe.
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<HealthStatus, BearDogError>> + Send;

    /// Declares feature strings consumers may query via [`Self::supports_capability`].
    fn capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;

    /// Numeric counters/gauges for observability stacks.
    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderMetrics, BearDogError>> + Send;

    /// Current link state to upstream dependencies.
    fn connection_status(&self) -> ConnectionStatus {
        ConnectionStatus::Connected
    }

    /// Aggregated health block including synthetic uptime counters.
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

    /// Fine-grained provider state for orchestrators.
    fn status(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderStatus, BearDogError>> + Send;

    /// Case-sensitive match against [`ProviderInfo::capabilities`].
    fn supports_capability(&self, capability: &str) -> bool {
        self.get_info()
            .capabilities
            .contains(&capability.to_string())
    }

    /// Release handles; implementors should flush buffers here.
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Semantic version string of this provider build.
    fn version(&self) -> &'static str {
        "3.0.0"
    }

    /// Hot-reload settings without restarting the process.
    fn reload_config(
        &self,
        config: ProviderConfig,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}
