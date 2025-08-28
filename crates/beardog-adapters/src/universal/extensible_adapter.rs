

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

#[derive(Debug, Clone, Serialize, Deserialize)]

impl Default for ExtensibleAdapterConfig {
    fn default() -> Self {
        Self {
            default_timeout_seconds: 30,
            max_concurrent_operations: 100,
            auto_discover_types: true,
            type_system_configs: HashMap::with_capacity(16),
            protocol_configs: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time_ms: f64,
    pub requests_by_type: HashMap<String, u64>,
    pub requests_by_protocol: HashMap<String, u64>,
}

impl Default for AdapterMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            requests_by_type: HashMap::with_capacity(16),
            requests_by_protocol: HashMap::with_capacity(16),
        }
    }
}

impl<T, P> ExtensibleUniversalAdapter<T, P>
where
    T: TypeSystemHandler + Clone + Send + Sync + 'static,
    P: ProtocolAdapter + Clone + Send + Sync + 'static,
{

    pub fn new(config: ExtensibleAdapterConfig) -> Self {
        Self {
            type_handlers: HashMap::with_capacity(16),
            protocol_adapters: HashMap::with_capacity(16),
            service_registry: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
            metrics: Arc::new(RwLock::new(AdapterMetrics::default())),
        }
    }

    pub fn register_type_handler(&mut self, type_id: &str, handler: T) {
        self.type_handlers.insert(type_id, handler);
    }

    pub fn register_protocol_adapter(&mut self, protocol_id: &str, adapter: P) {
        self.protocol_adapters.insert(protocol_id, adapter);
    }

    pub async fn process_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {
        let start_time = std::time::Instant::now();

        let handler = self.find_type_handler(request).await?;

        let response = handler.handle_request(request).await?;

        let processing_time = start_time.elapsed().as_millis() as u64;
        self.update_metrics(&handler.type_system_id(), processing_time, true).await;
        
        Ok(response)
    }

    pub async fn send_request(&self, endpoint: &str, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {
        let protocol_adapter = self.find_protocol_adapter(endpoint).await?;
        protocol_adapter.send_request(endpoint, request).await
    }

    pub async fn register_service(&self, metadata: UniversalServiceRegistration) -> Result<(), BearDogError> {
        let service_id = metadata.service_id.clone();
        let mut registry = self.service_registry.write().await;
        registry.insert(service_id.clone(), metadata);
        tracing::info!("Registered service: {}", service_id);
        Ok(())
    }

    pub async fn discover_services(&self, capability: &str) -> Result<Vec<UniversalServiceRegistration>, BearDogError>> {
        let registry = self.service_registry.read().await;
        let services: Vec<UniversalServiceRegistration> = registry
            .values()
            .filter(|service| service.capabilities.iter().any(|cap| cap == capability))
            .cloned()
            .collect();
        Ok(services)
    }

    async fn find_type_handler(&self, request: &UniversalRequest) -> Result<&T, BearDogError> {
        for handler in self.type_handlers.values() {
            if handler.can_handle(request).await {
                return Ok(handler);
            }
        }
        Err(BearDogError::not_found(format_args!("No type handler found for operation: {}", request.operation).to_string()))
    }

    async fn find_protocol_adapter(&self, endpoint: &str) -> Result<&P, BearDogError> {
        for adapter in self.protocol_adapters.values() {
            if adapter.can_handle_endpoint(endpoint) {
                return Ok(adapter);
            }
        }
        Err(BearDogError::not_found(format_args!("No protocol adapter found for endpoint: {}", endpoint).to_string()))
    }

    async fn update_metrics(&self, type_system: &str, processing_time: u64, success: bool) {
        let mut metrics = self.metrics.write().await;
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

    pub async fn get_metrics(&self) -> AdapterMetrics {
        self.metrics.read().await.clone()
    }
}

#[allow(async_fn_in_trait)]
pub trait TypeSystemHandler: Send + Sync {

    fn type_system_id(&self) -> &str;

    fn supported_operations(&self) -> Vec<String>;

    async fn can_handle(&self, request: &UniversalRequest) -> bool;

    async fn handle_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError>;
}

#[allow(async_fn_in_trait)]
pub trait ProtocolAdapter: Send + Sync {

    fn protocol_id(&self) -> &str;

    fn can_handle_endpoint(&self, endpoint: &str) -> bool;

    async fn send_request(&self, endpoint: &str, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError>;
}

#[derive(Clone)]
pub struct ExampleTypeSystemHandler {
    type_id: String,
    config: HashMap<String, serde_json::Value>,
}

impl ExampleTypeSystemHandler {
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

    async fn can_handle(&self, request: &UniversalRequest) -> bool {
        self.supported_operations().contains(&request.operation)
    }

    async fn handle_request(&self, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {

        let result = match request.operation.as_str() {
            "create" => serde_json::json!({"created": true, "id": "example_id"}),
            "read" => serde_json::json!({"data": request.payload}),
            "update" => serde_json::json!({"updated": true}),
            "delete" => serde_json::json!({"deleted": true}),
            _ => return Err(BearDogError::unsupported_operation(request.operation.clone())),
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

#[derive(Clone)]
pub struct ExampleProtocolAdapter {
    protocol_id: String,
}

impl ExampleProtocolAdapter {
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

    async fn send_request(&self, endpoint: &str, request: &UniversalRequest) -> Result<UniversalResponse, BearDogError> {

        tracing::info!("Sending request to {}: {:?}", endpoint, request);
        
        Ok(UniversalResponse {
            request_id: uuid::Uuid::new_v4().to_string(),
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({"endpoint": endpoint, "protocol": self.protocol_id})),
            error: None,
            timestamp: chrono::Utc::now(),
        })
    }
}
