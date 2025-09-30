// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
// MODERNIZED: Use canonical unified provider system
use beardog_types::canonical::providers_unified::traits::{ConsolidatedProvider, ProviderInfo, ProviderHealth, ProviderMetrics, ProviderConfiguration};
use beardog_types::canonical::capabilities::HealthStatus;
use beardog_types::canonical::config::UnifiedBearDogConfig;
use crate::cloud::config::CloudProvider;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn};

/// Universal provider implementation supporting multiple backends
#[derive(Debug, Clone)]
pub enum UniversalProviderImpl {
    /// Cloud provider backend
    Cloud(CloudProvider),
    /// Container orchestration backend  
    ContainerOrchestration(KubernetesProvider),
    /// Database backend
    Database(DatabaseProvider),
    /// Monitoring backend
    Monitoring(PrometheusProvider),
}

/// Kubernetes provider for container orchestration
#[derive(Debug, Clone)]
pub struct KubernetesProvider {
    pub cluster_url: String,
    pub namespace: String,
}

/// Database provider for data persistence
#[derive(Debug, Clone)]
pub struct DatabaseProvider {
    pub connection_string: String,
    pub max_connections: u32,
}

/// Prometheus provider for monitoring
#[derive(Debug, Clone)]
pub struct PrometheusProvider {
    pub endpoint: String,
    pub scrape_interval: Duration,
}

// MODERNIZED: Native async implementation without async_trait
impl ConsolidatedProvider for UniversalProviderImpl {
    fn provider_info(&self) -> ProviderInfo {
        match self {
            Self::Cloud(p) => p.provider_info(),
            Self::ContainerOrchestration(p) => p.provider_info(),
            Self::Database(p) => p.provider_info(),
        }
    }
    
    
    fn health_check(&self) -> Result<ProviderHealth, BearDogError> {
        match self {
            Self::Cloud(p) => p.health_check(),
            Self::ContainerOrchestration(p) => p.health_check(),
            Self::Database(p) => p.health_check(),
        }
    }
    
    
    fn metrics(&self) -> Result<ProviderMetrics, BearDogError> {
        match self {
            Self::Cloud(p) => p.metrics(),
            Self::ContainerOrchestration(p) => p.metrics(),
            Self::Database(p) => p.metrics(),
        }
    }
    
    
    fn capabilities(&self) -> Vec<String> {
        match self {
            Self::Cloud(p) => p.capabilities(),
            Self::ContainerOrchestration(p) => p.capabilities(),
            Self::Database(p) => p.capabilities(),
        }
    }
    
    /// Initializes componentialize
    
    fn initialize(&mut self) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.initialize(),
            Self::ContainerOrchestration(p) => p.initialize(),
            Self::Database(p) => p.initialize(),
        }
    }
    
    
    fn shutdown(&mut self) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.shutdown(),
            Self::ContainerOrchestration(p) => p.shutdown(),
            Self::Database(p) => p.shutdown(),
        }
    }
    
    
    fn configuration(&self) -> ProviderConfiguration {
        match self {
            Self::Cloud(p) => p.configuration(),
            Self::ContainerOrchestration(p) => p.configuration(),
            Self::Database(p) => p.configuration(),
        }
    }
}

// Migration to UnifiedProvider system complete - use UnifiedProvider trait instead

// MODERNIZED: Remove async_trait and use native async functions
impl beardog_traits::canonical::BaseProvider for UniversalProviderImpl {
    fn provider_id(&self) -> &str {
        match self {
            Self::Cloud(_) => "cloud-universal",
            Self::ContainerOrchestration(_) => "container-orchestration-universal",
            Self::Database(_) => "database-universal",
        }
    }

    fn provider_info(&self) -> beardog_traits::canonical::ProviderInfo {
        match self {
            Self::Cloud(p) => p.provider_info(),
            Self::ContainerOrchestration(p) => p.provider_info(),
            Self::Database(p) => p.provider_info(),
        }
    }

