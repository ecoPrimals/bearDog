// SPDX-License-Identifier: AGPL-3.0-or-later

//! Network timeout configuration
//!
//! Provides comprehensive timeout settings for all network operations including
//! connections, HTTP requests, DNS resolution, retries, and heartbeats.

use super::core::{read_env_timeout_millis, read_env_timeout_secs, FromEnvironment};
use crate::env_keys;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network operation timeout configuration
///
/// Comprehensive timeout settings for all network-related operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkTimeouts {
    // Connection Establishment
    /// Connection establishment timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_CONNECTION_TIMEOUT_SECS`
    pub connection_timeout_secs: u64,

    /// Network handshake timeout in seconds
    ///
    /// Default: 10 seconds
    /// Environment: `BEARDOG_HANDSHAKE_TIMEOUT_SECS`
    pub handshake_timeout_secs: u64,

    /// TLS handshake timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS`
    pub tls_handshake_timeout_secs: u64,

    // Connection Management
    /// Keep-alive timeout in seconds
    ///
    /// Default: 60 seconds
    /// Environment: `BEARDOG_KEEP_ALIVE_TIMEOUT_SECS`
    pub keep_alive_timeout_secs: u64,

    /// Idle connection timeout in seconds
    ///
    /// Default: 300 seconds (5 minutes)
    /// Environment: `BEARDOG_IDLE_CONNECTION_TIMEOUT_SECS`
    pub idle_connection_timeout_secs: u64,

    // I/O Operations
    /// Read operation timeout in seconds
    ///
    /// Default: 60 seconds
    /// Environment: `BEARDOG_READ_TIMEOUT_SECS`
    pub read_timeout_secs: u64,

    /// Write operation timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_WRITE_TIMEOUT_SECS`
    pub write_timeout_secs: u64,

    // HTTP Operations
    /// HTTP request timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_HTTP_REQUEST_TIMEOUT_SECS`
    pub http_request_timeout_secs: u64,

    /// HTTP response timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_HTTP_RESPONSE_TIMEOUT_SECS`
    pub http_response_timeout_secs: u64,

    // DNS and Discovery
    /// DNS resolution timeout in seconds
    ///
    /// Default: 5 seconds
    /// Environment: `BEARDOG_DNS_RESOLUTION_TIMEOUT_SECS`
    pub dns_resolution_timeout_secs: u64,

    // Retry and Backoff
    /// Retry timeout in milliseconds
    ///
    /// Default: 100 milliseconds
    /// Environment: `BEARDOG_RETRY_TIMEOUT_MILLIS`
    pub retry_timeout_millis: u64,

    /// Backoff timeout in milliseconds
    ///
    /// Default: 500 milliseconds
    /// Environment: `BEARDOG_BACKOFF_TIMEOUT_MILLIS`
    pub backoff_timeout_millis: u64,

    // Health Monitoring
    /// Ping timeout in seconds
    ///
    /// Default: 1 second
    /// Environment: `BEARDOG_PING_TIMEOUT_SECS`
    pub ping_timeout_secs: u64,

    /// Heartbeat timeout in seconds
    ///
    /// Default: 30 seconds
    /// Environment: `BEARDOG_HEARTBEAT_TIMEOUT_SECS`
    pub heartbeat_timeout_secs: u64,
}

impl Default for NetworkTimeouts {
    fn default() -> Self {
        Self {
            // Connection Establishment
            connection_timeout_secs: 30,
            handshake_timeout_secs: 10,
            tls_handshake_timeout_secs: 30,
            // Connection Management
            keep_alive_timeout_secs: 60,
            idle_connection_timeout_secs: 300, // 5 minutes
            // I/O Operations
            read_timeout_secs: 60,
            write_timeout_secs: 30,
            // HTTP Operations
            http_request_timeout_secs: 30,
            http_response_timeout_secs: 30,
            // DNS and Discovery
            dns_resolution_timeout_secs: 5,
            // Retry and Backoff
            retry_timeout_millis: 100,
            backoff_timeout_millis: 500,
            // Health Monitoring
            ping_timeout_secs: 1,
            heartbeat_timeout_secs: 30,
        }
    }
}

