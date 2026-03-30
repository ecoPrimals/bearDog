// SPDX-License-Identifier: AGPL-3.0-only

//! Advertised capabilities, endpoints, authentication, and security posture for discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::capability_type::CapabilityType;

/// Coarse security posture tier used when advertising or filtering discovered capabilities.
///
/// Higher tiers imply stricter controls (audit, crypto, isolation). Callers should treat this as
/// advisory unless paired with concrete policy checks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    /// Basic security level
    Basic,
    /// Standard security level
    #[default]
    Standard,
    /// High security level
    High,
    /// Critical security level
    Critical,
}

/// A single discoverable capability advertisement: who provides it, how to reach it, and health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalCapability {
    /// The capability type
    /// The capability type value
    pub capability_type: CapabilityType,
    /// Identity and classification of the provider offering this capability.
    pub provider: ProviderInfo,
    /// Endpoint configuration
    /// The endpoint value
    pub endpoint: EndpointConfig,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// Health status
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Observed latency, success rate, and load for this endpoint (best-effort, may be stale).
    pub performance: PerformanceMetrics,
    /// Security level
    /// The security level value
    pub security_level: SecurityLevel,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Describes who is advertising a capability (vendor, primal, or custom integration).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider identifier (discovered dynamically via capability-based discovery)
    pub provider_id: String,
    /// Human-readable provider name
    pub provider_name: String,
    /// Provider type (vendor, primal, custom)
    pub provider_type: crate::canonical::providers_unified::core::ProviderType,
    /// Provider version
    /// The version value
    pub version: String,
    /// Provider region or location
    /// Optional region
    pub region: Option<String>,
}

/// Connection parameters for reaching a capability provider (base URL, timeouts, retries).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// The base url value
    pub base_url: String,
    /// API version
    /// Optional api version
    pub api_version: Option<String>,
    /// Timeout configuration
    pub timeout_ms: u64,
    /// Retry configuration
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Circuit breaker configuration
    /// The circuit breaker value
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Open-circuit dwell time in milliseconds before a half-open trial is allowed.
    pub timeout_ms: u64,
    /// Number of `success_threshold`
    pub success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    /// Pure defaults without environment variable access
    /// Concurrent-safe and suitable for testing
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout_ms: 60000, // 60 seconds
            success_threshold: 3,
        }
    }
}

impl CircuitBreakerConfig {
    /// Load from environment or use defaults (production use)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            failure_threshold: get("BEARDOG_CIRCUIT_BREAKER_FAILURE_THRESHOLD")
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            timeout_ms: get("BEARDOG_CIRCUIT_BREAKER_TIMEOUT_MS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(60000), // 60 seconds
            success_threshold: get("BEARDOG_CIRCUIT_BREAKER_SUCCESS_THRESHOLD")
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
        }
    }
}

/// Client-side authentication material and method for calling a capability provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication type
    /// The auth type value
    pub auth_type: AuthType,
    /// API key (if applicable)
    /// Optional api key
    pub api_key: Option<String>,
    /// Bearer token (if applicable)
    /// Optional bearer token
    pub bearer_token: Option<String>,
    /// Certificate path (if applicable)
    /// Optional cert path
    pub cert_path: Option<String>,
    /// Additional auth parameters
    /// Mapping of custom params
    pub custom_params: HashMap<String, String>,
}

/// Authentication type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of auth
pub enum AuthType {
    /// No authentication required
    None,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    Bearer,
    /// Mutual TLS authentication
    MutualTLS,
    /// OAuth 2.0 authentication
    OAuth2,
    /// Custom authentication method
    Custom(String),
}

/// Liveness classification for a capability endpoint as observed by discovery or health checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Capability is healthy and available
    Healthy,
    /// Capability is degraded but functional
    Degraded,
    /// Capability is unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
}

/// Rolling operational metrics for a capability provider (used for ranking and circuit breaking).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Success rate (0.0 to 1.0)
    /// The success rate value
    pub success_rate: f64,
    /// Throughput (requests per second)
    /// The throughput rps value
    pub throughput_rps: f64,
    /// Current load (0.0 to 1.0)
    /// The current load value
    pub current_load: f64,
    /// Last updated timestamp
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            success_rate: 1.0,
            throughput_rps: 0.0,
            current_load: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Security-related feature flags and certifications advertised by a provider or system profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Authentication methods supported
    /// Collection of authentication methods
    pub authentication_methods: Vec<String>,
    /// Role-based access control enabled
    /// Whether rbac is enabled
    pub rbac: bool,
    /// Audit logging enabled
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Secure protocols supported
    /// Collection of secure protocols
    pub secure_protocols: Vec<String>,
    /// Compliance certifications held
    /// Collection of compliance certifications
    pub compliance_certifications: Vec<String>,
    /// Security level classification
    /// The security level value
    pub security_level: SecurityLevel,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            authentication_methods: vec![
                "password".to_string(),
                "biometric".to_string(),
                "hardware_key".to_string(),
            ],
            rbac: true,
            audit_logging: true,
            secure_protocols: vec!["TLS".to_string(), "HTTPS".to_string()],
            compliance_certifications: vec!["ISO27001".to_string()],
            security_level: SecurityLevel::High,
        }
    }
}
