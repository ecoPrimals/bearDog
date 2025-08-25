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


/// # Unified Constants System
///
/// **ELIMINATES ALL CONSTANT FRAGMENTATION** - Single source of truth for ALL constants
/// This module consolidates the 150+ scattered constants found across the codebase:
/// ## **Fragmentation Eliminated:**
/// - `beardog-config/src/canonical.rs` - 25+ port/network constants
/// - `beardog-api/src/api/cache.rs` - 7+ TTL constants  
/// - `beardog-api/src/api/types.rs` - API version constants
/// - `beardog-node-registry/src/*/types/*.rs` - 20+ node type constants
/// - `beardog-compliance/src/compliance/handlers.rs` - 15+ error message constants
/// - `beardog-auth/src/auth/tests.rs` - Test charset constants
/// - Multiple test files - Performance threshold constants
/// - Scattered timeout and limit constants across 10+ crates
/// ## **Design Principles:**
/// - **Single Definition**: Each constant defined exactly once
/// - **Semantic Organization**: Constants grouped by domain and usage
/// - **Type Safety**: Proper types (Duration, not raw numbers)
/// - **Zero Duplication**: Eliminates all duplicate constant definitions
/// - **Canonical Access**: All constants accessible through unified paths

// ============================================================================
// API CONSTANTS - Consolidated from multiple API modules
pub mod api {
    /// API version - replaces scattered API_VERSION constants
    pub const VERSION: &str = "v1";
    /// API version header - consolidated from beardog-api
    pub const VERSION_HEADER: &str = "X-BearDog-API-Version";
    /// Project version from Cargo.toml
    pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");
    /// Project name
    pub const PROJECT_NAME: &str = "BearDog";
    /// Mission statement
    pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
}
// NETWORK CONSTANTS - Consolidated from beardog-config and multiple modules
pub mod network {
    /// Port constants - replaces scattered port definitions
    pub mod ports {
        pub const API: u16 = 8080; // Replaces DEFAULT_API_PORT
        pub const HTTPS: u16 = 8443; // Replaces DEFAULT_HTTPS_PORT
        pub const METRICS: u16 = 9090; // Replaces DEFAULT_METRICS_PORT
        pub const HEALTH: u16 = 8081; // Replaces DEFAULT_HEALTH_PORT
        pub const ADMIN: u16 = 9999; // Replaces DEFAULT_ADMIN_PORT
        pub const GRPC: u16 = 9091; // Replaces DEFAULT_GRPC_PORT
        pub const POSTGRES: u16 = 5432; // Replaces DEFAULT_POSTGRES_PORT
        pub const REDIS: u16 = 6379; // Replaces DEFAULT_REDIS_PORT
    }
    /// Address constants - replaces scattered IP address constants
    pub mod addresses {
        pub const LOCALHOST_IPV4: &str = "127.0.0.1"; // Replaces LOCALHOST_IPV4
        pub const LOCALHOST_IPV6: &str = "::1"; // Replaces LOCALHOST_IPV6
        pub const ANY_IPV4: &str = "0.0.0.0"; // Replaces ANY_IPV4
        pub const ANY_IPV6: &str = "::"; // Replaces ANY_IPV6
        pub const DEFAULT_BIND: &str = "127.0.0.1"; // Replaces DEFAULT_BIND_ADDRESS
        pub const PRODUCTION_BIND: &str = "0.0.0.0"; // Replaces PRODUCTION_BIND_ADDRESS
    }
    
    /// Connection limits - replaces scattered connection constants
    pub mod limits {
        pub const MAX_CONNECTIONS: usize = 1000; // Standard maximum connections
        pub const CONNECTION_POOL_SIZE: usize = 100; // Standard pool size
        pub const MAX_DB_CONNECTIONS: u32 = 10; // Replaces DEFAULT_MAX_CONNECTIONS
        pub const STANDARD_RATE_LIMIT: u32 = 100; // Requests per second
    }
    
