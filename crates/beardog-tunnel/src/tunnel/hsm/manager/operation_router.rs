// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Operation Router
//!
//! Routes operations to appropriate HSM providers based on requirements and availability.

use beardog_errors::BearDogError;
use std::collections::HashMap;

/// Operation types for routing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationType {
    /// Key generation
    KeyGeneration,
    /// Encryption
    Encryption,
    /// Decryption
    Decryption,
    /// Signing
    Signing,
    /// Verification
    Verification,
    /// Key derivation
    KeyDerivation,
}

/// Operation routing configuration
#[derive(Debug, Clone)]
pub struct OperationRouterConfig {
    /// Prefer hardware for these operations
    pub hardware_preferred_operations: Vec<OperationType>,
    /// Prefer software for these operations
    pub software_preferred_operations: Vec<OperationType>,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Operation timeout in milliseconds
    pub operation_timeout_ms: u64,
}

impl Default for OperationRouterConfig {
    fn default() -> Self {
        Self {
            hardware_preferred_operations: vec![
                OperationType::KeyGeneration,
                OperationType::Signing,
            ],
            software_preferred_operations: vec![
                OperationType::Encryption,
                OperationType::Decryption,
            ],
            max_retries: 3,
            operation_timeout_ms: 5000,
        }
    }
}

/// Operation routing statistics
#[derive(Debug, Clone, Default)]
pub struct OperationStats {
    /// Success rate per provider
    pub success_rate: HashMap<String, f64>,
    /// Average latency per provider
    pub average_latency_ms: HashMap<String, f64>,
    /// Operations processed per provider
    pub operations_processed: HashMap<String, u64>,
    /// Error counts per provider
    pub error_counts: HashMap<String, u64>,
}

/// Routing decision information
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    /// Selected provider ID
    pub provider_id: String,
    /// Operation priority
    pub priority: u8,
    /// Estimated latency
    pub estimated_latency_ms: f64,
    /// Routing reason
    pub reason: String,
}

/// HSM selection result
pub type HsmSelectionResult = Result<String, BearDogError>;

/// Operation routing rules
#[derive(Debug, Clone, Default)]
pub struct OperationRoutingRules {
    /// Rules for operation types
    pub rules: HashMap<OperationType, Vec<String>>,
}

/// HSM operation router
pub struct HsmOperationRouter {
    /// Routing configuration
    config: OperationRouterConfig,
    /// Operation statistics
    stats: OperationStats,
}

impl HsmOperationRouter {
    /// Create a new operation router
    pub fn new() -> Self {
        Self {
            config: OperationRouterConfig::default(),
            stats: OperationStats::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: OperationRouterConfig) -> Self {
        Self {
            config,
            stats: OperationStats::default(),
        }
    }

    /// Route an operation to the best provider
    pub fn route_operation(
        &self,
        operation_type: &OperationType,
        available_providers: &[String],
    ) -> Result<RoutingDecision, BearDogError> {
        if available_providers.is_empty() {
            return Err(BearDogError::unavailable(
                "No providers available for operation".to_string(),
            ));
        }

        // Simple routing logic: prefer hardware for certain operations
        let is_hardware_preferred = self
            .config
            .hardware_preferred_operations
            .contains(operation_type);

        let provider_id = if is_hardware_preferred {
            // Try to find a hardware provider
            available_providers
                .iter()
                .find(|p| {
                    p.contains("hardware") || p.contains("strongbox") || p.contains("enclave")
                })
                .or_else(|| available_providers.first())
                // Safe: available_providers is guaranteed non-empty by check on line 141
                .ok_or_else(|| BearDogError::system(
                    "No available providers found despite non-empty check - invariant violated".to_string()
                ))?
                .clone()
        } else {
            // Prefer software
            available_providers
                .iter()
                .find(|p| p.contains("software"))
                .or_else(|| available_providers.first())
                // Safe: available_providers is guaranteed non-empty by check on line 141
                .ok_or_else(|| BearDogError::system(
                    "No available providers found despite non-empty check - invariant violated".to_string()
                ))?
                .clone()
        };

        let estimated_latency = self
            .stats
            .average_latency_ms
            .get(&provider_id)
            .copied()
            .unwrap_or(10.0);

        Ok(RoutingDecision {
            provider_id: provider_id.clone(),
            priority: if is_hardware_preferred { 1 } else { 2 },
            estimated_latency_ms: estimated_latency,
            reason: format!("Routed {operation_type:?} to {provider_id}"),
        })
    }

    /// Update operation statistics
    pub fn update_stats(&mut self, provider_id: String, success: bool, latency_ms: f64) {
        // Update operation count
        *self
            .stats
            .operations_processed
            .entry(provider_id.clone())
            .or_insert(0) += 1;

        // Update error count
        if !success {
            *self
                .stats
                .error_counts
                .entry(provider_id.clone())
                .or_insert(0) += 1;
        }

        // Update average latency (simple moving average)
        let current_avg = self
            .stats
            .average_latency_ms
            .entry(provider_id.clone())
            .or_insert(0.0);
        *current_avg = (*current_avg).mul_add(0.9, latency_ms * 0.1);

        // Calculate success rate
        let total_ops = self.stats.operations_processed[&provider_id];
        let errors = self
            .stats
            .error_counts
            .get(&provider_id)
            .copied()
            .unwrap_or(0);
        let success_rate = 1.0 - (errors as f64 / total_ops as f64);
        self.stats.success_rate.insert(provider_id, success_rate);
    }

    /// Get statistics
    pub const fn stats(&self) -> &OperationStats {
        &self.stats
    }

    /// Get configuration
    pub const fn config(&self) -> &OperationRouterConfig {
        &self.config
    }
}

impl Default for HsmOperationRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() -> Result<(), Box<dyn std::error::Error>> {
        let router = HsmOperationRouter::new();
        assert!(router.stats().success_rate.is_empty());
        Ok(())
    }

