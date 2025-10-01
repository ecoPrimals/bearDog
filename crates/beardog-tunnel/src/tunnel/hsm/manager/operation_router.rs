

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_traits::unified::HsmProvider;
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
    preferred_mobile_operations: Vec<OperationType>,

    flexible_operations: Vec<OperationType>,

    software_preferred_operations: Vec<OperationType>,

    max_retries: u32,

    operation_timeout_ms: u64,
}

#[derive(HashMap<String, f64>,

    average_latency_ms: HashMap<String, f64>,

    operations_processed: HashMap<String, u64>,

    error_counts: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
    /// Number of priority
    pub priority: u8,

    /// Whether requires_hardware is enabled
    pub requires_hardware: bool,

    /// Optional max latency ms
    pub max_latency_ms: Option<u64>,
}

impl<P: HsmProvider + Clone + 'static> HsmOperationRouter<P> {

/// New operation.
    /// Creates a new instance
    pub fn new(P,
        fallback_provider: Option<P>,
    ) -> Self {
        Self {
            primary_provider,
            fallback_provider,
            routing_rules: OperationRoutingRules::default(),
            performance_metrics: RoutingMetrics::default(RoutingContext,
        operation: F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(std::future::Future<Output = Result<T, BearDogError>> + Send,
        T: Send,
    {
        debug!("Routing operation: {:?}", context.operation_type);

        let should_use_primary = self.should_use_primary_provider({}", e);
                    self.update_error_metrics({}", e);
                    self.update_error_metrics(&P,
        operation: &F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(std::future::Future<Output = Result<T, BearDogError>> + Send,
        T: Send,
    {
        let start_time = std::time::Instant::now();
        
        let result = operation(&provider);
        
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

    /// Updates success_metrics
    fn update_success_metrics(&mut self, provider_name: &str) {
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

    /// Updates error_metrics
    fn update_error_metrics(&mut self, provider_name: &str) {
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

/// Get Metrics operation.
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> &RoutingMetrics {
        &self.performance_metrics
    }

/// Update Routing Rules operation.
    /// Updates routing_rules
    /// Updates routing_rules
    pub fn update_routing_rules(&mut self, rules: OperationRoutingRules) {
        self.routing_rules = rules;
    }
}

impl Default for OperationRoutingRules {
    fn default(vec![
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

/// Get Success Rate operation.
    /// Gets success_rate
    /// Gets success_rate
    pub fn get_success_rate(&self, provider: &str) -> f64 {
        self.success_rates.get(provider).copied().unwrap_or(0.0)
    }

/// Get Average Latency operation.
    /// Gets average_latency
    /// Gets average_latency
    pub fn get_average_latency(&self, provider: &str) -> f64 {
        self.average_latency_ms.get(provider).copied().unwrap_or(0.0)
    }

/// Get Operations Count operation.
    /// Gets operations_count
    /// Gets operations_count
    pub fn get_operations_count(&self, provider: &str) -> u64 {
        self.operations_processed.get(provider).copied().unwrap_or(0)
    }

/// Get Error Count operation.
    /// Gets error_count
    /// Gets error_count
    pub fn get_error_count(&self, provider: &str) -> u64 {
        self.error_counts.get(provider).copied().unwrap_or(0)
    }
}
