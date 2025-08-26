// PHASE 5 MODERNIZED: Comprehensive Arc<dyn> elimination
// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// HSM operation router with intelligent routing
///
/// **MODERNIZED ZERO-COST ARCHITECTURE** ✅
/// This router now uses generic composition instead of impl HsmProvider + Send + Sync + 'static for
/// zero-cost abstractions and better performance.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_traits::canonical::HsmProvider;
use beardog_types::canonical::hsm::{HsmKey, HsmTier, KeyMetadata, KeyType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Zero-cost HSM operation router using generic composition
/// 
/// **PERFORMANCE BENEFITS:**
/// - **No runtime dispatch** - All calls statically resolved
/// - **Zero heap allocations** - No Arc<dyn> boxing overhead
/// - **Better inlining** - Compiler can optimize across trait boundaries
/// - **Type safety** - Compile-time guarantees for all operations
pub struct HsmOperationRouter<P: HsmProvider + Clone + 'static> {
    /// Primary HSM provider
    primary_provider: P,
    /// Fallback HSM provider (optional)
    fallback_provider: Option<P>,
    /// Operation routing rules
    routing_rules: OperationRoutingRules,
    /// Performance metrics for routing decisions
    performance_metrics: RoutingMetrics,
}

/// Operation routing rules configuration
#[derive(Debug, Clone)]
pub struct OperationRoutingRules {
    /// Critical operations that require mobile HSM
    critical_operations: Vec<OperationType>,
    /// Operations that prefer mobile HSM but can fallback
    preferred_mobile_operations: Vec<OperationType>,
    /// Operations that can use any available HSM
    flexible_operations: Vec<OperationType>,
    /// Operations that should use software HSM for performance
    software_preferred_operations: Vec<OperationType>,
    /// Maximum retry attempts for operations
    max_retries: u32,
    /// Timeout for operations in milliseconds
    operation_timeout_ms: u64,
}

/// HSM operation types for routing decisions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationType {
    /// Key generation operations
    KeyGeneration,
    /// Digital signing operations
    DigitalSigning,
    /// Encryption/decryption operations
    Encryption,
    /// Key derivation operations
    KeyDerivation,
    /// Random number generation
    RandomGeneration,
    /// Key storage operations
    KeyStorage,
    /// Authentication operations
    Authentication,
    /// Health check operations
    HealthCheck,
}

/// Routing performance metrics
#[derive(Debug, Clone, Default)]
pub struct RoutingMetrics {
    /// Operation success rates by provider
    success_rates: HashMap<String, f64>,
    /// Average operation latency by provider (ms)
    average_latency_ms: HashMap<String, f64>,
    /// Total operations processed by provider
    operations_processed: HashMap<String, u64>,
    /// Error counts by provider
    error_counts: HashMap<String, u64>,
}

/// Operation routing context
#[derive(Debug, Clone)]
pub struct RoutingContext {
    /// Operation type being performed
    pub operation_type: OperationType,
    /// Priority level (1-10, 10 being highest)
    pub priority: u8,
    /// Whether this operation requires hardware security
    pub requires_hardware: bool,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: Option<u64>,
}