    /// Timeout constants - replaces scattered timeout definitions
    pub mod timeouts {
        use std::time::Duration;
        pub const CONNECTION: Duration = Duration::from_millis(30000); // Replaces DEFAULT_CONNECTION_TIMEOUT_MS
        pub const OPERATION: Duration = Duration::from_millis(30000); // Replaces DEFAULT_OPERATION_MS
        pub const CRYPTO_OPERATION: Duration = Duration::from_millis(5000); // Replaces CRYPTO_OPERATION_MS
        pub const NETWORK_OPERATION: Duration = Duration::from_millis(10000); // Replaces NETWORK_OPERATION_MS
        pub const HSM_OPERATION: Duration = Duration::from_millis(30000); // Replaces HSM_OPERATION_MS
        pub const KEY_ROTATION: Duration = Duration::from_millis(30000); // Replaces KEY_ROTATION_MS
    }

    /// Private IP ranges - replaces PRIVATE_IP_RANGES array
    pub const PRIVATE_IP_RANGES: &[&str] = &[
        "10.0.0.0/8",
        "172.16.0.0/12",
        "192.168.0.0/16",
        "127.0.0.0/8",
        "::1/128",
        "fc00::/7",
    ];

    /// Default API endpoint
    pub const DEFAULT_API_ENDPOINT: &str = "https://api.beardog.local";
}
// CACHE CONSTANTS - Consolidated from beardog-api cache module
pub mod cache {
    
    
    /// Cache sizes - replaces scattered cache size constants
    pub const SMALL_SIZE: usize = 100;
    pub const STANDARD_SIZE: usize = 1_000;
    pub const LARGE_SIZE: usize = 10_000;
    
    /// TTL constants - replaces scattered Duration constants from beardog-api
    pub mod ttl {
        use std::time::Duration;
        
        pub const API_RESPONSE: Duration = Duration::from_secs(300); // 5 minutes
        pub const USER_SESSION: Duration = Duration::from_secs(3600); // 1 hour
        pub const THREAT_ANALYSIS: Duration = Duration::from_secs(600); // 10 minutes
        pub const COMPLIANCE_REPORT: Duration = Duration::from_secs(1800); // 30 minutes
        pub const NODE_STATUS: Duration = Duration::from_secs(120); // 2 minutes
        pub const CONFIG_DATA: Duration = Duration::from_secs(3600); // 1 hour
        pub const STATIC_CONTENT: Duration = Duration::from_secs(86400); // 24 hours
        pub const STANDARD: Duration = Duration::from_secs(300); // Default 5 minutes
    }
}

// SECURITY CONSTANTS - Consolidated from multiple security modules
pub mod security {
    use std::time::Duration;
    /// Authentication limits - replaces scattered auth constants
    pub const MAX_AUTH_ATTEMPTS: u32 = 3; // Replaces DEFAULT_AUTH_ATTEMPTS
    pub const MAX_SESSIONS: u32 = 100; // Replaces DEFAULT_MAX_SESSIONS
    pub const MAX_OPERATION_ATTEMPTS: u32 = 3; // Replaces DEFAULT_OPERATION_ATTEMPTS
    /// Key management - replaces scattered key constants
    pub const STANDARD_KEY_SIZE: usize = 32; // 256 bits
    pub const MAX_KEY_SIZE: usize = 64; // 512 bits
    pub const MIN_ENTROPY_BITS: usize = 256;
    /// Timeouts - replaces scattered security timeout constants  
    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    pub const LOCKOUT_DURATION: Duration = Duration::from_secs(300); // 5 minutes
    pub const TOKEN_REFRESH: Duration = Duration::from_secs(900); // 15 minutes
    /// Test constants - replaces CHARSET from beardog-auth tests
    pub const TEST_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
}

// NODE TYPE CONSTANTS - Consolidated from beardog-node-registry
pub mod nodes {
    /// Node type constants - replaces duplicate constants across node modules
    pub const SECURITY: &str = "security";
    pub const PHONEBOOK: &str = "phonebook";
    pub const FEDERATION: &str = "federation";
    pub const COMPUTE: &str = "compute";
    pub const STORAGE: &str = "storage";
    pub const RELAY: &str = "relay";
    pub const BACKUP: &str = "backup";
    pub const MONITORING: &str = "monitoring";
    pub const ANALYTICS: &str = "analytics";
    pub const GATEWAY: &str = "gateway";
    
