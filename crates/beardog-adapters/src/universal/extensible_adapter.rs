//! # Extensible Universal Adapter
//!
//! This module provides an extensible adapter framework that supports
//! multiple type systems and protocols through trait-based abstractions.

use crate::universal::http_adapter::{ResponseStatus, UniversalRequest, UniversalResponse};
use crate::universal::service_registration::types::UniversalServiceRegistration;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================
// Configuration
// ============================================================

/// Configuration for the extensible adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensibleAdapterConfig {
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,

    /// Auto-discover types
    pub auto_discover_types: bool,

    /// Type system configurations
    pub type_system_configs: HashMap<String, serde_json::Value>,

    /// Protocol configurations
    pub protocol_configs: HashMap<String, serde_json::Value>,
}

impl Default for ExtensibleAdapterConfig {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 100,
            auto_discover_types: true,
            type_system_configs: HashMap::with_capacity(16),
            protocol_configs: HashMap::with_capacity(16),
        }
    }
}

// ============================================================
// Metrics
// ============================================================

/// Adapter metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdapterMetrics {
    /// Total requests
    pub total_requests: u64,

    /// Successful requests
    pub successful_requests: u64,

    /// Failed requests
    pub failed_requests: u64,

    /// Average response time in ms
    pub avg_response_time_ms: f64,

    /// Requests by type system
    pub requests_by_type: HashMap<String, u64>,

    /// Requests by protocol
    pub requests_by_protocol: HashMap<String, u64>,
}

// ============================================================
// Traits
// ============================================================

/// Type system handler trait
#[allow(async_fn_in_trait)]
pub trait TypeSystemHandler: Send + Sync {
    /// Get type system ID
    fn type_system_id(&self) -> &str;

    /// Get supported operations
    fn supported_operations(&self) -> Vec<String>;

    /// Check if handler can handle request
    fn can_handle(&self, request: &UniversalRequest) -> bool;

    /// Handle request
    fn handle_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError>;
}

/// Protocol adapter trait
#[allow(async_fn_in_trait)]
pub trait ProtocolAdapter: Send + Sync {
    /// Get protocol ID
    fn protocol_id(&self) -> &str;

    /// Check if adapter can handle endpoint
    fn can_handle_endpoint(&self, endpoint: &str) -> bool;

    /// Send request to endpoint
    fn send_request(
        &self,
        endpoint: &str,
        request: &UniversalRequest,
    ) -> Result<UniversalResponse, BearDogError>;
}

// ============================================================
// Extensible Adapter
// ============================================================

/// Extensible universal adapter
pub struct ExtensibleUniversalAdapter<T, P>
where
    T: TypeSystemHandler + Clone + Send + Sync + 'static,
    P: ProtocolAdapter + Clone + Send + Sync + 'static,
{
    /// Type handlers by ID
    type_handlers: HashMap<String, T>,

    /// Protocol adapters by ID
    protocol_adapters: HashMap<String, P>,

    /// Service registry
    service_registry: Arc<RwLock<HashMap<String, UniversalServiceRegistration>>>,

    /// Configuration
    config: ExtensibleAdapterConfig,

    /// Metrics
    metrics: Arc<RwLock<AdapterMetrics>>,
}

impl<T, P> ExtensibleUniversalAdapter<T, P>
where
    T: TypeSystemHandler + Clone + Send + Sync + 'static,
    P: ProtocolAdapter + Clone + Send + Sync + 'static,
{
    /// Create a new adapter
    pub fn new(config: ExtensibleAdapterConfig) -> Self {
        Self {
            type_handlers: HashMap::with_capacity(16),
            protocol_adapters: HashMap::with_capacity(16),
            service_registry: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
            metrics: Arc::new(RwLock::new(AdapterMetrics::default())),
        }
    }

    /// Register a type handler
    pub fn register_type_handler(&mut self, type_id: &str, handler: T) {
        self.type_handlers.insert(type_id.to_string(), handler);
    }

    /// Register a protocol adapter
    pub fn register_protocol_adapter(&mut self, protocol_id: &str, adapter: P) {
        self.protocol_adapters.insert(protocol_id.to_string(), adapter);
    }

    /// Process a request
    ///
    /// # Errors
    /// Returns an error if request processing fails.
    pub async fn process_request(
        &self,
        request: &UniversalRequest,
    ) -> Result<UniversalResponse, BearDogError> {
        let start_time = std::time::Instant::now();

        let handler = self.find_type_handler(request)?;
        let type_id = handler.type_system_id().to_string();

        let result = handler.handle_request(request);
        let elapsed = start_time.elapsed().as_millis() as u64;

        self.update_metrics(&type_id, elapsed, result.is_ok()).await;

        result
    }

    /// Forward request to external endpoint
    ///
    /// # Errors
    /// Returns an error if forwarding fails.
    pub fn forward_request(
        &self,
        endpoint: &str,
        request: &UniversalRequest,
    ) -> Result<UniversalResponse, BearDogError> {
        let protocol_adapter = self.find_protocol_adapter(endpoint)?;
        protocol_adapter.send_request(endpoint, request)
    }

    /// Register a service
    ///
    /// # Errors
    /// Returns an error if registration fails.
    pub async fn register_service(
        &self,
        metadata: UniversalServiceRegistration,
    ) -> Result<(), BearDogError> {
        let service_id = metadata.service_id.clone();
        let mut registry = self.service_registry.write().await;
        registry.insert(service_id.clone(), metadata);
        tracing::info!("Registered service: {}", service_id);
        Ok(())
    }

    /// Discover services by capability
    ///
    /// # Errors
    /// Returns an error if discovery fails.
    pub async fn discover_services(
        &self,
        capability: &str,
    ) -> Result<Vec<UniversalServiceRegistration>, BearDogError> {
        let registry = self.service_registry.read().await;
        let services: Vec<UniversalServiceRegistration> = registry
            .values()
            .filter(|service| service.capabilities.iter().any(|cap| cap == capability))
            .cloned()
            .collect();
        Ok(services)
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> AdapterMetrics {
        self.metrics.read().await.clone()
    }

    // Private methods

    fn find_type_handler(&self, request: &UniversalRequest) -> Result<&T, BearDogError> {
        for handler in self.type_handlers.values() {
            if handler.can_handle(request) {
                return Ok(handler);
            }
        }
        Err(BearDogError::not_found(format!(
            "No type handler for operation: {}",
            request.operation
        )))
    }

    fn find_protocol_adapter(&self, endpoint: &str) -> Result<&P, BearDogError> {
        for adapter in self.protocol_adapters.values() {
            if adapter.can_handle_endpoint(endpoint) {
                return Ok(adapter);
            }
        }
        Err(BearDogError::not_found(format!(
            "No protocol adapter for endpoint: {}",
            endpoint
        )))
    }

    async fn update_metrics(&self, type_system: &str, processing_time: u64, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;

        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }

        let total_time =
            metrics.avg_response_time_ms * (metrics.total_requests - 1) as f64 + processing_time as f64;
        metrics.avg_response_time_ms = total_time / metrics.total_requests as f64;

        *metrics
            .requests_by_type
            .entry(type_system.to_string())
            .or_insert(0) += 1;
    }
}

