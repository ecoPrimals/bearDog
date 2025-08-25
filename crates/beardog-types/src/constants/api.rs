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


/// # API Constants
///
/// **CANONICAL API-RELATED CONSTANTS**
/// API versioning, headers, and endpoint configuration constants.

/// **CANONICAL API VERSION CONSTANTS** - API versioning and compatibility
pub mod versions {
    /// Main `BearDog` API version
    pub use crate::constants::unified::api::VERSION as API_VERSION;
    /// API version header name
    pub use crate::constants::unified::api::VERSION_HEADER as API_VERSION_HEADER;
    /// HSM Foundation version - centralized from hsm_foundation/mod.rs
    pub const HSM_FOUNDATION_VERSION: &str = "2.0.0-clean";
    /// Software HSM version - centralized from software_hsm/mod.rs  
    pub const SOFTWARE_HSM_VERSION: &str = "1.0.0";
    /// Universal storage provider version
    pub const UNIVERSAL_STORAGE_VERSION: &str = "2.0.0";
    /// Core system version
    pub const BEARDOG_CORE_VERSION: &str = "2.0.0";
    /// Minimum supported client version
    pub const MIN_SUPPORTED_CLIENT_VERSION: &str = "1.0.0";
    /// Current protocol version
    pub const PROTOCOL_VERSION: &str = "2.0";
}
/// **CANONICAL PROJECT INFO CONSTANTS** - Project metadata and build information
pub mod project_info {
    /// Project name
    pub const PROJECT_NAME: &str = "`BearDog`";
    /// Project description
    pub const PROJECT_DESCRIPTION: &str =
        "Enterprise Security Platform with Universal Ecosystem Integration";
    /// Project repository URL
    pub const REPOSITORY_URL: &str = "https://github.com/ecoPrimals/beardog";
    /// Project license
    pub const LICENSE: &str = "MIT";
    /// Minimum supported Rust version
    pub const MSRV: &str = "1.70.0";
    /// Build target information (set at compile time)
    pub const BUILD_TARGET: &str = std::env::consts::ARCH;
    /// Build OS information
    pub const BUILD_OS: &str = std::env::consts::OS;
/// **CANONICAL API ENDPOINT CONSTANTS** - API routing and endpoint definitions};


pub mod endpoints {
    /// Health check endpoint
    pub const HEALTH_ENDPOINT: &str = "/health";
    /// Metrics endpoint
    pub const METRICS_ENDPOINT: &str = "/metrics";
    /// API base path
    pub const API_BASE_PATH: &str = "/api/v1";
    /// Authentication endpoint
    pub const AUTH_ENDPOINT: &str = "/api/v1/auth";
    /// Storage capability endpoint
    pub const STORAGE_ENDPOINT: &str = "/api/v1/storage";
    /// Capability discovery endpoint
    pub const CAPABILITIES_ENDPOINT: &str = "/api/v1/capabilities";
    /// Provider registration endpoint
    pub const PROVIDER_REGISTRATION_ENDPOINT: &str = "/api/v1/providers/register";
    /// System status endpoint
    pub const STATUS_ENDPOINT: &str = "/api/v1/status";
/// **CANONICAL HTTP CONSTANTS** - HTTP-related constants
pub mod http {
    /// Default request timeout in seconds
    pub const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 30;
    /// Maximum request body size in bytes (10MB)
    pub const MAX_REQUEST_BODY_SIZE: usize = 10 * 1024 * 1024;
    /// Default page size for pagination
    pub const DEFAULT_PAGE_SIZE: usize = 50;
    /// Maximum page size for pagination
    pub const MAX_PAGE_SIZE: usize = 1000;
    /// Rate limiting window in seconds
    pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 60;
    /// CORS max age in seconds
    pub const CORS_MAX_AGE_SECONDS: u64 = 86400; // 24 hours
/// **CANONICAL AUTHENTICATION CONSTANTS** - Authentication and authorization
pub mod auth {
    /// JWT token expiration time in seconds (1 hour)
    pub const JWT_EXPIRATION_SECONDS: u64 = 3600;
    /// Refresh token expiration time in seconds (7 days)
    pub const REFRESH_TOKEN_EXPIRATION_SECONDS: u64 = 604_800;
    /// Maximum authentication attempts
    pub const MAX_AUTH_ATTEMPTS: u32 = 5;
    /// Account lockout duration in seconds (15 minutes)
    pub const ACCOUNT_LOCKOUT_SECONDS: u64 = 900;
    /// Session timeout in seconds (2 hours)
    pub const SESSION_TIMEOUT_SECONDS: u64 = 7200;
    /// API key length in characters
    pub const API_KEY_LENGTH: usize = 32;
// Re-export commonly used constants at module level
pub use endpoints::{API_BASE_PATH, HEALTH_ENDPOINT, METRICS_ENDPOINT};
pub use project_info::{MSRV, PROJECT_NAME};
pub use beardog_types::constants::unified::api::VERSION as {API_VERSION, API_VERSION_HEADER};
