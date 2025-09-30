//! # Operational Configuration Module
//!
//! This module contains operational configurations:
//! Production and Development configs.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

// ============================================================================
// Production Configuration
// ============================================================================

/// **PRODUCTION CONFIGURATION** - Production environment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// Production core settings
    pub core: ProductionCoreConfig,
    /// Reliability settings
    pub reliability: ReliabilityConfig,
    /// Scaling configuration
    pub scaling: ScalingConfig,
    /// Operations configuration
    pub operations: OperationsConfig,
}

/// Production core configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCoreConfig {
    /// Production mode enabled
    pub enabled: bool,
    /// Environment name
    pub environment_name: String,
}

/// Reliability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReliabilityConfig {
    /// Enable circuit breakers
    pub circuit_breakers: bool,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Health check enabled
    pub health_checks: bool,
}

/// Scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Enable auto-scaling
    pub auto_scaling: bool,
    /// Minimum instances
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
}

/// Operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationsConfig {
    /// Enable maintenance mode
    pub maintenance_mode: bool,
    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,
}

// ============================================================================
// Development Configuration
// ============================================================================

/// **DEVELOPMENT CONFIGURATION** - Development environment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    /// Development core settings
    pub core: DevelopmentCoreConfig,
    /// Debugging configuration
    pub debugging: DebuggingConfig,
    /// Hot reload configuration
    pub hot_reload: HotReloadConfig,
    /// Development tools configuration
    pub tools: DevelopmentToolsConfig,
}

/// Development core configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentCoreConfig {
    /// Development mode enabled
    pub enabled: bool,
    /// Debug level
    pub debug_level: String,
}

/// Debugging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggingConfig {
    /// Enable debugging
    pub enabled: bool,
    /// Debug port
    pub port: Option<u16>,
}

/// Hot reload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotReloadConfig {
    /// Enable hot reload
    pub enabled: bool,
    /// Watch directories
    pub watch_directories: Vec<PathBuf>,
}

/// Development tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentToolsConfig {
    /// Enable linting
    pub linting: bool,
    /// Enable formatting
    pub formatting: bool,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            core: ProductionCoreConfig::default(),
            reliability: ReliabilityConfig::default(),
            scaling: ScalingConfig::default(),
            operations: OperationsConfig::default(),
        }
    }
}

impl Default for ProductionCoreConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            environment_name: "production".to_string(),
        }
    }
}

impl Default for ReliabilityConfig {
    fn default() -> Self {
        Self {
            circuit_breakers: true,
            retry_attempts: 3,
            health_checks: true,
        }
    }
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            auto_scaling: true,
            min_instances: 1,
            max_instances: 10,
        }
    }
}

impl Default for OperationsConfig {
    fn default() -> Self {
        Self {
            maintenance_mode: false,
            shutdown_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for DevelopmentConfig {
    fn default() -> Self {
        Self {
            core: DevelopmentCoreConfig::default(),
            debugging: DebuggingConfig::default(),
            hot_reload: HotReloadConfig::default(),
            tools: DevelopmentToolsConfig::default(),
        }
    }
}

impl Default for DevelopmentCoreConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            debug_level: "debug".to_string(),
        }
    }
}

impl Default for DebuggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: Some(9229),
        }
    }
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_directories: vec![PathBuf::from("src"), PathBuf::from("config")],
        }
    }
}

impl Default for DevelopmentToolsConfig {
    fn default() -> Self {
        Self {
            linting: true,
            formatting: true,
        }
    }
}
