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


/// # Canonical Constants
///
/// **SINGLE SOURCE OF TRUTH** for all constants used across the `BearDog` ecosystem.
/// This module provides unified constants that replace fragmented definitions across the codebase.
use std::time::Duration;

// ============================================================================
// API CONSTANTS
/// API version for all `BearDog` endpoints
pub use crate::constants::unified::api::VERSION as API_VERSION;
/// HTTP header name for API version
pub use crate::constants::unified::api::VERSION_HEADER as API_VERSION_HEADER;
/// Project information constants
pub const PROJECT_NAME: &str = "`BearDog`";
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");
// NETWORK CONSTANTS
/// Maximum number of concurrent connections
pub use crate::constants::unified::network::limits::MAX_CONNECTIONS;
/// Connection pool size for database and external services
pub const CONNECTION_POOL_SIZE: usize = 100;
/// Standard rate limit (requests per second)
pub const STANDARD_RATE_LIMIT: u32 = 100;
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
// Network constants will be available in the network module defined below
/// Default host for local bindings
pub const DEFAULT_HOST: &str = "127.0.0.1";
/// Load testing constants
pub const CONCURRENT_USERS: usize = 1000;
pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;
// SECURITY CONSTANTS
/// Maximum authentication attempts before lockout
pub const MAX_AUTH_ATTEMPTS: u32 = 5;
/// Account lockout duration
pub const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes
/// Session timeout duration
pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
/// Maximum concurrent sessions per user
pub const MAX_SESSIONS: u32 = 5;
/// Standard cryptographic key size in bits
pub const STANDARD_KEY_SIZE: u32 = 256;
/// Maximum operation attempts before failure
pub const MAX_OPERATION_ATTEMPTS: u32 = 3;
// PERFORMANCE CONSTANTS
/// Standard number of iterations for performance tests
pub const STANDARD_ITERATIONS: usize = 1000;
/// Light iterations for quick tests
pub const LIGHT_ITERATIONS: usize = 100;
/// Heavy iterations for stress tests
pub const HEAVY_ITERATIONS: usize = 10000;
/// Number of concurrent tasks for load testing
pub const CONCURRENT_TASKS: usize = 50;
/// Operations per task in concurrent tests
pub const OPERATIONS_PER_TASK: usize = 100;
/// Target requests per second for performance tests
pub const TARGET_RPS: u32 = 1000;
/// Test data size in bytes
pub const TEST_DATA_SIZE: usize = 1024;
// STORAGE CONSTANTS
/// Standard cache size (number of items)
pub const STANDARD_CACHE_SIZE: usize = 10_000;
/// Small cache size for limited use cases
pub const SMALL_CACHE_SIZE: usize = 1_000;
/// Default database connection timeout
pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
/// Default database idle timeout
pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300);
// HSM CONSTANTS
/// Maximum number of keys per HSM
pub const MAX_HSM_KEYS: usize = 1000;
/// HSM operation timeout
pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
/// HSM health check interval
pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
/// Minimum entropy bits for key generation
pub const MIN_ENTROPY_BITS: u32 = 256;
// ADDITIONAL SYSTEM CONSTANTS
/// Character set for secure token generation
pub const SECURE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
/// Batch size for processing operations
pub const STANDARD_BATCH_SIZE: usize = 100;
/// Compliance report cache duration
pub const COMPLIANCE_REPORT_TTL: Duration = Duration::from_secs(1800); // 30 minutes
/// Maximum metadata size per node
pub const MAX_NODE_METADATA_SIZE: usize = 4096; // 4KB
/// Node registration timeout
pub const NODE_REGISTRATION_TIMEOUT: Duration = Duration::from_secs(30);
/// Standard key size limit for HSM operations
pub const STANDARD_KEY_SIZE_LIMIT: usize = 4096;
/// Zero-cost software HSM provider identifier
pub const ZERO_COST_SOFTWARE_HSM_PROVIDER: &str = "software_hsm_zero_cost";
/// Default AES block size for encryption
pub const AES_BLOCK_SIZE: usize = 16;
/// Private IP address ranges for network security
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
// CONFIGURATION HELPERS
/// Get the default API host from environment or use constant
/// This function checks the `BEARDOG_API_HOST` environment variable first,
/// falling back to the default host constant if not set. This allows for
/// runtime configuration while maintaining sensible defaults.
/// # Returns
/// The API host as a String, either from environment or default constant.
/// # Examples
/// ```rust
/// use beardog_types::canonical::constants::default_api_host;
/// let host = default_api_host();
/// assert!(!host.is_empty());
/// ```
#[must_use]
pub fn default_api_host() -> String {
    std::env::var("BEARDOG_API_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}
/// Get the default API port from environment or use constant
/// This function checks the `BEARDOG_API_PORT` environment variable first,
/// falling back to the default HTTP port if not set or if parsing fails.
/// The API port as a u16, either from environment or default constant.
/// use beardog_types::canonical::constants::default_api_port;
/// let port = default_api_port();
/// assert!(port > 0);
pub fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_HTTP_PORT)
}
/// Get the default API bind address (host:port)
/// Combines the default API host and port into a bind address suitable
/// for server configuration. This is the recommended way to get a
/// complete bind address for API servers.
/// A String in the format "host:port" ready for binding.
/// use beardog_types::canonical::constants::default_api_bind_address;
/// let addr = default_api_bind_address();
/// assert!(addr.contains(':'));}