    fn id(&self) -> &str {
        match self {
            Self::Cloud(p) => p.id(),
            Self::ContainerOrchestration(p) => p.id(),
            Self::Database(p) => p.id(),
        }
    }

    fn version(&self) -> &'static str {
        match self {
            Self::Cloud(p) => p.version(),
            Self::ContainerOrchestration(p) => p.version(),
            Self::Database(p) => p.version(),
        }
    }

    /// Validates config
    fn validate_config(&self, config: &UnifiedBearDogConfig) -> Result<bool, BearDogError> {
        match self {
            Self::Cloud(p) => p.validate_config(config),
            Self::ContainerOrchestration(p) => p.validate_config(config),
            Self::Database(p) => p.validate_config(config),
        }
    }

    fn status(&self) -> Result<beardog_traits::canonical::ProviderStatus, BearDogError> {
        match self {
            Self::Cloud(p) => p.status(),
            Self::ContainerOrchestration(p) => p.status(),
            Self::Database(p) => p.status(),
        }
    }

    fn reload_config(&self, config: UnifiedBearDogConfig) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.reload_config(config),
            Self::ContainerOrchestration(p) => p.reload_config(config),
            Self::Database(p) => p.reload_config(config),
        }
    }

    fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        match self {
            Self::Cloud(p) => p.health_check(),
            Self::ContainerOrchestration(p) => p.health_check(),
            Self::Database(p) => p.health_check(),
        }
    }

    fn capabilities(&self) -> Result<Vec<String>, BearDogError> {
        match self {
            Self::Cloud(p) => p.capabilities(),
            Self::ContainerOrchestration(p) => p.capabilities(),
            Self::Database(p) => p.capabilities(),
        }
    }

    fn metrics(&self) -> Result<ProviderMetrics, BearDogError> {
        match self {
            Self::Cloud(p) => p.metrics(),
            Self::ContainerOrchestration(p) => p.metrics(),
            Self::Database(p) => p.metrics(),
        }
    }

    /// Starts service
    fn start(&mut self) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.start(),
            Self::ContainerOrchestration(p) => p.start(),
            Self::Database(p) => p.start(),
        }
    }

    /// Stops service
    fn stop(&mut self) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.stop(),
            Self::ContainerOrchestration(p) => p.stop(),
            Self::Database(p) => p.stop(),
        }
    }

    fn restart(&mut self) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.restart(),
            Self::ContainerOrchestration(p) => p.restart(),
            Self::Database(p) => p.restart(),
        }
    }

    fn configure(&mut self, config: UnifiedBearDogConfig) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.configure(config),
            Self::ContainerOrchestration(p) => p.configure(config),
            Self::Database(p) => p.configure(config),
        }
    }

    fn reset(&mut self) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.reset(),
            Self::ContainerOrchestration(p) => p.reset(),
            Self::Database(p) => p.reset(),
        }
    }

    fn backup(&self) -> Result<Vec<u8>, BearDogError> {
        match self {
            Self::Cloud(p) => p.backup(),
            Self::ContainerOrchestration(p) => p.backup(),
            Self::Database(p) => p.backup(),
        }
    }

    fn restore(&mut self, data: Vec<u8>) -> Result<(), BearDogError> {
        match self {
            Self::Cloud(p) => p.restore(data),
            Self::ContainerOrchestration(p) => p.restore(data),
            Self::Database(p) => p.restore(data),
        }
    }

    /// Gets logs
    fn get_logs(&self, lines: Option<usize>) -> Result<Vec<String>, BearDogError> {
        match self {
            Self::Cloud(p) => p.get_logs(lines),
            Self::ContainerOrchestration(p) => p.get_logs(lines),
            Self::Database(p) => p.get_logs(lines),
        }
    }

    /// Executes command
    fn execute_command(&self, command: &str, args: Vec<&str>) -> Result<String, BearDogError> {
        match self {
            Self::Cloud(p) => p.execute_command(command, args),
            Self::ContainerOrchestration(p) => p.execute_command(command, args),
            Self::Database(p) => p.execute_command(command, args),
        }
    }
}

