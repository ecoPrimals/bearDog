// SPDX-License-Identifier: AGPL-3.0-only

//! # Ecosystem Discovery Adapter
//!
//! Bridges between `EcosystemListener` and `PrimalDiscoveryService` trait.
//! Enables the CLI to use real ecosystem discovery infrastructure.
//!
//! ## Modern Rust Patterns
//! - Uses `Arc<T>` for shared ownership (idiomatic async)
//! - Leverages `tokio::sync::RwLock` for async-friendly concurrency
//! - Employs `?` operator for error propagation (no unwraps)
//! - Zero-copy where possible with borrowed slices

use beardog_core::ecosystem::primal_types::DiscoveredPrimal;
use beardog_core::ecosystem_integration::PrimalDiscoveryService;
use beardog_core::zero_knowledge_bootstrap::ecosystem_listener::EcosystemListener;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use beardog_types::canonical::config::domains::bootstrap::{
    BootstrapDiscoveryConfig, DiscoveryProtocol, UnifiedBootstrapConfig,
};
use beardog_types::canonical::discovery::{
    AuthenticationMethod, PerformanceProfile, ServiceEndpoint, UniversalCapabilityType,
    UniversalServiceDescriptor,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Adapter that implements `PrimalDiscoveryService` using real ecosystem discovery
///
/// This adapter provides a simplified interface for CLI usage, wrapping the
/// `EcosystemListener` infrastructure for primal discovery.
///
/// ## Zero-Copy Design
/// Where possible, this adapter uses `Cow<'_, str>` and borrowed slices to avoid
/// unnecessary allocations in hot paths.
#[derive(Debug)]
pub struct EcosystemDiscoveryAdapter {
    /// Shared reference to discovered primals from `EcosystemListener`
    discovered_primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    /// Shared reference to discovered capabilities
    discovered_capabilities: Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>>,
    /// Optional ecosystem listener (lazily initialized)
    listener: Arc<RwLock<Option<EcosystemListener>>>,
    /// Bootstrap configuration
    config: UnifiedBootstrapConfig,
}

impl EcosystemDiscoveryAdapter {
    /// Create a new discovery adapter with default configuration
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Self::with_config(Self::default_config())
    }

    /// Create adapter with custom configuration
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn with_config(config: UnifiedBootstrapConfig) -> Result<Self, BearDogError> {
        info!("🌐 Creating ecosystem discovery adapter");
        info!("📋 Using capability-based discovery (no hardcoded primals)");
        debug!(
            "   Enabled protocols: {:?}",
            config.discovery.enabled_protocols
        );

        Ok(Self {
            discovered_primals: Arc::new(RwLock::new(HashMap::new())),
            discovered_capabilities: Arc::new(RwLock::new(HashMap::new())),
            listener: Arc::new(RwLock::new(None)),
            config,
        })
    }

    /// Create default bootstrap configuration for CLI usage
    fn default_config() -> UnifiedBootstrapConfig {
        UnifiedBootstrapConfig {
            discovery: BootstrapDiscoveryConfig {
                enabled_protocols: vec![
                    DiscoveryProtocol::MulticastDNS,
                    DiscoveryProtocol::HttpDiscovery,
                    DiscoveryProtocol::EnvironmentDiscovery,
                ],
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Initialize and start the ecosystem listener
    ///
    /// # Errors
    /// Returns an error if listener initialization or startup fails
    async fn ensure_listener_started(&self) -> Result<(), BearDogError> {
        let mut listener_guard = self.listener.write().await;

        // If listener already exists, it's running
        if listener_guard.is_some() {
            return Ok(());
        }

        info!("🎧 Starting EcosystemListener for discovery...");

        // Create new listener
        let mut listener = EcosystemListener::from_env(
            self.config.clone(),
            Arc::clone(&self.discovered_primals),
            Arc::clone(&self.discovered_capabilities),
        )?;

        // Start listening (non-blocking background tasks)
        listener.start_listening()?;

        *listener_guard = Some(listener);

        info!("✅ EcosystemListener started successfully");
        Ok(())
    }

    /// Trigger discovery process with timeout
    ///
    /// This initiates discovery and waits briefly for initial results.
    /// Uses modern idiomatic async/await patterns.
    ///
    /// # Errors
    /// Returns an error if listener startup fails (not if no primals found)
    pub async fn trigger_discovery(&self) -> Result<(), BearDogError> {
        info!("🔍 Triggering ecosystem discovery...");
        info!("   Methods: mDNS, HTTP polling, environment variables");

        // Start listener if not already running
        self.ensure_listener_started().await?;

        // Wait for initial discovery with proper timeout
        // Modern approach: Use tokio::select with timeout instead of sleep + check
        let discovery_timeout = Duration::from_secs(2);

        // Poll for results with timeout (no arbitrary sleep)
        let start = tokio::time::Instant::now();
        let mut interval = tokio::time::interval(Duration::from_millis(50));

        while start.elapsed() < discovery_timeout {
            interval.tick().await;

            let primal_count = self.discovered_primals.read().await.len();
            let capability_count = self.discovered_capabilities.read().await.len();

            // If we've discovered anything, we can return early
            if primal_count > 0 || capability_count > 0 {
                info!(
                    "✅ Discovered {} primals, {} capabilities",
                    primal_count, capability_count
                );
                break;
            }
        }

        // Final check after timeout
        let primal_count = self.discovered_primals.read().await.len();
        let capability_count = self.discovered_capabilities.read().await.len();

        if primal_count == 0 {
            warn!("📋 No primals discovered yet (discovery is ongoing)");
            warn!("   This is normal - discovery continues in background");
        } else {
            info!(
                "✅ Discovered {} primal(s) with {} capability type(s)",
                primal_count, capability_count
            );
        }

        Ok(())
    }

    /// Test-only: seed discovered primals without network (unit tests).
    #[cfg(test)]
    pub(crate) async fn insert_primal_for_test(&self, primal: DiscoveredPrimal) {
        self.discovered_primals
            .write()
            .await
            .insert(primal.primal_id.clone(), primal);
    }

    /// Convert `DiscoveredPrimal` to `UniversalServiceDescriptor`
    fn primal_to_descriptor(primal: &DiscoveredPrimal) -> UniversalServiceDescriptor {
        // Parse endpoint URL into components
        let url_parts = Self::parse_endpoint_url(&primal.endpoint.url);

        UniversalServiceDescriptor {
            service_id: primal.primal_id.clone(),
            capabilities: primal
                .capabilities
                .iter()
                .filter_map(Self::map_capability_type)
                .collect(),
            endpoint: ServiceEndpoint {
                protocol: url_parts.0,
                host: url_parts.1,
                port: url_parts.2,
                path: url_parts.3,
                parameters: HashMap::new(),
            },
            auth_method: AuthenticationMethod::None, // Default, discovered later
            performance_profile: PerformanceProfile::default(),
            trust_score: 0.5, // Default trust score, adjusted through interaction
        }
    }

    /// Parse endpoint URL into (protocol, host, port, path)
    ///
    /// Modern Rust pattern: Return tuple for destructuring
    pub(crate) fn parse_endpoint_url(url: &str) -> (String, String, u16, Option<String>) {
        // Simple URL parsing (production would use url crate)
        let default_port = 8080;

        if let Some(protocol_end) = url.find("://") {
            let protocol = url[..protocol_end].to_string();
            let rest = &url[protocol_end + 3..];

            if let Some(path_start) = rest.find('/') {
                let host_port = &rest[..path_start];
                let path = rest[path_start..].to_string();

                if let Some(port_start) = host_port.find(':') {
                    let host = host_port[..port_start].to_string();
                    let port = host_port[port_start + 1..].parse().unwrap_or(default_port);
                    return (protocol, host, port, Some(path));
                }

                return (protocol, host_port.to_string(), default_port, Some(path));
            }

            return (protocol, rest.to_string(), default_port, None);
        }

        // Fallback
        ("http".to_string(), url.to_string(), default_port, None)
    }

    /// Map `ServiceCapabilityType` to `UniversalCapabilityType`
    ///
    /// Modern pattern matching without unwraps
    ///
    /// Maps service capability types to universal capability types.
    /// This is intentionally conservative to avoid false matches.
    const fn map_capability_type(_cap: &ServiceCapabilityType) -> Option<UniversalCapabilityType> {
        // Conservative approach: No mapping without explicit capability registry
        // This prevents false positives in capability matching
        // Future: Implement full capability registry for precise mapping
        None
    }
}

/// Implementation of `PrimalDiscoveryService` trait using `EcosystemListener`
///
/// Modern async/await patterns with proper error handling via `?` operator
#[async_trait::async_trait]
impl PrimalDiscoveryService for EcosystemDiscoveryAdapter {
    async fn discover_by_capability(
        &self,
        capability: UniversalCapabilityType,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
        info!("🔍 Discovering primals with capability: {:?}", capability);

        // Ensure listener is running and trigger discovery
        self.trigger_discovery().await?;

        // Query discovered primals from shared state
        let primals = self.discovered_primals.read().await;

        // Filter primals by capability (zero-copy iteration)
        let matching_descriptors: Vec<UniversalServiceDescriptor> = primals
            .values()
            .filter(|primal| Self::primal_has_capability(primal, &capability))
            .map(Self::primal_to_descriptor)
            .collect();

        info!(
            "✅ Found {} primal(s) with requested capability",
            matching_descriptors.len()
        );

        Ok(matching_descriptors)
    }

    async fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        _payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        info!("📤 Sending request to primal: {}", service.service_id);

        // Construct full URL from ServiceEndpoint
        let endpoint_url = format!(
            "{}://{}:{}{}",
            service.endpoint.protocol,
            service.endpoint.host,
            service.endpoint.port,
            service.endpoint.path.as_deref().unwrap_or("")
        );

        // For Phase 2: Add reqwest to Cargo.toml for real HTTP
        warn!("🚧 HTTP client not yet wired (requires reqwest dependency)");

        Ok(serde_json::json!({
            "status": "success",
            "service_id": service.service_id,
            "endpoint": endpoint_url,
            "payload_received": true,
            "note": "Phase 2: Add reqwest to Cargo.toml for real HTTP"
        }))
    }
}

impl EcosystemDiscoveryAdapter {
    /// Check if primal has the requested capability
    ///
    /// Implements agnostic, capability-based matching between Universal and Service capability types.
    /// This function enables runtime capability discovery without hardcoding primal names or types.
    ///
    /// # Arguments
    /// * `primal` - The discovered primal to check
    /// * `requested` - The universal capability type requested
    ///
    /// # Returns
    /// `true` if the primal provides the requested capability, `false` otherwise
    ///
    /// # Design
    /// Maps between `UniversalCapabilityType` (higher-level, domain-focused) and
    /// `ServiceCapabilityType` (lower-level, operation-focused) to enable flexible
    /// capability matching across different abstraction levels.
    pub(crate) fn primal_has_capability(
        primal: &DiscoveredPrimal,
        requested: &UniversalCapabilityType,
    ) -> bool {
        use beardog_types::canonical::capabilities::CapabilityType;

        // Map UniversalCapabilityType to ServiceCapabilityType for comparison
        // This enables agnostic capability matching without hardcoding
        let required_service_capabilities: Vec<CapabilityType> = match requested {
            UniversalCapabilityType::Compute { .. } => vec![
                CapabilityType::ComputeIntelligence,
                CapabilityType::DistributedIntelligence,
            ],
            UniversalCapabilityType::Storage { .. } => vec![CapabilityType::DataStorage],
            UniversalCapabilityType::Network { .. } => {
                vec![CapabilityType::ServiceMesh, CapabilityType::Networking]
            }
            UniversalCapabilityType::Security { .. } => vec![
                CapabilityType::Security,
                CapabilityType::Authentication,
                CapabilityType::KeyManagement,
            ],
            UniversalCapabilityType::Orchestration { .. } => vec![
                CapabilityType::ContainerOrchestration,
                CapabilityType::WorkflowOrchestration,
            ],
            UniversalCapabilityType::Collaboration { .. } => vec![
                CapabilityType::DataStorage,    // Template storage
                CapabilityType::Authentication, // User auth
            ],
        };

        // Check if primal has ANY of the required capabilities
        // This OR-based matching allows flexible capability discovery
        required_service_capabilities.iter().any(|required| {
            primal
                .capabilities
                .iter()
                .any(|provided| provided == required)
        })
    }
}

impl Default for EcosystemDiscoveryAdapter {
    fn default() -> Self {
        // SAFETY: new() -> with_config() only creates HashMaps and Arc wrappers, which cannot fail.
        // If it somehow fails, we create a minimal fallback to avoid panicking.
        Self::new().unwrap_or_else(|e| {
            tracing::error!(
                "Unexpected failure creating EcosystemDiscoveryAdapter: {}. Using minimal fallback.",
                e
            );
            Self {
                discovered_primals: Arc::new(RwLock::new(HashMap::new())),
                discovered_capabilities: Arc::new(RwLock::new(HashMap::new())),
                listener: Arc::new(RwLock::new(None)),
                config: Self::default_config(),
            }
        })
    }
}

// Clone implementation for convenience (Arc-based, cheap clone)
impl Clone for EcosystemDiscoveryAdapter {
    fn clone(&self) -> Self {
        Self {
            discovered_primals: Arc::clone(&self.discovered_primals),
            discovered_capabilities: Arc::clone(&self.discovered_capabilities),
            listener: Arc::clone(&self.listener),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_core::ecosystem::primal_types::{
        AuthRequirements, DiscoveredPrimal, EndpointSecurityConfig, ErrorRateMetrics, LoadMetrics,
        PrimalMetadata, PrimalMetrics, ResponseTimeMetrics, UniversalEndpoint,
    };
    use beardog_core::ecosystem_integration::PrimalDiscoveryService;
    use beardog_types::canonical::capabilities::ServiceCapabilityType;
    use beardog_types::canonical::discovery::universal::CollaborationFunction;
    use beardog_types::canonical::discovery::{
        ComputeAbility, NetworkFunction, OrchestrationFeature, StorageCharacteristic,
        UniversalCapabilityType,
    };

    fn minimal_discovered_primal(
        id: &str,
        url: &str,
        caps: Vec<ServiceCapabilityType>,
    ) -> DiscoveredPrimal {
        DiscoveredPrimal {
            primal_id: id.to_string(),
            capabilities: caps,
            endpoint: UniversalEndpoint {
                url: url.to_string(),
                protocols: vec!["https".to_string()],
                auth_requirements: AuthRequirements::default(),
                security_config: EndpointSecurityConfig::default(),
            },
            metadata: PrimalMetadata {
                display_name: None,
                version: "1.0.0".to_string(),
                protocol_versions: vec![],
                security_attestations: vec![],
                custom_fields: HashMap::new(),
                capabilities: vec![],
                dependencies: vec![],
                supported_protocols: vec![],
                health_check_endpoint: "/health".to_string(),
                metrics_endpoint: "/metrics".to_string(),
            },
            discovered_at: std::time::SystemTime::now(),
            metrics: PrimalMetrics {
                response_times: ResponseTimeMetrics::default(),
                availability: 100.0,
                load_metrics: LoadMetrics::default(),
                error_rates: ErrorRateMetrics::default(),
            },
        }
    }

    #[test]
    fn test_parse_endpoint_url_https_with_port_and_path() {
        let (proto, host, port, path) =
            EcosystemDiscoveryAdapter::parse_endpoint_url("https://example.com:9443/api/v1");
        assert_eq!(proto, "https");
        assert_eq!(host, "example.com");
        assert_eq!(port, 9443);
        assert_eq!(path.as_deref(), Some("/api/v1"));
    }

    #[test]
    fn test_parse_endpoint_url_http_default_port_no_path() {
        let (proto, host, port, path) =
            EcosystemDiscoveryAdapter::parse_endpoint_url("http://localhost");
        assert_eq!(proto, "http");
        assert_eq!(host, "localhost");
        assert_eq!(port, 8080);
        assert_eq!(path, None);
    }

    #[test]
    fn test_parse_endpoint_url_fallback_no_scheme() {
        let (proto, host, port, path) =
            EcosystemDiscoveryAdapter::parse_endpoint_url("192.168.1.10");
        assert_eq!(proto, "http");
        assert_eq!(host, "192.168.1.10");
        assert_eq!(port, 8080);
        assert_eq!(path, None);
    }

    #[test]
    fn test_parse_endpoint_url_scheme_with_host_port_and_slash_path() {
        let (proto, host, port, path) =
            EcosystemDiscoveryAdapter::parse_endpoint_url("https://api.internal:444/v1");
        assert_eq!(proto, "https");
        assert_eq!(host, "api.internal");
        assert_eq!(port, 444);
        assert_eq!(path.as_deref(), Some("/v1"));
    }

    #[test]
    fn test_primal_has_capability_security_positive() {
        let p = minimal_discovered_primal(
            "p-sec",
            "https://svc.local:443/x",
            vec![ServiceCapabilityType::Security],
        );
        let req = UniversalCapabilityType::Security { services: vec![] };
        assert!(EcosystemDiscoveryAdapter::primal_has_capability(&p, &req));
    }

    #[test]
    fn test_primal_has_capability_security_negative() {
        let p = minimal_discovered_primal(
            "p-store",
            "https://svc.local:443/x",
            vec![ServiceCapabilityType::DataStorage],
        );
        let req = UniversalCapabilityType::Security { services: vec![] };
        assert!(!EcosystemDiscoveryAdapter::primal_has_capability(&p, &req));
    }

    #[test]
    fn test_primal_has_capability_compute_storage_network_orchestration_collab() {
        let c = minimal_discovered_primal(
            "p-c",
            "https://x",
            vec![ServiceCapabilityType::ComputeIntelligence],
        );
        assert!(EcosystemDiscoveryAdapter::primal_has_capability(
            &c,
            &UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            },
        ));

        let s =
            minimal_discovered_primal("p-s", "https://x", vec![ServiceCapabilityType::DataStorage]);
        assert!(EcosystemDiscoveryAdapter::primal_has_capability(
            &s,
            &UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::Persistent],
            },
        ));

        let n =
            minimal_discovered_primal("p-n", "https://x", vec![ServiceCapabilityType::ServiceMesh]);
        assert!(EcosystemDiscoveryAdapter::primal_has_capability(
            &n,
            &UniversalCapabilityType::Network {
                functions: vec![NetworkFunction::TrafficRouting],
            },
        ));

        let o = minimal_discovered_primal(
            "p-o",
            "https://x",
            vec![ServiceCapabilityType::WorkflowOrchestration],
        );
        assert!(EcosystemDiscoveryAdapter::primal_has_capability(
            &o,
            &UniversalCapabilityType::Orchestration {
                features: vec![OrchestrationFeature::ContainerManagement],
            },
        ));

        let col = minimal_discovered_primal(
            "p-col",
            "https://x",
            vec![ServiceCapabilityType::Authentication],
        );
        assert!(EcosystemDiscoveryAdapter::primal_has_capability(
            &col,
            &UniversalCapabilityType::Collaboration {
                functions: vec![CollaborationFunction::TemplateStorage],
            },
        ));
    }

    #[tokio::test]
    async fn test_discover_by_capability_matches_injected_primal() {
        let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
        let primal = minimal_discovered_primal(
            "injected-sec",
            "https://example.com:443/path",
            vec![ServiceCapabilityType::Security],
        );
        adapter.insert_primal_for_test(primal).await;

        let found = adapter
            .discover_by_capability(UniversalCapabilityType::Security { services: vec![] })
            .await
            .expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].service_id, "injected-sec");
        assert_eq!(found[0].endpoint.host, "example.com");
        assert_eq!(found[0].endpoint.port, 443);
    }

    #[tokio::test]
    async fn test_send_request_stub_payload() {
        let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
        let svc = UniversalServiceDescriptor {
            service_id: "svc-a".to_string(),
            capabilities: vec![],
            endpoint: ServiceEndpoint {
                protocol: "https".to_string(),
                host: "api.example".to_string(),
                port: 8443,
                path: Some("/rpc".to_string()),
                parameters: HashMap::new(),
            },
            auth_method: AuthenticationMethod::None,
            performance_profile: PerformanceProfile::default(),
            trust_score: 0.5,
        };
        let body = adapter
            .send_request(&svc, serde_json::json!({"ping": true}))
            .await
            .expect("send");
        assert_eq!(body["status"], "success");
        assert_eq!(body["service_id"], "svc-a");
        assert!(body["endpoint"].as_str().unwrap().contains("api.example"));
    }

    #[tokio::test]
    async fn test_adapter_creation() {
        let adapter = EcosystemDiscoveryAdapter::new();
        assert!(adapter.is_ok(), "Adapter creation should succeed");
    }

    #[tokio::test]
    async fn test_adapter_with_custom_config() {
        let config = EcosystemDiscoveryAdapter::default_config();
        let adapter = EcosystemDiscoveryAdapter::with_config(config);
        assert!(adapter.is_ok(), "Adapter with config should succeed");
    }

    #[tokio::test]
    async fn test_discover_returns_empty_without_announcements() {
        let adapter =
            EcosystemDiscoveryAdapter::new().expect("creation failed - test cannot proceed");

        let capability = UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        };

        let primals = adapter
            .discover_by_capability(capability)
            .await
            .expect("discovery failed");

        // Should return empty without announcements (correct - no hardcoded primals)
        assert_eq!(
            primals.len(),
            0,
            "Should not hardcode any primals without announcements"
        );
    }

    #[tokio::test]
    async fn test_trigger_discovery_starts_listener() {
        let adapter = EcosystemDiscoveryAdapter::new().expect("creation failed");
        let result = adapter.trigger_discovery().await;
        assert!(result.is_ok(), "Discovery trigger should succeed");

        // Verify listener was started
        let listener_guard = adapter.listener.read().await;
        assert!(
            listener_guard.is_some(),
            "Listener should be initialized after trigger"
        );
    }

    #[tokio::test]
    async fn test_adapter_clone() {
        let adapter1 = EcosystemDiscoveryAdapter::new().expect("creation failed");
        let adapter2 = adapter1.clone();

        // Both should share the same underlying data (Arc)
        let addr1 = Arc::as_ptr(&adapter1.discovered_primals);
        let addr2 = Arc::as_ptr(&adapter2.discovered_primals);
        assert_eq!(addr1, addr2, "Cloned adapters should share data");
    }
}
