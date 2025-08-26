

pub mod api {

    pub const VERSION: &str = "v1";

    pub const VERSION_HEADER: &str = "X-BearDog-API-Version";

    pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

    pub const PROJECT_NAME: &str = "BearDog";

    pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
}

pub mod network {
    pub mod ports {
        pub const API: u16 = 8080;
        pub const HTTPS: u16 = 8443;
        pub const GRPC: u16 = 9090;
        pub const METRICS: u16 = 9091;
        pub const HEALTH: u16 = 8081;
    }

    pub mod limits {
        pub const MAX_CONNECTIONS: usize = 1000;
        pub const CONNECTION_POOL_SIZE: usize = 100;
        pub const STANDARD_RATE_LIMIT: u32 = 100;
    }

    pub mod addresses {
        pub const DEFAULT_BIND: &str = "127.0.0.1";
        pub const LOCALHOST: &str = "127.0.0.1";
    }

    pub mod timeouts {
        use std::time::Duration;
        pub const CONNECTION: Duration = Duration::from_millis(30000); // Replaces DEFAULT_CONNECTION_TIMEOUT_MS
        pub const OPERATION: Duration = Duration::from_millis(30000); // Replaces DEFAULT_OPERATION_MS
        pub const CRYPTO_OPERATION: Duration = Duration::from_millis(5000); // Replaces CRYPTO_OPERATION_MS
        pub const NETWORK_OPERATION: Duration = Duration::from_millis(10000); // Replaces NETWORK_OPERATION_MS
        pub const HSM_OPERATION: Duration = Duration::from_millis(30000); // Replaces HSM_OPERATION_MS
        pub const KEY_ROTATION: Duration = Duration::from_millis(30000); // Replaces KEY_ROTATION_MS
    }

    pub const PRIVATE_IP_RANGES: &[&str] = &[
        "10.0.0.0/8",
        "172.16.0.0/12",
        "192.168.0.0/16",
        "127.0.0.0/8",
        "::1/128",
        "fc00::/7",
    ];

    pub const DEFAULT_API_ENDPOINT: &str = "https://api.beardog.local";
}

pub mod cache {

    pub const SMALL_SIZE: usize = 100;
    pub const STANDARD_SIZE: usize = 1_000;
    pub const LARGE_SIZE: usize = 10_000;

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

pub mod security {
    use std::time::Duration;

    pub const MAX_AUTH_ATTEMPTS: u32 = 3; // Replaces DEFAULT_AUTH_ATTEMPTS
    pub const MAX_SESSIONS: u32 = 100; // Replaces DEFAULT_MAX_SESSIONS
    pub const MAX_OPERATION_ATTEMPTS: u32 = 3; // Replaces DEFAULT_OPERATION_ATTEMPTS

    pub const STANDARD_KEY_SIZE: usize = 32; // 256 bits
    pub const MAX_KEY_SIZE: usize = 64; // 512 bits
    pub const MIN_ENTROPY_BITS: usize = 256;

    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    pub const LOCKOUT_DURATION: Duration = Duration::from_secs(300); // 5 minutes
    pub const TOKEN_REFRESH: Duration = Duration::from_secs(900); // 15 minutes

    pub const TEST_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
}

pub mod nodes {
    pub const SECURITY: &str = "security";
    pub const PHONEBOOK: &str = "phonebook";
    pub const FEDERATION: &str = "federation";
    pub const REGISTRY: &str = "registry";
    pub const MONITORING: &str = "monitoring";
    pub const API_GATEWAY: &str = "api_gateway";
    pub const LOAD_BALANCER: &str = "load_balancer";
    pub const DATABASE: &str = "database";
    pub const CACHE: &str = "cache";
    pub const AUTHENTICATION: &str = "authentication";
}

pub mod compliance {

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

pub mod performance {

    pub const LIGHT_ITERATIONS: usize = 100; // Replaces various light test values
    pub const STANDARD_ITERATIONS: usize = 1_000; // Replaces various standard values
    pub const HEAVY_ITERATIONS: usize = 10_000; // Replaces various heavy test values

    pub const CONCURRENT_TASKS: usize = 50; // Replaces various concurrent values
    pub const CONCURRENT_USERS: usize = 1_000; // Load testing
    pub const OPERATIONS_PER_TASK: usize = 100;

    pub const TARGET_RPS: u32 = 1_000; // Requests per second
    pub const TEST_DATA_SIZE: usize = 1_024; // 1KB test data
    pub const MAX_SAMPLES: usize = 1_000; // Replaces MAX_SAMPLES from monitoring

    pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;

    pub const DEFAULT_RESPONSE_TIME_THRESHOLD: f64 = 5000.0; // 5 seconds
}

pub mod system {
    use std::time::Duration;

    pub const MAX_FILE_LINES: usize = 2_000;
    pub const RECOMMENDED_FILE_LINES: usize = 1_500;

    pub const MAX_MEMORY_MB: usize = 1_024; // 1GB default limit
    pub const MAX_CPU_CORES: usize = 8; // Default CPU limit

    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);
}

pub struct UnifiedConstantRegistry;
impl UnifiedConstantRegistry {

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

    pub fn is_unified_constant(module_path: &str, _constant_name: &str) -> bool {
        module_path.starts_with("beardog_types::constants::unified::")
            || module_path.starts_with("crate::constants::unified::")
    }

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

pub mod storage {
    use std::time::Duration;

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
