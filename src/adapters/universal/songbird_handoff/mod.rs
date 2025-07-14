//! Universal SongBird Handoff Module
//!
//! **Universal ecosystem integration with SongBird discovery and orchestration**
//!
//! This module provides a comprehensive system for integrating any ecosystem component
//! with SongBird's discovery and orchestration platform. It follows universal patterns
//! that work with any PrimalProvider implementation, making it truly ecosystem-agnostic.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                Universal Ecosystem Component                │
//! │  ┌─────────────────┐  ┌──────────────────────────────────┐  │
//! │  │   Capability    │  │     Universal Capability         │  │
//! │  │ Advertisement   │→ │       Management System          │  │
//! │  │    System       │  │                                  │  │
//! │  └─────────────────┘  └──────────────────────────────────┘  │
//! └─────────────────────────┬───────────────────────────────────┘
//!                           │ Universal PrimalProvider Interface
//!                           ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                 SongBird Orchestrator                       │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │   Service   │  │   Request   │  │    Load Balancer    │  │
//! │  │ Discovery   │  │   Routing   │  │   & Orchestration   │  │
//! │  └─────────────┘  └─────────────┘  └─────────────────────┘  │
//! └─────────────────────────────────────────────────────────────┘
//!                           │
//!                           ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │            Universal Ecosystem Service Mesh                │
//! │    ToadStool  │  NestGate  │  Squirrel  │  biomeOS        │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Features
//!
//! - **Universal Integration**: Works with any ecosystem component
//! - **Automatic Discovery**: Registers capabilities with SongBird
//! - **Load Balancing**: SongBird handles routing and load balancing
//! - **Health Monitoring**: Continuous health reporting to SongBird
//! - **Fault Tolerance**: Circuit breakers and retry mechanisms
//! - **Performance Metrics**: Comprehensive performance tracking
//! - **Orchestration**: Advanced routing, scaling, and affinity rules
//!
//! ## Usage
//!
//! ```rust
//! use beardog::adapters::universal::songbird_handoff::*;
//!
//! // Create universal handoff manager
//! let manager = UniversalSongBirdHandoffManager::new(
//!     core,
//!     capability_manager,
//!     config,
//! ).await?;
//!
//! // Register with SongBird
//! manager.register_with_songbird().await?;
//!
//! // Start monitoring
//! manager.start_monitoring().await?;
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use super::capability_manager::CapabilityManager;
use super::traits::PrimalProvider;
use crate::{BearDogCore, BearDogError, BearDogResult};

// Module components
pub mod types;
pub mod registration;
pub mod client;
pub mod health;

// Re-export public types
pub use types::{
    SongBirdHandoffConfig, RegistrationStatus, RegistrationState, AdvertisedService,
    ServiceEndpoint, LoadBalancerConfig, OrchestrationMetadata, ServiceRegistrationResult,
    ServiceHealth,
};
pub use registration::SongBirdRegistrationManager;
pub use client::SongBirdDiscoveryClient;
pub use health::{UniversalHealthMonitor, HealthMonitorConfig, PerformanceMetrics, HealthSummary};

/// Universal SongBird Handoff Manager
///
/// **Main orchestration manager for universal SongBird integration**
///
/// This manager coordinates all aspects of integrating any ecosystem component
/// with SongBird's discovery and orchestration platform. It provides a unified
/// interface for registration, health monitoring, and capability advertisement.
pub struct UniversalSongBirdHandoffManager {
    /// Universal ecosystem component core
    core: Arc<BearDogCore>,

    /// Capability manager for dynamic capability advertisement
    capability_manager: Arc<CapabilityManager>,

    /// Registration manager for SongBird integration
    registration_manager: Arc<SongBirdRegistrationManager>,

    /// Health monitor for continuous health reporting
    health_monitor: Arc<UniversalHealthMonitor>,

    /// Manager configuration
    config: SongBirdHandoffConfig,

    /// Manager status
    status: Arc<RwLock<HandoffManagerStatus>>,
}

/// Universal handoff manager status
#[derive(Debug, Clone)]
pub struct HandoffManagerStatus {
    /// Manager initialization status
    pub initialized: bool,

    /// SongBird registration status
    pub registered: bool,