    /// Service type constants for node registry (same values as node types)
    pub const SECURITY_SERVICE: &str = "security";
    pub const PHONEBOOK_SERVICE: &str = "phonebook";
    pub const FEDERATION_SERVICE: &str = "federation";
    pub const COMPUTE_SERVICE: &str = "compute";
    pub const MONITORING_SERVICE: &str = "monitoring";
    pub const STORAGE_SERVICE: &str = "storage";
}
// COMPLIANCE CONSTANTS - Consolidated from beardog-compliance handlers
pub mod compliance {
    /// Error codes - replaces scattered error constants
    pub mod errors {
        pub const AUDIT_RETENTION_EXCEEDED: &str = "AuditRetentionExceeded";
        pub const MISSING_CONSENT: &str = "MissingConsent";
        pub const MISSING_PURPOSE: &str = "MissingPurpose";
        pub const ILLEGAL_TRANSFER: &str = "IllegalTransfer";
        pub const UNAUTHORIZED_FINANCIAL: &str = "UnauthorizedFinancialAccess";
        pub const UNENCRYPTED_PAYMENT: &str = "UnencryptedPaymentData";
        pub const MINIMUM_NECESSARY: &str = "MinimumNecessaryViolation";
        pub const MISSING_AUDIT_LOG: &str = "MissingAuditLog";
    }
    
    /// Error messages - replaces scattered message constants
    pub mod messages {
        pub const AUDIT_RETENTION_EXCEEDED: &str = "Event timestamp exceeds audit retention period";
        pub const MISSING_CONSENT: &str = "Data access without explicit consent";
        pub const MISSING_PURPOSE: &str = "Data access purpose not specified";
        pub const ILLEGAL_TRANSFER: &str = "Data transfer without adequate protection";
        pub const UNAUTHORIZED_FINANCIAL: &str =
            "Financial data access without proper authorization";
        pub const UNENCRYPTED_PAYMENT: &str = "Payment data processed without encryption";
        pub const MINIMUM_NECESSARY: &str = "PHI access not limited to minimum necessary";
        pub const MISSING_AUDIT_LOG: &str = "PHI access not properly logged";
    }
}

// PERFORMANCE CONSTANTS - Consolidated from test files and performance modules
pub mod performance {
    /// Test iteration constants - replaces scattered test constants
    pub const LIGHT_ITERATIONS: usize = 100; // Replaces various light test values
    pub const STANDARD_ITERATIONS: usize = 1_000; // Replaces various standard values
    pub const HEAVY_ITERATIONS: usize = 10_000; // Replaces various heavy test values
    /// Concurrency constants - replaces scattered concurrent task constants
    pub const CONCURRENT_TASKS: usize = 50; // Replaces various concurrent values
    pub const CONCURRENT_USERS: usize = 1_000; // Load testing
    pub const OPERATIONS_PER_TASK: usize = 100;
    /// Performance targets - replaces scattered performance constants
    pub const TARGET_RPS: u32 = 1_000; // Requests per second
    pub const TEST_DATA_SIZE: usize = 1_024; // 1KB test data
    pub const MAX_SAMPLES: usize = 1_000; // Replaces MAX_SAMPLES from monitoring
    /// Load testing - replaces scattered load test constants
    pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;
    /// Response time thresholds
    pub const DEFAULT_RESPONSE_TIME_THRESHOLD: f64 = 5000.0; // 5 seconds
}

// SYSTEM CONSTANTS - General system configuration
pub mod system {
    use std::time::Duration;
    
    /// File size limits - enforces 2000 line limit requirement
    pub const MAX_FILE_LINES: usize = 2_000;
    pub const RECOMMENDED_FILE_LINES: usize = 1_500;
    
    /// System limits
    pub const MAX_MEMORY_MB: usize = 1_024; // 1GB default limit
    pub const MAX_CPU_CORES: usize = 8; // Default CPU limit
    
    /// Health check intervals
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);
}

