// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default implementations for production configuration structs used at runtime.

use super::{
    ApplicationConfig, AutoScalingConfig, ComplianceConfig, DataRetentionConfig,
    DatabaseConnection, TlsConfig,
};
use beardog_types::constants::domains::config::system::DEFAULT_SYSTEM_NAME;
use beardog_types::constants::network::{HTTP_DEV_PORT, POSTGRESQL_PORT};
use std::num::NonZeroUsize;

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            name: DEFAULT_SYSTEM_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            instance_id: uuid::Uuid::new_v4().to_string(),
            bind_address: beardog_errors::process_env::var("BEARDOG_BIND_ADDRESS").unwrap_or_else(
                |_| {
                    use beardog_types::constants::domains::network::config;
                    config::default_service_host()
                },
            ),
            port: HTTP_DEV_PORT,
            worker_threads: std::thread::available_parallelism()
                .map(NonZeroUsize::get)
                .unwrap_or(4),
            max_connections: 1000,
            request_timeout: 30,
            graceful_shutdown_timeout: 30,
        }
    }
}

impl Default for DatabaseConnection {
    fn default() -> Self {
        use beardog_types::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();

        Self {
            host: beardog_errors::process_env::var("BEARDOG_DB_HOST")
                .unwrap_or_else(|_| network_config.default_host.clone()),
            port: beardog_errors::process_env::var("BEARDOG_DB_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(POSTGRESQL_PORT),
            database: beardog_errors::process_env::var("BEARDOG_DB_NAME")
                .unwrap_or_else(|_| DEFAULT_SYSTEM_NAME.to_string()),
            username: beardog_errors::process_env::var("BEARDOG_DB_USER")
                .unwrap_or_else(|_| DEFAULT_SYSTEM_NAME.to_string()),
            password: String::new(),
            ssl_mode: "require".to_string(),
            connection_timeout: 5,
        }
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: String::new(),
            key_path: String::new(),
            ca_path: None,
            min_version: "1.2".to_string(),
            cipher_suites: vec![],
        }
    }
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_replicas: 2,
            max_replicas: 10,
            target_cpu_utilization: 70.0,
            target_memory_utilization: 80.0,
            scale_up_cooldown: 300,
            scale_down_cooldown: 600,
        }
    }
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            data_retention: DataRetentionConfig::default(),
            encryption_at_rest: false,
            encryption_in_transit: false,
            access_logging: true,
            compliance_standards: vec![],
        }
    }
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            logs: 30,
            metrics: 90,
            audit_trails: 2555, // 7 years
            user_data: 365,
        }
    }
}
