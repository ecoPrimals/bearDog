//! Canonical constants for the BearDog ecosystem

use std::time::Duration;

pub use crate::constants::unified::api::VERSION as API_VERSION;
pub use crate::constants::unified::api::VERSION_HEADER as API_VERSION_HEADER;

pub const PROJECT_NAME: &str = "`BearDog`";
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use crate::constants::unified::network::limits::MAX_CONNECTIONS;

pub const CONNECTION_POOL_SIZE: usize = 100;

pub const STANDARD_RATE_LIMIT: u32 = 100;

pub const DEFAULT_HTTP_PORT: u16 = 8080;

pub const DEFAULT_HTTPS_PORT: u16 = 8443;

pub const DEFAULT_GRPC_PORT: u16 = 9090;

pub const DEFAULT_METRICS_PORT: u16 = 9091;

pub const DEFAULT_HEALTH_PORT: u16 = 8081;

pub const DEFAULT_HOST: &str = "127.0.0.1";

pub const CONCURRENT_USERS: usize = 1000;
pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;

pub const MAX_AUTH_ATTEMPTS: u32 = 5;

pub const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes

pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour

pub const MAX_SESSIONS: u32 = 5;

pub const STANDARD_KEY_SIZE: u32 = 256;

pub const MAX_OPERATION_ATTEMPTS: u32 = 3;

pub const STANDARD_ITERATIONS: usize = 1000;

pub const LIGHT_ITERATIONS: usize = 100;

pub const HEAVY_ITERATIONS: usize = 10000;

pub const CONCURRENT_TASKS: usize = 50;

pub const OPERATIONS_PER_TASK: usize = 100;

pub const TARGET_RPS: u32 = 1000;

pub const TEST_DATA_SIZE: usize = 1024;

pub const STANDARD_CACHE_SIZE: usize = 10_000;

pub const SMALL_CACHE_SIZE: usize = 1_000;

pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300);

pub const MAX_HSM_KEYS: usize = 1000;

pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

pub const MIN_ENTROPY_BITS: u32 = 256;

pub const SECURE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub const STANDARD_BATCH_SIZE: usize = 100;

pub const COMPLIANCE_REPORT_TTL: Duration = Duration::from_secs(1800); // 30 minutes

pub const MAX_NODE_METADATA_SIZE: usize = 4096; // 4KB

pub const NODE_REGISTRATION_TIMEOUT: Duration = Duration::from_secs(30);

pub const STANDARD_KEY_SIZE_LIMIT: usize = 4096;

pub const ZERO_COST_SOFTWARE_HSM_PROVIDER: &str = "software_hsm_zero_cost";

pub const AES_BLOCK_SIZE: usize = 16;

pub const PRIVATE_IP_RANGES: &[&str] = &[
    "10.0.0.0/8",
    "172.16.0.0/12",
    "192.168.0.0/16",
    "127.0.0.0/8",
    "169.254.0.0/16",
    "::1/128",
    "fc00::/7",
    "fe80::/10",
];

#[must_use]
pub fn default_api_host() -> String {
    std::env::var("BEARDOG_API_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

pub fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_HTTP_PORT)
}

pub fn default_api_bind_address() -> String {
    format!("{}:{}", default_api_host(), default_api_port())
}

pub fn default_timeout_ms() -> u64 {
    std::env::var("BEARDOG_TIMEOUT_MS")
        .map(|t| t.parse().unwrap_or(30000))
        .unwrap_or(30000) // 30 seconds default
}

pub fn default_health_check_interval_ms() -> u64 {
    std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_MS")
        .map(|i| i.parse().unwrap_or(60000))
        .unwrap_or(60000) // 60 seconds default
}

pub fn default_key_rotation_timeout_ms() -> u64 {
    std::env::var("BEARDOG_KEY_ROTATION_TIMEOUT_MS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(30000) // 30 seconds default
}

pub mod api {
    pub use super::{PROJECT_NAME, PROJECT_VERSION};
    pub use crate::constants::unified::api::VERSION as API_VERSION;
    pub use crate::constants::unified::api::VERSION_HEADER;
}

pub mod security {
    pub use super::{
        LOCKOUT_DURATION, MAX_AUTH_ATTEMPTS, MAX_OPERATION_ATTEMPTS, MAX_SESSIONS, SESSION_TIMEOUT,
        STANDARD_KEY_SIZE,
    };
}

pub mod network {
    pub use crate::constants::unified::network::addresses::DEFAULT_BIND as DEFAULT_HOST;
    pub use crate::constants::unified::network::limits::CONNECTION_POOL_SIZE;
    pub use crate::constants::unified::network::limits::MAX_CONNECTIONS;
    pub use crate::constants::unified::network::limits::STANDARD_RATE_LIMIT;
    pub use crate::constants::unified::network::ports::{
        API as DEFAULT_HTTP_PORT, GRPC as DEFAULT_GRPC_PORT, HEALTH as DEFAULT_HEALTH_PORT,
        HTTPS as DEFAULT_HTTPS_PORT, METRICS as DEFAULT_METRICS_PORT,
    };
}

pub mod performance {
    pub use crate::constants::unified::performance::{
        CONCURRENT_TASKS, HEAVY_ITERATIONS, LIGHT_ITERATIONS, OPERATIONS_PER_TASK,
        STANDARD_ITERATIONS, TARGET_RPS, TEST_DATA_SIZE,
    };

    pub mod retry {

        pub const WORKFLOW_MAX_ATTEMPTS: u32 = 3;
    }

    pub mod batch {

        pub const KEY_ROTATION_BATCH_SIZE: usize = 100;
    }
}

pub mod storage {
    pub use super::{
        DEFAULT_CONNECTION_TIMEOUT, DEFAULT_IDLE_TIMEOUT, SMALL_CACHE_SIZE, STANDARD_CACHE_SIZE,
    };
}

pub mod hsm {
    pub use super::{
        HSM_HEALTH_CHECK_INTERVAL, HSM_OPERATION_TIMEOUT, MAX_HSM_KEYS, MIN_ENTROPY_BITS,
        STANDARD_KEY_SIZE_LIMIT, ZERO_COST_SOFTWARE_HSM_PROVIDER,
    };
}

pub struct CanonicalConstantRegistry;
impl CanonicalConstantRegistry {

    #[must_use]
    pub fn constant_domains() -> Vec<&'static str> {
        vec![
            "api",
            "security",
            "network",
            "performance",
            "system",
            "hsm",
            "storage",
            "workflows",
            "compliance",
            "nodes",
        ]
    }

    pub fn is_canonical_domain(domain: &str) -> bool {
        Self::constant_domains().contains(&domain)
    }

    pub fn common_constants() -> Vec<(&'static str, &'static str)> {
        vec![
            ("API_VERSION", "Current API version"),
            ("MAX_CONNECTIONS", "Maximum concurrent connections"),
            ("SESSION_TIMEOUT", "Default session timeout"),
            ("STANDARD_KEY_SIZE", "Standard cryptographic key size"),
            ("DEFAULT_HTTP_PORT", "Default HTTP port"),
            ("DEFAULT_HTTPS_PORT", "Default HTTPS port"),
            ("MAX_AUTH_ATTEMPTS", "Maximum authentication attempts"),
            ("STANDARD_RATE_LIMIT", "Standard rate limiting"),
        ]
    }
}
