// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
// MODERNIZED: Use canonical unified provider system
use beardog_types::canonical::providers_unified::traits::{UnifiedProvider, ProviderInfo, ProviderHealth, ProviderMetrics, ProviderCapability, ProviderConfiguration};
use beardog_types::canonical::{HealthStatus, ProviderConfig, ProviderStatus};
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

#[derive(Debug, Clone)]
pub enum UniversalProviderImpl {
    /// Represents cloud variant
    Cloud(providers::CloudProvider),
    /// Represents container orchestration variant
    ContainerOrchestration(providers::ContainerOrchestrationProvider),
    /// Represents database variant
    Database(providers::DatabaseProvider),
}

// MODERNIZED: Native async implementation without async_trait
impl UnifiedProvider for UniversalProviderImpl {
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
    
    
    fn capabilities(&self) -> Vec<ProviderCapability> {
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

// UNIFIED: Migrated from canonical to unified trait system
impl beardog_traits::unified::UniversalProvider for UniversalProviderImpl {
    fn provider_type(&self) -> &str {
        match self {
            Self::Cloud(_) => "cloud",
            Self::ContainerOrchestration(_) => "container-orchestration",
            Self::Database(_) => "database",
        }
    }

    /// Validates compatibility
    fn validate_compatibility(&self, target_version: &str) -> Result<bool, BearDogError> {
        match self {
            Self::Cloud(p) => p.validate_compatibility(target_version),
            Self::ContainerOrchestration(p) => p.validate_compatibility(target_version),
            Self::Database(p) => p.validate_compatibility(target_version),
        }
    }
}

// MODERNIZED: Using unified traits with native async functions
impl beardog_traits::unified::BaseProvider for UniversalProviderImpl {
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
    fn validate_config(&self, config: &ProviderConfig) -> Result<bool, BearDogError> {
        match self {
            Self::Cloud(p) => p.validate_config(config),
            Self::ContainerOrchestration(p) => p.validate_config(config),
            Self::Database(p) => p.validate_config(config),
        }
    }


    fn status(&self) -> Result<ProviderStatus, BearDogError> {
        match self {
            Self::Cloud(p) => p.status(),
            Self::ContainerOrchestration(p) => p.status(),
            Self::Database(p) => p.status(),
        }
    }


    fn reload_config(&self, config: ProviderConfig) -> Result<(), BearDogError> {
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


    fn configure(&mut self, config: ProviderConfig) -> Result<(), BearDogError> {
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

pub use beardog_types::canonical::services::{
    RequestPriority, ResponseStatus, UniversalRequest, UniversalResponse,
};

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
        let system_id = provider.provider_type({}", system_id);
        self.providers.insert(&UniversalRequest,
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

        match self.execute_with_retry(provider, request, timeout) {
            Ok(result) => Ok(UniversalResponse {
                request_id: request.request_id.clone(ResponseStatus::Success,
                payload: result,
                timestamp: chrono::Utc::now(),
                processing_time_ms: start_time.elapsed().as_millis() as u64,
            }),
            Err(e) => Ok(UniversalResponse {
                request_id: request.request_id.clone(ResponseStatus::Error,
                payload: serde_json::Value::String(e.to_string()),
                timestamp: chrono::Utc::now(),
                processing_time_ms: start_time.elapsed(&UniversalProviderImpl,
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

    /// Health Check operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<HashMap<String, bool>, BearDogError> {
        let mut results = HashMap::with_capacity(16);
        for (system_id, provider) in &self.providers {
            match provider.connection_status() {
                beardog_traits::canonical::ConnectionStatus::Connected => {
                    results.insert({}", system_id, "Unhealthy");
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
                    UniversalProviderImpl::ContainerOrchestration(_) => vec!["container-orchestration".to_string()],
                    UniversalProviderImpl::Database(_) => vec!["database".to_string()],
                };
                (id.clone(), capabilities)
            })
            .collect()
    }

    /// Unregister Provider operation.
    pub fn unregister_provider(&mut self, system_id: &str) -> bool {
        match self.providers.remove({}", system_id);
                true
            }
            None => {
                warn!("⚠️ Attempted to unregister unknown provider: {}", system_id);
                false
            }
        }
    }
}
