// SPDX-License-Identifier: AGPL-3.0-only

// Canonical Timeout Configuration
//
// Unified timeout configuration for all BearDog operations.
// Consolidates 8+ TimeoutConfig instances across the codebase.

use crate::canonical::traits::TimeoutPolicy;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical timeout configuration for all BearDog operations
///
/// **UNIFIED CONFIGURATION** - Consolidates all timeout configurations:
/// - Network-level timeouts (connect, read, write, operation, idle, keepalive)
/// - Domain-specific timeouts (health_check, hsm, discovery, ai)
///
/// This replaces:
/// - `UnifiedTimeoutConfig` (timeout_unified.rs) - now a type alias
/// - `TimeoutConfiguration` - now a type alias
/// - Various scattered timeout configs across crates
///
/// # Examples
///
/// ```
/// use beardog_types::canonical::config::domains::timeout::CanonicalTimeoutConfig;
/// use std::time::Duration;
///
/// let config = CanonicalTimeoutConfig {
///     connect_timeout: Duration::from_secs(5),
///     read_timeout: Duration::from_secs(30),
///     write_timeout: Duration::from_secs(30),
///     operation_timeout: Duration::from_secs(60),
///     idle_timeout: Some(Duration::from_secs(300)),
///     keepalive_timeout: Some(Duration::from_secs(60)),
///     health_check_timeout: Duration::from_secs(5),
///     hsm_operation_timeout: Duration::from_secs(2),
///     hsm_probe_timeout: Duration::from_millis(500),
///     discovery_timeout: Duration::from_secs(10),
///     ai_decision_timeout: Duration::from_secs(30),
///     ai_request_timeout: Duration::from_secs(30),
///     ai_batch_timeout: Duration::from_millis(10),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalTimeoutConfig {
    // ========================================================================
    // NETWORK-LEVEL TIMEOUTS
    // ========================================================================
    /// Timeout for establishing a connection
    ///
    /// This is the maximum time to wait when connecting to a remote service.
    /// Applies to: TCP connections, HTTP connections, HSM connections, etc.
    ///
    /// Recommended values:
    /// - LAN: 1-5 seconds
    /// - WAN: 5-15 seconds
    /// - Unreliable networks: 15-30 seconds
    #[serde(with = "humantime_serde")]
    pub connect_timeout: Duration,

    /// Timeout for reading data from a connection
    ///
    /// This is the maximum time to wait for data to arrive after a request.
    /// If no data arrives within this time, the operation fails.
    ///
    /// Recommended values:
    /// - Fast operations: 5-30 seconds
    /// - Normal operations: 30-60 seconds
    /// - Long-running operations: 60-300 seconds
    #[serde(with = "humantime_serde")]
    pub read_timeout: Duration,

    /// Timeout for writing data to a connection
    ///
    /// This is the maximum time to wait when sending data.
    /// If the write doesn't complete within this time, the operation fails.
    ///
    /// Recommended values:
    /// - Small payloads: 5-30 seconds
    /// - Large payloads: 30-60 seconds
    /// - Very large payloads: 60-300 seconds
    #[serde(with = "humantime_serde")]
    pub write_timeout: Duration,

    /// Timeout for the entire operation
    ///
    /// This is the maximum total time for an operation, including connection,
    /// request, and response. If the entire operation doesn't complete within
    /// this time, it fails.
    ///
    /// This should be >= connect_timeout + read_timeout + write_timeout
    #[serde(with = "humantime_serde")]
    pub operation_timeout: Duration,

    /// Timeout for idle connections
    ///
    /// Connections idle longer than this will be closed.
    /// None means connections never time out due to idleness.
    ///
    /// Recommended values:
    /// - Connection pools: 60-300 seconds
    /// - Long-lived connections: 300-3600 seconds
    /// - None: For connections that should never timeout
    #[serde(default, with = "humantime_serde_optional")]
    pub idle_timeout: Option<Duration>,

    /// Timeout for keepalive probes
    ///
    /// How often to send keepalive probes on idle connections.
    /// None means no keepalive probes are sent.
    ///
    /// Recommended values:
    /// - Active monitoring: 30-60 seconds
    /// - Normal monitoring: 60-120 seconds
    /// - None: For short-lived connections
    #[serde(default, with = "humantime_serde_optional")]
    pub keepalive_timeout: Option<Duration>,

    // ========================================================================
    // DOMAIN-SPECIFIC TIMEOUTS
    // ========================================================================
    /// Health check timeout
    ///
    /// Maximum time for health check probes.
    ///
    /// **Default**: 5 seconds\
    /// **Recommended range**: 1-30 seconds
    #[serde(default = "default_health_check_timeout", with = "humantime_serde")]
    pub health_check_timeout: Duration,

    /// HSM operation timeout
    ///
    /// Timeout for typical HSM operations (sign, verify, encrypt, decrypt).
    ///
    /// **Default**: 2 seconds\
    /// **Recommended range**: 1-10 seconds
    #[serde(default = "default_hsm_operation_timeout", with = "humantime_serde")]
    pub hsm_operation_timeout: Duration,

    /// HSM probe timeout
    ///
    /// Fast probe for HSM availability check.
    ///
    /// **Default**: 500 milliseconds\
    /// **Recommended range**: 100-5000 milliseconds
    #[serde(default = "default_hsm_probe_timeout", with = "humantime_serde")]
    pub hsm_probe_timeout: Duration,

    /// Service discovery timeout
    ///
    /// Timeout for service discovery operations.
    ///
    /// **Default**: 10 seconds\
    /// **Recommended range**: 1-60 seconds
    #[serde(default = "default_discovery_timeout", with = "humantime_serde")]
    pub discovery_timeout: Duration,

    /// AI decision timeout
    ///
    /// Timeout for AI decision-making processes.
    ///
    /// **Default**: 30 seconds\
    /// **Recommended range**: 5-300 seconds
    #[serde(default = "default_ai_decision_timeout", with = "humantime_serde")]
    pub ai_decision_timeout: Duration,

    /// AI request timeout
    ///
    /// Timeout for AI inference requests.
    ///
    /// **Default**: 30 seconds\
    /// **Recommended range**: 5-300 seconds
    #[serde(default = "default_ai_request_timeout", with = "humantime_serde")]
    pub ai_request_timeout: Duration,

    /// AI batch timeout
    ///
    /// Timeout for AI batch processing.
    ///
    /// **Default**: 10 milliseconds\
    /// **Recommended range**: 1-1000 milliseconds
    #[serde(default = "default_ai_batch_timeout", with = "humantime_serde")]
    pub ai_batch_timeout: Duration,
}

