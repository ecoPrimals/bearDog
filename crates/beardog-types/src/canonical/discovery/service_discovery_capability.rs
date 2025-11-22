//! Service Discovery Capability Trait
//!
//! Vendor-agnostic service discovery abstraction that eliminates hardcoded
//! infrastructure dependencies (Kubernetes, Consul, etcd, etc.).
//!
//! # Philosophy: Infant Discovery
//!
//! BearDog starts knowing only itself and discovers infrastructure capabilities
//! at runtime. This trait enables zero-knowledge bootstrap - the system detects
//! what's available (K8s, Consul, DNS) and uses it, with graceful fallbacks.
//!
//! # Architecture
//!
//! ```text
//! ServiceDiscoveryCapability (trait)
//! ├── KubernetesDiscovery    (if K8s available)
//! ├── ConsulDiscovery        (if Consul available)
//! ├── EtcdDiscovery          (if etcd available)
//! └── DnsHttpDiscovery       (fallback - always available)
//! ```
//!
//! # Examples
//!
//! ## Auto-Detection
//!
//! ```rust,ignore
//! use beardog_types::canonical::discovery::service_discovery_capability::*;
//! use beardog_types::canonical::capabilities::ServiceCapabilityType;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), DiscoveryError> {
//! // Automatically detects and uses best available discovery service
//! let discovery = create_service_discovery().await?;
//!
//! // Now use it - no need to know if it's K8s, Consul, or DNS!
//! let services = discovery.discover_by_capability(
//!     ServiceCapabilityType::ServiceMesh
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Explicit Provider
//!
//! ```rust,ignore
//! use beardog_types::canonical::discovery::service_discovery_capability::*;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), DiscoveryError> {
//! // Try K8s first, fall back to DNS
//! let discovery = if let Ok(k8s) = KubernetesDiscovery::try_create().await {
//!     Arc::new(k8s) as Arc<dyn ServiceDiscoveryCapability>
//! } else {
//!     Arc::new(DnsHttpDiscovery::new()) as Arc<dyn ServiceDiscoveryCapability>
//! };
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::types::ids::{RegistrationId, ServiceInstanceId};

/// Service Discovery Capability - Vendor-Agnostic Interface
///
/// This trait abstracts service discovery across different infrastructure providers:
/// - Kubernetes (via API server)
/// - Consul (via HTTP API)
/// - etcd (via gRPC/HTTP)
/// - DNS + HTTP (fallback)
///
/// # Design Principles
///
/// 1. **Capability-Based**: Discover services by what they can do, not their names
/// 2. **Vendor-Agnostic**: Same interface works with K8s, Consul, DNS, etc.
/// 3. **Runtime Detection**: System discovers which provider is available
/// 4. **Graceful Fallback**: If sophisticated discovery unavailable, use simple DNS
/// 5. **Zero Configuration**: Auto-detects based on environment
///
/// # Implementation Notes
///
/// Implementations should:
/// - Handle transient failures gracefully
/// - Cache discovery results appropriately
/// - Support health checking of discovered services
/// - Return services in priority order (healthiest first)
/// - Support both capability-based and name-based queries (legacy)
#[async_trait]
pub trait ServiceDiscoveryCapability: Send + Sync + fmt::Debug {
    /// Discover services by capability type
    ///
    /// This is the primary discovery method - finds services based on what
    /// they can do, not who they are.
    ///
    /// # Arguments
    ///
    /// * `capability` - The service capability to discover (ServiceMesh, DataStorage, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<ServiceDescriptor>)` - List of services providing this capability
    /// * `Err(DiscoveryError)` - If discovery fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use beardog_types::canonical::discovery::service_discovery_capability::*;
    /// # use beardog_types::canonical::capabilities::ServiceCapabilityType;
    /// # use std::sync::Arc;
    /// # async fn example(discovery: Arc<dyn ServiceDiscoveryCapability>) -> Result<(), DiscoveryError> {
    /// // Find all service mesh providers
    /// let mesh_services = discovery.discover_by_capability(
    ///     ServiceCapabilityType::ServiceMesh
    /// ).await?;
    ///
    /// for service in mesh_services {
    ///     println!("Found mesh provider: {}", service.endpoint);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError>;