pub fn default_api_bind_address() -> String {
    format!("{}:{}", default_api_host(), default_api_port())
}
/// Get timeout from environment or use default
/// Retrieves the default timeout for operations from the environment
/// variable `BEARDOG_TIMEOUT_MS`, falling back to 30 seconds if not set.
/// Timeout in milliseconds as u64.
/// use beardog_types::canonical::constants::default_timeout_ms;
/// let timeout = default_timeout_ms();
/// assert!(timeout > 0);
pub fn default_timeout_ms() -> u64 {
    std::env::var("BEARDOG_TIMEOUT_MS")
        .map(|t| t.parse().unwrap_or(30000))
        .unwrap_or(30000) // 30 seconds default
}
/// Get health check interval from environment or use default
/// Retrieves the health check interval from the environment variable
/// `BEARDOG_HEALTH_CHECK_INTERVAL_MS`, falling back to 60 seconds if not set.
/// Health check interval in milliseconds as u64.
/// use beardog_types::canonical::constants::default_health_check_interval_ms;
/// let interval = default_health_check_interval_ms();
/// assert!(interval > 0);}


pub fn default_health_check_interval_ms() -> u64 {
    std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_MS")
        .map(|i| i.parse().unwrap_or(60000))
        .unwrap_or(60000) // 60 seconds default
}
/// Get key rotation timeout from environment or use default
/// Retrieves the key rotation timeout from the environment variable
/// `BEARDOG_KEY_ROTATION_TIMEOUT_MS`, falling back to 30 seconds if not set.
/// Key rotation timeout in milliseconds as u64.
/// use beardog_types::canonical::constants::default_key_rotation_timeout_ms;
/// let timeout = default_key_rotation_timeout_ms();
pub fn default_key_rotation_timeout_ms() -> u64 {
    std::env::var("BEARDOG_KEY_ROTATION_TIMEOUT_MS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(30000) // 30 seconds default
}
// CANONICAL CONSTANT GROUPS - Organized access
/// **API Constants** - Version information and API configuration
pub mod api {
    pub use super::{PROJECT_NAME, PROJECT_VERSION};
    pub use crate::constants::unified::api::{
        VERSION as API_VERSION, VERSION_HEADER as API_VERSION_HEADER,
    };
}

/// **Security Constants** - Cryptographic parameters and security limits
pub mod security {
    pub use super::{
        LOCKOUT_DURATION, MAX_AUTH_ATTEMPTS, MAX_OPERATION_ATTEMPTS, MAX_SESSIONS, SESSION_TIMEOUT,
        STANDARD_KEY_SIZE,
    };
}

/// **Network Constants** - Network configuration and connection limits
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

/// **Performance Constants** - Testing and benchmarking parameters
pub mod performance {
    pub use crate::constants::unified::performance::{
        CONCURRENT_TASKS, HEAVY_ITERATIONS, LIGHT_ITERATIONS, OPERATIONS_PER_TASK,
        STANDARD_ITERATIONS, TARGET_RPS, TEST_DATA_SIZE,
    };
    
    /// **RETRY CONSTANTS** - Retry configuration for workflows
    pub mod retry {
        /// Maximum workflow attempts
        pub const WORKFLOW_MAX_ATTEMPTS: u32 = 3;
    }
    
    /// **BATCH CONSTANTS** - Batch processing configuration
    pub mod batch {
        /// Key rotation batch size
        pub const KEY_ROTATION_BATCH_SIZE: usize = 100;
    }
}

/// **Storage Constants** - Database and caching parameters
pub mod storage {
    pub use super::{
        DEFAULT_CONNECTION_TIMEOUT, DEFAULT_IDLE_TIMEOUT, SMALL_CACHE_SIZE, STANDARD_CACHE_SIZE,
    };
}

/// **HSM Constants** - Hardware Security Module parameters
pub mod hsm {
    pub use super::{
        HSM_HEALTH_CHECK_INTERVAL, HSM_OPERATION_TIMEOUT, MAX_HSM_KEYS, MIN_ENTROPY_BITS,
        STANDARD_KEY_SIZE_LIMIT, ZERO_COST_SOFTWARE_HSM_PROVIDER,
    };
}

// CANONICAL CONSTANT REGISTRY
/// Registry of all canonical constants for validation and tooling
pub struct CanonicalConstantRegistry;
impl CanonicalConstantRegistry {
    /// Get list of all constant domains
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

    /// Check if a constant domain is canonical
    pub fn is_canonical_domain(domain: &str) -> bool {
        Self::constant_domains().contains(&domain)
    }

    /// Get commonly used constants for quick reference
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