// Default functions for domain-specific timeouts
const fn default_health_check_timeout() -> Duration {
    Duration::from_secs(5)
}

const fn default_hsm_operation_timeout() -> Duration {
    Duration::from_secs(2)
}

const fn default_hsm_probe_timeout() -> Duration {
    Duration::from_millis(500)
}

const fn default_discovery_timeout() -> Duration {
    Duration::from_secs(10)
}

const fn default_ai_decision_timeout() -> Duration {
    Duration::from_secs(30)
}

const fn default_ai_request_timeout() -> Duration {
    Duration::from_secs(30)
}

const fn default_ai_batch_timeout() -> Duration {
    Duration::from_millis(10)
}

// Helper modules for Duration serialization
mod humantime_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // SAFETY: Duration::as_millis() returns u128, but in practice timeouts will never
        // exceed u64::MAX milliseconds (~584 million years). We clamp to u64::MAX for safety.
        // Using saturating cast ensures we don't wrap around on extreme values.
        let millis = duration.as_millis();
        let clamped = if millis > u64::MAX as u128 {
            u64::MAX
        } else {
            millis as u64
        };
        serializer.serialize_u64(clamped)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

mod humantime_serde_optional {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match duration {
            Some(d) => {
                // SAFETY: See milliseconds_serializer for rationale
                let millis = d.as_millis();
                let clamped = if millis > u64::MAX as u128 {
                    u64::MAX
                } else {
                    millis as u64
                };
                serializer.serialize_some(&clamped)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<u64>::deserialize(deserializer)?;
        Ok(opt.map(Duration::from_millis))
    }
}

impl Default for CanonicalTimeoutConfig {
    /// Default timeout configuration with balanced settings
    ///
    /// Suitable for most network operations in a LAN environment.
    ///
    /// Network timeouts:
    /// - connect_timeout: 5 seconds
    /// - read_timeout: 30 seconds
    /// - write_timeout: 30 seconds
    /// - operation_timeout: 60 seconds
    /// - idle_timeout: 5 minutes
    /// - keepalive_timeout: 60 seconds
    ///
    /// Domain timeouts:
    /// - health_check_timeout: 5 seconds
    /// - hsm_operation_timeout: 2 seconds
    /// - hsm_probe_timeout: 500 milliseconds
    /// - discovery_timeout: 10 seconds
    /// - ai_decision_timeout: 30 seconds
    /// - ai_request_timeout: 30 seconds
    /// - ai_batch_timeout: 10 milliseconds
    fn default() -> Self {
        Self {
            // Network timeouts
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(60),
            idle_timeout: Some(Duration::from_secs(300)),
            keepalive_timeout: Some(Duration::from_secs(60)),

            // Domain timeouts
            health_check_timeout: default_health_check_timeout(),
            hsm_operation_timeout: default_hsm_operation_timeout(),
            hsm_probe_timeout: default_hsm_probe_timeout(),
            discovery_timeout: default_discovery_timeout(),
            ai_decision_timeout: default_ai_decision_timeout(),
            ai_request_timeout: default_ai_request_timeout(),
            ai_batch_timeout: default_ai_batch_timeout(),
        }
    }
}

impl CanonicalTimeoutConfig {
    /// Create an aggressive timeout configuration
    ///
    /// Suitable for high-performance local networks where quick failure
    /// detection is preferred over waiting.
    pub const fn aggressive() -> Self {
        Self {
            // Network timeouts
            connect_timeout: Duration::from_secs(1),
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            operation_timeout: Duration::from_secs(10),
            idle_timeout: Some(Duration::from_secs(60)),
            keepalive_timeout: Some(Duration::from_secs(30)),

            // Domain timeouts
            health_check_timeout: Duration::from_secs(2),
            hsm_operation_timeout: Duration::from_secs(1),
            hsm_probe_timeout: Duration::from_millis(250),
            discovery_timeout: Duration::from_secs(5),
            ai_decision_timeout: Duration::from_secs(10),
            ai_request_timeout: Duration::from_secs(10),
            ai_batch_timeout: Duration::from_millis(5),
        }
    }

