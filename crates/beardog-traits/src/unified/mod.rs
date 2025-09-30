// Unified Trait System for BearDog
//
// This module provides a comprehensive, unified trait hierarchy that eliminates fragmentation
// and creates a coherent type system across the entire BearDog ecosystem.
//
// ## Architecture
//
// The unified trait system is organized into hierarchical families:
//
// ### Core Hierarchy
// ```
// BearDogCore
// ├── BearDogService
// ├── BearDogProvider
// └── BearDogComponent
// ```
//
// ### Specialized Traits
// - **Identity & Lifecycle**: Identifiable, Configurable, Versionable
// - **Security & Auth**: SecurityProvider, AuthProvider, CryptoProvider
// - **Genetics & Evolution**: GeneticsProvider, BiomeGenetics, EvolutionEngine
// - **Monitoring & Observability**: MonitoringProvider, MetricsCollector, HealthMonitored
// - **Storage & Persistence**: StorageProvider, CacheProvider, DatabaseProvider
// - **Communication & Network**: NetworkProvider, MessageHandler, EventPublisher

// async_trait no longer needed - using native fn
use beardog_errors::BearDogError;
use beardog_types::canonical::config::r#trait::BearDogConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
/// Core functionality
/// Core functionality
pub mod core;
pub mod genetics;
pub mod identity;
pub mod monitoring;
pub mod network;
pub mod providers;
pub mod security;
pub mod storage;
pub mod workflow; // NEW: Unified provider trait system

// Re-export unified traits
pub use core::*;
pub use genetics::*;
pub use security::*;

// **NEW**: Re-export unified provider traits (these replace all scattered provider traits)
pub use providers::{
    AdapterProvider, BearDogProvider, CryptoProvider, GeneticsProvider, HsmProvider,
    MonitoringProvider, SecurityProvider, WorkflowProvider,
};

// **MODERNIZED**: All legacy re-exports removed - use canonical providers system
// Use beardog_types::canonical::providers_unified::traits::* for all provider functionality

/// Core trait that all `BearDog` components must implement
///
/// This is the root of the trait hierarchy and provides fundamental functionality
/// that every component in the `BearDog` ecosystem requires.
pub trait BearDogCore: Send + Sync + Debug {
    type Error: Send + Sync + Into<BearDogError>;

    /// Get component identifier
    fn id(&self) -> &str;

    /// Get component version
    fn version(&self) -> &str;

    /// Get component name
    fn name(&self) -> &str {
        self.id()
    }

    /// Get component metadata
    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Initialize the component
    /// Initializes componentialize
    fn initialize(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Shutdown the component gracefully
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Get component health status
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<ComponentHealth, Self::Error>> + Send;
}

///
/// Services are stateful components that run continuously and provide ongoing functionality.
pub trait BearDogService: BearDogCore {
    /// Service-specific configuration
    type ServiceConfig: BearDogConfig;

    /// Start the service
    /// Starts service
    fn start(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Stop the service gracefully
    /// Stops service
    fn stop(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Restart the service
    fn restart(&mut self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            self.stop().await?;
            self.start().await?;
            Ok(())
        }
    }

    /// Get service status
    fn status(
        &self,
    ) -> impl std::future::Future<Output = Result<ServiceStatus, Self::Error>> + Send;

    /// Get service configuration
    /// Gets config
    fn get_config(&self) -> &Self::ServiceConfig;

    /// Update service configuration
    /// Updates config
    fn update_config(
        &mut self,
        config: Self::ServiceConfig,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Get service metrics
    /// Gets metrics
    fn get_metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ServiceMetrics, Self::Error>> + Send;

    /// Check if service is running
    /// Checks if running
    fn is_running(&self) -> bool;
}

///
/// Components are lightweight, stateless utilities that provide specific functionality.
pub trait BearDogComponent: BearDogCore {
    /// Component-specific parameters
    type Parameters: Send + Sync;

    /// Execute component with parameters
    /// Executes operation
    fn execute(
        &self,
        params: Self::Parameters,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Get component dependencies
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }

    /// Check if component is stateless
    /// Checks if stateless
    fn is_stateless(&self) -> bool {
        true
    }
}

/// Component health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentHealth {
    /// Whether `is_healthy` is enabled
    pub is_healthy: bool,
    /// Current status of the component
    pub status: String,
    /// The last check value
    pub last_check: std::time::SystemTime,
    /// Mapping of details
    pub details: HashMap<String, String>,
}

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is starting up
    Starting,
    /// Service is running normally
    Running,
    /// Service is stopping
    Stopping,
    /// Service is stopped
    Stopped,
    /// Service has encountered an error
    Error(String),
    /// Service is in maintenance mode
    Maintenance,
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub uptime_seconds: u64,
    /// Number of `requests_processed`
    pub requests_processed: u64,
    /// Number of `errors_enitemsered`
    pub errors_encountered: u64,
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Mapping of custom metrics
    pub custom_metrics: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum UnifiedTraitError {
    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    #[error("Operation not supported: {operation}")]
    NotSupported { operation: String },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Permission denied: {action} on {resource}")]
    PermissionDenied { action: String, resource: String },

    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl From<UnifiedTraitError> for BearDogError {
    fn from(error: UnifiedTraitError) -> Self {
        match error {
            UnifiedTraitError::Configuration { message } => BearDogError::System {
                message,
                category: beardog_errors::SystemErrorCategory::General,
            },
            UnifiedTraitError::Validation { field, message } => BearDogError::Business {
                message: format!("Validation failed for {field}: {message}"),
                category: beardog_errors::BusinessErrorCategory::Validation,
            },
            UnifiedTraitError::NotSupported { operation } => BearDogError::System {
                message: format!("Operation not supported: {operation}"),
                category: beardog_errors::SystemErrorCategory::General,
            },
            UnifiedTraitError::NotFound { resource } => BearDogError::System {
                message: format!("Resource not found: {resource}"),
                category: beardog_errors::SystemErrorCategory::General,
            },
            UnifiedTraitError::PermissionDenied { action, resource } => BearDogError::Security {
                message: format!("Permission denied: {action} on {resource}"),
                category: beardog_errors::SecurityErrorCategory::Authorization,
            },
            UnifiedTraitError::Internal { message } => BearDogError::System {
                message,
                category: beardog_errors::SystemErrorCategory::General,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestComponent {
        id: String,
    }

    impl BearDogCore for TestComponent {
        type Error = UnifiedTraitError;

        fn id(&self) -> &str {
            &self.id
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        /// Initializes componentialize
        fn initialize(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn shutdown(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn health_check(&self) -> Result<ComponentHealth, Self::Error> {
            Ok(ComponentHealth {
                is_healthy: true,
                status: "OK".to_string(),
                last_check: std::time::SystemTime::now(),
                details: HashMap::new(),
            })
        }
    }

    impl BearDogComponent for TestComponent {
        type Parameters = ();

        /// Executes operation
        fn execute(&self, _params: Self::Parameters) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    fn test_unified_traits() {
        let mut component = TestComponent {
            id: "test".to_string(),
        };

        assert_eq!(component.id(), "test");
        assert_eq!(component.version(), "1.0.0");
        assert!(component.initialize().is_ok());
        assert!(component.health_check().is_ok());
        assert!(component.execute(()).is_ok());
        assert!(component.shutdown().is_ok());
    }

    #[tokio::test]
    fn test_provider_status_serialization() {
        let status = ServiceStatus::Running;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: ServiceStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }
}