/// Request priority for universal adapters
#[derive(Debug, Clone, Copy)]
pub enum RequestPriority {
    Low,
    Medium,
    High,
}

/// Response status for universal adapters  
#[derive(Debug, Clone, Copy)]
pub enum ResponseStatus {
    Success,
    Error,
    Timeout,
}

/// Universal request type
#[derive(Debug, Clone)]
pub struct UniversalRequest {
    pub id: String,
    pub priority: RequestPriority,
    pub data: Vec<u8>,
}

/// Universal response type
#[derive(Debug, Clone)]
pub struct UniversalResponse {
    pub id: String,
    pub status: ResponseStatus,
    pub data: Vec<u8>,
}

impl UniversalExternalAdapter {
    /// New operation.
    /// Creates a new instance
    pub fn new(config: UniversalAdapterConfig) -> Self {
        Self {
            providers: HashMap::with_capacity(16),
            config,
        }
    }

    /// Register Provider operation.
    pub fn register_provider(&mut self, provider: UniversalProviderImpl) {
        let system_id = provider.provider_type().to_string();
        self.providers.insert(system_id, provider);
    }

    /// Process universal request
    pub async fn process_request(
        &self,
        request: &UniversalRequest,
    ) -> Result<UniversalResponse, BearDogError> {
        let start_time = std::time::Instant::now();

        let service_key = format!("{:?}", request.service_type)
            .to_string()
            .to_lowercase();
        let provider = self.providers.get(&service_key).ok_or_else(|| {
            BearDogError::system(format!("Error: {:?}", request.service_type
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

    async fn execute_with_retry(
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
                    HashMap::from([("payload", request.payload.clone())]),
                ),
            )
            {
                Ok(Ok(result)) => return Ok(result),
                Ok(Err(e)) => {
                    warn!(
                        "🔄 Attempt {}/{} failed for {:?}: {}",
                        attempt, max_retries, request.service_type, e
                    );
                    if attempt < max_retries {
                        let backoff_ms = beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE * attempt as u64; // Simple backoff
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    } else {
                        return Err(e);
                    }
                }
                Err(_) => {
                    return Err(BearDogError::network("Request timeout ".to_string()));
                }
            }
        }

        Err(BearDogError::system("Max retries exceeded ".to_string()))
    }

    /// Health Check operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<HashMap<String, bool>, BearDogError> {
        let mut results = HashMap::with_capacity(16);
        for (system_id, provider) in &self.providers {
            match provider.connection_status() {
                beardog_traits::canonical::ConnectionStatus::Connected => {
                    results.insert(system_id.clone(), true);
                }
                beardog_traits::canonical::ConnectionStatus::Disconnected => {
                    results.insert(system_id.clone(), false);
                }
            }
        }
        Ok(results)
    }

    /// Get Systems Info operation.
    /// Gets systems_info
    /// Gets systems_info
    pub fn get_systems_info(&self) -> HashMap<String, Vec<String>> {
        self.providers
            .iter()
            .map(|(id, provider)| {
                let capabilities = match provider {
                    UniversalProviderImpl::Cloud(_) => vec!["cloud".to_string()],
                    UniversalProviderImpl::ContainerOrchestration(_) => vec!["container-orchestration ".to_string()],
                    UniversalProviderImpl::Database(_) => vec!["database".to_string()],
                };
                (id.clone(), capabilities)
            })
            .collect()
    }

    /// Unregister Provider operation.
    pub fn unregister_provider(&mut self, system_id: &str) -> bool {
        match self.providers.remove(system_id) {
            Some(_) => {
                true
            }
            None => {
                warn!("⚠️ Attempted to unregister unknown provider: {}", system_id);
                false
            }
        }
    }
}
