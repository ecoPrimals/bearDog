

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {

    pub api_port: u16,

    pub metrics_port: u16,

    pub health_port: u16,

    pub admin_port: u16,

    pub grpc_port: u16,

    pub websocket_port: u16,

    pub debug_port: Option<u16>,

    pub dynamic_ranges: Vec<PortRange>,

    pub services: HashMap<String, ServicePortConfig>,

    pub bindings: Vec<PortBindingConfig>,
}

pub struct PortRange {

    pub name: String,

    pub start: u16,

    pub end: u16,

    pub description: Option<String>,

    pub reserved: bool,

pub struct ServicePortConfig {

    pub port: u16,

    pub alternative_ports: Vec<u16>,

    pub bind_address: String,

    pub secure: bool,

    pub protocol: ServiceProtocol,

pub struct PortBindingConfig {

    pub address: String,

    pub protocol: BindingProtocol,

    pub enabled: bool,

pub enum ServiceProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
    Tcp,
    Udp,

pub enum BindingProtocol {
    Both,

pub mod well_known {

    pub const HTTP: u16 = 80;

    pub const HTTPS: u16 = 443;

    pub const SSH: u16 = 22;

    pub const FTP: u16 = 21;

    pub const DNS: u16 = 53;

    pub const SMTP: u16 = 25;

    pub const POP3: u16 = 110;

    pub const IMAP: u16 = 143;

pub mod beardog_defaults {

    pub const API: u16 = 8080;

    pub const METRICS: u16 = 9090;

    pub const HEALTH: u16 = 8088;

    pub const ADMIN: u16 = 9999;

    pub const GRPC: u16 = 9091;

    pub const WEBSOCKET: u16 = 8081;

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
            services: HashMap::with_capacity(16),
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
