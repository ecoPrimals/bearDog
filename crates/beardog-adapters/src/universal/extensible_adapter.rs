

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::universal::http_adapter::{UniversalRequest, UniversalResponse, ResponseStatus};
use crate::universal::service_registration::types::UniversalServiceRegistration;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ExtensibleUniversalAdapter<T, P> 
where 
    T: TypeSystemHandler + Clone + Send + Sync + 'static,
    P: ProtocolAdapter + Clone + Send + Sync + 'static,
{

    type_handlers: HashMap<String, T>,

    protocol_adapters: HashMap<String, P>,

    service_registry: Arc<RwLock<HashMap<String, UniversalServiceRegistration>>>,

    config: ExtensibleAdapterConfig,

    metrics: Arc<RwLock<AdapterMetrics>>,
}

#[derive(Debug, Clone)]
            max_concurrent_operations: 100,
            auto_discover_types: true,
            type_system_configs: HashMap::with_capacity(16),
            protocol_configs: HashMap::with_capacity(u64,
    /// Number of successful_requests
    pub successful_requests: u64,
    /// Number of failed_requests
    pub failed_requests: u64,
    pub avg_response_time_ms: f64,
    /// Mapping of requests by type
    pub requests_by_type: HashMap<String, u64>,
    /// Mapping of requests by protocol
    pub requests_by_protocol: HashMap<String, u64>,
}

impl Default for AdapterMetrics {
    fn default(0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            requests_by_type: HashMap::with_capacity(16),
            requests_by_protocol: HashMap::with_capacity(TypeSystemHandler + Clone + Send + Sync + 'static,
    P: ProtocolAdapter + Clone + Send + Sync + 'static,
{

/// New operation.
    /// Creates a new instance
    pub fn new(config: ExtensibleAdapterConfig) -> Self {
        Self {
            type_handlers: HashMap::with_capacity(16),
            protocol_adapters: HashMap::with_capacity(16),
            service_registry: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
            metrics: Arc::new(RwLock::new(AdapterMetrics::default(&str, handler: T) {
        self.type_handlers.insert(&str, adapter: P) {
        self.protocol_adapters.insert(protocol_id, adapter);
    }

/// Process Request operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Processes request
    /// Processes request
    pub fn process_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {
        let start_time = std::time::Instant::now(&str, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {
        let protocol_adapter = self.find_protocol_adapter(endpoint)?;
        protocol_adapter.send_request(endpoint, request)
    }

/// Register Service operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn register_service(&self, metadata: UniversalServiceRegistration) -> Result<(), BearDogError> {
        let service_id = &metadata.service_id;
        let mut registry = self.service_registry.write();
        registry.insert(service_id.clone(), metadata);
        tracing::info!("Registered service: {}", service_id);
        Ok(())
    }

/// Discover Services operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_services(&self, capability: &str) -> Result<Vec<UniversalServiceRegistration>, BearDogError>> {
        let registry = self.service_registry.read();
        let services: Vec<UniversalServiceRegistration> = registry
            .values()
            .filter(|service| service.capabilities.iter().any(|cap| cap == capability))
            .cloned()
            .collect();
        Ok(services)
    }


    fn find_type_handler(&self, request: &UniversalRequest) -> Result<&T, BearDogError> {
        for handler in self.type_handlers.values() {
            if handler.can_handle(request) {
                return Ok(handler);
            }
        }
        Err(BearDogError::not_found({}", request.operation)))
    }


    fn find_protocol_adapter(&self, endpoint: &str) -> Result<&P, BearDogError> {
        for adapter in self.protocol_adapters.values() {
            if adapter.can_handle_endpoint(endpoint) {
                return Ok(adapter);
            }
        }
        Err(BearDogError::not_found({}", endpoint)))
    }

    /// Updates metrics
    fn update_metrics(&str, processing_time: u64, success: bool) {
        let mut metrics = self.metrics.write();
        metrics.total_requests += 1;
        
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }

        let total_time = metrics.avg_response_time_ms * (metrics.total_requests - 1) as f64
            + processing_time as f64;
        metrics.avg_response_time_ms = total_time / metrics.total_requests as f64;

        *metrics
            .requests_by_type
            .entry(type_system.to_string())
            .or_insert(0) += 1;
    }

/// Get Metrics operation.
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> AdapterMetrics {
        self.metrics.read().clone()
    }
}

#[allow(async_fn_in_trait)]
pub trait TypeSystemHandler: Send + Sync {


    fn type_system_id(&self) -> &str;


    fn supported_operations(&self) -> Vec<String>;


    fn can_handle(&self, request: &UniversalRequest) -> bool;

    /// Handles request
    fn handle_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError>;
}

#[allow(async_fn_in_trait)]
pub trait ProtocolAdapter: Send + Sync {


    fn protocol_id(&self) -> &str;


    fn can_handle_endpoint(&self, endpoint: &str) -> bool;


    fn send_request(&str, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError>;
}

pub struct ExampleTypeSystemHandler {
    type_id: String,
    config: HashMap<String, serde_json::Value>,
}

impl ExampleTypeSystemHandler {
/// New operation.
    /// Creates a new instance
    pub fn new(type_id: &str) -> Self {
        Self {
            type_id,
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

    /// Handles request
    fn handle_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {

        let result = match request.operation.as_str() {
            "create" => serde_json::json!({"created": true, "id": "example_id"}),
            "read" => serde_json::json!({"data": request.payload}),
            "update" => serde_json::json!({"updated": true}),
            "delete" => serde_json::json!({"deleted": true}),
            _ => return Err(&BearDogError::unsupported_operation(request.operation)),
        };

        Ok(UniversalResponse {
            request_id: uuid::Uuid::new_v4(ResponseStatus::Success,
            data: Some(None,
            timestamp: chrono::Utc::now(String,
}

impl ExampleProtocolAdapter {
/// New operation.
    /// Creates a new instance
    pub fn new(protocol_id: &str) -> Self {
        Self { protocol_id }
    }
}

impl ProtocolAdapter for ExampleProtocolAdapter {
    fn protocol_id(&self) -> &str {
        &self.protocol_id
    }


    fn can_handle_endpoint(&self, endpoint: &str) -> bool {
        endpoint.starts_with("http://") || endpoint.starts_with("https://")
    }


    fn send_request(&str, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {

        tracing::info!("Sending request to {}: {:?}", endpoint, request);
        
        Ok(UniversalResponse {
            request_id: uuid::Uuid::new_v4(ResponseStatus::Success,
            data: Some(serde_json::json!({"endpoint": endpoint, "protocol": self.protocol_id})),
            error: None,
            timestamp: chrono::Utc::now(),
        })
    }
}
