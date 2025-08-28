use beardog_errors::BearDogError;
use beardog_traits::canonical::UniversalProvider;
use beardog_traits::ProviderMetrics;
use beardog_types::canonical::providers::UniversalAdapterConfig;
use beardog_types::canonical::{HealthStatus, ProviderConfig};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn};

pub mod authentication;
pub mod protocols;
pub mod providers;
pub mod transformers;
pub use authentication::*;
pub use protocols::*;
pub use providers::*;
pub use transformers::*;

// Use an enum instead of Box<dyn> to avoid dyn compatibility issues
#[derive(Debug)]
pub enum UniversalProviderImpl {
    Kubernetes(providers::KubernetesProvider),
    Prometheus(providers::PrometheusProvider),
}

impl UniversalProvider for UniversalProviderImpl {
    fn provider_type(&self) -> &str {
        match self {
            Self::Kubernetes(p) => p.provider_type(),
            Self::Prometheus(p) => p.provider_type(),
        }
    }

    async fn discover_capabilities(&self) -> Result<Vec<String>, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.discover_capabilities().await,
            Self::Prometheus(p) => p.discover_capabilities().await,
        }
    }

    async fn execute_operation(
        &self,
        operation: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.execute_operation(operation, parameters).await,
            Self::Prometheus(p) => p.execute_operation(operation, parameters).await,
        }
    }

    async fn connection_status(
        &self,
    ) -> Result<beardog_traits::canonical::ConnectionStatus, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.connection_status().await,
            Self::Prometheus(p) => p.connection_status().await,
        }
    }

    async fn validate_compatibility(&self, target_version: &str) -> Result<bool, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.validate_compatibility(target_version).await,
            Self::Prometheus(p) => p.validate_compatibility(target_version).await,
        }
    }
}

impl beardog_traits::canonical::BaseProvider for UniversalProviderImpl {
    fn provider_info(&self) -> beardog_traits::canonical::ProviderInfo {
        match self {
            Self::Kubernetes(p) => p.provider_info(),
            Self::Prometheus(p) => p.provider_info(),
        }
    }

    fn id(&self) -> &str {
        match self {
            Self::Kubernetes(p) => p.id(),
            Self::Prometheus(p) => p.id(),
        }
    }

    fn version(&self) -> &str {
        match self {
            Self::Kubernetes(p) => p.version(),
            Self::Prometheus(p) => p.version(),
        }
    }

    async fn validate_config(&self, config: &ProviderConfig) -> Result<bool, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.validate_config(config).await,
            Self::Prometheus(p) => p.validate_config(config).await,
        }
    }

    async fn status(
        &self,
    ) -> Result<beardog_types::canonical::providers::ProviderStatus, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.status().await,
            Self::Prometheus(p) => p.status().await,
        }
    }

    async fn reload_config(&self, config: &ProviderConfig) -> Result<(), BearDogError> {
        match self {
            Self::Kubernetes(p) => p.reload_config(config).await,
            Self::Prometheus(p) => p.reload_config(config).await,
        }
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.health_check().await,
            Self::Prometheus(p) => p.health_check().await,
        }
    }

    async fn capabilities(&self) -> Result<Vec<String>, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.capabilities().await,
            Self::Prometheus(p) => p.capabilities().await,
        }
    }

    async fn initialize(&self, config: &ProviderConfig) -> Result<(), BearDogError> {
        match self {
            Self::Kubernetes(p) => p.initialize(config).await,
            Self::Prometheus(p) => p.initialize(config).await,
        }
    }

    async fn shutdown(&self) -> Result<(), BearDogError> {
        match self {
            Self::Kubernetes(p) => p.shutdown().await,
            Self::Prometheus(p) => p.shutdown().await,
        }
    }

    async fn metrics(&self) -> Result<ProviderMetrics, BearDogError> {
        match self {
            Self::Kubernetes(p) => p.metrics().await,
            Self::Prometheus(p) => p.metrics().await,
        }
    }
}

pub struct UniversalExternalAdapter {
    providers: HashMap<String, UniversalProviderImpl>,
    config: UniversalAdapterConfig,
}

// Use canonical types from beardog-types
pub use beardog_types::canonical::services::{
    RequestPriority, ResponseStatus, UniversalRequest, UniversalResponse,
};

