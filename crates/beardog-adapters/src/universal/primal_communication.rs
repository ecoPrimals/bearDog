//! # Primal Communication Adapter
//!
//! This module provides capability-based communication for inter-primal
//! communication. Following the principle that BearDog only knows itself
//! and discovers other primals dynamically through their capabilities.

use beardog_errors::BearDogError;
use beardog_types::SecurityContext;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

// ============================================================
// Configuration
// ============================================================

/// Communication configuration
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// Request timeout in seconds
    pub timeout_seconds: u64,

    /// Number of retry attempts
    pub retry_attempts: u32,

    /// Security level (low, medium, high)
    pub security_level: String,

    /// Whether running in test mode
    pub test_mode: bool,

    /// Whether discovery is enabled
    pub discovery_enabled: bool,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            retry_attempts: 3,
            security_level: "high".to_string(),
            test_mode: false,
            discovery_enabled: true,
        }
    }
}

// ============================================================
// Endpoint Types
// ============================================================

/// Security requirements for an endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Whether encryption is required
    pub encryption_required: bool,

    /// Authorization level required
    pub authorization_level: String,

    /// Minimum trust level
    pub min_trust_level: Option<String>,
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            encryption_required: true,
            authorization_level: "standard".to_string(),
            min_trust_level: None,
        }
    }
}

/// Primal endpoint discovered in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEndpoint {
    /// Dynamic endpoint ID (not the primal name)
    pub endpoint_id: String,

    /// Base URL for the endpoint
    pub base_url: String,

    /// Capabilities provided by this endpoint
    pub capabilities: Vec<String>,

    /// Security requirements
    pub security_requirements: SecurityRequirements,

    /// When this endpoint was discovered
    pub discovered_at: DateTime<Utc>,
}

// ============================================================
// Request/Response Types
// ============================================================

/// Priority for capability requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestPriority {
    /// Low priority
    Low,

    /// Normal priority
    Normal,

    /// High priority
    High,

    /// Critical priority
    Critical,
}

impl Default for RequestPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Capability request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    /// Unique request ID
    pub request_id: String,

    /// Source identity (we only know ourselves: "beardog")
    pub source_identity: String,

    /// Required capability
    pub required_capability: String,

    /// Request payload
    pub payload: serde_json::Value,

    /// Security context
    pub security_context: SecurityContext,

    /// Request priority
    pub priority: RequestPriority,
}

/// Capability response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    /// Request ID (correlation)
    pub request_id: String,

    /// Capability that was provided
    pub capability_provided: String,

    /// Provider endpoint ID (dynamic, not primal name)
    pub provider_endpoint_id: String,

    /// Whether the request succeeded
    pub success: bool,

    /// Result payload
    pub result: serde_json::Value,

    /// Error message if failed
    pub error_message: Option<String>,

    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

// ============================================================
// Communication Adapter
// ============================================================

/// Capability-based communication adapter
#[derive(Debug)]
pub struct PrimalCommunicationAdapter {
    /// Discovered endpoints by capability
    discovered_endpoints: HashMap<String, Vec<PrimalEndpoint>>,

    /// Configuration
    config: CommunicationConfig,
}

impl PrimalCommunicationAdapter {
    /// Create a new communication adapter
    pub fn new() -> Result<Self, BearDogError> {
        info!("🌐 Initializing capability-based communication adapter");
        info!("📋 PRINCIPLE: BearDog only knows itself - discovering ecosystem dynamically");

        let config = CommunicationConfig::default();

        Ok(Self {
            discovered_endpoints: HashMap::with_capacity(16),
            config,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: CommunicationConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            discovered_endpoints: HashMap::with_capacity(16),
            config,
        })
    }

    /// Discover ecosystem capabilities
    pub fn discover_ecosystem_capabilities(&mut self) -> Result<Vec<String>, BearDogError> {
        info!("🔍 Discovering ecosystem capabilities");

        if self.config.test_mode {
            // In test mode, simulate discovered capabilities
            self.simulate_capability_discovery()?;
        } else if self.config.discovery_enabled {
            self.perform_network_capability_discovery()?;
        }

        let capabilities: Vec<String> = self.discovered_endpoints.keys().cloned().collect();
        info!("✅ Discovered {} capabilities", capabilities.len());

        Ok(capabilities)
    }

    /// Request a capability
    pub fn request_capability(
        &self,
        required_capability: &str,
        payload: serde_json::Value,
        security_context: SecurityContext,
    ) -> Result<CapabilityResponse, BearDogError> {
        let request_id = uuid::Uuid::new_v4().to_string();

        info!(
            "🚀 Requesting capability '{}' with request {}",
            required_capability, request_id
        );

        let request = CapabilityRequest {
            request_id: request_id.clone(),
            source_identity: "beardog".to_string(), // We only know ourselves
            required_capability: required_capability.to_string(),
            payload,
            security_context,
            priority: RequestPriority::Normal,
        };

        if self.config.test_mode {
            return self.simulate_capability_response(&request);
        }

        self.route_capability_request(&request)
    }

    /// Check if a capability is available
    pub fn is_capability_available(&self, capability: &str) -> bool {
        self.discovered_endpoints.contains_key(capability)
    }

    /// Get all available capabilities
    pub fn get_available_capabilities(&self) -> Vec<String> {
        self.discovered_endpoints.keys().cloned().collect()
    }

    /// Get providers for a capability
    pub fn get_capability_providers(&self, capability: &str) -> Option<&Vec<PrimalEndpoint>> {
        self.discovered_endpoints.get(capability)
    }

    /// Register an endpoint
    pub fn register_endpoint(&mut self, capability: &str, endpoint: PrimalEndpoint) {
        self.discovered_endpoints
            .entry(capability.to_string())
            .or_insert_with(Vec::new)
            .push(endpoint);
    }

