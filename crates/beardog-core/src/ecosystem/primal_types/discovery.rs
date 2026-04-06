// SPDX-License-Identifier: AGPL-3.0-or-later

//! Service discovery types for capability-based ecosystem integration.

use super::attestation::{AuthRequirements, SecurityAttestation};
use super::health::PrimalMetrics;
use super::identity::EndpointSecurityConfig;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// `ServiceDependency` represents capability-based dependencies
/// instead of hardcoded primal dependencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceDependency {
    /// State indicating required
    /// Required service capability dependency
    Required {
        /// The capability type required
        capability: ServiceCapabilityType,
        /// Minimum version requirement
        min_version: String,
        /// Reason for the requirement
        reason: String,
    },
    /// Optional service capability dependency
    Optional {
        /// The capability type
        capability: ServiceCapabilityType,
        /// Minimum version if present
        min_version: String,
        /// Reason for the optional dependency
        reason: String,
    },
}

/// Service metadata describing capabilities without hardcoded references
///
/// Contains all information about a service's capabilities, dependencies,
/// and endpoints for dynamic discovery and integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Unique service identifier
    pub service_id: Uuid,
    /// Collection of capabilities this service provides
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Collection of dependencies this service requires
    pub dependencies: Vec<ServiceDependency>,
    /// Service version string
    pub version: String,
    /// Service endpoint configuration
    pub endpoints: ServiceEndpoints,
}

/// Service endpoint configuration
///
/// Defines all endpoints exposed by a service for health checks,
/// metrics, primary operations, and optional admin/websocket interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Health check endpoint URL
    pub health: String,
    /// Metrics/telemetry endpoint URL
    pub metrics: String,
    /// Primary service endpoint URL
    pub primary: String,
    /// Optional admin interface endpoint
    pub admin: Option<String>,
    /// Optional websocket endpoint for real-time communication
    pub websocket: Option<String>,
}

/// Capability integration configuration
///
/// Standard capability types for ecosystem integration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityType {
    /// Security capability integration
    Security,
    /// Storage capability integration
    Storage,
    /// Compute capability integration
    Compute,
    /// Networking capability integration
    Networking,
    /// AI capability integration
    AI,
}

/// Capability Integration Configuration
///
/// Uses capability-based configuration instead of hardcoded primal flags,
/// enabling dynamic service integration based on discovered capabilities.
///
/// Evolved from multiple boolean fields to a set-based approach for better
/// idiomatic Rust and easier runtime capability management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityIntegrationConfig {
    /// Enabled standard capability types
    #[serde(default = "default_enabled_capabilities")]
    pub enabled_capabilities: std::collections::HashSet<CapabilityType>,
    /// Custom capability configurations mapping capability names to their config values
    pub custom_capabilities: HashMap<String, serde_json::Value>,
}

fn default_enabled_capabilities() -> std::collections::HashSet<CapabilityType> {
    [
        CapabilityType::Security,
        CapabilityType::Storage,
        CapabilityType::Networking,
    ]
    .into_iter()
    .collect()
}

impl Default for CapabilityIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled_capabilities: default_enabled_capabilities(),
            custom_capabilities: HashMap::new(),
        }
    }
}

impl CapabilityIntegrationConfig {
    /// Check if security capability is enabled
    #[must_use]
    pub fn has_security_capability(&self) -> bool {
        self.enabled_capabilities
            .contains(&CapabilityType::Security)
    }

    /// Check if storage capability is enabled
    #[must_use]
    pub fn has_storage_capability(&self) -> bool {
        self.enabled_capabilities.contains(&CapabilityType::Storage)
    }

    /// Check if compute capability is enabled
    #[must_use]
    pub fn has_compute_capability(&self) -> bool {
        self.enabled_capabilities.contains(&CapabilityType::Compute)
    }

    /// Check if networking capability is enabled
    #[must_use]
    pub fn has_networking_capability(&self) -> bool {
        self.enabled_capabilities
            .contains(&CapabilityType::Networking)
    }

    /// Check if AI capability is enabled
    #[must_use]
    pub fn has_ai_capability(&self) -> bool {
        self.enabled_capabilities.contains(&CapabilityType::AI)
    }
}

/// Note: `PrimalType` enum was removed as it violated primal sovereignty
///
/// Migration completed: Use capability-based discovery instead
/// ✅ SOVEREIGNTY COMPLIANCE: Use capability-based discovery instead of hardcoded primal names
/// - Replace `PrimalType::BiomeOS` with `ServiceCapabilityType::ContainerOrchestration` discovery
///
/// Each primal now only knows itself and discovers others through universal adapter.
/// NEW: Capability-based primal identification (replaces `PrimalType`)
///
/// This represents a discovered primal in the ecosystem without hardcoding
/// its name or making assumptions about its identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Unique identifier discovered during bootstrap
    pub primal_id: String,
    /// Self-reported capabilities (what this primal can do)
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Communication endpoint
    /// The endpoint value
    pub endpoint: UniversalEndpoint,
    /// Self-reported metadata
    /// The metadata value
    pub metadata: PrimalMetadata,
    /// Discovery timestamp
    /// The discovered at value
    pub discovered_at: std::time::SystemTime,
    /// The metrics value
    pub metrics: PrimalMetrics,
}

/// Primal metadata discovered during ecosystem bootstrap
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimalMetadata {
    /// Name of the display
    pub display_name: Option<String>,
    /// The version value
    pub version: String,
    /// Supported protocol versions
    /// Collection of protocol versions
    pub protocol_versions: Vec<String>,
    /// Security level and attestations
    /// Collection of security attestations
    pub security_attestations: Vec<SecurityAttestation>,
    /// Custom metadata fields
    /// Mapping of custom fields
    pub custom_fields: HashMap<String, String>,
    /// Capabilities provided by this primal (discovered dynamically)
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Dependencies on other capabilities (not hardcoded primal names)
    /// Collection of dependencies
    pub dependencies: Vec<ServiceDependency>,
    /// Supported communication protocols
    /// Collection of supported protocols
    pub supported_protocols: Vec<String>,
    /// Health check endpoint path
    /// The health check endpoint value
    pub health_check_endpoint: String,
    /// Metrics endpoint path
    /// The metrics endpoint value
    pub metrics_endpoint: String,
}

/// Universal endpoint configuration for primal services
///
/// Defines a dynamically discovered service endpoint with protocol support,
/// authentication requirements, and security configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversalEndpoint {
    /// Base URL (discovered dynamically, not hardcoded)
    pub url: String,
    /// Supported communication protocols (HTTP, gRPC, WebSocket, etc.)
    pub protocols: Vec<String>,
    /// Authentication requirements for accessing this endpoint
    pub auth_requirements: AuthRequirements,
    /// TLS and security configuration for secure communication
    pub security_config: EndpointSecurityConfig,
}