    /// Health monitoring status
    pub monitoring_active: bool,

    /// Last status update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,

    /// Error count
    pub error_count: u64,

    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for HandoffManagerStatus {
    fn default() -> Self {
        Self {
            initialized: false,
            registered: false,
            monitoring_active: false,
            last_updated: chrono::Utc::now(),
            error_count: 0,
            uptime_seconds: 0,
        }
    }
}

impl UniversalSongBirdHandoffManager {
    /// Create a new universal SongBird handoff manager
    pub async fn new(
        core: Arc<BearDogCore>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> BearDogResult<Self> {
        info!("🌐 Initializing Universal SongBird Handoff Manager");

        // Create registration manager
        let registration_manager = Arc::new(
            SongBirdRegistrationManager::new(
                core.clone(),
                capability_manager.clone(),
                config.clone(),
            )
            .await?,
        );

        // Create SongBird client
        let client = Arc::new(
            SongBirdDiscoveryClient::new(
                config.songbird_endpoint.clone(),
                config.api_key.clone(),
            )
            .await?,
        );

        // Create health monitor
        let health_config = HealthMonitorConfig {
            check_interval_seconds: config.health_check_interval_seconds,
            check_timeout_seconds: 10,
            max_consecutive_failures: 3,
            history_retention_count: 100,
            enable_performance_metrics: true,
        };

        let health_monitor = Arc::new(
            UniversalHealthMonitor::new(client, health_config).await?,
        );

        // Initialize status
        let status = Arc::new(RwLock::new(HandoffManagerStatus::default()));

        let manager = Self {
            core,
            capability_manager,
            registration_manager,
            health_monitor,
            config,
            status,
        };

        // Mark as initialized
        {
            let mut status = manager.status.write().await;
            status.initialized = true;
            status.last_updated = chrono::Utc::now();
        }

        info!("✅ Universal SongBird Handoff Manager initialized");

        Ok(manager)
    }

    /// Register with SongBird discovery and orchestration
    pub async fn register_with_songbird(&self) -> BearDogResult<()> {
        info!("🔗 Registering with SongBird using universal patterns");

        // Register through registration manager
        self.registration_manager.register_with_songbird().await?;

        // Update status
        {
            let mut status = self.status.write().await;
            status.registered = true;
            status.last_updated = chrono::Utc::now();
        }

        info!("✅ Successfully registered with SongBird");

        Ok(())
    }

    /// Start health monitoring
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        info!("🏥 Starting universal health monitoring");

        // Start health monitoring
        self.health_monitor.start_monitoring().await?;

        // Update status
        {
            let mut status = self.status.write().await;
            status.monitoring_active = true;
            status.last_updated = chrono::Utc::now();
        }

        info!("✅ Health monitoring started");

        Ok(())
    }

    /// Perform health check
    pub async fn perform_health_check(&self) -> BearDogResult<health::HealthCheckResult> {
        debug!("🔍 Performing universal health check");

        self.health_monitor.perform_health_check().await
    }

    /// Update capability advertisement
    pub async fn update_capability_advertisement(&self) -> BearDogResult<()> {
        info!("🔄 Updating universal capability advertisement");

        // Update through registration manager
        self.registration_manager.update_capability_advertisement().await?;

        // Update status
        {
            let mut status = self.status.write().await;
            status.last_updated = chrono::Utc::now();
        }

        Ok(())
    }

    /// Get registration status
    pub async fn get_registration_status(&self) -> types::RegistrationStatus {
        self.registration_manager.get_registration_status().await
    }

    /// Get advertised services
    pub async fn get_advertised_services(&self) -> HashMap<String, AdvertisedService> {
        self.registration_manager.get_advertised_services().await
    }

    /// Get health status
    pub async fn get_health_status(&self) -> ServiceHealth {
        self.health_monitor.get_health_status().await
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.health_monitor.get_performance_metrics().await
    }

    /// Get health summary
    pub async fn get_health_summary(&self) -> HealthSummary {
        self.health_monitor.get_health_summary().await
    }

    /// Get manager status
    pub async fn get_manager_status(&self) -> HandoffManagerStatus {
        self.status.read().await.clone()
    }