    // Private methods

    fn simulate_capability_discovery(&mut self) -> Result<(), BearDogError> {
        debug!("🧪 Simulating capability discovery in test mode");

        // Simulate discovered capabilities
        let capabilities = vec![
            "service_mesh_coordination",
            "cryptographic_operations",
            "identity_management",
            "secure_storage",
        ];

        for capability in capabilities {
            let endpoint = PrimalEndpoint {
                endpoint_id: format!("test-endpoint-{}", uuid::Uuid::new_v4()),
                base_url: "http://localhost:8080".to_string(),
                capabilities: vec![capability.to_string()],
                security_requirements: SecurityRequirements::default(),
                discovered_at: Utc::now(),
            };
            self.register_endpoint(capability, endpoint);
        }

        Ok(())
    }

    fn perform_network_capability_discovery(&mut self) -> Result<(), BearDogError> {
        debug!("📡 Performing network capability discovery");
        // In production, this would perform actual network discovery
        // For now, we start with an empty set
        Ok(())
    }

    fn route_capability_request(
        &self,
        request: &CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        let providers = self
            .discovered_endpoints
            .get(&request.required_capability)
            .ok_or_else(|| {
                BearDogError::not_found(format!(
                    "No providers found for capability: {}",
                    request.required_capability
                ))
            })?;

        if providers.is_empty() {
            return Err(BearDogError::not_found(format!(
                "No providers available for capability: {}",
                request.required_capability
            )));
        }

        // Select best provider (simple selection for now)
        let selected_provider = &providers[0];

        self.send_request_to_provider(request, selected_provider)
    }

    fn simulate_capability_response(
        &self,
        request: &CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        debug!(
            "🧪 Simulating capability request for '{}'",
            request.required_capability
        );

        Ok(CapabilityResponse {
            request_id: request.request_id.clone(),
            capability_provided: request.required_capability.clone(),
            provider_endpoint_id: format!("test-provider-{}", uuid::Uuid::new_v4()),
            success: true,
            result: serde_json::json!({"status": "simulated_success"}),
            error_message: None,
            processing_time_ms: 50,
        })
    }

    fn send_request_to_provider(
        &self,
        request: &CapabilityRequest,
        provider: &PrimalEndpoint,
    ) -> Result<CapabilityResponse, BearDogError> {
        debug!(
            "📡 Sending capability request to provider: {}",
            provider.endpoint_id
        );

        // Build HTTP client
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to create HTTP client: {}", e)))?;

        let url = format!("{}/api/v1/capability", provider.base_url);

        let response = client.post(&url).json(&request.payload).send();

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: serde_json::Value = resp
                    .json()
                    .map_err(|e| BearDogError::network(format!("Failed to parse response: {}", e)))?;

                Ok(CapabilityResponse {
                    request_id: request.request_id.clone(),
                    capability_provided: request.required_capability.clone(),
                    provider_endpoint_id: provider.endpoint_id.clone(),
                    success: true,
                    result,
                    error_message: None,
                    processing_time_ms: 100, // Would measure actual time
                })
            }
            Ok(resp) => Err(BearDogError::network(format!(
                "Request failed with status: {}",
                resp.status()
            ))),
            Err(e) => Err(BearDogError::network(format!("Network error: {}", e))),
        }
    }
}

impl Default for PrimalCommunicationAdapter {
    fn default() -> Self {
        // SAFETY: new() only creates a HashMap and default config, which cannot fail.
        // If it somehow fails, we create a minimal fallback to avoid panicking.
        Self::new().unwrap_or_else(|e| {
            tracing::error!(
                "Unexpected failure creating PrimalCommunicationAdapter: {}. Using minimal fallback.",
                e
            );
            Self {
                discovered_endpoints: std::collections::HashMap::new(),
                config: CommunicationConfig::default(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = CommunicationConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.security_level, "high");
    }

    #[test]
    fn test_adapter_creation() {
        let adapter = PrimalCommunicationAdapter::new();
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_capability_discovery_test_mode() {
        let config = CommunicationConfig {
            test_mode: true,
            ..Default::default()
        };
        let mut adapter = PrimalCommunicationAdapter::with_config(config).unwrap();

        let capabilities = adapter.discover_ecosystem_capabilities().unwrap();
        assert!(!capabilities.is_empty());
    }

    #[test]
    fn test_capability_availability() {
        let config = CommunicationConfig {
            test_mode: true,
            ..Default::default()
        };
        let mut adapter = PrimalCommunicationAdapter::with_config(config).unwrap();
        adapter.discover_ecosystem_capabilities().unwrap();

        assert!(adapter.is_capability_available("service_mesh_coordination"));
        assert!(!adapter.is_capability_available("nonexistent_capability"));
    }

    #[test]
    fn test_capability_request_test_mode() {
        let config = CommunicationConfig {
            test_mode: true,
            ..Default::default()
        };
        let mut adapter = PrimalCommunicationAdapter::with_config(config).unwrap();
        adapter.discover_ecosystem_capabilities().unwrap();

        let response = adapter.request_capability(
            "service_mesh_coordination",
            serde_json::json!({"test": true}),
            SecurityContext::default(),
        );

        assert!(response.is_ok());
        let resp = response.unwrap();
        assert!(resp.success);
        assert_eq!(resp.capability_provided, "service_mesh_coordination");
    }

    #[test]
    fn test_request_priority_default() {
        assert_eq!(RequestPriority::default(), RequestPriority::Normal);
    }

    #[test]
    fn test_security_requirements_default() {
        let req = SecurityRequirements::default();
        assert!(req.encryption_required);
        assert_eq!(req.authorization_level, "standard");
    }
}