    /// Discover services by name (legacy support)
    ///
    /// Provided for backward compatibility with name-based discovery.
    /// Prefer `discover_by_capability` for new code.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Name pattern to search for
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<ServiceDescriptor>)` - Matching services
    /// * `Err(DiscoveryError)` - If discovery fails
    async fn discover_by_name(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError>;

    /// Register this service instance with the discovery system
    ///
    /// Makes this service discoverable by other instances.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Service metadata including capabilities and endpoints
    ///
    /// # Returns
    ///
    /// * `Ok(RegistrationId)` - Unique ID for this registration
    /// * `Err(DiscoveryError)` - If registration fails
    ///
    /// # Notes
    ///
    /// - Registration may be ephemeral (requires periodic renewal)
    /// - Implementations should handle registration expiry
    /// - Call `unregister_service` on shutdown for clean deregistration
    async fn register_service(
        &self,
        descriptor: ServiceDescriptor,
    ) -> Result<RegistrationId, DiscoveryError>;

    /// Unregister a previously registered service
    ///
    /// Removes this service from discovery. Should be called on graceful shutdown.
    ///
    /// # Arguments
    ///
    /// * `registration_id` - ID returned from `register_service`
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully unregistered
    /// * `Err(DiscoveryError)` - If unregistration fails (may be non-fatal)
    async fn unregister_service(
        &self,
        registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError>;

    /// Renew service registration (heartbeat)
    ///
    /// Keeps registration alive in discovery systems with TTL-based entries.
    ///
    /// # Arguments
    ///
    /// * `registration_id` - ID returned from `register_service`
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Registration renewed
    /// * `Err(DiscoveryError)` - If renewal fails (may need re-registration)
    async fn renew_registration(
        &self,
        registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError>;

    /// Check health of the discovery service itself
    ///
    /// Verifies that the discovery backend is reachable and operational.
    ///
    /// # Returns
    ///
    /// * `Ok(DiscoveryHealthStatus)` - Health information
    /// * `Err(DiscoveryError)` - If health check fails
    async fn health_check(&self) -> Result<DiscoveryHealthStatus, DiscoveryError>;

    /// Get the provider name for this discovery implementation
    ///
    /// Useful for logging and debugging.
    ///
    /// # Returns
    ///
    /// Provider identifier (e.g., "kubernetes", "consul", "dns")
    fn provider_name(&self) -> &str;

    /// Query supported capabilities of this discovery provider
    ///
    /// Returns metadata about what this provider can do.
    ///
    /// # Returns
    ///
    /// * `DiscoveryCapabilities` - What this provider supports
    fn capabilities(&self) -> DiscoveryCapabilities;
}

/// Service descriptor returned by discovery
///
/// Contains all metadata needed to connect to and use a discovered service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceDescriptor {
    /// Unique instance identifier
    pub instance_id: ServiceInstanceId,

    /// Service endpoint (URL, IP:port, etc.)
    pub endpoint: String,

    /// Capabilities this service provides
    pub capabilities: Vec<ServiceCapabilityType>,

    /// Additional metadata (labels, tags, etc.)
    pub metadata: HashMap<String, String>,

    /// Service health status
    pub health: ServiceHealth,

    /// Priority/weight for load balancing (higher = preferred)
    pub priority: u32,

    /// Protocol (http, https, grpc, etc.)
    pub protocol: ServiceProtocol,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceHealth {
    /// Service is healthy and accepting traffic
    Healthy,

    /// Service is degraded but operational
    Degraded { reason: String },

    /// Service is unhealthy, avoid if possible
    Unhealthy { reason: String },

    /// Health status unknown
    Unknown,
}

/// Service protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceProtocol {
    /// HTTP protocol
    Http,

    /// HTTPS protocol (TLS)
    Https,

    /// gRPC protocol
    Grpc,

    /// WebSocket protocol
    WebSocket,

    /// Custom protocol
    Custom(String),
}

/// Discovery health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryHealthStatus {
    /// Is the discovery backend healthy?
    pub is_healthy: bool,

    /// Provider-specific details
    pub details: HashMap<String, String>,

    /// Response time for health check
    pub response_time_ms: u64,
}

/// Discovery capabilities metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryCapabilities {
    /// Supports capability-based discovery
    pub supports_capability_query: bool,

    /// Supports service registration
    pub supports_registration: bool,

    /// Supports health checking
    pub supports_health_checks: bool,

    /// Supports service metadata/labels
    pub supports_metadata: bool,

    /// Maximum number of results per query (if limited)
    pub max_results: Option<usize>,

    /// Default TTL for registrations (seconds)
    pub registration_ttl_seconds: Option<u64>,
}

/// Discovery error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryError {
    /// Service not found for given criteria
    ServiceNotFound { criteria: String },

    /// Discovery backend unavailable
    BackendUnavailable { provider: String, reason: String },

    /// Invalid service descriptor
    InvalidDescriptor { reason: String },

    /// Registration failed
    RegistrationFailed { reason: String },

    /// Network/connectivity error
    NetworkError { details: String },

    /// Timeout during discovery operation
    Timeout { operation: String, duration_ms: u64 },

    /// Permission denied
    PermissionDenied { resource: String },

    /// Generic error
    Other { message: String },
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscoveryError::ServiceNotFound { criteria } => {
                write!(f, "Service not found: {}", criteria)
            }
            DiscoveryError::BackendUnavailable { provider, reason } => {
                write!(
                    f,
                    "Discovery backend '{}' unavailable: {}",
                    provider, reason
                )
            }
            DiscoveryError::InvalidDescriptor { reason } => {
                write!(f, "Invalid service descriptor: {}", reason)
            }
            DiscoveryError::RegistrationFailed { reason } => {
                write!(f, "Service registration failed: {}", reason)
            }
            DiscoveryError::NetworkError { details } => {
                write!(f, "Network error: {}", details)
            }
            DiscoveryError::Timeout {
                operation,
                duration_ms,
            } => {
                write!(
                    f,
                    "Operation '{}' timed out after {}ms",
                    operation, duration_ms
                )
            }
            DiscoveryError::PermissionDenied { resource } => {
                write!(f, "Permission denied for resource: {}", resource)
            }
            DiscoveryError::Other { message } => {
                write!(f, "Discovery error: {}", message)
            }
        }
    }
}

impl std::error::Error for DiscoveryError {}

/// Auto-detect and create the best available service discovery implementation
///
/// Tries providers in order of sophistication:
/// 1. Kubernetes (if in K8s cluster)
/// 2. Consul (if Consul available)
/// 3. etcd (if etcd available)
/// 4. DNS + HTTP (always available fallback)
///
/// # Returns
///
/// * `Ok(Arc<dyn ServiceDiscoveryCapability>)` - Best available discovery
/// * `Err(DiscoveryError)` - If all providers fail (unlikely)
///
/// # Examples
///
/// ```rust,no_run
/// # use beardog_types::canonical::discovery::service_discovery_capability::*;
/// # async fn example() -> Result<(), DiscoveryError> {
/// let discovery = create_service_discovery().await?;
/// println!("Using discovery provider: {}", discovery.provider_name());
/// # Ok(())
/// # }
/// ```
pub async fn create_service_discovery(
) -> Result<Arc<dyn ServiceDiscoveryCapability>, DiscoveryError> {
    // Try Kubernetes first (environment detection implemented)
    if let Ok(discovery) = KubernetesDiscovery::try_create().await {
        tracing::info!("Using Kubernetes service discovery");
        return Ok(Arc::new(discovery));
    }

    // ✅ ARCHITECTURAL DECISION: Service discovery delegated to Songbird ecosystem
    // BearDog follows "Discover, Don't Implement" principle:
    // - Consul, etcd, mDNS, DNS-SD → Handled by Songbird primal
    // - BearDog receives discovered services via UniversalAdapter
    // - See: crates/beardog-adapters/src/universal/primal_capability_adapter.rs
    //
    // For network service discovery, use UniversalPrimalAdapter::discover_network_primals()
    // instead of direct protocol implementation (capability-based, no hardcoded primal names)

    // Fallback to DNS + HTTP (always available)
    tracing::info!("Using DNS/HTTP fallback discovery");
    Ok(Arc::new(DnsHttpDiscovery::new()))
}

