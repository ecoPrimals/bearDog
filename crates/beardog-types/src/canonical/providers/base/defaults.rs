// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default implementations for provider configuration types and trait bridges.

use std::collections::BTreeMap;

use crate::canonical::traits::{CacheStrategy, TimeoutPolicy};
use crate::constants::time;

use super::configuration::{
use beardog_config::env_keys;
    AbacConfiguration, AuthenticationConfiguration, AuthenticationMethod,
    AuthorizationConfiguration, AuthorizationMethod, EncryptionAlgorithm,
    EncryptionConfiguration, EnvironmentSettings, KeyDerivationAlgorithm,
    KeyDerivationConfiguration, KeyManagementConfiguration, KeyProvider, PolicyDecision,
    ProviderConfiguration, RbacConfiguration, ResourceLimits, SecurityConfiguration,
};
use crate::canonical::config::domains::system::LoggingConfig;
use super::performance::{
    BackoffStrategy, CachingConfiguration, ConnectionPoolConfiguration, EvictionPolicy,
    PerformanceConfiguration, RetryConfiguration, TimeoutConfiguration,
};
use super::schema::ConfigurationSchema;

// Default implementations
impl Default for ProviderConfiguration {
    fn default() -> Self {
        Self {
            provider_id: String::new(),
            parameters: BTreeMap::new(),
            environment: EnvironmentSettings::default(),
            security: SecurityConfiguration::default(),
            performance: PerformanceConfiguration::default(),
            metadata: BTreeMap::new(),
        }
    }
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            variables: BTreeMap::new(),
            resource_limits: ResourceLimits::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_cpu_percent: None,
            max_disk_bytes: None,
            max_network_bps: None,
            max_concurrent_operations: Some(100),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            structured: true,
            targets: vec!["stdout".to_string()],
        }
    }
}

impl Default for SecurityConfiguration {
    fn default() -> Self {
        Self {
            enable_tls: true,
            tls_cert_path: None,
            tls_key_path: None,
            authentication: AuthenticationConfiguration::default(),
            authorization: AuthorizationConfiguration::default(),
            encryption: EncryptionConfiguration::default(),
        }
    }
}

impl Default for AuthenticationConfiguration {
    fn default() -> Self {
        Self {
            method: AuthenticationMethod::ApiKey,
            parameters: BTreeMap::new(),
            session_timeout: std::env::var(env_keys::ENV_PROVIDER_SESSION_TIMEOUT_SECS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(time::SECONDS_PER_HOUR), // 1 hour
            enable_mfa: false,
        }
    }
}

impl Default for AuthorizationConfiguration {
    fn default() -> Self {
        Self {
            method: AuthorizationMethod::RoleBased,
            rbac: RbacConfiguration::default(),
            abac: AbacConfiguration::default(),
        }
    }
}

impl Default for RbacConfiguration {
    fn default() -> Self {
        Self {
            roles: Vec::new(),
            assignments: BTreeMap::new(),
            default_role: Some("user".to_string()),
        }
    }
}

impl Default for AbacConfiguration {
    fn default() -> Self {
        Self {
            attributes: Vec::new(),
            policies: Vec::new(),
            default_decision: PolicyDecision::Deny,
        }
    }
}

impl Default for EncryptionConfiguration {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_management: KeyManagementConfiguration::default(),
            encrypt_at_rest: true,
            encrypt_in_transit: true,
        }
    }
}

