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


/// # Core Network Configuration
///
/// **BASIC NETWORK SETTINGS AND CORE CONFIGURATION**
/// Contains fundamental network configuration structs including basic networking,
/// HTTP/HTTPS settings, and core communication protocols.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// **ENHANCED CANONICAL NETWORK CONFIGURATION** - Use this instead of all duplicates
/// This consolidates all NetworkConfig variations found across:
/// - beardog-config/src/manager.rs (metrics, health, admin addresses, trusted hosts)
/// - beardog-config/src/runtime.rs (timeout, connection settings)
/// - beardog-config/src/network.rs (service discovery, load balancing)
/// - beardog-config/src/network_new.rs (external services, database connection)
/// - beardog-adapters (service mesh, routing)
/// All modules should migrate to this comprehensive, production-ready configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Primary API bind address
    pub api_bind_address: String,
    /// API port
    pub api_port: u16,
    /// Additional service endpoints
    pub metrics_bind_address: Option<String>,
    pub health_bind_address: Option<String>,
    pub admin_bind_address: Option<String>,
    /// TLS/SSL configuration
    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    /// HTTP configuration
    pub http: HttpConfig,
    /// HTTPS configuration  
    pub https: HttpsConfig,
    /// Node communication settings
    pub node_communication: Option<NodeCommunicationConfig>,
    /// Connection timeouts and limits
    pub connection_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub max_connections: u32,
    /// Keep-alive settings
    pub keep_alive_enabled: bool,
    pub keep_alive_timeout: Duration,
    /// Request/response limits
    pub max_request_size: usize,
    pub max_response_size: usize,
    /// Trusted hosts for security
    pub trusted_hosts: Vec<String>,
    /// Custom headers
    pub custom_headers: HashMap<String, String>,
}
/// HTTP server configuration
pub struct HttpConfig {
    /// Whether HTTP is enabled
    pub enabled: bool,
    /// HTTP port
    pub port: u16,
    /// Bind address
    pub bind_address: String,
    /// Request timeout
    pub request_timeout: Duration,
    /// Keep-alive timeout
    /// Maximum concurrent connections
    /// Maximum request body size
    pub max_request_body_size: usize,
/// HTTPS server configuration
pub struct HttpsConfig {
    /// Whether HTTPS is enabled
    /// HTTPS port
    /// TLS certificate path
    pub cert_path: String,
    /// TLS private key path
    pub key_path: String,
/// Node communication configuration
pub struct NodeCommunicationConfig {
    /// Communication protocol
    pub protocol: CommunicationProtocol,
    /// Authentication method
    pub auth_method: AuthMethod,
    /// Retry configuration
    pub retry: RetryConfig,
    /// Webhook configuration
    pub webhook: Option<WebhookConfig>,
    /// Message compression enabled
    pub compression_enabled: bool,
    /// Encryption enabled for node communication
    pub encryption_enabled: bool,
/// Communication protocol options
pub enum CommunicationProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
    Tcp,
    Udp,
/// Authentication methods for node communication}


pub enum AuthMethod {
    None,
    ApiKey,
    Bearer,
    Certificate,
    Mutual,
/// Retry configuration for network operations
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Exponential backoff enabled
    pub exponential_backoff: bool,
    /// Maximum retry delay
    pub max_retry_delay: Duration,
/// Webhook configuration
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Headers to include
    pub headers: HashMap<String, String>,
    pub timeout: Duration,}


impl Default for NetworkConfig {}


    fn default() -> Self {
        Self {
            api_bind_address: "127.0.0.1:8080".to_string(),
            api_port: 8080,
            metrics_bind_address: Some("127.0.0.1:9090".to_string()),
            health_bind_address: Some("127.0.0.1:8088".to_string()),
            admin_bind_address: Some("127.0.0.1:9999".to_string()),
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            http: HttpConfig::default(),
            https: HttpsConfig::default(),
            node_communication: None,
            connection_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(60),
            write_timeout: Duration::from_secs(60),
            max_connections: 1000,
            keep_alive_enabled: true,
            keep_alive_timeout: Duration::from_secs(75),
            max_request_size: 16 * 1024 * 1024,  // 16MB
            max_response_size: 32 * 1024 * 1024, // 32MB
            trusted_hosts: vec!["localhost".to_string(), "127.0.0.1".to_string()],
            custom_headers: HashMap::new(),
        }
    }
impl Default for HttpConfig {
            enabled: true,
            port: 8080,
            bind_address: "127.0.0.1".to_string(),
            request_timeout: Duration::from_secs(30),
            max_request_body_size: 16 * 1024 * 1024, // 16MB}


impl Default for HttpsConfig {
            enabled: false, // HTTPS disabled by default until certificates are configured
            port: 8443,
            cert_path: "cert.pem".to_string(),
            key_path: "key.pem".to_string(),
impl Default for RetryConfig {
            max_attempts: 3,
            retry_delay: Duration::from_millis(1000),
            exponential_backoff: true,
            max_retry_delay: Duration::from_secs(30),