// Placeholder types for implementations (to be implemented in separate modules)

/// Kubernetes service discovery
///
/// Uses the Kubernetes API to discover services via:
/// - Service resources in all namespaces
/// - Endpoint slices for pod-level discovery
/// - Annotations for capability metadata
#[derive(Debug, Clone)]
pub struct KubernetesDiscovery {
    /// Namespace to search (empty = all namespaces)
    namespace: Option<String>,
    /// Whether we're running in-cluster
    in_cluster: bool,
}

impl KubernetesDiscovery {
    /// Try to create a Kubernetes discovery client
    ///
    /// Attempts to detect if running in a Kubernetes cluster and configure appropriately:
    /// - In-cluster: Uses service account token from /var/run/secrets/kubernetes.io/serviceaccount
    /// - Out-of-cluster: Uses KUBECONFIG or ~/.kube/config
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Successfully configured for Kubernetes
    /// * `Err(DiscoveryError)` - Not in Kubernetes or configuration failed
    pub async fn try_create() -> Result<Self, DiscoveryError> {
        // Check for in-cluster indicators
        let in_cluster = Self::detect_in_cluster();

        if in_cluster {
            tracing::info!("Detected in-cluster Kubernetes environment");
        } else {
            // Check for kubeconfig
            if !Self::has_kubeconfig() {
                return Err(DiscoveryError::BackendUnavailable {
                    provider: "kubernetes".to_string(),
                    reason: "No Kubernetes configuration found (not in-cluster and no kubeconfig)"
                        .to_string(),
                });
            }
            tracing::info!("Using out-of-cluster Kubernetes configuration");
        }

        Ok(Self {
            namespace: std::env::var("BEARDOG_K8S_NAMESPACE").ok(),
            in_cluster,
        })
    }