    /// Create a conservative timeout configuration
    ///
    /// Suitable for unreliable networks or long-running operations where
    /// waiting longer is acceptable.
    pub const fn conservative() -> Self {
        Self {
            // Network timeouts
            connect_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(120),
            write_timeout: Duration::from_secs(120),
            operation_timeout: Duration::from_secs(300),
            idle_timeout: Some(Duration::from_secs(600)),
            keepalive_timeout: Some(Duration::from_secs(120)),

            // Domain timeouts
            health_check_timeout: Duration::from_secs(15),
            hsm_operation_timeout: Duration::from_secs(5),
            hsm_probe_timeout: Duration::from_millis(2000),
            discovery_timeout: Duration::from_secs(30),
            ai_decision_timeout: Duration::from_secs(120),
            ai_request_timeout: Duration::from_secs(120),
            ai_batch_timeout: Duration::from_millis(50),
        }
    }

    /// Create a minimal timeout configuration
    ///
    /// Suitable for extremely fast local operations where any delay
    /// indicates a problem.
    pub const fn minimal() -> Self {
        Self {
            // Network timeouts
            connect_timeout: Duration::from_millis(500),
            read_timeout: Duration::from_secs(1),
            write_timeout: Duration::from_secs(1),
            operation_timeout: Duration::from_secs(2),
            idle_timeout: Some(Duration::from_secs(30)),
            keepalive_timeout: None,

            // Domain timeouts
            health_check_timeout: Duration::from_secs(1),
            hsm_operation_timeout: Duration::from_millis(500),
            hsm_probe_timeout: Duration::from_millis(100),
            discovery_timeout: Duration::from_secs(2),
            ai_decision_timeout: Duration::from_secs(5),
            ai_request_timeout: Duration::from_secs(5),
            ai_batch_timeout: Duration::from_millis(1),
        }
    }

