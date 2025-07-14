//! # HSM Manager Configuration
//!
//! This module contains all configuration structures for the HSM manager,
//! including health monitoring, failover, performance, and overall manager config.

use std::time::Duration;
use super::super::types::HsmConfig;

/// Simple HSM tier enum for internal tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleHsmTier {
    Smartphone,
    Software,
    Hardware,
    Hybrid,
}

impl SimpleHsmTier {
    pub fn to_string(&self) -> String {
        match self {
            SimpleHsmTier::Smartphone => "Smartphone".to_string(),
            SimpleHsmTier::Software => "Software".to_string(),
            SimpleHsmTier::Hardware => "Hardware".to_string(),
            SimpleHsmTier::Hybrid => "Hybrid".to_string(),
        }
    }
}

/// HSM Manager configuration
#[derive(Debug, Clone)]
pub struct HsmManagerConfig {
    pub hsm_configs: Vec<HsmConfig>,
    pub health_config: HealthConfig,
    pub failover_config: FailoverConfig,
    pub performance_config: PerformanceConfig,
}

/// Health monitoring configuration
#[derive(Debug, Clone)]
pub struct HealthConfig {
    pub check_interval: Duration,
    pub failure_threshold: u32,
    pub recovery_threshold: u32,
    pub timeout: Duration,
}

/// Failover configuration
#[derive(Debug, Clone)]
pub struct FailoverConfig {
    pub enabled: bool,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub circuit_breaker_threshold: u32,
    pub circuit_breaker_timeout: Duration,
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_load_balancing: bool,
    pub enable_caching: bool,
    pub max_concurrent_operations: usize,
    pub operation_timeout: Duration,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 5,
            timeout: Duration::from_secs(5),
        }
    }
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_load_balancing: true,
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for HsmManagerConfig {
    fn default() -> Self {
        Self {
            hsm_configs: Vec::new(),
            health_config: HealthConfig::default(),
            failover_config: FailoverConfig::default(),
            performance_config: PerformanceConfig::default(),
        }
    }
} 