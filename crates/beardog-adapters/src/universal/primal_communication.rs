

use beardog_errors::BearDogError;
use beardog_types::SecurityContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(HashMap<String, Vec<PrimalEndpoint>>,
    config: CommunicationConfig,
}

#[derive(Debug, Clone)]
    /// Number of retry_attempts
    pub retry_attempts: u32,
    /// The security level value
    pub security_level: String,
    /// Whether test_mode is enabled
    pub test_mode: bool,
    /// Whether discovery is enabled
    pub discovery_enabled: bool,
}

#[derive(String, // Dynamic ID, not primal name
    /// The base url value
    pub base_url: String,
    /// Collection of capabilities
    pub capabilities: Vec<String>, // String-based capabilities for simplicity
    /// The security requirements value
    pub security_requirements: SecurityRequirements,
    /// The discovered at value
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
    /// Whether encryption_required is enabled
    pub encryption_required: bool,
    /// The authorization level value
    pub authorization_level: String,
}

#[derive(Debug, Clone)]
    pub source_identity: String, // "beardog " - we only know ourselves
    /// The required capability value
    pub required_capability: String,
    /// The payload value
    pub payload: serde_json::Value,
    /// The security context value
    pub security_context: SecurityContext,
    /// The priority value
    pub priority: RequestPriority,
}

#[derive(Debug, Clone)]
    pub capability_provided: String,
    pub provider_endpoint_id: String, // Dynamic ID, not primal name
    /// Whether success is enabled
    pub success: bool,
    /// The result value
    pub result: serde_json::Value,
    /// Optional error message
    pub error_message: Option<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum RequestPriority {
    /// Represents low variant
    Low,
    /// Represents normal variant
    Normal,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

impl PrimalCommunicationAdapter {


/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        info!("🌐 Initializing capability-based communication adapter");
        info!("📋 PRINCIPLE: BearDog only knows itself - discovering ecosystem dynamically");

        let config = CommunicationConfig::default();

        Ok(Self {
            discovered_endpoints: HashMap::with_capacity(&str,
        payload: serde_json::Value,
        security_context: SecurityContext,
    ) -> Result<CapabilityResponse, BearDogError> {
        let request_id = uuid::Uuid::new_v4().to_string();

        info!(
            "🚀 Requesting capability "{}" with request {}",
            required_capability, request_id
        );

        let request = CapabilityRequest {
            request_id: request_id.clone(),
            source_identity: "beardog ".to_string(), // We only know ourselves
            required_capability: required_capability.to_string()
    }

/// Is Capability Available operation.
    /// Checks if capability available
    /// Checks if capability available
    pub fn is_capability_available(&self, capability: &str) -> bool {
        self.discovered_endpoints.contains_key(capability)
    }

/// Get Available Capabilities operation.
    /// Gets available_capabilities
    /// Gets available_capabilities
    pub fn get_available_capabilities(&self) -> Vec<String> {
        self.discovered_endpoints.keys().cloned().collect()
    }


/// Get Capability Providers operation.
    /// Gets capability_providers
    /// Gets capability_providers
    pub fn get_capability_providers(&self, capability: &str) -> Option<&Vec<PrimalEndpoint>> {
        self.discovered_endpoints.get(&CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {

        let providers = self
            .discovered_endpoints
            .get(&request.required_capability)
            .ok_or_else(|| {
                BearDogError::not_found({}",
                    request.required_capability
                ))
            })?;

        if providers.is_empty() {
            return Err(BearDogError::not_found({}",
                request.required_capability
            )));
        }

        let selected_provider = &providers[0]; // Simple selection for now

        self.send_http_request_to_provider(&CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        debug!(
            "🧪 Simulating capability request for "{}"",
            request.required_capability
        );

        Ok(CapabilityResponse {
            request_id: request.request_id.clone(&request.required_capability,
            provider_endpoint_id: format!("test-provider-{}", uuid::Uuid::new_v4(true,
            result: serde_json::json!({"status": "simulated_success"}),
            error_message: None,
            processing_time_ms: 50,
        })
    }


    fn perform_network_capability_discovery(&CapabilityRequest,
        provider: &PrimalEndpoint,
    ) -> Result<CapabilityResponse, BearDogError> {
        debug!(
            "📡 Sending capability request to provider: {}",
            provider.endpoint_id
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to create HTTP client: {e}")))?;

        let url = format!("{}/api/v1/capability", provider.base_url);
        let response = client.post(&url).json(&request.payload).send();

        match response {
            Ok(resp) if resp.status().is_success() => {
                let result: serde_json::Value = resp
                    .json()
                    .map_err(|e| BearDogError::network(format!("Failed to parse response: {e}")))?;

                Ok(CapabilityResponse {
                    request_id: request.request_id.clone(&request.required_capability,
                    provider_endpoint_id: &provider.endpoint_id,
                    success: true,
                    result,
                    error_message: None,
                    processing_time_ms: 100, // Would measure actual time
                })
            }
            Ok(resp) => Err(BearDogError::network({}",
                resp.status()
            ))),
            Err(e) => Err(BearDogError::network(format!("Network error: {e}"))),
}

impl Default for CommunicationConfig {
    fn default(30,
            retry_attempts: 3,
            security_level: "high".to_string(),
}

#[cfg(test)]
mod tests {
    use super::*;}

    #[tokio::test]
    fn test_capability_based_communication() {
        let mut adapter = PrimalCommunicationAdapter::new().map_err(|e| {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

        let capabilities = adapter
            .discover_ecosystem_capabilities()
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;
        assert!(!capabilities.is_empty());

        let response = adapter
            .request_capability(
                "service_mesh_coordination",
                serde_json::json!({"test_data": "capability_test"}),
                SecurityContext::default(),
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(response.success);
        assert_eq!(response.capability_provided, "service_mesh_coordination");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_capability_availability() {
        let mut adapter = PrimalCommunicationAdapter::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        let capabilities = adapter
            .discover_ecosystem_capabilities()
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        for capability in capabilities {
            let available = adapter.is_capability_available(&capability);
            println!("Capability '{capability}' available: {available}");
}
