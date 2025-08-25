// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Network Configuration - Canonical
///
/// **UNIFIED NETWORK CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Unified Network Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedNetworkConfig {
    pub ports: NetworkPortsConfig,
    pub tls: TlsConfig,
    pub load_balancing: LoadBalancingConfig,
    pub connection: ConnectionConfig,
    pub security: NetworkSecurityConfig,
    pub backend: BackendConfig,
}


/// **CANONICAL** Network Ports Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPortsConfig {
    pub http_port: u16,
    pub https_port: u16,
    pub admin_port: u16,
    pub metrics_port: u16,
    pub health_port: u16,
}

impl Default for NetworkPortsConfig {
    fn default() -> Self {
        Self {
            http_port: 8080,
            https_port: 8443,
            admin_port: 9090,
            metrics_port: 9091,
            health_port: 9092,
        }
    }
}

/// **CANONICAL** TLS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
    pub ca_path: Option<String>,
    pub verify_client: bool,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cert_path: "/etc/ssl/certs/beardog.crt".to_string(),
            key_path: "/etc/ssl/private/beardog.key".to_string(),
            ca_path: None,
            verify_client: false,
        }
    }
}

/// **CANONICAL** Load Balancing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub enabled: bool,
    pub strategy: String,
    pub health_check_interval: Duration,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "round_robin".to_string(),
            health_check_interval: Duration::from_secs(30),
        }
    }
}

/// **CANONICAL** Connection Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub keep_alive_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(60),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
        }
    }
}

/// **CANONICAL** Network Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    pub enable_rate_limiting: bool,
    pub max_requests_per_minute: u32,
    pub enable_ddos_protection: bool,
    pub allowed_origins: Vec<String>,
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            enable_rate_limiting: true,
            max_requests_per_minute: 1000,
            enable_ddos_protection: true,
            allowed_origins: vec!["*".to_string()],
        }
    }
}

/// **CANONICAL** Backend Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub service_discovery_enabled: bool,
    pub circuit_breaker_enabled: bool,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            service_discovery_enabled: false,
            circuit_breaker_enabled: true,
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
} 