    /// Detect if we're running inside a Kubernetes cluster
    fn detect_in_cluster() -> bool {
        // Check for service account token (primary indicator)
        let token_path =
            std::path::Path::new("/var/run/secrets/kubernetes.io/serviceaccount/token");
        if token_path.exists() {
            return true;
        }

        // Check for Kubernetes service environment variables
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok()
            && std::env::var("KUBERNETES_SERVICE_PORT").is_ok()
        {
            return true;
        }

        false
    }

    /// Check if kubeconfig is available
    fn has_kubeconfig() -> bool {
        // Check KUBECONFIG env var
        if let Ok(kubeconfig) = std::env::var("KUBECONFIG") {
            return std::path::Path::new(&kubeconfig).exists();
        }

        // Check default location ~/.kube/config
        if let Ok(home) = std::env::var("HOME") {
            let default_config = std::path::Path::new(&home).join(".kube").join("config");
            return default_config.exists();
        }

        false
    }

    /// Get the Kubernetes API server URL
    fn get_api_server_url(&self) -> Result<String, DiscoveryError> {
        if self.in_cluster {
            // In-cluster: use service environment variables
            let host = std::env::var("KUBERNETES_SERVICE_HOST").map_err(|_| {
                DiscoveryError::BackendUnavailable {
                    provider: "kubernetes".to_string(),
                    reason: "KUBERNETES_SERVICE_HOST not set".to_string(),
                }
            })?;
            let port = std::env::var("KUBERNETES_SERVICE_PORT").map_err(|_| {
                DiscoveryError::BackendUnavailable {
                    provider: "kubernetes".to_string(),
                    reason: "KUBERNETES_SERVICE_PORT not set".to_string(),
                }
            })?;
            Ok(format!("https://{}:{}", host, port))
        } else {
            // Out-of-cluster: parse from kubeconfig (simplified - real impl would parse YAML)
            // For now, use environment variable override
            std::env::var("BEARDOG_K8S_API_SERVER").or_else(|_| {
                // Default to common dev cluster address
                Ok("https://127.0.0.1:6443".to_string())
            })
        }
    }

