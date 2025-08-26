

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {

    pub api_bind_address: String,

    pub api_port: u16,

    pub metrics_bind_address: Option<String>,
    pub health_bind_address: Option<String>,
    pub admin_bind_address: Option<String>,

    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,

    pub http: HttpConfig,

    pub https: HttpsConfig,

    pub node_communication: Option<NodeCommunicationConfig>,

    pub connection_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub max_connections: u32,

    pub keep_alive_enabled: bool,
    pub keep_alive_timeout: Duration,

    pub max_request_size: usize,
    pub max_response_size: usize,

    pub trusted_hosts: Vec<String>,

    pub custom_headers: HashMap<String, String>,
}

pub struct HttpConfig {

    pub enabled: bool,

    pub port: u16,

    pub bind_address: String,

    pub request_timeout: Duration,

    pub max_request_body_size: usize,

pub struct HttpsConfig {

    pub cert_path: String,

    pub key_path: String,

pub struct NodeCommunicationConfig {

    pub protocol: CommunicationProtocol,

    pub auth_method: AuthMethod,

    pub retry: RetryConfig,

    pub webhook: Option<WebhookConfig>,

    pub compression_enabled: bool,

    pub encryption_enabled: bool,

pub enum CommunicationProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
    Tcp,
    Udp,

pub enum AuthMethod {
    None,
    ApiKey,
    Bearer,
    Certificate,
    Mutual,

pub struct RetryConfig {

    pub max_attempts: u32,

    pub retry_delay: Duration,

    pub exponential_backoff: bool,

    pub max_retry_delay: Duration,

pub struct WebhookConfig {

    pub url: String,

    pub method: String,

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
            custom_headers: HashMap::with_capacity(16),
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
