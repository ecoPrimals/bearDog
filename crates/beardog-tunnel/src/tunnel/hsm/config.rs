// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Configuration
//!
//! Configuration types for HSM operations.
//!

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Simple HSM tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Return the canonical string representation for this tier.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Smartphone => "Smartphone",
            Self::Software => "Software",
            Self::Hardware => "Hardware",
            Self::Hybrid => "Hybrid",
        }
    }
    /// Convert to string representation
    #[must_use]
    pub fn to_string_repr(&self) -> String {
        self.as_str().to_string()
    }
}

impl std::fmt::Display for SimpleHsmTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
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
        assert_eq!(SimpleHsmTier::Smartphone.to_string(), "Smartphone");
        assert_eq!(SimpleHsmTier::Software.to_string(), "Software");
        assert_eq!(SimpleHsmTier::Hardware.to_string(), "Hardware");
        assert_eq!(SimpleHsmTier::Hybrid.to_string(), "Hybrid");
    }

    #[test]
    fn test_simple_hsm_tier_as_str() {
        assert_eq!(SimpleHsmTier::Software.as_str(), "Software");
        assert_eq!(SimpleHsmTier::Software.to_string_repr(), "Software");
    }

    #[test]
    fn test_simple_hsm_tier_serde() {
        let tier = SimpleHsmTier::Hardware;
        let serialized = serde_json::to_string(&tier).expect("SimpleHsmTier should serialize");
        let deserialized: SimpleHsmTier =
            serde_json::from_str(&serialized).expect("SimpleHsmTier round-trip should deserialize");
        assert_eq!(tier, deserialized);
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
