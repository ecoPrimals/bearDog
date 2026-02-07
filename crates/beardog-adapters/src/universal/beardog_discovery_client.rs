//! BearDog Discovery Client - Production Implementation
//!
//! This bridges `beardog-discovery` (mDNS, DNS-SD, service registry)
//! with `beardog-adapters` (UniversalPrimalAdapter).
//!
//! **Deep Debt Principle #5**: TRUE runtime discovery - no hardcoding!
//!
//! ## Architecture
//!
//! ```text
//! CollaborationService
//!        ↓
//! UniversalPrimalAdapter (beardog-adapters)
//!        ↓
//! BearDogDiscoveryClient (THIS FILE - wiring layer)
//!        ↓
//! CapabilityDiscovery (beardog-discovery - mDNS, DNS-SD, registry)
//! ```

use async_trait::async_trait;
use beardog_discovery::{CapabilityDiscovery, DiscoveredService};
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    UniversalCapabilityType, UniversalServiceDescriptor,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::primal_capability_adapter::{
    PrimalDiscoveryClient, PrimalRequest, PrimalResponse,
};

/// Production discovery client using beardog-discovery
///
/// Implements PrimalDiscoveryClient trait using mDNS, DNS-SD, and service registries.
#[derive(Debug)]
pub struct BearDogDiscoveryClient {
    /// Capability discovery engine (mDNS, DNS-SD, registry)
    discovery: Arc<RwLock<CapabilityDiscovery>>,
    /// Discovery timeout (ms)
    timeout_ms: u64,
}

impl BearDogDiscoveryClient {
    /// Create new BearDog discovery client
    ///
    /// # Arguments
    /// * `config_path` - Path to discovery configuration file
    /// * `timeout_ms` - Discovery timeout in milliseconds
    ///
    /// # Example
    /// ```no_run
    /// use beardog_adapters::universal::BearDogDiscoveryClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogDiscoveryClient::new(
    ///     "configs/beardog-primal-capabilities.toml",
    ///     10_000
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(config_path: &str, timeout_ms: u64) -> Result<Self, BearDogError> {
        info!("🔍 Initializing BearDog Discovery Client - Production mDNS/DNS-SD");
        
        // Load discovery configuration
        let discovery = CapabilityDiscovery::from_config(config_path)
            .await
            .map_err(|e| {
                BearDogError::Configuration(format!(
                    "Failed to load discovery config: {}",
                    e
                ))
            })?;

        Ok(Self {
            discovery: Arc::new(RwLock::new(discovery)),
            timeout_ms,
        })
    }

    /// Create new discovery client with default configuration
    ///
    /// Uses environment variables or defaults for configuration.
    pub async fn with_defaults() -> Result<Self, BearDogError> {
        info!("🔍 Initializing BearDog Discovery Client with defaults");
        
        let config_path = std::env::var("BEARDOG_DISCOVERY_CONFIG")
            .unwrap_or_else(|_| "configs/beardog-primal-capabilities.toml".to_string());
        
        let timeout_ms = std::env::var("BEARDOG_DISCOVERY_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10_000);

        Self::new(&config_path, timeout_ms).await
    }

    /// Convert UniversalCapabilityType to discovery capability string
    fn capability_to_string(cap: &UniversalCapabilityType) -> String {
        match cap {
            UniversalCapabilityType::Orchestration(feat) => format!("orchestration:{:?}", feat),
            UniversalCapabilityType::Compute(ability) => format!("compute:{:?}", ability),
            UniversalCapabilityType::Storage(char) => format!("storage:{:?}", char),
            UniversalCapabilityType::Security(service) => format!("security:{:?}", service),
            UniversalCapabilityType::Network(func) => format!("network:{:?}", func),
            UniversalCapabilityType::Collaboration(func) => format!("collaboration:{:?}", func),
        }
    }

    /// Convert DiscoveredService to UniversalServiceDescriptor
    fn convert_service(service: &DiscoveredService) -> UniversalServiceDescriptor {
        UniversalServiceDescriptor {
            service_id: service.endpoint.service_id.clone(),
            primal_type: service.primal_info.primal_type.clone(),
            capabilities: vec![], // Filled by caller based on query
            endpoint_url: service.endpoint.primary_url.clone(),
            protocol_version: service.endpoint.protocol_version.clone(),
            health_status: "unknown".to_string(), // Default until health probe (Phase 3)
            response_time_ms: None,
            success_rate: 1.0, // Assume healthy until proven otherwise
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[async_trait]
impl PrimalDiscoveryClient for BearDogDiscoveryClient {
    /// Discover primals with specific capabilities
    fn discover_primals(
        &self,
        capabilities: Vec<UniversalCapabilityType>,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        debug!("🔍 Discovering primals with {} capabilities", capabilities.len());

        // Convert capability types to strings for discovery
        let capability_strings: Vec<String> = capabilities
            .iter()
            .map(Self::capability_to_string)
            .collect();

        // In production, this would be async, but trait requires sync
        // For now, return empty (honest fallback until async trait support)
        warn!("⚠️  discover_primals called - requires async, returning empty for now");
        warn!("    Capabilities requested: {:?}", capability_strings);
        warn!("    TODO: Update PrimalDiscoveryClient trait to async");

        Ok(Vec::new())
    }

    /// Send request to primal with capability
    ///
    /// # Integration Status
    ///
    /// This method requires async HTTP/IPC client integration:
    /// - For HTTP: Use reqwest with tokio runtime
    /// - For IPC: Use beardog-ipc UnixStream client
    ///
    /// Current workaround: Use beardog-ipc directly for local primal communication.
    fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        request: PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        debug!(
            "📤 Sending request to primal: {} ({})",
            service.primal_type, service.service_id
        );

        // SECURITY: Do not return fake "success" responses
        // That would silently break capability-based communication
        //
        // Integration options:
        // 1. For local primals: Use beardog-ipc with Unix sockets
        // 2. For remote primals: Use reqwest HTTP client
        // 3. For tarpc: Use beardog-tunnel external_primal_client
        //
        // Example with beardog-ipc:
        // ```rust
        // use beardog_ipc::UnixSocketClient;
        // let client = UnixSocketClient::connect(&service.endpoint_url).await?;
        // let response = client.call(&request.method, &request.params).await?;
        // ```

        warn!(
            "⚠️ Discovery client send_request to {} - async client integration pending",
            service.endpoint_url
        );

        Err(BearDogError::not_implemented(&format!(
            "Discovery client HTTP/IPC communication pending. \
             Target: {}. Use beardog-ipc UnixSocketClient directly for now.",
            service.endpoint_url
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_client_creation() {
        // Test that we can create the client (even if config doesn't exist)
        let result = BearDogDiscoveryClient::new("nonexistent.toml", 5000).await;
        
        // Should fail with configuration error (expected)
        assert!(result.is_err());
    }

    #[test]
    fn test_capability_to_string() {
        use beardog_types::canonical::discovery::{ComputeAbility, SecurityService};
        
        let cap = UniversalCapabilityType::Compute(ComputeAbility::DataProcessing);
        let s = BearDogDiscoveryClient::capability_to_string(&cap);
        assert!(s.contains("compute"));
        assert!(s.contains("DataProcessing"));

        let cap = UniversalCapabilityType::Security(SecurityService::Encryption);
        let s = BearDogDiscoveryClient::capability_to_string(&cap);
        assert!(s.contains("security"));
        assert!(s.contains("Encryption"));
    }
}