impl Default for KeyManagementConfiguration {
    fn default() -> Self {
        Self {
            provider: KeyProvider::Local,
            rotation_interval: std::env::var(env_keys::ENV_PROVIDER_KEY_ROTATION_INTERVAL_SECS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(time::SECONDS_PER_DAY), // 24 hours
            derivation: KeyDerivationConfiguration::default(),
        }
    }
}

impl Default for KeyDerivationConfiguration {
    fn default() -> Self {
        Self {
            algorithm: KeyDerivationAlgorithm::Pbkdf2,
            iterations: std::env::var(env_keys::ENV_KEY_DERIVATION_ITERATIONS)
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(100000), // PBKDF2 recommended iterations
            salt_length: std::env::var(env_keys::ENV_KEY_DERIVATION_SALT_LENGTH)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
        }
    }
}

impl Default for PerformanceConfiguration {
    fn default() -> Self {
        Self {
            connection_pool: ConnectionPoolConfiguration::default(),
            caching: CachingConfiguration::default(),
            timeouts: TimeoutConfiguration::default(),
            retry: RetryConfiguration::default(),
        }
    }
}

impl Default for ConnectionPoolConfiguration {
    fn default() -> Self {
        Self {
            min_size: std::env::var(env_keys::ENV_CONNECTION_POOL_MIN_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
            max_size: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
            connection_timeout: std::env::var(env_keys::ENV_POOL_CONNECTION_TIMEOUT)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
            idle_timeout: std::env::var(env_keys::ENV_POOL_IDLE_TIMEOUT)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(300), // 5 minutes default
        }
    }
}

impl Default for CachingConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size: crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64,
            ttl: std::env::var(env_keys::ENV_CACHE_TTL_SECS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(time::SECONDS_PER_HOUR), // 1 hour default
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

// Implement CacheStrategy trait for provider caching configuration
impl CacheStrategy for CachingConfiguration {
    fn max_entries(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.max_size as usize
    }

    fn ttl(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.ttl)
    }

    fn eviction_policy(&self) -> crate::canonical::traits::cache::EvictionPolicy {
        use crate::canonical::traits::cache::EvictionPolicy as TraitPolicy;
        match self.eviction_policy {
            EvictionPolicy::Lru => TraitPolicy::Lru,
            EvictionPolicy::Lfu => TraitPolicy::Lfu,
            EvictionPolicy::Fifo => TraitPolicy::Fifo,
            EvictionPolicy::Random => TraitPolicy::Random,
            EvictionPolicy::Custom(_) => TraitPolicy::Lru, // Fallback to LRU
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.max_size == 0 {
            return Err("max_size must be > 0 when caching is enabled".to_string());
        }
        if self.ttl == 0 {
            return Err("TTL cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        if !self.enabled {
            return true;
        }
        self.max_size >= 100 && // At least 100 entries
        self.max_size <= 1_000_000 && // At most 1M entries
        self.ttl >= 60 && // At least 1 minute
        self.ttl <= time::SECONDS_PER_DAY && // At most 1 day
        self.validate().is_ok()
    }
}

impl Default for TimeoutConfiguration {
    fn default() -> Self {
        Self {
            request_timeout: std::env::var(env_keys::ENV_REQUEST_TIMEOUT_SECS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
            connection_timeout: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            read_timeout: std::env::var(env_keys::ENV_READ_TIMEOUT_SECS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
            write_timeout: std::env::var(env_keys::ENV_WRITE_TIMEOUT_SECS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
        }
    }
}

// Implement TimeoutPolicy trait for provider timeout configuration
impl TimeoutPolicy for TimeoutConfiguration {
    fn connection_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.connection_timeout)
    }

    fn operation_timeout(&self, operation: &str) -> std::time::Duration {
        let timeout_secs = match operation {
            "request" => self.request_timeout,
            "connect" | "connection" => self.connection_timeout,
            "read" => self.read_timeout,
            "write" => self.write_timeout,
            _ => self.request_timeout, // Default to request timeout
        };
        std::time::Duration::from_secs(timeout_secs)
    }

    fn should_timeout(&self, elapsed: std::time::Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }

    fn global_timeout(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(self.request_timeout))
    }

    fn read_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.read_timeout)
    }

    fn write_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.write_timeout)
    }

    fn idle_timeout(&self) -> Option<std::time::Duration> {
        None // Provider config doesn't have idle timeout
    }

    fn remaining_time(&self, elapsed: std::time::Duration, operation: &str) -> std::time::Duration {
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    fn validate(&self) -> Result<(), String> {
        if self.connection_timeout == 0 {
            return Err("Connection timeout cannot be zero".to_string());
        }
        if self.request_timeout == 0 {
            return Err("Request timeout cannot be zero".to_string());
        }
        if self.read_timeout == 0 {
            return Err("Read timeout cannot be zero".to_string());
        }
        if self.write_timeout == 0 {
            return Err("Write timeout cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        self.connection_timeout >= 1 &&
        self.connection_timeout <= 60 &&
        self.request_timeout >= 5 &&
        self.read_timeout >= 5 &&
        self.write_timeout >= 5 &&
        self.validate().is_ok()
    }
}

impl Default for RetryConfiguration {
    fn default() -> Self {
        Self {
            max_retries: std::env::var(env_keys::ENV_MAX_RETRIES)
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(3), // 3 retries default
            base_delay_ms: crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64,
            max_delay_ms: std::env::var(env_keys::ENV_MAX_RETRY_DELAY_MS)
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(10000), // 10 seconds default
            backoff_strategy: BackoffStrategy::Exponential,
        }
    }
}

impl Default for ConfigurationSchema {
    fn default() -> Self {
        Self {
            parameters: Vec::new(),
            required: Vec::new(),
            version: "1.0.0".to_string(),
        }
    }
} 