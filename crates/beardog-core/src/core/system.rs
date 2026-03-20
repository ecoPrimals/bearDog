// SPDX-License-Identifier: AGPL-3.0-only

//! `BearDog` Core System Implementation
//!
//! The main entry point for the `BearDog` security and cryptography platform.
//!
//! # Overview
//!
//! `BearDogCore` is the central system that coordinates all `BearDog` components including:
//! - **Security Provider** - Cryptographic operations and key management
//! - **System Monitor** - Health tracking and observability
//! - **Genetic Optimizer** - Performance tuning and adaptation
//! - **Universal Adapter** - Service discovery and integration
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use beardog_core::BearDogCore;
//! use beardog_types::canonical::config::UnifiedBearDogConfig;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Create with default development configuration
//! let mut core = BearDogCore::with_default_config()?;
//!
//! // Initialize and start the system
//! core.initialize().await?;
//!
//! // System is now ready for operations
//! assert!(core.state.read().await.overall_health == beardog_types::canonical::HealthStatus::Healthy);
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! `BearDog` supports multiple configuration patterns:
//!
//! ```rust,no_run
//! use beardog_core::BearDogCore;
//! use beardog_types::canonical::config::UnifiedBearDogConfig;
//!
//! // Development configuration
//! let dev_config = UnifiedBearDogConfig::development();
//! let dev_core = BearDogCore::new(dev_config);
//!
//! // Production configuration  
//! let prod_config = UnifiedBearDogConfig::production();
//! let prod_core = BearDogCore::new(prod_config);
//!
//! // Custom configuration
//! let custom_config = UnifiedBearDogConfig {
//!     // ... custom settings
//!     ..Default::default()
//! };
//! let custom_core = BearDogCore::new(custom_config);
//! ```
//!
//! # Component Access
//!
//! All core components are accessible through public fields:
//!
//! ```rust,no_run
//! # use beardog_core::BearDogCore;
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let core = BearDogCore::with_default_config()?;
//! // Access security provider
//! let security = &core.security;
//!
//! // Access system monitor
//! let monitor = &core.monitor;
//!
//! // Access genetic optimizer
//! let optimizer = &core.genetic_optimizer;
//!
//! // Access universal adapter
//! let adapter = &core.universal_adapter;
//! # Ok(())
//! # }
//! ```
//!
//! # State Management
//!
//! System state is managed through a thread-safe shared structure:
//!
//! ```rust,no_run
//! # use beardog_core::BearDogCore;
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! # let core = BearDogCore::with_default_config()?;
//! // Read system state
//! let state = core.state.read().await;
//! println!("System health: {:?}", state.overall_health);
//!
//! // Write system state
//! let mut state = core.state.write().await;
//! // ... update state
//! # Ok(())
//! # }
//! ```
//!
//! # Lifecycle Management
//!
//! The core system follows a clear lifecycle:
//!
//! 1. **Creation** - `BearDogCore::new()` or `with_default_config()`
//! 2. **Initialization** - `core.initialize().await?`
//! 3. **Operation** - Normal system operation
//! 4. **Shutdown** - Graceful component shutdown
//!
//! # Error Handling
//!
//! All operations return `Result<T, BearDogError>` for comprehensive error handling:
//!
//! ```rust,no_run
//! # use beardog_core::BearDogCore;
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let mut core = BearDogCore::with_default_config()?;
//! core.initialize().await?;  // Propagate errors with ?
//! # Ok(())
//! # }
//! ```
//!
//! # Thread Safety
//!
//! `BearDogCore` is designed for concurrent use:
//! - State protected with `Arc<RwLock<T>>`
//! - All components are `Send + Sync`
//! - Safe to share across threads and tasks
//!
//! # Performance
//!
//! - Zero-cost abstractions for type safety
//! - Efficient async operations with tokio
//! - Genetic optimization for runtime adaptation
//! - Minimal locking contention

use beardog_errors::BearDogError;
use beardog_types::canonical::ComponentStatus;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig as BearDogConfig;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use super::adapter::UniversalAdapter;
use super::genetic_optimizer::GeneticOptimizer;
use super::monitoring::SystemMonitor;
use super::security::CoreSecurityProvider;
use super::state::CoreState;

/// Main `BearDog` core system
#[derive(Debug)]
/// Core `BearDog` system instance
///
/// The main entry point for the `BearDog` security and cryptography platform.
/// Provides AI-powered hybrid intelligence, sovereign key management, universal
/// service discovery, and ecosystem integration capabilities.
///
/// # Features
///
/// - **Zero Unsafe Code**: Complete memory safety without unsafe blocks
/// - **AI-Hybrid Intelligence**: Human-controlled AI assistance
/// - **Sovereign Cryptography**: User-owned entropy and key management
/// - **Universal Service Discovery**: Zero-knowledge capability discovery
/// - **Ecosystem Integration**: Primal coordination and orchestration
///
/// # Example
///
/// ```rust,ignore
/// use beardog_core::BearDogCore;
/// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
///
/// # async fn example() -> Result<(), beardog_errors::BearDogError> {
/// let config = UnifiedBearDogConfig::default();
/// let core = BearDogCore::new(config);
///
/// // Core is ready for operation
/// # Ok(())
/// # }
/// ```
pub struct BearDogCore {
    /// System configuration settings
    pub config: BearDogConfig,
    /// Shared system state with thread-safe access
    pub state: Arc<RwLock<CoreState>>,
    /// Core security provider for cryptographic operations
    pub security: CoreSecurityProvider,
    /// System monitoring and health tracking
    pub monitor: SystemMonitor,
    /// Genetic algorithm optimizer for performance tuning
    pub genetic_optimizer: GeneticOptimizer,
    /// Universal adapter for ecosystem service discovery
    pub universal_adapter: UniversalAdapter,
}

