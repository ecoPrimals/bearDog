// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration types for the hybrid intelligence system

mod deployment;
mod metrics_alerting;
mod monitoring;
mod optimizer;
mod preprocessing;
mod registry;
mod serving;
mod training_config;

#[expect(
    deprecated,
    reason = "re-export of deprecated type for backward-compatible migration path"
)]
pub use deployment::HealthCheckConfig;
pub use deployment::{DeploymentConfig, DeploymentStrategy, ResourceRequirements};
pub use metrics_alerting::{
    AlertRule, AlertingConfig, ComparisonOperator, MetricType, NotificationChannel,
};
#[expect(
    deprecated,
    reason = "re-export of deprecated type for backward-compatible migration path"
)]
pub use monitoring::MonitoringConfig;
pub use monitoring::{AIMetricType, AIMonitoringConfig};
pub use optimizer::{
    EarlyStoppingConfig, LearningRateSchedule, OptimizerConfig, OptimizerType, RegularizationConfig,
};
pub use preprocessing::{
    AugmentationTechnique, DataAugmentationConfig, FeatureSelectionConfig, FeatureSelectionMethod,
    MissingValueStrategy, NormalizationStrategy,
};
#[expect(
    deprecated,
    reason = "re-export of deprecated type for backward-compatible migration path"
)]
pub use registry::RegistryConfig;
pub use registry::{AIRegistryConfig, AuthConfig, AuthType, RegistryType, VersioningStrategy};
pub use serving::{CachingConfig, EvictionPolicy, LoadBalancingStrategy, ServingConfig};
pub use training_config::{
    DecisionEngineConfig, InferenceConfig, LearningConfig, ModelManagementConfig,
    NeuralNetworkConfig, OptimizationConfig, PredictionConfig, PreprocessingConfig, TrainingConfig,
};

// ═══════════════════════════════════════════════════════════════════════════
// REMOVED: Deprecated type aliases (Nov 8, 2025)
// ═══════════════════════════════════════════════════════════════════════════
//
// The following deprecated type aliases were removed as they had zero usage:
// - LoggingConfig → use beardog_types::canonical::config::domains::system::LoggingConfig
// - LogLevel → use beardog_types::canonical::config::domains::system::LogLevel
// - LogFormat → use beardog_types::canonical::config::domains::system::LogFormat
// - LogDestination → use beardog_types::canonical::config::domains::system::LogTargetType
//
// All types are available at their canonical locations in beardog-types.
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests;
