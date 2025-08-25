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


/// # Network Constants
///
/// **CANONICAL NETWORK CONSTANTS** - Network configuration and limits

use std::time::Duration;

/// Default HTTP port
pub const DEFAULT_HTTP_PORT: u16 = 8080;

/// Default HTTPS port  
pub const DEFAULT_HTTPS_PORT: u16 = 8443;

/// Default gRPC port
pub const DEFAULT_GRPC_PORT: u16 = 9090;

/// Default metrics port
pub const DEFAULT_METRICS_PORT: u16 = 9091;

/// Default health check port
pub const DEFAULT_HEALTH_PORT: u16 = 8081;

/// Default host for local bindings
pub const DEFAULT_HOST: &str = "127.0.0.1";

/// Maximum number of concurrent connections
pub const MAX_CONNECTIONS: usize = 10000;

/// Connection timeout duration
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Request timeout duration
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Keep-alive timeout
pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(300);

/// Maximum request size in bytes (10MB)
pub const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024;

/// Maximum response size in bytes (100MB)
pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024;

/// Network buffer size
pub const NETWORK_BUFFER_SIZE: usize = 8192;

/// Default rate limit (requests per second)
pub const DEFAULT_RATE_LIMIT: u32 = 1000;

/// **PRIVATE IP RANGES** - For validation across the system
pub const PRIVATE_IP_RANGES: &[&str] = &[
    "10.0.0.0/8",
    "172.16.0.0/12",
    "192.168.0.0/16",
    "127.0.0.0/8",
    "169.254.0.0/16",
];
/// **CANONICAL NETWORK CONSTANTS** - Network and connection settings
pub mod core {
    /// Maximum concurrent connections
    /// Consolidates: MAX_CONNECTIONS from multiple modules
    pub use beardog_types::constants::unified::network::limits::MAX_CONNECTIONS;
    /// Standard connection pool size
    pub const CONNECTION_POOL_SIZE: usize = 100;
    /// Large connection pool for high-throughput
    pub const LARGE_CONNECTION_POOL_SIZE: usize = 500;
    /// Private IP address ranges for validation
    /// Consolidates: PRIVATE_IP_RANGES from security utils
    pub const PRIVATE_IP_RANGES: &[&str] = &[
        "10.0.0.0/8",
        "172.16.0.0/12",
        "192.168.0.0/16",
        "127.0.0.0/8",
        "169.254.0.0/16",
    ];
    /// Standard rate limiting - requests per minute
    pub const STANDARD_RATE_LIMIT: u32 = 100;
    /// Burst size for rate limiting
    pub const RATE_LIMIT_BURST_SIZE: u32 = 20;
}
/// **CANONICAL BIND ADDRESSES** - Default service bind addresses
/// **MIGRATED FROM**: `beardog-config/src/constants.rs::bind_addresses`
/// These constants define default bind addresses for various services
pub mod bind_addresses {
    /// Default API bind addresses (configurable via BEARDOG_API_BIND_ADDRESS)
    // Moved to canonical configuration - use beardog_types::config::canonical::constants::network
    pub const DEFAULT_API_BIND_ADDRESS: &str = "127.0.0.1:8080";
    /// Default metrics bind address (configurable via BEARDOG_METRICS_BIND_ADDRESS)
    pub const DEFAULT_METRICS_BIND_ADDRESS: &str = "127.0.0.1:9090";
    /// Default health check bind address (configurable via BEARDOG_HEALTH_BIND_ADDRESS)
    pub const DEFAULT_HEALTH_BIND_ADDRESS: &str = "127.0.0.1:8081";
    /// Default admin interface bind address (configurable via BEARDOG_ADMIN_BIND_ADDRESS)
    pub const DEFAULT_ADMIN_BIND_ADDRESS: &str = "127.0.0.1:9999";
    /// Default gRPC bind address (configurable via BEARDOG_GRPC_BIND_ADDRESS)
    pub const DEFAULT_GRPC_BIND_ADDRESS: &str = "127.0.0.1:9091";
/// **CANONICAL DEFAULT HOSTS** - Standard host addresses};


pub mod hosts {
    /// Localhost IPv4 address
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";
    /// Localhost IPv6 address
    pub const LOCALHOST_IPV6: &str = "::1";
    /// All interfaces IPv4
    pub const ALL_INTERFACES_IPV4: &str = "0.0.0.0";
    /// All interfaces IPv6
    pub const ALL_INTERFACES_IPV6: &str = "::";
/// **CANONICAL ENDPOINTS** - Default service endpoints
/// **ELIMINATES HARDCODED URLS** across the codebase
pub mod endpoints {
    /// Default ecosystem endpoints
    pub const ECOSYSTEM_REGISTRY_ENDPOINT: &str = "https://ecosystem-registry.local:8443";
    pub const PROMETHEUS_ENDPOINT: &str = "http://prometheus.ecosystem.internal:9090";
    pub const GRAFANA_ENDPOINT: &str = "http://grafana.ecosystem.internal:3000";
    pub const VAULT_ENDPOINT: &str = "http://vault.ecosystem.internal:8200";
    /// Default service mesh endpoints
    pub const SERVICE_MESH_ENDPOINT: &str = "https://service-mesh.ecosystem.internal:8443";
    pub const SONGBIRD_ENDPOINT: &str = "https://songbird.ecosystem.internal:8443";
    /// Test endpoints
    pub const TEST_LOCALHOST_HTTP: &str = "http://localhost:8080";
    pub const TEST_LOCALHOST_HTTPS: &str = "https://localhost:8443";
    pub const TEST_BEARDOG_ENDPOINT: &str = "https://beardog.security.internal";
    pub const TEST_NESTGATE_ENDPOINT: &str = "https://nestgate.local:8443";
    pub const TEST_SONGBIRD_ENDPOINT: &str = "https://songbird.local:8444";
    /// Webhook endpoints
    pub const TEST_WEBHOOK_ENDPOINT: &str = "http://localhost:8080/webhook";
    pub const SLACK_WEBHOOK_BASE: &str = "https://hooks.slack.com/services";
    pub const TEAMS_WEBHOOK_BASE: &str = "https://outlook.office.com/webhook";
    /// External service endpoints
    pub const TWILIO_API_BASE: &str = "https://api.twilio.com/2010-04-01";
    pub const AWS_SNS_BASE: &str = "https://sns.amazonaws.com";
    pub const MATRIX_DEFAULT: &str = "https://matrix.org";
    pub const AZURE_VAULT_EXAMPLE: &str = "https://example-vault.vault.azure.net";
/// **CANONICAL PORT CONFIGURATIONS** - Standard port assignments
pub mod ports {
    /// HTTP default port
    pub const HTTP: u16 = 80;
    /// HTTPS default port
    pub const HTTPS: u16 = 443;
    /// SSH default port
    pub const SSH: u16 = 22;
    /// FTP default port
    pub const FTP: u16 = 21;
    /// DNS default port
    pub const DNS: u16 = 53;
    /// SMTP default port
    pub const SMTP: u16 = 25;
    /// POP3 default port
    pub const POP3: u16 = 110;
    /// IMAP default port
    pub const IMAP: u16 = 143;
    /// `BearDog` service ports
    /// Default API port
    pub const API: u16 = 8080;
    /// Default metrics port
    pub const METRICS: u16 = 9090;
    /// Default health check port
    pub const HEALTH: u16 = 8088;
    /// Default admin port
    pub const ADMIN: u16 = 9999;
    /// Default gRPC port
    pub const GRPC: u16 = 9091;
    /// Default WebSocket port
    pub const WEBSOCKET: u16 = 8081;
    /// Default debug port
    pub const DEBUG: u16 = 8082;
/// **ENVIRONMENT VARIABLE CONFIGURATION** - Replace hardcoded values
/// This module provides functions to get network configuration from environment
/// variables with sensible defaults, eliminating hardcoded values throughout the codebase.
pub mod env_config {
    use super::bind_addresses::{DEFAULT_ADMIN_BIND_ADDRESS, DEFAULT_API_BIND_ADDRESS};
    use super::ports::API as DEFAULT_HTTP_PORT;
    use std::env;
    /// Get API bind address from environment or default
    #[must_use]
    pub fn get_api_bind_address() -> String {
        env::var("BEARDOG_API_BIND_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_API_BIND_ADDRESS.to_string())
    }
    /// Get API base URL from environment or construct from host/port
    pub fn get_api_base_url() -> String {
        env::var("BEARDOG_API_BASE_URL").unwrap_or_else(|_| {
            let host = env::var("BEARDOG_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("BEARDOG_PORT").unwrap_or_else(|_| DEFAULT_HTTP_PORT.to_string());
            format!("http://{host}:{port}")
        })
    /// Get service registry endpoint from environment or default
    pub fn get_service_registry_endpoint() -> String {
        env::var("BEARDOG_REGISTRY_ENDPOINT")
            .unwrap_or_else(|_| format!("{}/registry", get_api_base_url()))
    /// Get webhook base URL from environment or default
    pub fn get_webhook_base_url() -> String {
        env::var("BEARDOG_WEBHOOK_BASE_URL").unwrap_or_else(|_| get_api_base_url())
    /// Get SongBird endpoint from environment or default}


    pub fn get_songbird_endpoint() -> String {
        env::var("SONGBIRD_ENDPOINT")
            .unwrap_or_else(|_| "http://songbird.ecosystem.internal:8080".to_string())
    /// Get ToadStool endpoint from environment or default
    pub fn get_toadstool_endpoint() -> String {
        env::var("TOADSTOOL_ENDPOINT")
            .unwrap_or_else(|_| "http://toadstool.ecosystem.internal:8080".to_string())
    /// Get database URL from environment or default}


    pub fn get_database_url() -> String {
        env::var("BEARDOG_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://beardog:password@localhost:5432/beardog".to_string())
    /// Get metrics endpoint from environment or default
    pub fn get_metrics_bind_address() -> String {
        env::var("BEARDOG_METRICS_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:9090".to_string())
    /// Get health check endpoint from environment or default}


    pub fn get_health_bind_address() -> String {
        env::var("BEARDOG_HEALTH_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8088".to_string())
    /// Get admin endpoint from environment or default
    pub fn get_admin_bind_address() -> String {
        env::var("BEARDOG_ADMIN_BIND_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_ADMIN_BIND_ADDRESS.to_string())
    /// Get discovery endpoint from environment or default}


    pub fn get_discovery_endpoint() -> String {
        env::var("BEARDOG_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| format!("{}/discovery", get_api_base_url()))
    /// Get trusted hosts from environment or defaults
    pub fn get_trusted_hosts() -> Vec<String> {
        if let Ok(hosts_str) = env::var("BEARDOG_TRUSTED_HOSTS") {
            hosts_str.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec!["localhost".to_string(), "127.0.0.1".to_string()]
        }
    /// Get registry endpoints from environment or defaults
    pub fn get_registry_endpoints() -> Vec<String> {
        if let Ok(endpoints_str) = env::var("BEARDOG_REGISTRY_ENDPOINTS") {
            endpoints_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
            vec![
                "http://localhost:8080".to_string(),
                "http://127.0.0.1:8080".to_string(),
            ]
