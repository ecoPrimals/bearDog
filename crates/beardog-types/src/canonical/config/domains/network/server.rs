// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Server Configuration Module
//!
//! This module contains server-side network configuration structs and implementations.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Server configuration - consolidates server-side network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfiguration {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Enable IPv6
    pub enable_ipv6: bool,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Server backlog size
    pub backlog_size: usize,
    /// Enable keep-alive
    pub enable_keepalive: bool,
    /// Keep-alive timeout seconds
    pub keepalive_timeout_seconds: u64,
    /// Enable TCP no-delay
    pub tcp_nodelay: bool,
    /// Socket buffer sizes
    pub socket_buffers: SocketBufferConfiguration,
}

/// Socket buffer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketBufferConfiguration {
    /// Send buffer size in bytes
    pub send_buffer_size: usize,
    /// Receive buffer size in bytes
    pub recv_buffer_size: usize,
    /// Enable auto-tuning
    pub enable_auto_tuning: bool,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        // Use config system for defaults instead of hardcoded constants
        use beardog_config::global::BEARDOG_CONFIG;
        Self {
            bind_address: BEARDOG_CONFIG.network.api.bind_address.to_string(),
            port: crate::constants::domains::network::defaults::default_api_port(),
            enable_ipv6: true,
            max_connections: crate::constants::domains::system::defaults::DEFAULT_MAX_CONNECTIONS,
            backlog_size: std::env::var(env_keys::ENV_SERVER_BACKLOG_SIZE)
                .ok()
                .and_then(|b| b.parse().ok())
                .unwrap_or(1024), // 1024 connections default
            enable_keepalive: true,
            keepalive_timeout_seconds: 60,
            tcp_nodelay: true,
            socket_buffers: SocketBufferConfiguration::default(),
        }
    }
}

impl Default for SocketBufferConfiguration {
    fn default() -> Self {
        Self {
            send_buffer_size: crate::constants::domains::system::defaults::DEFAULT_BUFFER_SIZE,
            recv_buffer_size: crate::constants::domains::system::defaults::DEFAULT_BUFFER_SIZE,
            enable_auto_tuning: true,
        }
    }
}

impl ServerConfiguration {
    /// Validate server configuration
    ///
    /// # Errors
    ///
    /// Returns an error if port, max connections, or bind address is invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.port == 0 {
            return Err(BearDogError::configuration("Server port cannot be 0"));
        }

        if self.max_connections == 0 {
            return Err(BearDogError::configuration("Max connections cannot be 0"));
        }

        if self.bind_address.is_empty() {
            return Err(BearDogError::configuration("Bind address cannot be empty"));
        }

        Ok(())
    }
}

impl SocketBufferConfiguration {
    /// Validate socket buffer configuration
    ///
    /// # Errors
    ///
    /// Returns an error if send or receive buffer size is zero.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.send_buffer_size == 0 {
            return Err(BearDogError::configuration("Send buffer size cannot be 0"));
        }

        if self.recv_buffer_size == 0 {
            return Err(BearDogError::configuration(
                "Receive buffer size cannot be 0",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_server_config() {
        let config = ServerConfiguration::default();
        assert!(config.validate().is_ok());
        assert_eq!(
            config.port,
            crate::constants::domains::network::defaults::default_api_port()
        );
        assert!(config.enable_ipv6);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.enable_keepalive);
        assert!(config.tcp_nodelay);
    }

    #[test]
    fn test_invalid_server_port() {
        let mut config = ServerConfiguration::default();
        config.port = 0;
        assert!(config.validate().is_err());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important

    #[test]
    fn test_invalid_max_connections() {
        let mut config = ServerConfiguration::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
        config.max_connections = 0;
        assert!(config.validate().is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_socket_buffer_validation() {
        let config = SocketBufferConfiguration::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config;
        invalid_config.send_buffer_size = 0;
        assert!(invalid_config.validate().is_err());
    }
}