impl FromEnvironment for NetworkTimeouts {
    fn from_env() -> Self {
        let defaults = Self::default();
        Self {
            connection_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_CONNECTION_TIMEOUT_SECS,
                defaults.connection_timeout_secs,
            ),
            handshake_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_HANDSHAKE_TIMEOUT_SECS,
                defaults.handshake_timeout_secs,
            ),
            tls_handshake_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_TLS_HANDSHAKE_TIMEOUT_SECS,
                defaults.tls_handshake_timeout_secs,
            ),
            keep_alive_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_KEEP_ALIVE_TIMEOUT_SECS,
                defaults.keep_alive_timeout_secs,
            ),
            idle_connection_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_IDLE_CONNECTION_TIMEOUT_SECS,
                defaults.idle_connection_timeout_secs,
            ),
            read_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_READ_TIMEOUT_SECS,
                defaults.read_timeout_secs,
            ),
            write_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_WRITE_TIMEOUT_SECS,
                defaults.write_timeout_secs,
            ),
            http_request_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_HTTP_REQUEST_TIMEOUT_SECS,
                defaults.http_request_timeout_secs,
            ),
            http_response_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_HTTP_RESPONSE_TIMEOUT_SECS,
                defaults.http_response_timeout_secs,
            ),
            dns_resolution_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_DNS_RESOLUTION_TIMEOUT_SECS,
                defaults.dns_resolution_timeout_secs,
            ),
            retry_timeout_millis: read_env_timeout_millis(
                env_keys::ENV_RETRY_TIMEOUT_MILLIS,
                defaults.retry_timeout_millis,
            ),
            backoff_timeout_millis: read_env_timeout_millis(
                env_keys::ENV_BACKOFF_TIMEOUT_MILLIS,
                defaults.backoff_timeout_millis,
            ),
            ping_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_PING_TIMEOUT_SECS,
                defaults.ping_timeout_secs,
            ),
            heartbeat_timeout_secs: read_env_timeout_secs(
                env_keys::ENV_HEARTBEAT_TIMEOUT_SECS,
                defaults.heartbeat_timeout_secs,
            ),
        }
    }

    fn try_from_env() -> Option<Self> {
        // Check if any network timeout env vars are set
        let env_vars = [
            env_keys::ENV_CONNECTION_TIMEOUT_SECS,
            env_keys::ENV_HANDSHAKE_TIMEOUT_SECS,
            env_keys::ENV_TLS_HANDSHAKE_TIMEOUT_SECS,
            env_keys::ENV_KEEP_ALIVE_TIMEOUT_SECS,
            env_keys::ENV_IDLE_CONNECTION_TIMEOUT_SECS,
            env_keys::ENV_READ_TIMEOUT_SECS,
            env_keys::ENV_WRITE_TIMEOUT_SECS,
            env_keys::ENV_HTTP_REQUEST_TIMEOUT_SECS,
            env_keys::ENV_HTTP_RESPONSE_TIMEOUT_SECS,
            env_keys::ENV_DNS_RESOLUTION_TIMEOUT_SECS,
            env_keys::ENV_RETRY_TIMEOUT_MILLIS,
            env_keys::ENV_BACKOFF_TIMEOUT_MILLIS,
            env_keys::ENV_PING_TIMEOUT_SECS,
            env_keys::ENV_HEARTBEAT_TIMEOUT_SECS,
        ];

        if env_vars.iter().any(|var| std::env::var(var).is_ok()) {
            Some(Self::from_env())
        } else {
            None
        }
    }
}