    /// Create a long-running timeout configuration
    ///
    /// Suitable for operations that may take a long time to complete,
    /// such as large file transfers or complex computations.
    pub const fn long_running() -> Self {
        Self {
            // Network timeouts
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(600),
            write_timeout: Duration::from_secs(600),
            operation_timeout: Duration::from_secs(1800),
            idle_timeout: None,
            keepalive_timeout: Some(Duration::from_secs(300)),

            // Domain timeouts
            health_check_timeout: Duration::from_secs(30),
            hsm_operation_timeout: Duration::from_secs(10),
            hsm_probe_timeout: Duration::from_millis(5000),
            discovery_timeout: Duration::from_secs(60),
            ai_decision_timeout: Duration::from_secs(300),
            ai_request_timeout: Duration::from_secs(300),
            ai_batch_timeout: Duration::from_millis(100),
        }
    }

    /// Validate the timeout configuration
    ///
    /// Returns an error message if the configuration is invalid.
    pub fn validate(&self) -> Result<(), String> {
        // Check that timeouts are non-zero
        if self.connect_timeout.is_zero() {
            return Err("connect_timeout must be non-zero".to_string());
        }
        if self.read_timeout.is_zero() {
            return Err("read_timeout must be non-zero".to_string());
        }
        if self.write_timeout.is_zero() {
            return Err("write_timeout must be non-zero".to_string());
        }
        if self.operation_timeout.is_zero() {
            return Err("operation_timeout must be non-zero".to_string());
        }

        // Check that operation timeout is reasonable
        let min_operation = self.connect_timeout + self.read_timeout.min(self.write_timeout);
        if self.operation_timeout < min_operation {
            return Err(format!(
                "operation_timeout ({:?}) should be at least {:?}",
                self.operation_timeout, min_operation
            ));
        }

        // Check optional timeouts if present
        if let Some(idle) = self.idle_timeout {
            if idle.is_zero() {
                return Err("idle_timeout, if set, must be non-zero".to_string());
            }
        }

        if let Some(keepalive) = self.keepalive_timeout {
            if keepalive.is_zero() {
                return Err("keepalive_timeout, if set, must be non-zero".to_string());
            }
        }

        Ok(())
    }

    /// Check if the configuration is suitable for the given network type
    pub fn is_suitable_for_network(&self, network_type: NetworkType) -> bool {
        match network_type {
            NetworkType::Local => {
                self.connect_timeout <= Duration::from_secs(2)
                    && self.operation_timeout <= Duration::from_secs(30)
            }
            NetworkType::Lan => {
                self.connect_timeout <= Duration::from_secs(10)
                    && self.operation_timeout <= Duration::from_secs(120)
            }
            NetworkType::Wan => {
                self.connect_timeout <= Duration::from_secs(30)
                    && self.operation_timeout <= Duration::from_secs(300)
            }
            NetworkType::Unreliable => self.connect_timeout >= Duration::from_secs(15),
        }
    }
}

/// Network type for timeout validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    /// Local loopback (127.0.0.1, ::1)
    Local,
    /// Local Area Network
    Lan,
    /// Wide Area Network (Internet)
    Wan,
    /// Unreliable or high-latency network
    Unreliable,
}

/// Type alias for backwards compatibility
pub type TimeoutConfig = CanonicalTimeoutConfig;

// Implement TimeoutPolicy trait for canonical timeout configuration
impl TimeoutPolicy for CanonicalTimeoutConfig {
    fn connection_timeout(&self) -> Duration {
        self.connect_timeout
    }

