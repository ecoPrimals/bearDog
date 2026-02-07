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
    ///
    /// Maps the discovery types from beardog-discovery to the canonical
    /// UniversalServiceDescriptor from beardog-types.
    fn convert_service(
        service: &DiscoveredService,
        queried_capabilities: &[UniversalCapabilityType],
    ) -> UniversalServiceDescriptor {
        use beardog_types::canonical::discovery::{
            AuthenticationMethod, PerformanceProfile, ServiceEndpoint,
        };
        use std::collections::HashMap;

        // Parse the primary_url to extract protocol, host, port
        let (protocol, host, port, path) = Self::parse_url(&service.endpoint.primary_url);

        UniversalServiceDescriptor {
            service_id: service.id.clone(),
            capabilities: queried_capabilities.to_vec(),
            endpoint: ServiceEndpoint {
                protocol,
                host,
                port,
                path,
                parameters: HashMap::new(),
            },
            auth_method: if service.endpoint.use_tls {
                AuthenticationMethod::MutualTls {
                    cert_path: String::new(), // Determined at connection time
                    key_path: String::new(),
                }
            } else {
                AuthenticationMethod::None
            },
            performance_profile: PerformanceProfile {
                avg_response_time_ms: service.qos.avg_response_time_ms,
                p95_response_time_ms: service.qos.avg_response_time_ms * 1.5, // Estimate
                success_rate: service.qos.success_rate,
                throughput_ops_per_sec: service.qos.requests_per_second,
                availability: service.qos.availability,
            },
            trust_score: 0.5, // Default trust, earned through interaction
        }
    }

    /// Parse URL into (protocol, host, port, path)
    fn parse_url(url: &str) -> (String, String, u16, Option<String>) {
        // Simple URL parsing - handles common formats
        let default_port = 8080u16;

        if let Some(rest) = url.strip_prefix("https://") {
            Self::parse_host_port_path(rest, "https", 443)
        } else if let Some(rest) = url.strip_prefix("http://") {
            Self::parse_host_port_path(rest, "http", default_port)
        } else if let Some(rest) = url.strip_prefix("grpc://") {
            Self::parse_host_port_path(rest, "grpc", 9090)
        } else if let Some(rest) = url.strip_prefix("unix://") {
            ("unix".to_string(), rest.to_string(), 0, None)
        } else {
            // Assume http if no protocol
            Self::parse_host_port_path(url, "http", default_port)
        }
    }

    fn parse_host_port_path(
        host_port_path: &str,
        protocol: &str,
        default_port: u16,
    ) -> (String, String, u16, Option<String>) {
        let (host_port, path) = match host_port_path.find('/') {
            Some(idx) => (&host_port_path[..idx], Some(host_port_path[idx..].to_string())),
            None => (host_port_path, None),
        };

        let (host, port) = match host_port.rfind(':') {
            Some(idx) => {
                let h = &host_port[..idx];
                let p = host_port[idx + 1..].parse().unwrap_or(default_port);
                (h.to_string(), p)
            }
            None => (host_port.to_string(), default_port),
        };

        (protocol.to_string(), host, port, path)
    }
}

#[async_trait]
impl PrimalDiscoveryClient for BearDogDiscoveryClient {
    /// Discover primals with specific capabilities
    ///
    /// # Async Discovery Flow
    ///
    /// 1. Query the CapabilityDiscovery engine for services matching capabilities
    /// 2. For each discovered service, create a UniversalServiceDescriptor
    /// 3. Filter by health status if available
    async fn discover_primals(
        &self,
        capabilities: Vec<UniversalCapabilityType>,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        debug!("🔍 Discovering primals with {} capabilities", capabilities.len());

        // Convert capability types to strings for discovery
        let capability_strings: Vec<String> = capabilities
            .iter()
            .map(Self::capability_to_string)
            .collect();

        debug!("🔍 Querying for capabilities: {:?}", capability_strings);

        // Query the discovery engine for services
        let discovery = self.discovery.read().await;
        
        let mut results: Vec<UniversalServiceDescriptor> = Vec::new();
        
        for capability in &capability_strings {
            // Use the discovery engine to find services with this capability
            match discovery.find_by_capability(capability).await {
                Ok(services) => {
                    for service in services {
                        // Convert DiscoveredService to UniversalServiceDescriptor
                        results.push(Self::convert_service(&service, &capabilities));
                    }
                }
                Err(e) => {
                    warn!("⚠️ Discovery error for capability {}: {}", capability, e);
                    // Continue with other capabilities
                }
            }
        }

        debug!("🔍 Found {} services matching capabilities", results.len());
        Ok(results)
    }

    /// Send request to primal with capability
    ///
    /// # Async Integration
    ///
    /// This method is now async and ready for proper IPC integration.
    /// Integration with beardog-ipc UnixSocketClient should be added.
    ///
    /// # Current Behavior
    ///
    /// Returns NotImplemented until beardog-ipc integration is completed.
    /// This ensures callers know they need to handle the async response properly.
    async fn send_request(
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
        // TODO: Integration with beardog-ipc UnixSocketClient:
        // ```rust
        // use beardog_ipc::UnixSocketClient;
        // let client = UnixSocketClient::connect(&service.endpoint_url).await?;
        // let response = client.call(&request.method, &request.params).await?;
        // ```

        Err(BearDogError::not_implemented(&format!(
            "Async discovery client ready - beardog-ipc integration pending. \
             Target: {}. Method: {}",
            service.endpoint_url, request.method
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