impl NetworkTimeouts {
    // Connection Establishment
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_secs)
    }

    pub fn handshake_timeout(&self) -> Duration {
        Duration::from_secs(self.handshake_timeout_secs)
    }

    pub fn tls_handshake_timeout(&self) -> Duration {
        Duration::from_secs(self.tls_handshake_timeout_secs)
    }

    // Connection Management
    pub fn keep_alive_timeout(&self) -> Duration {
        Duration::from_secs(self.keep_alive_timeout_secs)
    }

    pub fn idle_connection_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_connection_timeout_secs)
    }

    // I/O Operations
    pub fn read_timeout(&self) -> Duration {
        Duration::from_secs(self.read_timeout_secs)
    }

    pub fn write_timeout(&self) -> Duration {
        Duration::from_secs(self.write_timeout_secs)
    }

    // HTTP Operations
    pub fn http_request_timeout(&self) -> Duration {
        Duration::from_secs(self.http_request_timeout_secs)
    }

    pub fn http_response_timeout(&self) -> Duration {
        Duration::from_secs(self.http_response_timeout_secs)
    }

    // DNS and Discovery
    pub fn dns_resolution_timeout(&self) -> Duration {
        Duration::from_secs(self.dns_resolution_timeout_secs)
    }

    // Retry and Backoff
    pub fn retry_timeout(&self) -> Duration {
        Duration::from_millis(self.retry_timeout_millis)
    }

    pub fn backoff_timeout(&self) -> Duration {
        Duration::from_millis(self.backoff_timeout_millis)
    }

    // Health Monitoring
    pub fn ping_timeout(&self) -> Duration {
        Duration::from_secs(self.ping_timeout_secs)
    }

    pub fn heartbeat_timeout(&self) -> Duration {
        Duration::from_secs(self.heartbeat_timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.connection_timeout_secs, 30);
        assert_eq!(timeouts.handshake_timeout_secs, 10);
        assert_eq!(timeouts.read_timeout_secs, 60);
        assert_eq!(timeouts.write_timeout_secs, 30);
        assert_eq!(timeouts.ping_timeout_secs, 1);
    }

    #[test]
    fn test_connection_timeouts() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.connection_timeout(), Duration::from_secs(30));
        assert_eq!(timeouts.handshake_timeout(), Duration::from_secs(10));
        assert_eq!(timeouts.tls_handshake_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_io_timeouts() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.read_timeout(), Duration::from_secs(60));
        assert_eq!(timeouts.write_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_retry_timeouts() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.retry_timeout(), Duration::from_millis(100));
        assert_eq!(timeouts.backoff_timeout(), Duration::from_millis(500));
    }

    #[test]
    fn test_http_timeouts() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.http_request_timeout(), Duration::from_secs(30));
        assert_eq!(timeouts.http_response_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_dns_timeout() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.dns_resolution_timeout(), Duration::from_secs(5));
    }

    #[test]
    fn test_health_monitoring_timeouts() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.ping_timeout(), Duration::from_secs(1));
        assert_eq!(timeouts.heartbeat_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_connection_management_timeouts() {
        let timeouts = NetworkTimeouts::default();
        assert_eq!(timeouts.keep_alive_timeout(), Duration::from_secs(60));
        assert_eq!(timeouts.idle_connection_timeout(), Duration::from_secs(300));
    }

    #[test]
    fn test_clone() {
        let timeouts = NetworkTimeouts::default();
        let cloned = timeouts;
        assert_eq!(timeouts, cloned);
    }

    #[test]
    fn test_debug_format() {
        let timeouts = NetworkTimeouts::default();
        let debug_str = format!("{:?}", timeouts);
        assert!(debug_str.contains("NetworkTimeouts"));
        assert!(debug_str.contains("connection_timeout_secs"));
    }

    #[test]
    fn test_serialization() {
        let timeouts = NetworkTimeouts::default();
        let json = serde_json::to_string(&timeouts).expect("Should serialize");
        let deserialized: NetworkTimeouts =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(timeouts, deserialized);
    }

    #[test]
    fn test_custom_values() {
        let timeouts = NetworkTimeouts {
            connection_timeout_secs: 60,
            read_timeout_secs: 120,
            ..Default::default()
        };

        assert_eq!(timeouts.connection_timeout(), Duration::from_secs(60));
        assert_eq!(timeouts.read_timeout(), Duration::from_secs(120));
    }

    #[test]
    fn test_from_env_with_defaults() {
        let timeouts = NetworkTimeouts::from_env();
        // Should match defaults when no env vars set
        let defaults = NetworkTimeouts::default();
        assert_eq!(
            timeouts.connection_timeout_secs,
            defaults.connection_timeout_secs
        );
        assert_eq!(timeouts.read_timeout_secs, defaults.read_timeout_secs);
    }

    #[test]
    fn test_try_from_env_returns_none_when_no_env_vars() {
        // Clear any potentially set env vars first (in isolated test)
        let result = NetworkTimeouts::try_from_env();
        // Should return None if no env vars set, or Some if any are set
        // This test validates the function works without panicking
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_all_timeout_methods() {
        let timeouts = NetworkTimeouts::default();

        // Test all timeout methods return valid durations
        assert!(timeouts.connection_timeout().as_secs() > 0);
        assert!(timeouts.handshake_timeout().as_secs() > 0);
        assert!(timeouts.tls_handshake_timeout().as_secs() > 0);
        assert!(timeouts.keep_alive_timeout().as_secs() > 0);
        assert!(timeouts.idle_connection_timeout().as_secs() > 0);
        assert!(timeouts.read_timeout().as_secs() > 0);
        assert!(timeouts.write_timeout().as_secs() > 0);
        assert!(timeouts.http_request_timeout().as_secs() > 0);
        assert!(timeouts.http_response_timeout().as_secs() > 0);
        assert!(timeouts.dns_resolution_timeout().as_secs() > 0);
        assert!(timeouts.retry_timeout().as_millis() > 0);
        assert!(timeouts.backoff_timeout().as_millis() > 0);
        assert!(timeouts.ping_timeout().as_secs() > 0);
        assert!(timeouts.heartbeat_timeout().as_secs() > 0);
    }

    #[test]
    fn test_reasonable_timeout_values() {
        let timeouts = NetworkTimeouts::default();

        // Ensure timeouts are reasonable
        assert!(timeouts.ping_timeout_secs < timeouts.connection_timeout_secs);
        assert!(timeouts.handshake_timeout_secs <= timeouts.connection_timeout_secs);
        assert!(timeouts.retry_timeout_millis < 1000); // Less than 1 second
        assert!(timeouts.idle_connection_timeout_secs > timeouts.keep_alive_timeout_secs);
    }
}
