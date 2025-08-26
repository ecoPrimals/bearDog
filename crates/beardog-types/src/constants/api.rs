

pub mod versions {

    pub use beardog_types::constants::unified::api::VERSION as API_VERSION;

    pub use beardog_types::constants::unified::api::VERSION as API_VERSION_HEADER;

    pub const HSM_FOUNDATION_VERSION: &str = "2.0.0-clean";

    pub const SOFTWARE_HSM_VERSION: &str = "1.0.0";

    pub const UNIVERSAL_STORAGE_VERSION: &str = "2.0.0";

    pub const BEARDOG_CORE_VERSION: &str = "2.0.0";

    pub const MIN_SUPPORTED_CLIENT_VERSION: &str = "1.0.0";

    pub const PROTOCOL_VERSION: &str = "2.0";
}

pub mod project_info {

    pub const PROJECT_NAME: &str = "`BearDog`";

    pub const PROJECT_DESCRIPTION: &str =
        "Enterprise Security Platform with Universal Ecosystem Integration";

    pub const REPOSITORY_URL: &str = "https://github.com/ecoPrimals/beardog";

    pub const LICENSE: &str = "MIT";

    pub const MSRV: &str = "1.70.0";

    pub const BUILD_TARGET: &str = std::env::consts::ARCH;

    pub const BUILD_OS: &str = std::env::consts::OS;

pub mod endpoints {

    pub const HEALTH_ENDPOINT: &str = "/health";

    pub const METRICS_ENDPOINT: &str = "/metrics";

    pub const API_BASE_PATH: &str = "/api/v1";

    pub const AUTH_ENDPOINT: &str = "/api/v1/auth";

    pub const STORAGE_ENDPOINT: &str = "/api/v1/storage";

    pub const CAPABILITIES_ENDPOINT: &str = "/api/v1/capabilities";

    pub const PROVIDER_REGISTRATION_ENDPOINT: &str = "/api/v1/providers/register";

    pub const STATUS_ENDPOINT: &str = "/api/v1/status";

pub mod http {

    pub const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 30;

    pub const MAX_REQUEST_BODY_SIZE: usize = 10 * 1024 * 1024;

    pub const DEFAULT_PAGE_SIZE: usize = 50;

    pub const MAX_PAGE_SIZE: usize = 1000;

    pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 60;

    pub const CORS_MAX_AGE_SECONDS: u64 = 86400; // 24 hours

pub mod auth {

    pub const JWT_EXPIRATION_SECONDS: u64 = 3600;

    pub const REFRESH_TOKEN_EXPIRATION_SECONDS: u64 = 604_800;

    pub const MAX_AUTH_ATTEMPTS: u32 = 5;

    pub const ACCOUNT_LOCKOUT_SECONDS: u64 = 900;

    pub const SESSION_TIMEOUT_SECONDS: u64 = 7200;

    pub const API_KEY_LENGTH: usize = 32;

pub use endpoints::{API_BASE_PATH, HEALTH_ENDPOINT, METRICS_ENDPOINT};
pub use project_info::{MSRV, PROJECT_NAME};
pub use beardog_types::constants::unified::api::VERSION as {API_VERSION, API_VERSION_HEADER};
