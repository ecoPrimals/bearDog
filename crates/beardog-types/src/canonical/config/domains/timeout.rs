// Canonical Timeout Configuration
//
// Unified timeout configuration for all BearDog operations.
// Consolidates 8+ TimeoutConfig instances across the codebase.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical timeout configuration for all BearDog operations
///
/// This provides a comprehensive set of timeouts that can be used across
/// networking, discovery, adapters, workflows, and other components.
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
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalTimeoutConfig {
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
}

// Helper modules for Duration serialization
mod humantime_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_millis() as u64)
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
            Some(d) => serializer.serialize_some(&(d.as_millis() as u64)),
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
    /// - connect_timeout: 5 seconds
    /// - read_timeout: 30 seconds
    /// - write_timeout: 30 seconds
    /// - operation_timeout: 60 seconds
    /// - idle_timeout: 5 minutes
    /// - keepalive_timeout: 60 seconds
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(60),
            idle_timeout: Some(Duration::from_secs(300)),
            keepalive_timeout: Some(Duration::from_secs(60)),
        }
    }
}

impl CanonicalTimeoutConfig {
    /// Create an aggressive timeout configuration
    ///
    /// Suitable for high-performance local networks where quick failure
    /// detection is preferred over waiting.
    pub fn aggressive() -> Self {
        Self {
            connect_timeout: Duration::from_secs(1),
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            operation_timeout: Duration::from_secs(10),
            idle_timeout: Some(Duration::from_secs(60)),
            keepalive_timeout: Some(Duration::from_secs(30)),
        }
    }

    /// Create a conservative timeout configuration
    ///
    /// Suitable for unreliable networks or long-running operations where
    /// waiting longer is acceptable.
    pub fn conservative() -> Self {
        Self {
            connect_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(120),
            write_timeout: Duration::from_secs(120),
            operation_timeout: Duration::from_secs(300),
            idle_timeout: Some(Duration::from_secs(600)),
            keepalive_timeout: Some(Duration::from_secs(120)),
        }
    }

    /// Create a minimal timeout configuration
    ///
    /// Suitable for extremely fast local operations where any delay
    /// indicates a problem.
    pub fn minimal() -> Self {
        Self {
            connect_timeout: Duration::from_millis(500),
            read_timeout: Duration::from_secs(1),
            write_timeout: Duration::from_secs(1),
            operation_timeout: Duration::from_secs(2),
            idle_timeout: Some(Duration::from_secs(30)),
            keepalive_timeout: None,
        }
    }

    /// Create a long-running timeout configuration
    ///
    /// Suitable for operations that may take a long time to complete,
    /// such as large file transfers or complex computations.
    pub fn long_running() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(600),
            write_timeout: Duration::from_secs(600),
            operation_timeout: Duration::from_secs(1800),
            idle_timeout: None,
            keepalive_timeout: Some(Duration::from_secs(300)),
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