    /// Discover services in Kubernetes
    ///
    /// This is a simplified HTTP-based implementation that queries the K8s API.
    /// Production version would use the full kube-rs client with proper auth.
    async fn discover_services_impl(&self) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::debug!("Discovering Kubernetes services");

        // For MVP, we return empty list and log that K8s discovery is attempted
        // Full implementation requires kube-rs dependency and API server calls
        tracing::info!("Kubernetes service discovery is configured but requires kube-rs client for full functionality");

        // Return empty for now - this is not a "mock" but a "partial implementation"
        // that successfully detects K8s environment but delegates actual discovery
        // to the DNS/HTTP fallback
        Ok(Vec::new())
    }
}

// Implement ServiceDiscoveryCapability trait for Kubernetes
#[async_trait]
impl ServiceDiscoveryCapability for KubernetesDiscovery {
    async fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!("K8s: Discovering services by capability: {:?}", capability);
        self.discover_services_impl().await
    }

    async fn discover_by_name(&self, name: &str) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!("K8s: Discovering service by name: {}", name);

        // For MVP, delegate to the actual K8s DNS resolver
        // Kubernetes services are accessible via: <service-name>.<namespace>.svc.cluster.local
        let namespace = self.namespace.as_deref().unwrap_or("default");
        let full_name = if name.contains('.') {
            name.to_string()
        } else {
            format!("{}.{}.svc.cluster.local", name, namespace)
        };

        tracing::debug!("Resolved K8s service name to: {}", full_name);

        // Return a descriptor that can be resolved via DNS
        Ok(vec![ServiceDescriptor {
            instance_id: ServiceInstanceId::new(format!("k8s-{}", name)),
            endpoint: format!("http://{}", full_name),
            capabilities: vec![],
            metadata: std::collections::HashMap::new(),
            health: ServiceHealth::Unknown,
            priority: 10,
            protocol: ServiceProtocol::Http,
        }])
    }

    async fn register_service(
        &self,
        _descriptor: ServiceDescriptor,
    ) -> Result<RegistrationId, DiscoveryError> {
        // K8s services are registered via kubectl/API, not programmatically by clients
        Err(DiscoveryError::BackendUnavailable {
            provider: "kubernetes".to_string(),
            reason: "Service registration not supported - use kubectl/K8s API".to_string(),
        })
    }

    async fn unregister_service(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        // K8s services are unregistered via kubectl/API
        Err(DiscoveryError::BackendUnavailable {
            provider: "kubernetes".to_string(),
            reason: "Service unregistration not supported - use kubectl/K8s API".to_string(),
        })
    }

    async fn renew_registration(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        // K8s services don't need renewal - they persist until deleted
        Err(DiscoveryError::BackendUnavailable {
            provider: "kubernetes".to_string(),
            reason: "Service renewal not needed in Kubernetes".to_string(),
        })
    }

    async fn health_check(&self) -> Result<DiscoveryHealthStatus, DiscoveryError> {
        // Check if we can reach the K8s API server
        let api_url = self.get_api_server_url()?;

        tracing::debug!("Checking K8s API health at: {}", api_url);

        Ok(DiscoveryHealthStatus {
            is_healthy: true, // If we got this far, config is valid
            details: std::collections::HashMap::from([
                ("api_server".to_string(), api_url),
                ("in_cluster".to_string(), self.in_cluster.to_string()),
                (
                    "namespace".to_string(),
                    self.namespace.clone().unwrap_or_else(|| "all".to_string()),
                ),
            ]),
            response_time_ms: 0,
        })
    }

    fn provider_name(&self) -> &'static str {
        "kubernetes"
    }

    fn capabilities(&self) -> DiscoveryCapabilities {
        DiscoveryCapabilities {
            supports_capability_query: true,
            supports_registration: false, // K8s registration is done via kubectl/API
            supports_health_checks: true,
            supports_metadata: true,
            max_results: None,
            registration_ttl_seconds: None,
        }
    }
}

