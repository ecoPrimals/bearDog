//! Production Deployment Configuration
//!
//! This module provides comprehensive production deployment configuration for BearDog,
//! including deployment strategies, environment management, validation, and monitoring.

use serde::{Deserialize, Serialize};

pub mod environment;
pub mod health_checks;
pub mod monitoring;
pub mod network;
pub mod rollback;
pub mod security;
pub mod services;
pub mod storage;
pub mod strategy;
pub mod validation;

pub use environment::*;
pub use health_checks::*;
pub use monitoring::*;
pub use network::*;
pub use rollback::*;
pub use security::*;
pub use services::*;
pub use storage::*;
pub use strategy::*;
pub use validation::*;

/// Production deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionDeploymentConfig {
    /// Enable production deployment
    pub enabled: bool,
    /// Deployment strategy
    pub strategy: DeploymentStrategy,
    /// Environment configuration
    pub environment: DeploymentEnvironment,
    /// Validation configuration
    pub validation: DeploymentValidation,
    /// Rollback configuration
    pub rollback: RollbackConfig,
    /// Health checks configuration
    pub health_checks: DeploymentHealthChecks,
    /// Security configuration
    pub security: DeploymentSecurity,
    /// Monitoring configuration
    pub monitoring: DeploymentMonitoring,
}

/// Deployment resource estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResourceEstimate {
    /// CPU cores required
    pub cpu_cores: u32,
    /// Memory in GB
    pub memory_gb: u32,
    /// Storage in GB
    pub storage_gb: u32,
    /// Network bandwidth in Mbps
    pub network_mbps: u32,
    /// Estimated cost per hour
    pub estimated_cost_per_hour: f64,
}

impl Default for ProductionDeploymentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: DeploymentStrategy::default(),
            environment: DeploymentEnvironment::default(),
            validation: DeploymentValidation::default(),
            rollback: RollbackConfig::default(),
            health_checks: DeploymentHealthChecks::default(),
            security: DeploymentSecurity::default(),
            monitoring: DeploymentMonitoring::default(),
        }
    }
}

impl ProductionDeploymentConfig {
    /// Create a production configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            strategy: DeploymentStrategy::BlueGreen {
                blue: EnvironmentConfig::default(),
                green: EnvironmentConfig::default(),
                switch: SwitchConfig::default(),
            },
            environment: DeploymentEnvironment::production(),
            validation: DeploymentValidation::production(),
            rollback: RollbackConfig::production(),
            health_checks: DeploymentHealthChecks::production(),
            security: DeploymentSecurity::production(),
            monitoring: DeploymentMonitoring::production(),
        }
    }

    /// Create a development configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            strategy: DeploymentStrategy::Immediate {
                pre_checks: Vec::new(),
                post_checks: Vec::new(),
            },
            environment: DeploymentEnvironment::development(),
            validation: DeploymentValidation::development(),
            rollback: RollbackConfig::development(),
            health_checks: DeploymentHealthChecks::development(),
            security: DeploymentSecurity::development(),
            monitoring: DeploymentMonitoring::development(),
        }
    }

    /// Calculate deployment resources
    pub fn calculate_deployment_resources(&self) -> DeploymentResourceEstimate {
        let (cpu_cores, memory_gb, storage_gb, network_mbps) = match &self.strategy {
            DeploymentStrategy::BlueGreen { .. } => {
                // Blue-green needs double resources
                (64, 128, 2048, 20000)
            }
            DeploymentStrategy::Rolling { .. } => {
                // Rolling deployment needs extra capacity
                (48, 96, 1536, 15000)
            }
            DeploymentStrategy::Canary { .. } => {
                // Canary needs additional resources for monitoring
                (40, 80, 1280, 12000)
            }
            DeploymentStrategy::Immediate { .. } => {
                // Immediate deployment uses base resources
                (32, 64, 1024, 10000)
            }
        };

        DeploymentResourceEstimate {
            cpu_cores,
            memory_gb,
            storage_gb,
            network_mbps,
            estimated_cost_per_hour: Self::estimate_deployment_cost(
                cpu_cores, memory_gb, storage_gb,
            ),
        }
    }

    /// Estimate deployment cost
    fn estimate_deployment_cost(cpu_cores: u32, memory_gb: u32, storage_gb: u32) -> f64 {
        // AWS pricing estimates for deployment
        let cpu_cost_per_hour = 0.048; // c5.large pricing
        let memory_cost_per_gb_hour = 0.0058;
        let storage_cost_per_gb_hour = 0.10 / 24.0 / 30.0;

        let cpu_cost = (cpu_cores as f64 / 2.0) * cpu_cost_per_hour; // 2 vCPUs per core
        let memory_cost = memory_gb as f64 * memory_cost_per_gb_hour;
        let storage_cost = storage_gb as f64 * storage_cost_per_gb_hour;

        cpu_cost + memory_cost + storage_cost
    }

    /// Generate deployment configuration summary
    pub fn generate_deployment_summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str("Production Deployment Configuration Summary\n");
        summary.push_str("==========================================\n\n");

        summary.push_str(&format!(
            "Deployment Strategy: {}\n",
            match &self.strategy {
                DeploymentStrategy::BlueGreen { .. } => "Blue-Green",
                DeploymentStrategy::Rolling { .. } => "Rolling",
                DeploymentStrategy::Canary { .. } => "Canary",
                DeploymentStrategy::Immediate { .. } => "Immediate",
            }
        ));

        summary.push_str(&format!("Environment: {}\n", self.environment.name));
        summary.push_str(&format!(
            "Validation Checks: {}\n",
            self.validation.checks.len()
        ));
        summary.push_str(&format!(
            "Health Check Endpoints: {}\n",
            self.health_checks.endpoints.len()
        ));
        summary.push_str(&format!(
            "Security Policies: {}\n",
            self.security.policies.len()
        ));
        summary.push_str(&format!(
            "Monitoring Enabled: {}\n",
            self.monitoring.enabled
        ));
        summary.push_str(&format!("Rollback Enabled: {}\n", self.rollback.enabled));

        let resources = self.calculate_deployment_resources();
        summary.push_str("\nDeployment Resource Requirements:\n");
        summary.push_str(&format!("- CPU Cores: {}\n", resources.cpu_cores));
        summary.push_str(&format!("- Memory: {} GB\n", resources.memory_gb));
        summary.push_str(&format!("- Storage: {} GB\n", resources.storage_gb));
        summary.push_str(&format!("- Network: {} Mbps\n", resources.network_mbps));
        summary.push_str(&format!(
            "- Estimated Cost: ${:.2}/hour\n",
            resources.estimated_cost_per_hour
        ));

        summary
    }
}