    #[test]
    fn test_route_operation() -> Result<(), Box<dyn std::error::Error>> {
        let router = HsmOperationRouter::new();
        let providers = vec!["software-hsm".to_string(), "hardware-hsm".to_string()];

        let decision = router.route_operation(&OperationType::KeyGeneration, &providers);
        assert!(decision.is_ok());

        let decision = decision?;
        assert!(decision.provider_id.contains("hardware"));
        Ok(())
    }

    #[test]
    fn test_route_operation_no_providers() -> Result<(), Box<dyn std::error::Error>> {
        let router = HsmOperationRouter::new();
        let providers = vec![];

        let decision = router.route_operation(&OperationType::Encryption, &providers);
        assert!(decision.is_err());
        Ok(())
    }

    #[test]
    fn test_update_stats() -> Result<(), Box<dyn std::error::Error>> {
        let mut router = HsmOperationRouter::new();

        router.update_stats("test-provider".to_string(), true, 15.0);
        router.update_stats("test-provider".to_string(), true, 25.0);
        router.update_stats("test-provider".to_string(), false, 50.0);

        assert_eq!(router.stats().operations_processed["test-provider"], 3);
        assert_eq!(router.stats().error_counts.get("test-provider"), Some(&1));
        Ok(())
    }

    #[test]
    fn test_success_rate_calculation() -> Result<(), Box<dyn std::error::Error>> {
        let mut router = HsmOperationRouter::new();

        router.update_stats("provider-1".to_string(), true, 10.0);
        router.update_stats("provider-1".to_string(), true, 10.0);
        router.update_stats("provider-1".to_string(), false, 10.0);

        let success_rate = router.stats().success_rate["provider-1"];
        assert!((success_rate - 0.666).abs() < 0.01);
        Ok(())
    }

    #[test]
    fn test_average_latency() -> Result<(), Box<dyn std::error::Error>> {
        let mut router = HsmOperationRouter::new();

        router.update_stats("provider-1".to_string(), true, 10.0);
        router.update_stats("provider-1".to_string(), true, 20.0);

        let avg_latency = router.stats().average_latency_ms["provider-1"];
        assert!(avg_latency > 0.0);
        assert!(avg_latency <= 20.0);
        Ok(())
    }

    #[test]
    fn test_operation_type_equality() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(OperationType::Signing, OperationType::Signing);
        assert_ne!(OperationType::Signing, OperationType::Encryption);
        Ok(())
    }

    #[test]
    fn test_routing_config_default() -> Result<(), Box<dyn std::error::Error>> {
        let config = OperationRouterConfig::default();
        assert!(!config.hardware_preferred_operations.is_empty());
        assert!(!config.software_preferred_operations.is_empty());
        assert_eq!(config.max_retries, 3);
        Ok(())
    }
}
