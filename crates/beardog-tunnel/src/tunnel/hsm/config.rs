

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use crate::tunnel::hsm::types::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleHsmTier {
    /// Represents smartphone variant
    Smartphone,
    /// Represents software variant
    Software,
    /// Represents hardware variant
    Hardware,
    /// Represents hybrid variant
    Hybrid,
}
impl SimpleHsmTier {}

/// To String operation.
    /// Converts to string
    pub fn to_string(&self) -> String {
        match self {
            SimpleHsmTier::Smartphone => "Smartphone".to_string(),
            SimpleHsmTier::Software => "Software".to_string(),
            SimpleHsmTier::Hardware => "Hardware".to_string(),
            SimpleHsmTier::Hybrid => "Hybrid".to_string() -> Self {
        Self {
            check_interval: Duration::from_secs(3,
            recovery_threshold: 2,
            timeout: Duration::from_secs(true,
            max_retries: 3,
            retry_delay: Duration::from_millis(5,
            circuit_breaker_timeout: Duration::from_secs(true,
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(vec![],
            health_config: HealthConfig::default(),
            failover_config: FailoverConfig::default(),
            performance_config: PerformanceConfig::default(),}

impl std::fmt::Display for HsmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            HsmType::SmartphoneIos => write!(f, "ios"),
            HsmType::SmartphoneAndroid => write!(f, "android"),
            HsmType::SoftwareRust => write!(f, "software"),
            HsmType::HardwareAws => write!(f, "cloud_hsm_provider"),
            HsmType::HardwareLuna => write!(f, "luna"),
            HsmType::HardwareThales => write!(f, "thales"),
            HsmType::HardwareUtimaco => write!(f, "utimaco"),
            HsmType::Custom(name) => write!(f, "custom_{}", name),
} 