// Default implementation is provided by the canonical module

impl UniversalExternalAdapter {
    pub fn new(config: UniversalAdapterConfig) -> Self {
        Self {
            providers: HashMap::with_capacity(16),
            config,
        }
    }

    pub fn register_provider(&mut self, provider: UniversalProviderImpl) {
        let system_id = provider.provider_type().to_string();
        info!("🔌 Registering external system provider: {}", system_id);
        self.providers.insert(system_id, provider);
    }

    /// Get adapter configuration
    pub fn config(&self) -> &UniversalAdapterConfig {
        &self.config
    }

    pub async fn execute_request(
        &self,
        request: &UniversalRequest,
    ) -> Result<UniversalResponse, BearDogError> {
        let start_time = std::time::Instant::now();
        // Map service_type to string for provider lookup
        let service_key = format!("{:?}", request.service_type).to_lowercase();
        let provider = self.providers.get(&service_key).ok_or_else(|| {
            BearDogError::system(format!(
                "No provider found for service type: {:?}",
                request.service_type
            ))
        })?;

        let timeout = Duration::from_secs(30);

        match self.execute_with_retry(provider, request, timeout).await {
            Ok(result) => Ok(UniversalResponse {
                request_id: request.request_id.clone(),
                status: ResponseStatus::Success,
                payload: result,
                timestamp: chrono::Utc::now(),
                processing_time_ms: start_time.elapsed().as_millis() as u64,
            }),
            Err(e) => Ok(UniversalResponse {
                request_id: request.request_id.clone(),
                status: ResponseStatus::Error,
                payload: serde_json::Value::String(e.to_string()),
                timestamp: chrono::Utc::now(),
                processing_time_ms: start_time.elapsed().as_millis() as u64,
            }),
        }
    }

    pub async fn execute_with_retry(
        &self,
        provider: &UniversalProviderImpl,
        request: &UniversalRequest,
        timeout: Duration,
    ) -> Result<serde_json::Value, BearDogError> {
        let max_retries = 3; // Use default retry count

        for attempt in 1..=max_retries {
            match tokio::time::timeout(
                timeout,
                provider.execute_operation(
                    &request.operation,
                    HashMap::from([("payload".to_string(), request.payload.clone())]),
                ),
            )
            .await
            {
                Ok(Ok(result)) => return Ok(result),
                Ok(Err(e)) => {
                    warn!(
                        "🔄 Attempt {}/{} failed for {:?}: {}",
                        attempt, max_retries, request.service_type, e
                    );
                    if attempt < max_retries {
                        let backoff_ms = 1000 * attempt as u64; // Simple backoff
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    } else {
                        return Err(e);
                    }
                }
                Err(_) => {
                    return Err(BearDogError::network("Request timeout".to_string()));
                }
            }
        }

        Err(BearDogError::system("Max retries exceeded".to_string()))
    }

    pub async fn health_check(&self) -> Result<HashMap<String, bool>, BearDogError> {
        let mut results = HashMap::with_capacity(16);
        for (system_id, provider) in &self.providers {
            match provider.connection_status().await {
                Ok(beardog_traits::canonical::ConnectionStatus {
                    connected: true, ..
                }) => {
                    results.insert(system_id.clone(), true);
                }
                _ => {
                    warn!("🏥 Health check failed for {}: {}", system_id, "Unhealthy");
                    results.insert(system_id.clone(), false);
                }
            }
        }
        Ok(results)
    }

    pub fn get_systems_info(&self) -> HashMap<String, Vec<String>> {
        self.providers
            .iter()
            .map(|(id, provider)| {
                let capabilities = match provider {
                    UniversalProviderImpl::Kubernetes(_) => vec!["kubernetes".to_string()],
                    UniversalProviderImpl::Prometheus(_) => vec!["prometheus".to_string()],
                };
                (id.clone(), capabilities)
            })
            .collect()
    }

    pub fn unregister_provider(&mut self, system_id: &str) -> bool {
        match self.providers.remove(system_id) {
            Some(_) => {
                info!("🔌 Unregistered external system provider: {}", system_id);
                true
            }
            None => {
                warn!("⚠️ Attempted to unregister unknown provider: {}", system_id);
                false
            }
        }
    }
}
