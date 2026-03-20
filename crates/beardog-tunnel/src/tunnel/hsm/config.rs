// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Configuration
//!
//! Configuration types for HSM operations.
//!

use std::time::Duration;

/// Simple HSM tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleHsmTier {
    /// Smartphone HSM (iOS/Android)
    Smartphone,
    /// Software HSM
    Software,
    /// Hardware HSM
    Hardware,
    /// Hybrid HSM
    Hybrid,
}

impl SimpleHsmTier {
    /// Convert to string representation
    pub fn to_string_repr(&self) -> String {
        match self {
            Self::Smartphone => "Smartphone".to_string(),
            Self::Software => "Software".to_string(),
            Self::Hardware => "Hardware".to_string(),
            Self::Hybrid => "Hybrid".to_string(),
        }
    }
}

impl std::fmt::Display for SimpleHsmTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

/// HSM configuration
#[derive(Debug, Clone)]
pub struct HsmConfig {
    /// Health check interval
    pub check_interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Circuit breaker timeout
    pub circuit_breaker_timeout: Duration,
    /// Enable caching
    pub enable_caching: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Operation timeout
    pub operation_timeout: Duration,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            circuit_breaker_timeout: Duration::from_secs(60),
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hsm_tier_display() {
        assert_eq!(SimpleHsmTier::Software.to_string(), "Software");
        assert_eq!(SimpleHsmTier::Hardware.to_string(), "Hardware");
    }

    #[test]
    fn test_simple_hsm_tier_equality() {
        assert_eq!(SimpleHsmTier::Software, SimpleHsmTier::Software);
        assert_ne!(SimpleHsmTier::Software, SimpleHsmTier::Hardware);
    }

    #[test]
    fn test_hsm_config_default() {
        let config = HsmConfig::default();
        assert_eq!(config.check_interval, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert!(config.enable_caching);
        assert_eq!(config.max_concurrent_operations, 100);
    }
}
