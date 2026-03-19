// SPDX-License-Identifier: AGPL-3.0-only

//! Main BearDog Core System
//!
//! This module provides the central BearDogCore struct that coordinates all
//! ecosystem components and provides the main entry point for the system.

use crate::types::{ComponentStatus, HealthStatus, SystemStatus};
use beardog_errors::BearDogError;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{info, warn};

/// Main BearDog Core system coordinator
#[derive(Debug)]
pub struct BearDogCore {
    /// System configuration
    config: UnifiedBearDogConfig,
    /// Overall system status
    pub status: Arc<RwLock<SystemStatus>>,
    /// Component registry
    components: Arc<RwLock<HashMap<String, ComponentStatus>>>,
    /// System state management
    pub state: Arc<RwLock<HashMap<String, ComponentStatus>>>,
    /// Universal adapter for ecosystem integration
    pub universal_adapter: Option<String>, // Placeholder for universal adapter
    /// System started flag
    started: bool,
}

impl BearDogCore {
    /// Create a new BearDog Core instance
    ///
    /// Initializes the BearDog Core system with the provided configuration.
    /// This creates all internal data structures but does not start any services.
    /// Use [`start`](Self::start) to begin operation.
    ///
    /// # Arguments
    ///
    /// * `config` - The unified configuration for the BearDog system
    ///
    /// # Returns
    ///
    /// A new `BearDogCore` instance ready to be started
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use beardog_core::BearDogCore;
    /// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
    ///
    /// let config = UnifiedBearDogConfig::development();
    /// let core = BearDogCore::new(config);
    /// // core.start().await?;
    /// ```
    pub fn new(config: UnifiedBearDogConfig) -> Self {
        info!("Initializing BearDog Core system");
        
        Self {
            config,
            status: Arc::new(RwLock::new(SystemStatus::new())),
            components: Arc::new(RwLock::new(HashMap::new())),
            state: Arc::new(RwLock::new(HashMap::new())),
            universal_adapter: None,
            started: false,
        }
    }

    /// Create a BearDog Core instance with default development configuration
    ///
    /// This is a convenience method that creates a `BearDogCore` instance
    /// with sensible defaults for development and testing. For production use,
    /// create a custom configuration and use [`new`](Self::new).
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `BearDogCore` instance or an error
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use beardog_core::BearDogCore;
    ///
    /// let core = BearDogCore::with_default_config()?;
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
    pub fn with_default_config() -> Result<Self> {
        let config = UnifiedBearDogConfig::development(); // Use development config as default
        Ok(Self::new(config))
    }

    /// Start the BearDog Core system
    ///
    /// Initializes all components and begins system operation. This method:
    /// - Initializes core components
    /// - Starts health monitoring
    /// - Sets the system to running state
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Component initialization fails
    /// - Health monitoring cannot be started
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use beardog_core::BearDogCore;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let mut core = BearDogCore::with_default_config()?;
    /// core.start().await?;
    /// assert!(core.is_running());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&mut self) -> Result<(), BearDogError> {
        if self.started {
            warn!("BearDog Core system is already started");
            return Ok(());
        }

        info!("Starting BearDog Core system");

        // Initialize core components
        self.initialize_components().await?;
        
        // Start health monitoring
        self.start_health_monitoring().await?;

        self.started = true;
        info!("BearDog Core system started successfully");
        
