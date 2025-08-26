

use super::super::types::HsmConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleHsmTier {

    Smartphone,

    Software,

    Hardware,

    Hybrid,
}
impl std::fmt::Display for SimpleHsmTier {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleHsmTier::Smartphone => write!(f, "Smartphone"),
            SimpleHsmTier::Software => write!(f, "Software"),
            SimpleHsmTier::Hardware => write!(f, "Hardware"),
            SimpleHsmTier::Hybrid => write!(f, "Hybrid"),
        }
    }

#[derive(Debug, Clone, Default)]
pub struct HsmManagerConfig {

    pub hsm_configs: Vec<HsmConfig>,

    pub health_config: HealthConfig,

    pub failover_config: FailoverConfig,

    pub performance_config: PerformanceConfig,

#[derive(Debug, Clone)]
pub struct HealthConfig {

    pub check_interval: Duration,

    pub failure_threshold: u32,

    pub recovery_threshold: u32,

    pub timeout: Duration,

pub struct FailoverConfig {

    pub enabled: bool,

    pub max_retries: u32,

    pub retry_delay: Duration,

    pub circuit_breaker_threshold: u32,

    pub circuit_breaker_timeout: Duration,

impl Default for HealthConfig {}

    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 5,
            timeout: Duration::from_secs(5),
impl Default for FailoverConfig {
            enabled: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(30),}

impl Default for PerformanceConfig {
            enable_load_balancing: true,
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(30),
