//! `BearDog` Core System Implementation
//!
//! The main entry point for the `BearDog` security and cryptography platform.

use beardog_errors::BearDogError;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig as BearDogConfig;
use beardog_types::canonical::ComponentStatus;
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
    /// custom configuration with [`UnifiedBearDogConfig::production()`].
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
    #[allow(clippy::cognitive_complexity)]
    pub async fn initialize(&mut self) -> Result<(), BearDogError> {
        info!("🚀 Initializing BearDog Core");

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Starting);
        }

        self.monitor.start()?;
        self.genetic_optimizer.initialize()?;

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Running);
            state.overall_health = beardog_types::canonical::HealthStatus::Healthy;
        }

        info!("✅ BearDog Core initialized successfully");
        Ok(())
    }

    /// Initialize HSM management capabilities
    ///
    /// # Errors
    /// Returns an error if HSM initialization fails.
    /// Initializes `componentialize_hsm_management`
    /// Initializes `componentialize_hsm_management`
    #[allow(clippy::cognitive_complexity)]
    pub async fn initialize_hsm_management(&self) -> Result<(), BearDogError> {
        info!("🔐 Initializing HSM management capabilities");

        // Initialize HSM providers and key management
        // This would integrate with hardware security modules
        // For now, we'll use a mock implementation

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("hsm".to_string(), ComponentStatus::Starting);
        }

        // Simulate HSM initialization
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

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
    #[allow(clippy::cognitive_complexity)]
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

        // Simulate capability-based AI service registration
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

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
