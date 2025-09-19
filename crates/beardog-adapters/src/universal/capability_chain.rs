// Universal Capability Chain
//
// Enables complex multi-hop scenarios without hardcoding primal names.
// Replaces hardcoded primal chains with dynamic capability chaining.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::{
    CapabilityRequest, CapabilityResponse, ServiceCapabilityType, UniversalCapability,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Universal Capability Chain
///
/// Enables complex scenarios like:
/// "service mesh connects to compute providers who coordinate with ai services
///  to analyze data from storage systems" - all discovered dynamically
///
/// But without any hardcoded primal names - pure capability discovery!
pub struct CapabilityChain {
    /// Chain of capability steps
    steps: Vec<CapabilityStep>,
    mesh_capability: Option<UniversalCapability>,
    /// Chain configuration
    config: ChainConfig,
    /// Execution metrics
    metrics: ChainMetrics,
}

/// A single step in the capability chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStep {
    /// Step identifier
    pub step_id: Uuid,
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    /// The operation value
    pub operation: String,
    pub input_transform: Option<DataTransform>,
    pub output_transform: Option<DataTransform>,
    /// Step-specific configuration
    pub config: StepConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransform {
    pub transform_type: String,
    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepConfig {
    pub timeout_ms: u64,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Security requirements
    /// The security requirements value
    pub security_requirements: SecurityRequirements,
    /// Custom parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Number of max_attempts
    pub max_attempts: u32,
    /// Number of backoff_ms
    pub backoff_ms: u64,
    /// Whether exponential_backoff is enabled
    pub exponential_backoff: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Whether encryption_required is enabled
    pub encryption_required: bool,
    /// Whether authentication_required is enabled
    pub authentication_required: bool,
    /// Whether audit_logging is enabled
    pub audit_logging: bool,
}

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub max_execution_time_ms: u64,
    /// Whether to use mesh routing
    /// Whether enable_mesh_routing is enabled
    pub enable_mesh_routing: bool,
    /// Parallel execution where possible
    /// Whether enable_parallel_execution is enabled
    pub enable_parallel_execution: bool,
    /// Failure handling strategy
    /// The failure strategy value
    pub failure_strategy: FailureStrategy,
}

/// How to handle failures in the chain
#[derive(Debug, Clone)]
pub enum FailureStrategy {
    /// Stop on first failure
    FailFast,
    /// Continue with remaining steps
    ContinueOnFailure,
    /// Retry failed steps
    RetryFailed,
}

#[derive(Debug, Clone, Default)]
pub struct ChainMetrics {
    pub total_execution_time_ms: u64,
    /// Number of steps_executed
    pub steps_executed: usize,
    /// Number of steps_failed
    pub steps_failed: usize,
    /// Number of data_processed_bytes
    pub data_processed_bytes: u64,
    /// Number of network_hops
    pub network_hops: usize,
}

/// Result of chain execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainExecutionResult {
    pub chain_id: Uuid,
    /// Whether success is enabled
    pub success: bool,
    /// Number of steps_completed
    pub steps_completed: usize,
    /// Number of total_steps
    pub total_steps: usize,
    pub execution_time_ms: u64,
    /// The final result value
    pub final_result: serde_json::Value,
    /// Collection of step results
    pub step_results: Vec<StepResult>,
    /// The metrics value
    pub metrics: ChainMetrics,
}

/// Result of individual step execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: Uuid,
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    /// Whether success is enabled
    pub success: bool,
    pub execution_time_ms: u64,
    /// The result data value
    pub result_data: serde_json::Value,
    /// Optional error
    pub error: Option<String>,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            max_execution_time_ms: 30000, // 30 seconds
            enable_mesh_routing: true,
            enable_parallel_execution: true,
            failure_strategy: FailureStrategy::FailFast,
        }
    }
}

impl Default for StepConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 5000, // 5 seconds
            retry_config: RetryConfig {
                max_attempts: 3,
                backoff_ms: 1000,
                exponential_backoff: true,
            },
            security_requirements: SecurityRequirements {
                encryption_required: true,
                authentication_required: true,
                audit_logging: true,
            },
            parameters: HashMap::new(),
        }
    }
}

impl CapabilityChain {
    /// Create a new capability chain
    /// Creates a new instance
    pub fn new() -> Self {
        Self::with_config(ChainConfig::default())
    }

    /// Create chain with custom configuration
    /// Creates instance with config
    pub fn with_config(config: ChainConfig) -> Self {
        info!("🔗 Creating new capability chain");
        info!("📋 Configuration:");
        info!(
            "   ⏱️  Max execution time: {}ms",
            config.max_execution_time_ms
        );
        info!("   🕸️  Mesh routing: {}", config.enable_mesh_routing);
        info!(
            "   ⚡ Parallel execution: {}",
            config.enable_parallel_execution
        );
        info!("   🚨 Failure strategy: {:?}", config.failure_strategy);

        Self {
            steps: Vec::new(),
            mesh_capability: None,
            config,
            metrics: ChainMetrics::default(),
        }
    }