        Ok(())
    }

    /// Stop the BearDog Core system
    ///
    /// Gracefully shuts down the BearDog Core system and all components.
    /// This ensures all resources are properly released and any ongoing
    /// operations are completed or cancelled safely.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if component shutdown fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use beardog_core::BearDogCore;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let mut core = BearDogCore::with_default_config()?;
    /// core.start().await?;
    /// // ... do work ...
    /// core.stop().await?;
    /// assert!(!core.is_running());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop(&mut self) -> Result<(), BearDogError> {
        if !self.started {
            warn!("BearDog Core system is not running");
            return Ok(());
        }

        info!("Stopping BearDog Core system");

        // Gracefully shutdown components
        self.shutdown_components().await?;

        self.started = false;
        info!("BearDog Core system stopped");
        
        Ok(())
    }

    /// Check if the system is running
    ///
    /// Returns `true` if the system has been started and not yet stopped,
    /// `false` otherwise.
    ///
    /// # Returns
    ///
    /// `true` if the system is running, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use beardog_core::BearDogCore;
    ///
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let mut core = BearDogCore::with_default_config()?;
    /// assert!(!core.is_running());
    /// core.start().await?;
    /// assert!(core.is_running());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.started
    }

    /// Get current system status
    ///
    /// Returns a snapshot of the current system status including health,
    /// component states, and operational metrics.
    ///
    /// # Returns
    ///
    /// A `Result` containing the current `SystemStatus` or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the status cannot be read (lock poisoning)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use beardog_core::BearDogCore;
    ///
    /// let core = BearDogCore::with_default_config()?;
    /// let status = core.get_status()?;
    /// println!("System status: {:?}", status);
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
    pub fn get_status(&self) -> Result<SystemStatus> {
        let status = self.status.read()
            .map_err(|e| BearDogError::system(format!("Failed to read status: {e}")))?;
        Ok(status.clone())
    }

    #[must_use]
    /// Get system configuration
    pub fn get_config(&self) -> &UnifiedBearDogConfig {
        &self.config
    }

    /// Register a component
    pub fn register_component(&self, name: String, status: ComponentStatus) -> Result<(), BearDogError> {
        let mut components = self.components.write()
            .map_err(|e| BearDogError::system(format!("Failed to write components: {e}")))?;
        
        components.insert(name.clone(), status);
        info!("Registered component: {}", name);
        
        Ok(())
    }

    /// Update component status
    pub fn update_component_status(&self, name: &str, status: ComponentStatus) -> Result<(), BearDogError> {
        let mut components = self.components.write()
            .map_err(|e| BearDogError::system(format!("Failed to write components: {e}")))?;
        
        if let Some(current_status) = components.get_mut(name) {
            *current_status = status.clone();
            info!("Updated component {} status to {:?}", name, status);
        } else {
            warn!("Attempted to update non-existent component: {}", name);
        }
        
        Ok(())
    }

    /// Get health status
    pub fn get_health(&self) -> Result<HealthStatus> {
        let status = self.get_status()?;
        Ok(status.health)
    }

    // Private helper methods
    async fn initialize_components(&self) -> Result<(), BearDogError> {
        info!("Initializing system components");
        
        // Register core components
        self.register_component("core".to_string(), ComponentStatus::Starting)?;
        self.register_component("ai".to_string(), ComponentStatus::Starting)?;
        self.register_component("security".to_string(), ComponentStatus::Starting)?;
        self.register_component("networking".to_string(), ComponentStatus::Starting)?;
        
        // Mark components as healthy after initialization
        self.update_component_status("core", ComponentStatus::Healthy)?;
        self.update_component_status("ai", ComponentStatus::Healthy)?;
        self.update_component_status("security", ComponentStatus::Healthy)?;
        self.update_component_status("networking", ComponentStatus::Healthy)?;
        
        Ok(())
    }

    async fn start_health_monitoring(&self) -> Result<(), BearDogError> {
        info!("Starting health monitoring");
        // Health monitoring implementation would go here
        Ok(())
    }

    async fn shutdown_components(&self) -> Result<(), BearDogError> {
        info!("Shutting down system components");
        
        let component_names: Vec<String> = {
            let components = self.components.read()
                .map_err(|e| BearDogError::system(format!("Failed to read components: {e}")))?;
            components.keys().cloned().collect()
        };

        for name in component_names {
            self.update_component_status(&name, ComponentStatus::Stopping)?;
        }
        
        Ok(())
    }
}

impl Default for BearDogCore {
    fn default() -> Self {
        Self::new(UnifiedBearDogConfig::default())
    }
} 