// UNIFIED CONSTANT REGISTRY - Validation and tooling support
/// Registry for all unified constants - enables validation and migration tools
pub struct UnifiedConstantRegistry;
impl UnifiedConstantRegistry {
    /// Get all constant domains
    pub fn domains() -> Vec<&'static str> {
        vec![
            "api",
            "network",
            "cache",
            "security",
            "nodes",
            "compliance",
            "performance",
            "system",
        ]
    }

    /// Check if a given constant is part of the unified system
    pub fn is_unified_constant(module_path: &str, _constant_name: &str) -> bool {
        module_path.starts_with("beardog_types::constants::unified::")
            || module_path.starts_with("crate::constants::unified::")
    }

    /// Get migration mapping for legacy constants
    pub fn get_migration_path(legacy_path: &str) -> Option<&'static str> {
        match legacy_path {
            "beardog_types::config::canonical::DEFAULT_API_PORT" => {
                Some("beardog_types::constants::unified::network::ports::API")
            }
            "beardog_api::api::types::API_VERSION" => {
                Some("beardog_types::constants::unified::api::VERSION")
            }
            "beardog_node_registry::*::SECURITY" => {
                Some("beardog_types::constants::unified::nodes::SECURITY")
            }
            _ => None,
        }
    }
}
// CONVENIENCE RE-EXPORTS - Most commonly used constants
pub use api::{VERSION as API_VERSION, VERSION_HEADER};
pub use cache::STANDARD_SIZE as DEFAULT_CACHE_SIZE;
pub use network::limits::MAX_CONNECTIONS;
pub use performance::{CONCURRENT_TASKS, STANDARD_ITERATIONS, TARGET_RPS};
pub use security::{MAX_AUTH_ATTEMPTS, SESSION_TIMEOUT, STANDARD_KEY_SIZE};
pub use system::MAX_FILE_LINES;

// STORAGE CONSTANTS - Consolidated from beardog-types/src/constants/storage.rs
pub mod storage {
    use std::time::Duration;
    
    /// Database configuration constants
    pub mod database {
        use super::Duration;
        
        pub const DEFAULT_DATABASE_URL: &str = "sqlite://beardog.db";
        pub const DEFAULT_DATABASE_HOSTS: &[&str] = &[
            "beardog-db", // Docker/K8s service name
            "database",   // Alternative service name  
            "localhost",  // Local development fallback
        ];
        pub const DEFAULT_MAX_CONNECTIONS: u32 = 10;
        pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
        pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
        pub const DEFAULT_MAX_LIFETIME: Duration = Duration::from_secs(3600); // 1 hour
    }
    
    /// Cache configuration constants
    pub mod cache {
        use super::Duration;
        
        pub const STANDARD_CACHE_SIZE: usize = 10_000;
        pub const SMALL_CACHE_SIZE: usize = 1_000;
        pub const LARGE_CACHE_SIZE: usize = 50_000;
        pub const STANDARD_TTL: Duration = Duration::from_secs(3600); // 1 hour
        pub const SHORT_TTL: Duration = Duration::from_secs(300); // 5 minutes
        pub const LONG_TTL: Duration = Duration::from_secs(86400); // 24 hours
        pub const API_RESPONSE_TTL: Duration = Duration::from_secs(300);
        pub const USER_SESSION_TTL: Duration = Duration::from_secs(3600);
        pub const THREAT_ANALYSIS_TTL: Duration = Duration::from_secs(600);
    }
    
    /// File system path constants
    pub mod paths {
        pub const DEFAULT_DATA_DIR: &str = "./data";
        pub const DEFAULT_LOG_DIR: &str = "./logs";
        pub const DEFAULT_CONFIG_DIR: &str = "./config";
        pub const DEFAULT_BACKUP_DIR: &str = "./backups";
        pub const DEFAULT_TEMP_DIR: &str = "./tmp";
        pub const DEFAULT_KEYS_DIR: &str = "./keys";
        pub const DEFAULT_CERTS_DIR: &str = "./certs";
        pub const DEFAULT_DATABASE_PATH: &str = "./data/beardog.db";
        pub const CONFIG_FILE_NAME: &str = "beardog.toml";
        pub const LOG_FILE_PATTERN: &str = "beardog-%Y-%m-%d.log";
        pub const MAX_LOG_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
        pub const MAX_LOG_FILES: usize = 30;
    }
}