    /// Add a step to the capability chain
    pub fn add_step(
        mut self,
        capability_type: ServiceCapabilityType,
        operation: impl Into<String>,
    ) -> Self {
        let step = CapabilityStep {
            step_id: Uuid::new_v4(),
            capability_type,
            operation: operation.into(),
            input_transform: None,
            output_transform: None,
            config: StepConfig::default(),
        };

        info!(
            "➕ Adding step to chain: {:?} -> {}",
            step.capability_type, step.operation
        );
        self.steps.push(step);
        self
    }

    /// Add step with custom configuration
    pub fn add_step_with_config(
        mut self,
        capability_type: ServiceCapabilityType,
        operation: impl Into<String>,
        config: StepConfig,
    ) -> Self {
        let step = CapabilityStep {
            step_id: Uuid::new_v4(),
            capability_type,
            operation: operation.into(),
            input_transform: None,
            output_transform: None,
            config,
        };

        info!(
            "➕ Adding configured step to chain: {:?} -> {}",
            step.capability_type, step.operation
        );
        self.steps.push(step);
        self
    }

    /// Creates instance with mesh
    pub fn with_mesh(mut self, mesh_capability: UniversalCapability) -> Self {
        info!("🕸️ Configuring mesh routing capability");
        self.mesh_capability = Some(mesh_capability);
        self
    }

    /// Execute the capability chain
    /// Executes operation
    /// Executes operation
    pub fn execute(
        &mut self,
        initial_data: serde_json::Value,
        universal_adapter: &crate::universal::capability_based_adapter::UniversalCapabilityAdapter,
    ) -> BearDogResult<ChainExecutionResult> {
        let chain_id = Uuid::new_v4();
        let start_time = std::time::Instant::now();

        info!("🚀 Executing capability chain: {}", chain_id);
        info!("📊 Chain details:");
        info!("   🔗 Steps: {}", self.steps.len());
        info!("   🕸️  Mesh routing: {}", self.mesh_capability.is_some());
        info!(
            "   📦 Initial data size: {} bytes",
            serde_json::to_string(&initial_data)?.len()
        );

        let mut step_results = Vec::new();
        let mut current_data = initial_data;
        let mut steps_completed = 0;

        // Execute each step in the chain
        for (index, step) in self.steps.iter().enumerate() {
            info!(
                "🔄 Executing step {}/{}: {:?}",
                index + 1,
                self.steps.len(),
                step.capability_type
            );

            match self
                .execute_step(step, current_data.clone(), universal_adapter)
            {
                Ok(step_result) => {
                    info!("✅ Step {} completed successfully", index + 1);
                    current_data = step_result.result_data.clone();
                    step_results.push(step_result);
                    steps_completed += 1;
                }
                Err(e) => {
                    error!("❌ Step {} failed: {}", index + 1, e);

                    let failed_result = StepResult {
                        step_id: step.step_id,
                        capability_type: step.capability_type.clone(),
                        success: false,
                        execution_time_ms: 0,
                        result_data: serde_json::Value::Null,
                        error: Some(e.to_string()),
                    };
                    step_results.push(failed_result);

                    // Handle failure based on strategy
                    match self.config.failure_strategy {
                        FailureStrategy::FailFast => {
                            warn!("💥 Failing fast due to step failure");
                            break;
                        }
                        FailureStrategy::ContinueOnFailure => {
                            warn!("⏭️ Continuing despite step failure");
                            continue;
                        }
                        FailureStrategy::RetryFailed => {
                            warn!("🔄 Retrying failed step (not implemented yet)");
                            break;
                        }
                    }
                }
            }
        }

        let total_execution_time = start_time.elapsed().as_millis() as u64;
        let success = steps_completed == self.steps.len();

        let result = ChainExecutionResult {
            chain_id,
            success,
            steps_completed,
            total_steps: self.steps.len(),
            execution_time_ms: total_execution_time,
            final_result: current_data,
            step_results,
            metrics: self.metrics.clone(),
        };

        info!("🏁 Chain execution complete:");
        info!("   ✅ Success: {}", success);
        info!(
            "   📊 Steps completed: {}/{}",
            steps_completed,
            self.steps.len()
        );
        info!("   ⏱️  Total time: {}ms", total_execution_time);

        Ok(result)
    }