    fn operation_timeout(&self, operation: &str) -> Duration {
        // Map specific operations to appropriate timeouts
        match operation {
            "read" => self.read_timeout,
            "write" => self.write_timeout,
            "connect" => self.connect_timeout,
            _ => self.operation_timeout,
        }
    }

    fn should_timeout(&self, elapsed: Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }

    fn global_timeout(&self) -> Option<Duration> {
        Some(self.operation_timeout)
    }

    fn read_timeout(&self) -> Duration {
        self.read_timeout
    }

    fn write_timeout(&self) -> Duration {
        self.write_timeout
    }

    fn idle_timeout(&self) -> Option<Duration> {
        self.idle_timeout
    }

    fn remaining_time(&self, elapsed: Duration, operation: &str) -> Duration {
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    fn validate(&self) -> Result<(), String> {
        if self.connect_timeout.is_zero() {
            return Err("Connection timeout cannot be zero".to_string());
        }
        if self.operation_timeout < self.connect_timeout {
            return Err("Operation timeout must be >= connection timeout".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        // Production-ready if all timeouts are reasonable
        self.connect_timeout >= Duration::from_secs(1)
            && self.connect_timeout <= Duration::from_secs(30)
            && self.operation_timeout >= Duration::from_secs(5)
            && self.validate().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CanonicalTimeoutConfig::default();
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
        assert_eq!(config.read_timeout, Duration::from_secs(30));
        assert_eq!(config.write_timeout, Duration::from_secs(30));
        assert_eq!(config.operation_timeout, Duration::from_secs(60));
        assert_eq!(config.idle_timeout, Some(Duration::from_secs(300)));
        assert_eq!(config.keepalive_timeout, Some(Duration::from_secs(60)));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_aggressive_config() {
        let config = CanonicalTimeoutConfig::aggressive();
        assert_eq!(config.connect_timeout, Duration::from_secs(1));
        assert!(config.validate().is_ok());
        assert!(config.is_suitable_for_network(NetworkType::Local));
        assert!(config.is_suitable_for_network(NetworkType::Lan));
    }

    #[test]
    fn test_conservative_config() {
        let config = CanonicalTimeoutConfig::conservative();
        assert_eq!(config.connect_timeout, Duration::from_secs(30));
        assert!(config.validate().is_ok());
        assert!(config.is_suitable_for_network(NetworkType::Wan));
        assert!(config.is_suitable_for_network(NetworkType::Unreliable));
    }

    #[test]
    fn test_validation() {
        let mut config = CanonicalTimeoutConfig::default();

        // Valid config
        assert!(config.validate().is_ok());

        // Invalid: zero connect timeout
        config.connect_timeout = Duration::from_secs(0);
        assert!(config.validate().is_err());
        config.connect_timeout = Duration::from_secs(5);

        // Invalid: operation timeout too small
        config.operation_timeout = Duration::from_secs(1);
        assert!(config.validate().is_err());
        config.operation_timeout = Duration::from_secs(60);

        // Invalid: zero idle timeout
        config.idle_timeout = Some(Duration::from_secs(0));
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_network_suitability() {
        let aggressive = CanonicalTimeoutConfig::aggressive();
        let conservative = CanonicalTimeoutConfig::conservative();

        // Aggressive suitable for fast networks
        assert!(aggressive.is_suitable_for_network(NetworkType::Local));
        assert!(aggressive.is_suitable_for_network(NetworkType::Lan));

        // Conservative suitable for slow networks
        assert!(conservative.is_suitable_for_network(NetworkType::Wan));
        assert!(conservative.is_suitable_for_network(NetworkType::Unreliable));
    }

    #[test]
    fn test_presets() {
        let minimal = CanonicalTimeoutConfig::minimal();
        assert_eq!(minimal.connect_timeout, Duration::from_millis(500));

        let long = CanonicalTimeoutConfig::long_running();
        assert_eq!(long.operation_timeout, Duration::from_secs(1800));
        assert!(long.idle_timeout.is_none());
    }

    #[test]
    fn test_serialization() {
        let config = CanonicalTimeoutConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CanonicalTimeoutConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