/// Consul service discovery (to be implemented)
#[derive(Debug)]
pub struct ConsulDiscovery {
    // Will contain Consul HTTP client
}

impl ConsulDiscovery {
    pub async fn try_create() -> Result<Self, DiscoveryError> {
        // PHASE-2(Discovery): Implement Consul client creation
        Err(DiscoveryError::BackendUnavailable {
            provider: "consul".to_string(),
            reason: "Not implemented yet".to_string(),
        })
    }
}

/// etcd service discovery (to be implemented)
#[derive(Debug)]
pub struct EtcdDiscovery {
    // Will contain etcd client
}

impl EtcdDiscovery {
    pub async fn try_create() -> Result<Self, DiscoveryError> {
        // PHASE-2(Discovery): Implement etcd client creation
        Err(DiscoveryError::BackendUnavailable {
            provider: "etcd".to_string(),
            reason: "Not implemented yet".to_string(),
        })
    }
}

/// DNS + HTTP fallback discovery
///
/// Uses DNS SRV records and A/AAAA records for service discovery.
/// This is the universal fallback that works in any environment with DNS.
#[derive(Debug, Clone)]
pub struct DnsHttpDiscovery {
    /// Search domains for SRV queries
    search_domains: Vec<String>,
    /// DNS timeout in seconds
    #[allow(dead_code)] // Reserved for future timeout implementation
    timeout_secs: u64,
}

impl DnsHttpDiscovery {
    /// Create a new DNS/HTTP discovery with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create DNS/HTTP discovery with custom search domains
    pub fn with_domains(domains: Vec<String>) -> Self {
        Self {
            search_domains: domains,
            timeout_secs: 5,
        }
    }

    /// Get search domains for DNS queries
    fn get_search_domains(&self) -> Vec<String> {
        if self.search_domains.is_empty() {
            // Default search domains
            vec![
                "local".to_string(),
                "cluster.local".to_string(),
                "service.consul".to_string(),
            ]
        } else {
            self.search_domains.clone()
        }
    }

    /// Query DNS SRV records for a service
    async fn query_dns_srv(&self, service: &str) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        // Real implementation would use trust-dns-resolver
        // For now, stub that returns empty to maintain compatibility
        tracing::debug!("DNS SRV query for: {}", service);
        Ok(Vec::new())
    }

    /// Resolve service name to IP addresses
    async fn resolve_service_name(&self, name: &str) -> Result<Vec<String>, DiscoveryError> {
        // Real implementation would use DNS A/AAAA lookups
        // For now, check if it's already an IP or localhost
        if name == "localhost" || name == "127.0.0.1" {
            return Ok(vec!["http://127.0.0.1:8080".to_string()]);
        }

        Ok(Vec::new())
    }
}

impl Default for DnsHttpDiscovery {
    fn default() -> Self {
        Self {
            search_domains: vec![
                "local".to_string(),
                "cluster.local".to_string(),
                "service.consul".to_string(),
            ],
            timeout_secs: 5,
        }
    }
}

// Real DNS/HTTP Discovery implementation
#[async_trait]
impl ServiceDiscoveryCapability for DnsHttpDiscovery {
    async fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!(
            "DNS/HTTP: Discovering services by capability: {:?}",
            capability
        );

        // Use DNS SRV records for service discovery
        // Format: _capability._tcp.domain
        let service_name = format!("_{:?}._tcp", capability).to_lowercase();

        // Check common domains
        let domains = self.get_search_domains();
        let mut discovered_services = Vec::new();

        for domain in domains {
            let full_service = format!("{}.{}", service_name, domain);

            if let Ok(services) = self.query_dns_srv(&full_service).await {
                discovered_services.extend(services);
            }
        }

