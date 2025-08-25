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


/// # Port Configuration
///
/// **PORT DEFINITIONS AND BINDING CONFIGURATIONS**
/// Contains port-related configuration structs including port ranges,
/// service port mappings, and binding configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// **CANONICAL PORT CONFIGURATION** - Eliminates port config duplicates
/// Consolidates port configurations from multiple locations:
/// - beardog-config/src/manager.rs (PortConfig struct)
/// - Various service-specific port configurations
/// - Network module port definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    /// API server port
    pub api_port: u16,
    /// Metrics server port
    pub metrics_port: u16,
    /// Health check server port
    pub health_port: u16,
    /// Admin server port
    pub admin_port: u16,
    /// gRPC server port
    pub grpc_port: u16,
    /// WebSocket server port
    pub websocket_port: u16,
    /// Debug server port (development only)
    pub debug_port: Option<u16>,
    /// Dynamic port ranges for services
    pub dynamic_ranges: Vec<PortRange>,
    /// Service-specific port configurations
    pub services: HashMap<String, ServicePortConfig>,
    /// Port binding configurations
    pub bindings: Vec<PortBindingConfig>,
}
/// Port range configuration
pub struct PortRange {
    /// Range name/identifier
    pub name: String,
    /// Start port (inclusive)
    pub start: u16,
    /// End port (inclusive)
    pub end: u16,
    /// Range description
    pub description: Option<String>,
    /// Whether range is reserved for system use
    pub reserved: bool,
/// Service-specific port configuration
pub struct ServicePortConfig {
    /// Primary service port
    pub port: u16,
    /// Alternative ports for the service
    pub alternative_ports: Vec<u16>,
    /// Bind address for the service
    pub bind_address: String,
    /// Whether the port is secure (HTTPS/TLS)
    pub secure: bool,
    /// Service protocol
    pub protocol: ServiceProtocol,
    /// Port description
/// Port binding configuration
pub struct PortBindingConfig {
    /// Port number
    /// Bind address (IP)
    pub address: String,
    /// Protocol (TCP/UDP)
    pub protocol: BindingProtocol,
    /// Whether binding is enabled
    pub enabled: bool,
    /// Binding description
/// Supported service protocols
pub enum ServiceProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
    Tcp,
    Udp,
/// Network binding protocols}


pub enum BindingProtocol {
    Both,
/// Well-known port definitions
pub mod well_known {
    /// HTTP port
    pub const HTTP: u16 = 80;
    /// HTTPS port
    pub const HTTPS: u16 = 443;
    /// SSH port
    pub const SSH: u16 = 22;
    /// FTP port
    pub const FTP: u16 = 21;
    /// DNS port
    pub const DNS: u16 = 53;
    /// SMTP port
    pub const SMTP: u16 = 25;
    /// POP3 port
    pub const POP3: u16 = 110;
    /// IMAP port
    pub const IMAP: u16 = 143;
/// `BearDog` default port definitions
pub mod beardog_defaults {
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
    pub const DEBUG: u16 = 8082;}


impl Default for PortConfig {}


    fn default() -> Self {
        Self {
            api_port: beardog_defaults::API,
            metrics_port: beardog_defaults::METRICS,
            health_port: beardog_defaults::HEALTH,
            admin_port: beardog_defaults::ADMIN,
            grpc_port: beardog_defaults::GRPC,
            websocket_port: beardog_defaults::WEBSOCKET,
            debug_port: Some(beardog_defaults::DEBUG),
            dynamic_ranges: vec![PortRange {
                name: "ephemeral".to_string(),
                start: 32768,
                end: 65535,
                description: Some("Ephemeral port range".to_string()),
                reserved: false,
            }],
            services: HashMap::new(),
            bindings: vec![],
        }
    }
impl Default for ServicePortConfig {
            port: beardog_defaults::API,
            alternative_ports: vec![],
            bind_address: "127.0.0.1".to_string(),
            secure: false,
            protocol: ServiceProtocol::Http,
            description: None,
