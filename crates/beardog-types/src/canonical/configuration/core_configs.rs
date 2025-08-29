/// Core Configuration Module
//!
//! Contains the fundamental configuration structs for BearDog core functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Core application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub log_level: LogLevel,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub features: HashMap<String, bool>,
    pub rollout_percentages: HashMap<String, f64>,
    pub dependencies: HashMap<String, Vec<String>>,
}

/// Network and connectivity configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub enable_tls: bool,
    pub circuit_breaker: crate::canonical::providers::CircuitBreakerConfig,
    pub load_balancing: LoadBalancingConfig,
    pub bandwidth_mbps: u32,
    pub port_ranges: Vec<PortRange>,
}

/// Security and authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    pub authentication: AuthConfig,
    pub authorization: AuthorizationConfig,
    pub encryption: EncryptionConfig,
    pub rate_limiting: RateLimitConfig,
    pub session: SessionConfig,
    pub mfa: MfaConfig,
    pub audit: AuditConfig,
    pub provider: SecurityProviderConfig,
}

/// HSM and cryptographic configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmConfig {
    pub provider_type: HsmProvider,
    pub connection_config: HsmConnectionConfig,
    pub key_management: KeyManagementConfig,
    pub performance: HsmPerformanceConfig,
    pub monitoring: HsmMonitoringConfig,
}

/// Database and storage configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub pool_size: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub migration: MigrationConfig,
    pub backup: BackupConfig,
    pub encryption: DatabaseEncryptionConfig,
}

// Supporting types and enums
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Environment {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum HsmProvider {
    #[default]
    Software,
    Hardware,
    AndroidStrongBox,
    CloudHsm,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    pub strategy: LoadBalancingStrategy,
    pub health_check_interval: Duration,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoadBalancingStrategy {
    #[default]
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    ResourceBased,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

// Supporting configuration types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub method: String,
    pub timeout: Duration,
    pub max_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    pub enabled: bool,
    pub policy_engine: String,
    pub cache_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub rotation_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    pub timeout: Duration,
    pub max_sessions: u32,
    pub secure_cookies: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MfaConfig {
    pub enabled: bool,
    pub methods: Vec<String>,
    pub backup_codes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditConfig {
    pub enabled: bool,
    pub log_level: LogLevel,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityProviderConfig {
    pub max_failed_attempts: u32,
    pub lockout_duration_minutes: u32,
    pub session_timeout_minutes: u32,
    pub enable_audit_logging: bool,
    pub require_mfa: bool,
}

// HSM-related configuration types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmConnectionConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyManagementConfig {
    pub rotation_interval: Duration,
    pub backup_enabled: bool,
    pub hardware_backed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmPerformanceConfig {
    pub max_concurrent_operations: usize,
    pub operation_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmMonitoringConfig {
    pub health_check_interval: Duration,
    pub metrics_enabled: bool,
}

// Database-related configuration types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationConfig {
    pub auto_migrate: bool,
    pub backup_before_migrate: bool,
    pub migration_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseEncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_rotation_interval: Duration,
}
