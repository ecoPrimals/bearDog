// SPDX-License-Identifier: AGPL-3.0-only

//! Deployment strategies, resource requirements, and health-check type aliases.

use serde::{Deserialize, Serialize};

/// Deployment configuration for model rollout strategies
///
/// Defines how models are deployed to production environments,
/// including deployment strategy, resource allocation, and health monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Strategy for rolling out model updates (blue-green, canary, rolling, or recreate)
    pub strategy: DeploymentStrategy,
    /// Required computational resources (CPU, memory, GPU, storage)
    pub resources: ResourceRequirements,
    /// Configuration for monitoring deployment health and readiness
    pub health_check:
        beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            strategy: DeploymentStrategy::Rolling,
            resources: ResourceRequirements::default(),
            health_check: beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration::default(),
        }
    }
}

/// Deployment strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentStrategy {
    /// Blue-green deployment
    BlueGreen,
    /// Canary deployment
    Canary,
    /// Rolling deployment
    Rolling,
    /// Recreate deployment
    Recreate,
}

/// Resource requirements for model deployment
///
/// Specifies the computational resources required to run a model
/// in production, used for scheduling and capacity planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Number of CPU cores required (fractional values allowed, e.g., 0.5 for half a core)
    pub cpu: f64,
    /// Memory requirement in megabytes (MB)
    pub memory: u64,
    /// Optional number of GPU devices required for acceleration
    pub gpu: Option<u32>,
    /// Persistent storage requirement in gigabytes (GB) for model artifacts and logs
    pub storage: u64,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu: 1.0,
            memory: 1024,
            gpu: None,
            storage: 10,
        }
    }
}

impl ResourceRequirements {
    /// Load resource hints from `BEARDOG_AI_RESOURCE_*` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            cpu: std::env::var("BEARDOG_AI_RESOURCE_CPU")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1.0),
            memory: std::env::var("BEARDOG_AI_RESOURCE_MEMORY_MB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024),
            gpu: std::env::var("BEARDOG_AI_RESOURCE_GPU")
                .ok()
                .and_then(|s| s.parse().ok()),
            storage: std::env::var("BEARDOG_AI_RESOURCE_STORAGE_GB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }
}

/// Health check configuration
///
/// **DEPRECATED**: Use `beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration instead"
)]
pub type HealthCheckConfig =
    beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration;

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration;
    use serde_json;

    #[test]
    fn deployment_config_default_roundtrip() {
        let d = DeploymentConfig::default();
        assert!(matches!(d.strategy, DeploymentStrategy::Rolling));
        let j = serde_json::to_string(&d).expect("serialize DeploymentConfig");
        let back: DeploymentConfig = serde_json::from_str(&j).expect("deserialize");
        assert_eq!(d.strategy, back.strategy);
        assert_eq!(d.resources.cpu, back.resources.cpu);
        assert_eq!(d.resources.memory, back.resources.memory);
    }

    #[test]
    fn deployment_strategy_variants_serde() {
        for s in [
            DeploymentStrategy::BlueGreen,
            DeploymentStrategy::Canary,
            DeploymentStrategy::Rolling,
            DeploymentStrategy::Recreate,
        ] {
            let j = serde_json::to_string(&s).expect("serialize strategy");
            let back: DeploymentStrategy = serde_json::from_str(&j).expect("deserialize");
            assert_eq!(s, back);
        }
    }

    #[test]
    fn resource_requirements_default_and_from_env() {
        let r = ResourceRequirements::default();
        assert_eq!(r.cpu, 1.0);
        assert_eq!(r.memory, 1024);
        assert!(r.gpu.is_none());
        assert_eq!(r.storage, 10);
        let r2 = ResourceRequirements::from_env();
        assert!(r2.cpu.is_finite());
        assert!(r2.memory > 0);
    }

    #[test]
    fn resource_requirements_serde_roundtrip() {
        let r = ResourceRequirements {
            cpu: 2.5,
            memory: 4096,
            gpu: Some(1),
            storage: 50,
        };
        let j = serde_json::to_string(&r).expect("serialize ResourceRequirements");
        let back: ResourceRequirements = serde_json::from_str(&j).expect("deserialize");
        assert_eq!(r.cpu, back.cpu);
        assert_eq!(r.memory, back.memory);
        assert_eq!(r.gpu, back.gpu);
        assert_eq!(r.storage, back.storage);
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_health_check_alias_matches_target() {
        let h: HealthCheckConfig = HealthCheckConfiguration::default();
        assert!(h.enabled);
        assert_eq!(h.interval_seconds, 30);
        assert_eq!(h.endpoint, "/health");
    }
}
