

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::primal_registry::{global_registry, PrimalId, PrimalRegistration};
use super::traits::*;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    target_primal: Arc<RwLock<Option<PrimalRegistration>>>,

    client: reqwest::Client,

    connection_state: Arc<RwLock<ConnectionState>>,

#[derive(Debug, Clone)]
    /// Optional last connected
    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,

    /// Optional last error
    pub last_error: Option<String>,

    /// The metrics value
    pub metrics: ConnectionMetrics,

#[derive(Debug, Clone)]
    /// Number of successful_requests
    pub successful_requests: u64,

    /// Number of failed_requests
    pub failed_requests: u64,


    pub avg_response_time_ms: u64,}

impl UniversalAdapter {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: UniversalAdapterConfig) -> Result<Self, BearDogError> {

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to create HTTP client: {}e"),
            })?;
        let connection_state = Arc::new(RwLock::new(false,
            last_connected: None,
            last_error: None,
            metrics: ConnectionMetrics::default(),
        }));
        Ok(Self {
            config,
            target_primal: Arc::new(RwLock::new(None)),
            client,
            connection_state,
        })
    }

/// Connect operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn connect(&self) -> Result<(), BearDogError> {

        let registry = global_registry();
        let primal_registration = registry
            .discover_primal(&self.config.target_primal.id)
            ?;
        if let Some(registration) = primal_registration {

            {
                let mut target_primal = self.target_primal.write();
                *target_primal = Some(registration);
            }

            let health_endpoint = format!("{}/health", registration.discovery_endpoint);
            let response = self.client.get(&health_endpoint).send();
            match response {
                Ok(resp) if resp.status().is_success() => {
                    let mut state = self.connection_state.write();
                    state.connected = true;
                    state.last_connected = Some(chrono::Utc::now({}", resp.status());
                    state.connected = false;
                    state.last_error = Some(error);
                    Err(BearDogError::network(error ))
                Err(e) => {
                    let error = format!("Connection failed: {e}");
        } else {
            Err(BearDogError::not_found(format!(
                    "Primal '}' not found in registry",
                    self.config.target_primal.id
                ),
            })
        }

/// Send Request operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn send_request(&self, request: ServiceRequest) -> Result<ServiceResponse, BearDogError> {

        {
            let state = self.connection_state.read();
            if !state.connected {
                return Err(BearDogError::network("Not connected to target primal"));

        let endpoint = {
            let target_primal = self.target_primal.read();
            if let Some(ref registration) = *target_primal {
                &registration.discovery_endpoint
            } else {
                return Err(BearDogError::not_found("Target primal not registered"));
        };

        let request_url = format!("{endpoint}/api/v1/request");
        let mut http_request = self.client.post(&request_url);

        match &self.config.auth.auth_type {
            AuthType::ApiKey => {
                if let Some(ref api_key) = self.config.auth.api_key {
                    http_request = http_request.header("X-API-Key", api_key);
            AuthType::TlsCert => {

            AuthType::Custom(_custom_type) => {

                for (key, value) in &self.config.auth.custom_auth {
                    http_request = http_request.header(key, value);
            AuthType::None => {

        let start_time = std::time::Instant::now();
        let response = http_request.json(&request).send();
        let response_time = start_time.elapsed().as_millis() as u64;

            let mut state = self.connection_state.write();
            state.metrics.total_requests += 1;

            let current_avg = state.metrics.avg_response_time_ms;
            let total_requests = state.metrics.total_requests;
            state.metrics.avg_response_time_ms =
                (current_avg * (total_requests - 1) + response_time) / total_requests;
        match response {
            Ok(resp) if resp.status().is_success() => {
                let mut state = self.connection_state.write();
                state.metrics.successful_requests += 1;
                drop(state);
                let service_response: ServiceResponse =
                    resp.json().map_err(|e| BearDogError::Serialization {
                        message: format!("Failed to deserialize response: {e}"),
                    })?;
                Ok({}", resp.status());
                state.last_error = Some(error);
                Err(BearDogError::network(error ))
            Err(e) => {
                let error = format!("Network error: {e}");

/// Get Connection State operation.
    /// Gets connection_state
    /// Gets connection_state
    pub fn get_connection_state(PrimalId,
        endpoint: &str,
        auth: UnifiedAuthConfig,
    ) -> Result<UniversalAdapter, BearDogError> {
        let config = UniversalAdapterConfig {
            target_primal,
            endpoint,
            auth,
            timeout_seconds: 30,
            max_retries: 3,
            custom_config: HashMap::with_capacity(16),
        UniversalAdapter::new(&str,
        let auth = AuthConfig {
            auth_type: AuthType::ApiKey,
            api_key: Some(None,
            custom_auth: HashMap::with_capacity(16),
        Self::create_adapter_legacy(endpoint, auth)

/// Create capability adapter using unified discovery
    /// Creates capability_adapter
    /// Creates capability_adapter
    pub fn create_capability_adapter(
        capability_type: CapabilityType,
    ) -> Result<Self, BearDogError> {
        info!("🔧 Creating capability adapter for: {:?}", capability_type);
        
        let mut adapter = Self::new()?;
        
        // Use capability-based discovery instead of hardcoded integrations
        let discovered_capabilities = adapter.discovery_engine
            .discover_capabilities(vec![capability_type.clone()])
            ?;
            
        if discovered_capabilities.is_empty() {
            return Err(BearDogError::validation(format!(
                "No providers found for capability: {:?}", 
                capability_type
            )));
        }
        
        // Register discovered capabilities
        for capability in discovered_capabilities {
            adapter.register_capability_handler(capability)?;
        }
        
        info!("✅ Capability adapter created successfully");
        Ok(adapter)
    }

    /// Register capability handler
    fn register_capability_handler(
        &mut self, 
        capability: DiscoveredCapability
    ) -> Result<(), BearDogError> {
        // Implementation would register appropriate handlers
        // based on discovered capability metadata
        Ok(())
    }

    /// Creates universal_adapter
    /// Creates universal_adapter
    pub fn create_universal_adapter() -> Result<Self, BearDogError> {
        info!("🌐 Creating universal ecosystem adapter");
        
        let mut adapter = Self::new()?;
        
        // Discover all available capabilities in the ecosystem
        let all_capabilities = vec![
            CapabilityType::ServiceMesh,
            CapabilityType::DataStorage,
            CapabilityType::ComputeIntelligence,
            CapabilityType::DistributedIntelligence,
            CapabilityType::Security,
            CapabilityType::Network,
        ];
        
        let discovered = adapter.discovery_engine
            .discover_capabilities(all_capabilities)
            ?;
            
        for capability in discovered {
            adapter.register_capability_handler(capability)?;
        }
        
        info!("✅ Universal adapter created with {} capabilities", 
              adapter.capability_handlers.len());
        Ok(adapter)
    }

    /// DEPRECATED: Use create_capability_adapter instead
    #[deprecated(note = "Use create_capability_adapter for capability-based discovery")]
    /// Creates adapter_legacy
    /// Creates adapter_legacy
    pub fn create_adapter_legacy(
        endpoint: &str,
        auth: AuthConfig,
    ) -> Result<Self, BearDogError> {
        warn!("🚨 Using deprecated hardcoded adapter creation - migrate to capability-based discovery");
        
        // Create universal adapter with capability discovery
        let mut adapter = Self::new()?;
        
        // Try to infer capabilities from endpoint
        let inferred_capabilities = Self::infer_capabilities_from_endpoint(endpoint)?;
        
        for capability in inferred_capabilities {
            adapter.register_capability_handler(DiscoveredCapability {
                capability_type: capability,
                endpoint: endpoint.to_string(),
                provider_id: format!("legacy-{}", uuid::Uuid::new_v4()),
                metadata: HashMap::new(),
                health_status: HealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            })?;
        }
        
        Ok(adapter)
    }
    
    /// Infer capabilities from endpoint URL patterns
    fn infer_capabilities_from_endpoint(endpoint: &str) -> Result<Vec<CapabilityType>, BearDogError> {
        let mut capabilities = vec![];
        
        // Pattern matching for common service types
        if endpoint.contains(ServiceCapabilityType::ServiceMesh) || endpoint.contains("mesh") {
            capabilities.push(CapabilityType::ServiceMesh);
        }
        if endpoint.contains(ServiceCapabilityType::ComputeIntelligence) || endpoint.contains("compute") {
            capabilities.push(CapabilityType::ComputeIntelligence);
        }
        if endpoint.contains(ServiceCapabilityType::DistributedIntelligence) || endpoint.contains("ai") {
            capabilities.push(CapabilityType::DistributedIntelligence);
        }
        if endpoint.contains(ServiceCapabilityType::DataStorage) || endpoint.contains("storage") {
            capabilities.push(CapabilityType::DataStorage);
        }
        
        // Default to universal capability if no specific pattern matched
        if capabilities.is_empty() {
            capabilities.push(CapabilityType::Universal);
        }
        
        Ok(capabilities)
    }

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]}


    fn test_universal_adapter_factory() {
            api_key: Some("test-key".to_string()),
        let adapter = UniversalAdapterFactory::create_capability_adapter(
            CapabilityType::ServiceMesh)
        ;
        assert!(adapter.is_ok(AuthType::None,
            api_key: None,
        let adapter = UniversalAdapterFactory::create_custom_adapter(
            "my-custom-ai",
            "MyCustomAI",
            "https://my-custom-ai.example.com".to_string(),
