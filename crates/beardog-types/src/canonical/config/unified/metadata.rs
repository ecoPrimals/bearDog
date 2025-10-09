//! System Metadata and Environment Configuration
//!
//! This module contains metadata types for system identification, versioning,
//! and environment configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// System metadata including version, environment, and instance information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetadata {
    /// The version value
    pub version: UnifiedVersionInfo,

    /// **ENVIRONMENT CONFIGURATION**
    /// Current deployment environment (dev, staging, prod)
    /// The environment value
    pub environment: Environment,
    /// Deployment mode and strategy
    /// The deployment mode value
    pub deployment_mode: DeploymentMode,
    /// Mapping of feature flags
    pub feature_flags: HashMap<String, bool>,
    pub rollout_config: RolloutConfig,

    /// **SYSTEM IDENTIFICATION**
    pub instance_id: String,
    pub cluster_id: Option<String>,
    /// Unique node identifier within the cluster
    pub node_id: String,
    /// Optional region
    pub region: Option<String>,
    /// Availability zone within the region
    /// Optional zone
    pub zone: Option<String>,
}

/// **UNIFIED VERSION INFORMATION** - Eliminates all version constant duplication
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedVersionInfo {
    /// **PRIMARY VERSIONS** - Core system versions
    /// Main `BearDog` system version (e.g., "3.0.0")
    /// The beardog version value
    pub beardog_version: String, // "3.0.0" - Main BearDog version
    pub config_schema_version: String, // "1.0.0" - Configuration schema version
    /// The api version value
    pub api_version: String, // "2.0" - API protocol version

    /// **COMPONENT VERSIONS** - Individual component versions
    /// The workflow system version value
    pub workflow_system_version: String, // "3.1.0" - Workflow engine version
    /// Hsm Foundation Version
    /// The hsm foundation version value
    pub hsm_foundation_version: String, // "2.0.0-clean" - HSM foundation version
    /// Software Hsm Version
    /// The software hsm version value
    pub software_hsm_version: String, // "1.0.0" - Software HSM version
    /// Tunnel Version
    /// The tunnel version value
    pub tunnel_version: String, // "1.0.0" - Tunnel system version
    /// Genetics Version
    /// The genetics version value
    pub genetics_version: String, // "1.0.0" - Genetics engine version

    /// **COMPATIBILITY VERSIONS** - Supported version ranges
    /// The min client version value
    pub min_client_version: String, // "1.0.0" - Minimum supported client
    /// Min Rust Version
    /// The min rust version value
    pub min_rust_version: String, // "1.70.0" - Minimum Rust version
    /// Max Supported Version
    /// The max supported version value
    pub max_supported_version: String, // "4.0.0" - Maximum supported version

    /// **BUILD INFORMATION**
    pub build_timestamp: String,
    /// Git Commit
    /// Optional git commit
    pub git_commit: Option<String>,
    /// Build Profile
    /// The build profile value
    pub build_profile: String, // "release", "debug", "production"
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Deployment environment configuration
pub enum Environment {
    #[default]
    /// Development Environment
    Development,
    /// Testing variant
    Testing,
    /// Staging variant
    Staging,
    /// Production variant
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DeploymentMode {
    /// Standalone deployment mode
    #[default]
    /// Represents standalone variant
    Standalone,
    /// Cluster variant
    Cluster,
    /// Federation variant
    Federation,
    /// Cloud variant
    Cloud,
}

/// Logging level configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    /// Trace variant
    Trace,
    /// Debug variant
    Debug,
    /// Info log level (default)
    #[default]
    /// Represents info variant
    Info,
    /// Warn variant
    Warn,
    /// Error variant
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RolloutConfig {
    /// Enable Gradual Rollout
    /// Whether `enable_gradual_rollout` is enabled
    pub enable_gradual_rollout: bool,
    /// Rollout Percentage
    /// The rollout percentage value
    pub rollout_percentage: f64,
    /// Rollout Strategy
    /// The rollout strategy value
    pub rollout_strategy: RolloutStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RolloutStrategy {
    /// Blue-green deployment strategy (default)
    #[default]
    /// Represents blue green variant
    BlueGreen,
    /// Canary variant
    Canary,
    /// `RollingUpdate` variant
    RollingUpdate,
    /// Immediate variant
    Immediate,
}
