

use std::time::Duration;

pub const DEFAULT_HTTP_PORT: u16 = 8080;

pub const DEFAULT_HTTPS_PORT: u16 = 8443;

pub const DEFAULT_GRPC_PORT: u16 = 9090;

pub const DEFAULT_METRICS_PORT: u16 = 9091;

pub const DEFAULT_HEALTH_PORT: u16 = 8081;

pub const DEFAULT_HOST: &str = "127.0.0.1";

pub // use beardog_types::constants::unified::network::limits::MAX_CONNECTIONS // TODO: Add this import;

pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(300);

pub const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024;

pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024;

pub const NETWORK_BUFFER_SIZE: usize = 8192;

pub const DEFAULT_RATE_LIMIT: u32 = 1000;

pub const PRIVATE_IP_RANGES: &[&str] = &[
    "10.0.0.0/8",
    "172.16.0.0/12",
    "192.168.0.0/16",
    "127.0.0.0/8",
    "169.254.0.0/16",
];

pub mod core {

    pub use beardog_types::constants::unified::network::limits::MAX_CONNECTIONS;

    pub const CONNECTION_POOL_SIZE: usize = 100;

    pub const LARGE_CONNECTION_POOL_SIZE: usize = 500;

    pub const PRIVATE_IP_RANGES: &[&str] = &[
        "10.0.0.0/8",
        "172.16.0.0/12",
        "192.168.0.0/16",
        "127.0.0.0/8",
        "169.254.0.0/16",
    ];

    pub const STANDARD_RATE_LIMIT: u32 = 100;

    pub const RATE_LIMIT_BURST_SIZE: u32 = 20;
}

pub mod bind_addresses {

    pub const DEFAULT_API_BIND_ADDRESS: &str = "127.0.0.1:8080";

    pub const DEFAULT_METRICS_BIND_ADDRESS: &str = "127.0.0.1:9090";

    pub const DEFAULT_HEALTH_BIND_ADDRESS: &str = "127.0.0.1:8081";

    pub const DEFAULT_ADMIN_BIND_ADDRESS: &str = "127.0.0.1:9999";

    pub const DEFAULT_GRPC_BIND_ADDRESS: &str = "127.0.0.1:9091";

pub mod hosts {

    pub const LOCALHOST_IPV4: &str = "127.0.0.1";

    pub const LOCALHOST_IPV6: &str = "::1";

    pub const ALL_INTERFACES_IPV4: &str = "0.0.0.0";

    pub const ALL_INTERFACES_IPV6: &str = "::";

pub mod endpoints {

    pub const ECOSYSTEM_REGISTRY_ENDPOINT: &str = "https://ecosystem-registry.local:8443";
    pub const PROMETHEUS_ENDPOINT: &str = "http://prometheus.ecosystem.internal:9090";
    pub const GRAFANA_ENDPOINT: &str = "http://grafana.ecosystem.internal:3000";
    pub const VAULT_ENDPOINT: &str = "http://vault.ecosystem.internal:8200";

    pub const SERVICE_MESH_ENDPOINT: &str = "https://service-mesh.ecosystem.internal:8443";
    pub const SONGBIRD_ENDPOINT: &str = "https://songbird.ecosystem.internal:8443";

    pub const TEST_LOCALHOST_HTTP: &str = "http://localhost:8080";
    pub const TEST_LOCALHOST_HTTPS: &str = "https://localhost:8443";
    pub const TEST_BEARDOG_ENDPOINT: &str = "https://beardog.security.internal";
    pub const TEST_NESTGATE_ENDPOINT: &str = "https://nestgate.local:8443";
    pub const TEST_SONGBIRD_ENDPOINT: &str = "https://songbird.local:8444";

    pub const TEST_WEBHOOK_ENDPOINT: &str = "http://localhost:8080/webhook";
    pub const SLACK_WEBHOOK_BASE: &str = "https://hooks.slack.com/services";
    pub const TEAMS_WEBHOOK_BASE: &str = "https://outlook.office.com/webhook";

