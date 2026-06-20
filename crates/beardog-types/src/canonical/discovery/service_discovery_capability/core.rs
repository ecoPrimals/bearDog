// SPDX-License-Identifier: AGPL-3.0-or-later

//! Service discovery trait, descriptors, errors, and shared types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::future::Future;

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
pub trait ServiceDiscoveryCapability: Send + Sync + fmt::Debug {
    /// Discover services by capability type
    ///
    /// This is the primary discovery method - finds services based on what
    /// they can do, not who they are.
    ///
    /// # Arguments
    ///
    /// * `capability` - The service capability to discover (`ServiceMesh`, `DataStorage`, etc.)
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
    /// # async fn example(discovery: Arc<ServiceDiscoveryBackend>) -> Result<(), DiscoveryError> {
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
    fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> impl Future<Output = Result<Vec<ServiceDescriptor>, DiscoveryError>> + Send;

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
    fn discover_by_name(
        &self,
        service_name: &str,
    ) -> impl Future<Output = Result<Vec<ServiceDescriptor>, DiscoveryError>> + Send;

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
    fn register_service(
        &self,
        descriptor: ServiceDescriptor,
    ) -> impl Future<Output = Result<RegistrationId, DiscoveryError>> + Send;

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
    fn unregister_service(
        &self,
        registration_id: &RegistrationId,
    ) -> impl Future<Output = Result<(), DiscoveryError>> + Send;

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
    fn renew_registration(
        &self,
        registration_id: &RegistrationId,
    ) -> impl Future<Output = Result<(), DiscoveryError>> + Send;

    /// Check health of the discovery service itself
    ///
    /// Verifies that the discovery backend is reachable and operational.
    ///
    /// # Returns
    ///
    /// * `Ok(DiscoveryHealthStatus)` - Health information
    /// * `Err(DiscoveryError)` - If health check fails
    fn health_check(
        &self,
    ) -> impl Future<Output = Result<DiscoveryHealthStatus, DiscoveryError>> + Send;

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
    Degraded {
        /// Human-readable explanation (e.g. elevated error rate).
        reason: String,
    },

    /// Service is unhealthy, avoid if possible
    Unhealthy {
        /// Human-readable explanation (e.g. failed health check).
        reason: String,
    },

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
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum DiscoveryError {
    /// Service not found for given criteria
    #[error("Service not found: {criteria}")]
    ServiceNotFound {
        /// Query or filter that produced no matches.
        criteria: String,
    },

    /// Discovery backend unavailable
    #[error("Discovery backend '{provider}' unavailable: {reason}")]
    BackendUnavailable {
        /// Backend identifier (e.g. `consul`, `kubernetes`).
        provider: String,
        /// Why the backend could not be used.
        reason: String,
    },

    /// Invalid service descriptor
    #[error("Invalid service descriptor: {reason}")]
    InvalidDescriptor {
        /// Validation failure detail.
        reason: String,
    },

    /// Registration failed
    #[error("Service registration failed: {reason}")]
    RegistrationFailed {
        /// Provider-specific registration error.
        reason: String,
    },

    /// Network/connectivity error
    #[error("Network error: {details}")]
    NetworkError {
        /// Transport- or DNS-level failure description.
        details: String,
    },

    /// Timeout during discovery operation
    #[error("Operation '{operation}' timed out after {duration_ms}ms")]
    Timeout {
        /// Operation name that exceeded its deadline.
        operation: String,
        /// Observed wait time in milliseconds.
        duration_ms: u64,
    },

    /// Permission denied
    #[error("Permission denied for resource: {resource}")]
    PermissionDenied {
        /// Resource or action that was not allowed.
        resource: String,
    },

    /// Generic error
    #[error("Discovery error: {message}")]
    Other {
        /// Fallback error message.
        message: String,
    },
}