    /// Update performance metrics
    pub async fn update_performance_metrics(
        &self,
        requests_processed: u64,
        errors_encountered: u64,
        response_time_ms: u64,
    ) -> BearDogResult<()> {
        self.health_monitor
            .update_performance_metrics(requests_processed, errors_encountered, response_time_ms)
            .await
    }

    /// Shutdown the handoff manager
    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("🛑 Shutting down Universal SongBird Handoff Manager");

        // TODO: Implement graceful shutdown
        // This would:
        // - Stop health monitoring
        // - Unregister from SongBird
        // - Clean up resources
        // - Update status

        // Update status
        {
            let mut status = self.status.write().await;
            status.initialized = false;
            status.registered = false;
            status.monitoring_active = false;
            status.last_updated = chrono::Utc::now();
        }

        info!("✅ Universal SongBird Handoff Manager shut down");

        Ok(())
    }
}

/// Universal handoff manager builder
///
/// Provides a fluent API for creating and configuring the handoff manager.
pub struct UniversalSongBirdHandoffManagerBuilder {
    /// Core component
    core: Option<Arc<BearDogCore>>,

    /// Capability manager
    capability_manager: Option<Arc<CapabilityManager>>,

    /// Configuration
    config: SongBirdHandoffConfig,
}

impl UniversalSongBirdHandoffManagerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            core: None,
            capability_manager: None,
            config: SongBirdHandoffConfig::default(),
        }
    }

    /// Set the core component
    pub fn with_core(mut self, core: Arc<BearDogCore>) -> Self {
        self.core = Some(core);
        self
    }

    /// Set the capability manager
    pub fn with_capability_manager(mut self, capability_manager: Arc<CapabilityManager>) -> Self {
        self.capability_manager = Some(capability_manager);
        self
    }

    /// Set the configuration
    pub fn with_config(mut self, config: SongBirdHandoffConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the SongBird endpoint
    pub fn with_songbird_endpoint(mut self, endpoint: String) -> Self {
        self.config.songbird_endpoint = endpoint;
        self
    }

    /// Set the API key
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.config.api_key = api_key;
        self
    }

    /// Build the handoff manager
    pub async fn build(self) -> BearDogResult<UniversalSongBirdHandoffManager> {
        let core = self.core.ok_or_else(|| BearDogError::internal("Core component is required"))?;
        let capability_manager = self.capability_manager.ok_or_else(|| BearDogError::internal("Capability manager is required"))?;

        UniversalSongBirdHandoffManager::new(core, capability_manager, self.config).await
    }
}

impl Default for UniversalSongBirdHandoffManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal handoff manager factory
///
/// Provides convenience methods for creating handoff managers.
pub struct UniversalSongBirdHandoffManagerFactory;

impl UniversalSongBirdHandoffManagerFactory {
    /// Create a default handoff manager
    pub async fn create_default(
        core: Arc<BearDogCore>,
        capability_manager: Arc<CapabilityManager>,
    ) -> BearDogResult<UniversalSongBirdHandoffManager> {
        UniversalSongBirdHandoffManager::new(
            core,
            capability_manager,
            SongBirdHandoffConfig::default(),
        )
        .await
    }

    /// Create a handoff manager with custom configuration
    pub async fn create_with_config(
        core: Arc<BearDogCore>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> BearDogResult<UniversalSongBirdHandoffManager> {
        UniversalSongBirdHandoffManager::new(core, capability_manager, config).await
    }

    /// Create a handoff manager using builder pattern
    pub fn builder() -> UniversalSongBirdHandoffManagerBuilder {
        UniversalSongBirdHandoffManagerBuilder::new()
    }
}

// Convenience re-exports for common usage patterns
pub use registration::SongBirdRegistrationManager as RegistrationManager;
pub use client::SongBirdDiscoveryClient as DiscoveryClient;
pub use health::UniversalHealthMonitor as HealthMonitor;

/// Universal handoff manager alias for convenience
pub type HandoffManager = UniversalSongBirdHandoffManager;

/// Universal handoff manager builder alias for convenience
pub type HandoffManagerBuilder = UniversalSongBirdHandoffManagerBuilder;

/// Universal handoff manager factory alias for convenience
pub type HandoffManagerFactory = UniversalSongBirdHandoffManagerFactory; 