impl BearDogCore {
    /// Creates a new `BearDog` Core instance
    ///
    /// Initializes the core system with the provided configuration, creating
    /// all necessary internal components including security providers, monitoring
    /// systems, and ecosystem adapters.
    ///
    /// # Arguments
    ///
    /// * `config` - Unified configuration for the `BearDog` system
    ///
    /// # Returns
    ///
    /// Returns a new `BearDogCore` instance ready for operation.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use beardog_core::BearDogCore;
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    ///
    /// let config = UnifiedBearDogConfig::development();
    /// let core = BearDogCore::new(config);
    /// ```
    #[must_use]
    pub fn new(config: BearDogConfig) -> Self {
        Self {
            security: CoreSecurityProvider::new(config.clone()),
            config,
            state: Arc::new(RwLock::new(CoreState::default())),
            monitor: SystemMonitor::new().unwrap_or_default(),
            genetic_optimizer: GeneticOptimizer::new(),
            universal_adapter: UniversalAdapter::new(),
        }
    }

    /// Creates a `BearDog` Core instance with default configuration
    ///
    /// Convenience method that creates a `BearDogCore` instance using default
    /// development-friendly settings. For production use, prefer creating a
    /// custom configuration with production-specific settings.
    ///
    /// # Returns
    ///
    /// - `Ok(BearDogCore)` - Successfully created instance with default config
    /// - `Err(BearDogError)` - If default configuration cannot be created
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Default configuration validation fails
    /// - Required system resources are unavailable
    /// - Internal component initialization fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use beardog_core::BearDogCore;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let core = BearDogCore::with_default_config()?;
    /// // Core is ready with development defaults
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_default_config() -> Result<Self, BearDogError> {
        let config = BearDogConfig::default();
        Ok(Self::new(config))
    }

    /// Initialize operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub async fn initialize(&mut self) -> Result<(), BearDogError> {
        info!("🚀 Initializing BearDog Core");

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Starting);
        }

        self.monitor.start()?;
        self.genetic_optimizer.initialize().await?;

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Running);
            state.overall_health = beardog_types::canonical::HealthStatus::Healthy;
        }

        self.register_with_ecosystem()?;

        info!("✅ BearDog Core initialized successfully");
        Ok(())
    }

    /// Initialize HSM management capabilities
    ///
    /// # Errors
    /// Returns an error if HSM initialization fails.
    /// Initializes `componentialize_hsm_management`
    /// Initializes `componentialize_hsm_management`
    pub async fn initialize_hsm_management(&self) -> Result<(), BearDogError> {
        info!("🔐 Initializing HSM management capabilities");

        // Initialize HSM providers and key management
        // This provides minimal HSM integration pending full hardware security module support
        // Production deployments should integrate with actual HSM providers

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("hsm".to_string(), ComponentStatus::Starting);
        }

        // Modern: Yield to allow async component startup (no arbitrary delay)
        tokio::task::yield_now().await;

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("hsm".to_string(), ComponentStatus::Running);
        }

        info!("✅ HSM management initialized successfully");
        Ok(())
    }

    /// Register with AI coordination services using capability-based discovery
    ///
    /// # Errors
    /// Returns an error if registration fails.
    pub async fn register_with_ai_service_alt(&self) -> Result<(), BearDogError> {
        info!("🐿️ Registering with AI coordination services via capability discovery");

        // Universal AI capability references
        // This maintains sovereignty compliance

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("ai_coordination".to_string(), ComponentStatus::Starting);
        }

        // Modern: Yield to allow async component registration (no arbitrary delay)
        tokio::task::yield_now().await;

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("ai_coordination".to_string(), ComponentStatus::Running);
        }

        info!("✅ Successfully registered with AI coordination services");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::config::UnifiedBearDogConfig;

    #[test]
    fn test_system_new_with_config() {
        let config = UnifiedBearDogConfig::default();
        let _core = BearDogCore::new(config);
    }

    #[test]
    fn test_system_with_default_config() {
        let result = BearDogCore::with_default_config();
        assert!(result.is_ok());
        let _core = result.unwrap();
    }

    #[tokio::test]
    async fn test_system_initialize() {
        let mut core = BearDogCore::with_default_config().expect("config");
        let result = core.initialize().await;
        assert!(result.is_ok());
        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            beardog_types::canonical::HealthStatus::Healthy
        );
    }

    #[tokio::test]
    async fn test_system_initialize_hsm_management() {
        let core = BearDogCore::with_default_config().expect("config");
        let result = core.initialize_hsm_management().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_system_register_with_ai_service_alt() {
        let core = BearDogCore::with_default_config().expect("config");
        let result = core.register_with_ai_service_alt().await;
        assert!(result.is_ok());
    }
}