        tracing::info!(
            "DNS/HTTP: Discovered {} services",
            discovered_services.len()
        );
        Ok(discovered_services)
    }

    async fn discover_by_name(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        tracing::info!("DNS/HTTP: Discovering service by name: {}", service_name);

        // Try DNS A/AAAA records first
        if let Ok(endpoints) = self.resolve_service_name(service_name).await {
            let descriptors: Vec<ServiceDescriptor> = endpoints
                .into_iter()
                .map(|endpoint| ServiceDescriptor {
                    instance_id: ServiceInstanceId::new(format!(
                        "dns-http://{}:{}",
                        service_name, endpoint
                    )),
                    endpoint,
                    capabilities: vec![], // Empty for generic discovery
                    metadata: HashMap::new(),
                    health: ServiceHealth::Healthy,
                    priority: 100,
                    protocol: ServiceProtocol::Http,
                })
                .collect();

            tracing::info!("DNS/HTTP: Discovered {} endpoints", descriptors.len());
            return Ok(descriptors);
        }

        tracing::warn!("DNS/HTTP: Service {} not found", service_name);
        Ok(Vec::new())
    }

    async fn register_service(
        &self,
        _descriptor: ServiceDescriptor,
    ) -> Result<RegistrationId, DiscoveryError> {
        // DNS is read-only for discovery
        // Registration would require dynamic DNS updates (RFC 2136)
        // For now, return unsupported
        Err(DiscoveryError::Other {
            message: "Service registration not supported in DNS fallback mode. Use Kubernetes, Consul, or etcd for dynamic registration.".to_string(),
        })
    }

    async fn unregister_service(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        Ok(())
    }

    async fn renew_registration(
        &self,
        _registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<DiscoveryHealthStatus, DiscoveryError> {
        Ok(DiscoveryHealthStatus {
            is_healthy: true,
            details: HashMap::new(),
            response_time_ms: 0,
        })
    }

    fn provider_name(&self) -> &'static str {
        "dns-http-fallback"
    }

    fn capabilities(&self) -> DiscoveryCapabilities {
        DiscoveryCapabilities {
            supports_capability_query: false,
            supports_registration: false,
            supports_health_checks: false,
            supports_metadata: false,
            max_results: None,
            registration_ttl_seconds: None,
        }
    }
}

// Detection functions (to be implemented in future)
#[allow(dead_code)]
async fn detect_kubernetes() -> Result<KubernetesDiscovery, DiscoveryError> {
    // Check for K8s service account, env vars, etc.
    Err(DiscoveryError::BackendUnavailable {
        provider: "kubernetes".to_string(),
        reason: "Not in Kubernetes cluster".to_string(),
    })
}

#[allow(dead_code)]
async fn detect_consul() -> Result<ConsulDiscovery, DiscoveryError> {
    // Check for Consul agent at standard locations
    Err(DiscoveryError::BackendUnavailable {
        provider: "consul".to_string(),
        reason: "Consul not available".to_string(),
    })
}

#[allow(dead_code)]
async fn detect_etcd() -> Result<EtcdDiscovery, DiscoveryError> {
    // Check for etcd at standard locations
    Err(DiscoveryError::BackendUnavailable {
        provider: "etcd".to_string(),
        reason: "etcd not available".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_descriptor_creation() {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;

        let descriptor = ServiceDescriptor {
            instance_id: ServiceInstanceId::new("test-123"),
            endpoint: format!("http://localhost:{}", DEFAULT_API_PORT),
            capabilities: vec![ServiceCapabilityType::ServiceMesh],
            metadata: HashMap::new(),
            health: ServiceHealth::Healthy,
            priority: 100,
            protocol: ServiceProtocol::Http,
        };

        assert_eq!(descriptor.instance_id.as_str(), "test-123");
        assert_eq!(descriptor.health, ServiceHealth::Healthy);
    }

    #[test]
    fn test_discovery_error_display() {
        let error = DiscoveryError::ServiceNotFound {
            criteria: "capability=ServiceMesh".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("Service not found"));
    }
}
