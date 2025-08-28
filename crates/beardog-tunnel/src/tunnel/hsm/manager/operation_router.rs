

use beardog_errors::BearDogError;
use beardog_traits::canonical::HsmProvider;
use beardog_types::canonical::hsm::{HsmKey, HsmTier, KeyMetadata, KeyType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

pub struct HsmOperationRouter<P: HsmProvider + Clone + 'static> {

    primary_provider: P,

    fallback_provider: Option<P>,

    routing_rules: OperationRoutingRules,

    performance_metrics: RoutingMetrics,
}

#[derive(Debug, Clone)]
pub struct OperationRoutingRules {

    critical_operations: Vec<OperationType>,

    preferred_mobile_operations: Vec<OperationType>,

    flexible_operations: Vec<OperationType>,

    software_preferred_operations: Vec<OperationType>,

    max_retries: u32,

    operation_timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationType {

    KeyGeneration,

    DigitalSigning,

    Encryption,

    KeyDerivation,

    RandomGeneration,

    KeyStorage,

    Authentication,

    HealthCheck,
}

#[derive(Debug, Clone, Default)]
pub struct RoutingMetrics {

    success_rates: HashMap<String, f64>,

    average_latency_ms: HashMap<String, f64>,

    operations_processed: HashMap<String, u64>,

    error_counts: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct RoutingContext {

    pub operation_type: OperationType,

    pub priority: u8,

    pub requires_hardware: bool,

    pub max_latency_ms: Option<u64>,
}

impl<P: HsmProvider + Clone + 'static> HsmOperationRouter<P> {

    pub fn new(
        primary_provider: P,
        fallback_provider: Option<P>,
    ) -> Self {
        Self {
            primary_provider,
            fallback_provider,
            routing_rules: OperationRoutingRules::default(),
            performance_metrics: RoutingMetrics::default(),
        }
    }

    pub async fn route_operation<T, F, Fut>(
        &mut self,
        context: RoutingContext,
        operation: F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(P) -> Fut + Send + Clone,
        Fut: std::future::Future<Output = Result<T, BearDogError>> + Send,
        T: Send,
    {
        debug!("Routing operation: {:?}", context.operation_type);

        let should_use_primary = self.should_use_primary_provider(&context);

        if should_use_primary {
            match self.execute_with_provider(&self.primary_provider.clone(), &operation).await {
                Ok(result) => {
                    self.update_success_metrics("primary").await;
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Primary provider failed: {}", e);
                    self.update_error_metrics("primary").await;

                    if let Some(fallback) = &self.fallback_provider {
                        return self.execute_with_provider(fallback, &operation).await;
                    }
                    return Err(e);
                }
            }
        }

        if let Some(fallback) = &self.fallback_provider {
            match self.execute_with_provider(fallback, &operation).await {
                Ok(result) => {
                    self.update_success_metrics("fallback").await;
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Fallback provider failed: {}", e);
                    self.update_error_metrics("fallback").await;

                    return self.execute_with_provider(&self.primary_provider.clone(), &operation).await;
                }
            }
        }

        self.execute_with_provider(&self.primary_provider.clone(), &operation).await
    }

    async fn execute_with_provider<T, F, Fut>(
        &mut self,
        provider: &P,
        operation: &F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(P) -> Fut + Send + Clone,
        Fut: std::future::Future<Output = Result<T, BearDogError>> + Send,
        T: Send,
    {
        let start_time = std::time::Instant::now();
        
        let result = operation(provider.clone()).await;
        
        let elapsed = start_time.elapsed();
        debug!("Operation completed in {:?}", elapsed);
        
        result
    }

    fn should_use_primary_provider(&self, context: &RoutingContext) -> bool {

        if context.priority >= 8 {
            return true;
        }

        if context.requires_hardware {
            return true;
        }

        if self.routing_rules.critical_operations.contains(&context.operation_type) {
            return true;
        }

        let primary_success_rate = self.performance_metrics
            .success_rates
            .get("primary")
            .unwrap_or(&0.95);

        let fallback_success_rate = self.performance_metrics
            .success_rates
            .get("fallback")
            .unwrap_or(&0.90);

        primary_success_rate > fallback_success_rate
    }

    async fn update_success_metrics(&mut self, provider_name: &str) {
        let current_rate = self.performance_metrics
            .success_rates
            .get(provider_name)
            .unwrap_or(&0.95);

        let new_rate = current_rate * 0.9 + 0.1;
        self.performance_metrics.success_rates.insert(provider_name.to_string(), new_rate);
        
        let operations = self.performance_metrics
            .operations_processed
            .get(provider_name)
            .unwrap_or(&0);
        self.performance_metrics.operations_processed.insert(provider_name.to_string(), operations + 1);
    }

    async fn update_error_metrics(&mut self, provider_name: &str) {
        let current_rate = self.performance_metrics
            .success_rates
            .get(provider_name)
            .unwrap_or(&0.95);

        let new_rate = current_rate * 0.9;
        self.performance_metrics.success_rates.insert(provider_name.to_string(), new_rate);
        
        let errors = self.performance_metrics
            .error_counts
            .get(provider_name)
            .unwrap_or(&0);
        self.performance_metrics.error_counts.insert(provider_name.to_string(), errors + 1);
    }

    pub fn get_metrics(&self) -> &RoutingMetrics {
        &self.performance_metrics
    }

    pub fn update_routing_rules(&mut self, rules: OperationRoutingRules) {
        self.routing_rules = rules;
    }
}

impl Default for OperationRoutingRules {
    fn default() -> Self {
        Self {
            critical_operations: vec![
                OperationType::KeyGeneration,
                OperationType::DigitalSigning,
            ],
            preferred_mobile_operations: vec![
                OperationType::Authentication,
                OperationType::KeyDerivation,
            ],
            flexible_operations: vec![
                OperationType::Encryption,
                OperationType::RandomGeneration,
            ],
            software_preferred_operations: vec![
                OperationType::HealthCheck,
            ],
            max_retries: 3,
            operation_timeout_ms: 5000,
        }
    }
}

impl RoutingMetrics {

    pub fn get_success_rate(&self, provider: &str) -> f64 {
        self.success_rates.get(provider).copied().unwrap_or(0.0)
    }

    pub fn get_average_latency(&self, provider: &str) -> f64 {
        self.average_latency_ms.get(provider).copied().unwrap_or(0.0)
    }

    pub fn get_operations_count(&self, provider: &str) -> u64 {
        self.operations_processed.get(provider).copied().unwrap_or(0)
    }

    pub fn get_error_count(&self, provider: &str) -> u64 {
        self.error_counts.get(provider).copied().unwrap_or(0)
    }
}