impl<P: HsmProvider + Clone + 'static> HsmOperationRouter<P> {
    /// Create new zero-cost HSM operation router
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

    /// Route an operation to the most appropriate HSM provider
    pub async fn route_operation<T, F, Fut>(
        &mut self,
        context: RoutingContext,
        operation: F,
    ) -> BearDogResult<T>
    where
        F: Fn(P) -> Fut + Send + Clone,
        Fut: std::future::Future<Output = BearDogResult<T>> + Send,
        T: Send,
    {
        debug!("Routing operation: {:?}", context.operation_type);

        // Determine the best provider based on routing rules and metrics
        let should_use_primary = self.should_use_primary_provider(&context);

        // Try primary provider first if it's the preferred choice
        if should_use_primary {
            match self.execute_with_provider(&self.primary_provider.clone(), &operation).await {
                Ok(result) => {
                    self.update_success_metrics("primary").await;
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Primary provider failed: {}", e);
                    self.update_error_metrics("primary").await;
                    
                    // Try fallback if available
                    if let Some(fallback) = &self.fallback_provider {
                        return self.execute_with_provider(fallback, &operation).await;
                    }
                    return Err(e);
                }
            }
        }

        // Try fallback provider first if primary is not preferred
        if let Some(fallback) = &self.fallback_provider {
            match self.execute_with_provider(fallback, &operation).await {
                Ok(result) => {
                    self.update_success_metrics("fallback").await;
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Fallback provider failed: {}", e);
                    self.update_error_metrics("fallback").await;
                    
                    // Try primary as last resort
                    return self.execute_with_provider(&self.primary_provider.clone(), &operation).await;
                }
            }
        }

        // Only primary provider available
        self.execute_with_provider(&self.primary_provider.clone(), &operation).await
    }

    /// Execute operation with a specific provider
    async fn execute_with_provider<T, F, Fut>(
        &mut self,
        provider: &P,
        operation: &F,
    ) -> BearDogResult<T>
    where
        F: Fn(P) -> Fut + Send + Clone,
        Fut: std::future::Future<Output = BearDogResult<T>> + Send,
        T: Send,
    {
        let start_time = std::time::Instant::now();
        
        let result = operation(provider.clone()).await;
        
        let elapsed = start_time.elapsed();
        debug!("Operation completed in {:?}", elapsed);
        
        result
    }

    /// Determine if primary provider should be used based on context
    fn should_use_primary_provider(&self, context: &RoutingContext) -> bool {
        // High priority operations prefer primary
        if context.priority >= 8 {
            return true;
        }

        // Operations requiring hardware security
        if context.requires_hardware {
            return true;
        }

        // Check routing rules
        if self.routing_rules.critical_operations.contains(&context.operation_type) {
            return true;
        }

        // Consider performance metrics
        let primary_success_rate = self.performance_metrics
            .success_rates
            .get("primary")
            .unwrap_or(&0.95);

        let fallback_success_rate = self.performance_metrics
            .success_rates
            .get("fallback")
            .unwrap_or(&0.90);

        // Prefer primary if it has better success rate
        primary_success_rate > fallback_success_rate
    }

    /// Update success metrics for a provider
    async fn update_success_metrics(&mut self, provider_name: &str) {
        let current_rate = self.performance_metrics
            .success_rates
            .get(provider_name)
            .unwrap_or(&0.95);
        
        // Simple exponential moving average
        let new_rate = current_rate * 0.9 + 0.1;
        self.performance_metrics.success_rates.insert(provider_name.to_string(), new_rate);
        
        let operations = self.performance_metrics
            .operations_processed
            .get(provider_name)
            .unwrap_or(&0);
        self.performance_metrics.operations_processed.insert(provider_name.to_string(), operations + 1);
    }

    /// Update error metrics for a provider
    async fn update_error_metrics(&mut self, provider_name: &str) {
        let current_rate = self.performance_metrics
            .success_rates
            .get(provider_name)
            .unwrap_or(&0.95);
        
        // Decrease success rate on error
        let new_rate = current_rate * 0.9;
        self.performance_metrics.success_rates.insert(provider_name.to_string(), new_rate);
        
        let errors = self.performance_metrics
            .error_counts
            .get(provider_name)
            .unwrap_or(&0);
        self.performance_metrics.error_counts.insert(provider_name.to_string(), errors + 1);
    }

    /// Get routing performance metrics
    pub fn get_metrics(&self) -> &RoutingMetrics {
        &self.performance_metrics
    }

    /// Update routing rules
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
    /// Get success rate for a provider
    pub fn get_success_rate(&self, provider: &str) -> f64 {
        self.success_rates.get(provider).copied().unwrap_or(0.0)
    }

    /// Get average latency for a provider
    pub fn get_average_latency(&self, provider: &str) -> f64 {
        self.average_latency_ms.get(provider).copied().unwrap_or(0.0)
    }

    /// Get total operations for a provider
    pub fn get_operations_count(&self, provider: &str) -> u64 {
        self.operations_processed.get(provider).copied().unwrap_or(0)
    }

    /// Get error count for a provider
    pub fn get_error_count(&self, provider: &str) -> u64 {
        self.error_counts.get(provider).copied().unwrap_or(0)
    }
}
