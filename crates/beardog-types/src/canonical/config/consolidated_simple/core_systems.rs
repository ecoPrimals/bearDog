//! # Core Systems Configuration - Modular Architecture
//!
//! This module contains the essential core system configurations organized
//! into focused domain modules for better maintainability.
//!
//! ## Modular Architecture
//!
//! The core systems are organized into focused modules:
//! - [`system`] - System configuration and resource management
//! - [`network`] - Network configuration and service discovery
//! - [`security`] - Security configuration and authentication
//! - [`database`] - Database configuration and connection management
//! - [`monitoring`] - Monitoring, metrics, and observability

pub mod system;
pub mod network;
pub mod security;
pub mod database;
pub mod monitoring;

// Re-export main configuration types for backward compatibility
pub use system::{SystemConfig, LoggingConfig, ResourceConfig, FilesystemConfig};
pub use network::{
    NetworkConfig, NetworkCoreConfig, NetworkSecurityConfig, 
    NetworkPerformanceConfig, ServiceDiscoveryConfig,
};
pub use security::{
    SecurityConfig, SecurityCoreConfig, AuthenticationConfig,
    AuthorizationConfig, SessionConfig, MfaConfig, EncryptionConfig, AuditConfig,
};
pub use database::{
    DatabaseConfig, DatabaseConnectionConfig, DatabasePoolingConfig,
    DatabasePerformanceConfig, DatabaseSecurityConfig, DatabaseBackupConfig,
};
pub use monitoring::{
    MonitoringConfig, MonitoringCoreConfig, MetricsConfig, HealthConfig,
    AlertingConfig, AlertRule, MonitoringLoggingConfig, TracingConfig,
    PerformanceMonitoringConfig, SystemMonitoringConfig, SecurityMonitoringConfig,
    PrometheusConfig,
}; 