    pub const TWILIO_API_BASE: &str = "https://api.twilio.com/2010-04-01";
    pub const AWS_SNS_BASE: &str = "https://sns.amazonaws.com";
    pub const MATRIX_DEFAULT: &str = "https://matrix.org";
    pub const AZURE_VAULT_EXAMPLE: &str = "https://example-vault.vault.azure.net";

pub mod ports {

    pub const HTTP: u16 = 80;

    pub const HTTPS: u16 = 443;

    pub const SSH: u16 = 22;

    pub const FTP: u16 = 21;

    pub const DNS: u16 = 53;

    pub const SMTP: u16 = 25;

    pub const POP3: u16 = 110;

    pub const IMAP: u16 = 143;

    pub const API: u16 = 8080;

    pub const METRICS: u16 = 9090;

    pub const HEALTH: u16 = 8088;

    pub const ADMIN: u16 = 9999;

    pub const GRPC: u16 = 9091;

    pub const WEBSOCKET: u16 = 8081;

    pub const DEBUG: u16 = 8082;

pub mod env_config {
    use super::bind_addresses::{DEFAULT_ADMIN_BIND_ADDRESS, DEFAULT_API_BIND_ADDRESS};
    use super::ports::API as DEFAULT_HTTP_PORT;
    use std::env;

    #[must_use]
    pub fn get_api_bind_address() -> String {
        env::var("BEARDOG_API_BIND_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_API_BIND_ADDRESS.to_string())
    }

    pub fn get_api_base_url() -> String {
        env::var("BEARDOG_API_BASE_URL").unwrap_or_else(|_| {
            let host = env::var("BEARDOG_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("BEARDOG_PORT").unwrap_or_else(|_| DEFAULT_HTTP_PORT.to_string());
            format!("http://{host}:{port}")
        })

    pub fn get_service_registry_endpoint() -> String {
        env::var("BEARDOG_REGISTRY_ENDPOINT")
            .unwrap_or_else(|_| format_args!("{}/registry", get_api_base_url().to_string()))

    pub fn get_webhook_base_url() -> String {
        env::var("BEARDOG_WEBHOOK_BASE_URL").unwrap_or_else(|_| get_api_base_url())

    pub fn get_songbird_endpoint() -> String {
        env::var("SONGBIRD_ENDPOINT")
            .unwrap_or_else(|_| "http://songbird.ecosystem.internal:8080".to_string())

    pub fn get_toadstool_endpoint() -> String {
        env::var("TOADSTOOL_ENDPOINT")
            .unwrap_or_else(|_| "http://toadstool.ecosystem.internal:8080".to_string())

    pub fn get_database_url() -> String {
        env::var("BEARDOG_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://beardog:password@localhost:5432/beardog".to_string())

    pub fn get_metrics_bind_address() -> String {
        env::var("BEARDOG_METRICS_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:9090".to_string())

    pub fn get_health_bind_address() -> String {
        env::var("BEARDOG_HEALTH_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8088".to_string())

    pub fn get_admin_bind_address() -> String {
        env::var("BEARDOG_ADMIN_BIND_ADDRESS")
            .unwrap_or_else(|_| DEFAULT_ADMIN_BIND_ADDRESS.to_string())

    pub fn get_discovery_endpoint() -> String {
        env::var("BEARDOG_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| format_args!("{}/discovery", get_api_base_url().to_string()))

    pub fn get_trusted_hosts() -> Vec<String> {
        if let Ok(hosts_str) = env::var("BEARDOG_TRUSTED_HOSTS") {
            hosts_str.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec!["localhost".to_string(), "127.0.0.1".to_string()]
        }

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

pub fn test_localhost_http() -> String {
    std::env::var("BEARDOG_TEST_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8080".to_string())
}

pub fn test_webhook_endpoint() -> String {
    std::env::var("BEARDOG_WEBHOOK_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8080/webhook".to_string())
}