// ============================================================
// Example Implementations
// ============================================================

/// Example type system handler
#[derive(Debug, Clone)]
pub struct ExampleTypeSystemHandler {
    type_id: String,
    config: HashMap<String, serde_json::Value>,
}

impl ExampleTypeSystemHandler {
    /// Create a new handler
    pub fn new(type_id: &str) -> Self {
        Self {
            type_id: type_id.to_string(),
            config: HashMap::with_capacity(16),
        }
    }
}

impl TypeSystemHandler for ExampleTypeSystemHandler {
    fn type_system_id(&self) -> &str {
        &self.type_id
    }

    fn supported_operations(&self) -> Vec<String> {
        vec![
            "create".to_string(),
            "read".to_string(),
            "update".to_string(),
            "delete".to_string(),
        ]
    }

    fn can_handle(&self, request: &UniversalRequest) -> bool {
        self.supported_operations().contains(&request.operation)
    }

    fn handle_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {
        let result = match request.operation.as_str() {
            "create" => serde_json::json!({"created": true, "id": "example_id"}),
            "read" => serde_json::json!({"data": request.payload}),
            "update" => serde_json::json!({"updated": true}),
            "delete" => serde_json::json!({"deleted": true}),
            _ => {
                return Err(BearDogError::unsupported_operation(&request.operation));
            }
        };

        Ok(UniversalResponse {
            request_id: uuid::Uuid::new_v4().to_string(),
            status: ResponseStatus::Success,
            data: Some(result),
            error: None,
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Example protocol adapter
#[derive(Debug, Clone)]
pub struct ExampleProtocolAdapter {
    protocol_id: String,
}

impl ExampleProtocolAdapter {
    /// Create a new adapter
    pub fn new(protocol_id: &str) -> Self {
        Self {
            protocol_id: protocol_id.to_string(),
        }
    }
}

impl ProtocolAdapter for ExampleProtocolAdapter {
    fn protocol_id(&self) -> &str {
        &self.protocol_id
    }

    fn can_handle_endpoint(&self, endpoint: &str) -> bool {
        endpoint.starts_with("http://") || endpoint.starts_with("https://")
    }

    fn send_request(
        &self,
        endpoint: &str,
        request: &UniversalRequest,
    ) -> Result<UniversalResponse, BearDogError> {
        tracing::info!("Sending request to {}: {:?}", endpoint, request);

        Ok(UniversalResponse {
            request_id: uuid::Uuid::new_v4().to_string(),
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({
                "endpoint": endpoint,
                "protocol": self.protocol_id
            })),
            error: None,
            timestamp: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ExtensibleAdapterConfig::default();
        assert_eq!(config.max_concurrent_operations, 100);
        assert!(config.auto_discover_types);
    }

    #[test]
    fn test_metrics_default() {
        let metrics = AdapterMetrics::default();
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.successful_requests, 0);
    }

    #[test]
    fn test_example_handler() {
        let handler = ExampleTypeSystemHandler::new("example");
        assert_eq!(handler.type_system_id(), "example");
        assert!(handler.supported_operations().contains(&"create".to_string()));
    }

    #[test]
    fn test_example_adapter() {
        let adapter = ExampleProtocolAdapter::new("http");
        assert!(adapter.can_handle_endpoint("http://example.com"));
        assert!(!adapter.can_handle_endpoint("ws://example.com"));
    }
}
