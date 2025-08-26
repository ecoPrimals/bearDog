

use crate::tunnel::hsm::types::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleHsmTier {
    Smartphone,
    Software,
    Hardware,
    Hybrid,
}
impl SimpleHsmTier {}

    pub fn to_string(&self) -> String {
        match self {
            SimpleHsmTier::Smartphone => "Smartphone".to_string(),
            SimpleHsmTier::Software => "Software".to_string(),
            SimpleHsmTier::Hardware => "Hardware".to_string(),
            SimpleHsmTier::Hybrid => "Hybrid".to_string(),
        }
    }

#[derive(Debug, Clone)]
pub struct HsmManagerConfig {
    pub hsm_configs: Vec<HsmConfig>,
    pub health_config: HealthConfig,
    pub failover_config: FailoverConfig,
    pub performance_config: PerformanceConfig,

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
            recovery_threshold: 2,
            timeout: Duration::from_secs(5),
impl Default for FailoverConfig {
            enabled: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(60),}

impl Default for PerformanceConfig {
            enable_load_balancing: true,
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(10),
impl Default for HsmManagerConfig {
            hsm_configs: vec![],
            health_config: HealthConfig::default(),
            failover_config: FailoverConfig::default(),
            performance_config: PerformanceConfig::default(),}

impl std::fmt::Display for HsmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            HsmType::SmartphoneIos => write!(f, "ios"),
            HsmType::SmartphoneAndroid => write!(f, "android"),
            HsmType::SoftwareRust => write!(f, "software"),
            HsmType::HardwareAws => write!(f, "aws"),
            HsmType::HardwareLuna => write!(f, "luna"),
            HsmType::HardwareThales => write!(f, "thales"),
            HsmType::HardwareUtimaco => write!(f, "utimaco"),
            HsmType::Custom(name) => write!(f, "custom_{}", name),
} 