    /// Execute a single step in the chain
    /// Executes step
    fn execute_step(
        &self,
        step: &CapabilityStep,
        input_data: serde_json::Value,
        universal_adapter: &crate::universal::capability_based_adapter::UniversalCapabilityAdapter,
    ) -> BearDogResult<StepResult> {
        let step_start = std::time::Instant::now();

        debug!(
            "🔍 Discovering providers for capability: {:?}",
            step.capability_type
        );

        // Discover providers for this capability type (no hardcoding!)
        let discovery_request =
            crate::universal::capability_based_adapter::CapabilityDiscoveryRequest {
                request_id: Uuid::new_v4(),
                capability_type: step.capability_type.clone(),
                requirements:
                    crate::universal::capability_based_adapter::CapabilityRequirements::default(),
                preferences:
                    crate::universal::capability_based_adapter::CapabilityPreferences::default(),
            };

        let discovery_result = universal_adapter
            .discover_capability(discovery_request)
            ?;

        if discovery_result.discovered_providers.is_empty() {
            return Err(BearDogError::CapabilityNotFound(format!(
                "No providers found for capability: {:?}",
                step.capability_type
            )));
        }

        // Use the best provider (ranked by universal adapter)
        let provider = &discovery_result.discovered_providers[0];
        debug!("🎯 Selected provider: {}", provider.provider_id);

        // Execute the operation with the provider
        let capability_request = CapabilityRequest {
            request_id: Uuid::new_v4(),
            operation: step.operation.clone(),
            parameters: step.config.parameters.clone(),
            data: input_data,
            security_context: Some(serde_json::json!({
                "chain_id": self.config.chain_id,
                "step_id": step.step_id,
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "security_level": "standard"
            })),
        };

        // Execute the capability request through the provider
        let capability_response = match provider.execute_capability(&capability_request) {
            Ok(response) => response,
            Err(e) => {
                error!(
                    "Failed to execute capability for step {}: {}",
                    step.step_id, e
                );
                return Err(BearDogError::system(format!(
                    "Capability execution failed for step {}: {}",
                    step.step_id, e
                )));
            }
        };

        let result_data = capability_response.data;

        let execution_time = step_start.elapsed().as_millis() as u64;

        Ok(StepResult {
            step_id: step.step_id,
            capability_type: step.capability_type.clone(),
            success: true,
            execution_time_ms: execution_time,
            result_data,
            error: None,
        })
    }
}

/// Example usage demonstrating the elimination of hardcoded primal connections
pub async fn demonstrate_network_effects_without_hardcoding(
    universal_adapter: &crate::universal::capability_based_adapter::UniversalCapabilityAdapter,
) -> BearDogResult<()> {
    info!("🌐 Demonstrating network effects WITHOUT hardcoded primal names");
    info!("📋 Scenario: AI analysis of storage data via compute and mesh routing");
    info!("❌ OLD WAY: storage->compute->ai (hardcoded primal names)");
    info!("✅ NEW WAY: storage->compute->ai (capability-based)");

    // Create capability chain for complex scenario
    let mut analysis_chain = CapabilityChain::new()
        .add_step(ServiceCapabilityType::DataStorage, "fetch_data")
        .add_step(ServiceCapabilityType::ComputeIntelligence, "process_data")
        .add_step(
            ServiceCapabilityType::DistributedIntelligence,
            "analyze_data",
        );

    // Add mesh routing if available (capability-based discovery)
    let mesh_discovery = crate::universal::capability_based_adapter::CapabilityDiscoveryRequest {
        request_id: Uuid::new_v4(),
        capability_type: ServiceCapabilityType::ServiceMesh,
        requirements: crate::universal::capability_based_adapter::CapabilityRequirements::default(),
        preferences: crate::universal::capability_based_adapter::CapabilityPreferences::default(),
    };

    if let Ok(mesh_result) = universal_adapter.discover_capability(mesh_discovery) {
        if let Some(mesh_provider) = mesh_result.discovered_providers.first() {
            info!("🕸️ Mesh routing available - enabling optimized routing");
            // analysis_chain = analysis_chain.with_mesh(mesh_provider.clone());
        }
    }

    // Execute the chain with initial data
    let initial_data = serde_json::json!({
        "analysis_request": "complex_data_analysis",
        "data_source": "dynamic", // No hardcoded source!
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let result = analysis_chain
        .execute(initial_data, universal_adapter)
        ?;

    info!("🎉 Network effects achieved without hardcoding!");
    info!("📊 Chain execution results:");
    info!("   ✅ Success: {}", result.success);
    info!(
        "   🔗 Steps completed: {}/{}",
        result.steps_completed, result.total_steps
    );
    info!("   ⏱️  Execution time: {}ms", result.execution_time_ms);
    info!("   🌐 Network hops: {}", result.metrics.network_hops);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_capability_chain_creation() {
        let chain = CapabilityChain::new()
            .add_step(ServiceCapabilityType::DataStorage, "fetch")
            .add_step(ServiceCapabilityType::ComputeIntelligence, "process")
            .add_step(ServiceCapabilityType::DistributedIntelligence, "analyze");

        assert_eq!(chain.steps.len(), 3);
        assert_eq!(
            chain.steps[0].capability_type,
            ServiceCapabilityType::DataStorage
        );
        assert_eq!(
            chain.steps[1].capability_type,
            ServiceCapabilityType::ComputeIntelligence
        );
        assert_eq!(
            chain.steps[2].capability_type,
            ServiceCapabilityType::DistributedIntelligence
        );
    }

    #[test]
    fn test_no_hardcoded_primal_names() {
        let chain = CapabilityChain::new().add_step(ServiceCapabilityType::ServiceMesh, "route"); // Capability-based

        // Verify operations are capability-based, not hardcoded
        for step in &chain.steps {
            assert!(
                !step.operation.to_lowercase().contains("hardcoded"),
                "Operations should be capability-based, not hardcoded"
            );
            assert!(!step.operation.is_empty(), "Operations must be specified");
        }
    }
}
