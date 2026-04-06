// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concurrent-Safe Configuration Loader
//!
//! Provides hierarchical configuration loading with explicit environment variable handling.
//!
//! ## Design Pattern
//!
//! The loader now uses the **`ConfigHierarchy`** for proper layered configuration:
//! 1. Fallback defaults (static)
//! 2. Platform-specific defaults
//! 3. Configuration file (auto-discovered or specified)
//! 4. Environment variables
//! 5. CLI arguments
//!
//! See `hierarchy` module for implementation details.

use crate::BearDogConfig;
use crate::error::ConfigResult;
use crate::hierarchy::ConfigHierarchy;
use std::collections::HashMap;
use std::path::Path;

/// Configuration loader builder
///
/// This is a convenience wrapper around `ConfigHierarchy` for backward compatibility.
/// New code should use `ConfigHierarchy` directly for better control.
pub struct ConfigLoader {
    hierarchy: ConfigHierarchy,
}

impl ConfigLoader {
    /// Create a new configuration loader with static defaults
    pub fn new() -> Self {
        Self {
            hierarchy: ConfigHierarchy::new(),
        }
    }

    /// Apply defaults (already applied in `new()`)
    pub const fn with_defaults(self) -> Self {
        self
    }

    /// Apply platform detection
    pub fn with_platform_defaults(mut self) -> Self {
        self.hierarchy = self.hierarchy.with_platform_defaults();
        self
    }

    /// Load from config file (auto-discover)
    ///
    /// # Errors
    ///
    /// Propagates errors from [`ConfigHierarchy::with_auto_config_file`].
    pub fn with_config_file(mut self) -> ConfigResult<Self> {
        self.hierarchy = self.hierarchy.with_auto_config_file()?;
        Ok(self)
    }

    /// Load from specific file
    ///
    /// # Errors
    ///
    /// Propagates errors from [`ConfigHierarchy::with_file`].
    pub fn with_file<P: AsRef<Path>>(mut self, path: P) -> ConfigResult<Self> {
        self.hierarchy = self.hierarchy.with_file(path)?;
        Ok(self)
    }

    /// Apply environment variables (EXPLICIT loading)
    ///
    /// This method explicitly loads configuration from environment variables.
    pub fn with_env_vars(mut self) -> Self {
        self.hierarchy = self.hierarchy.with_env_vars();
        self
    }

    /// Add CLI arguments
    pub fn with_cli_args(mut self, args: HashMap<String, String>) -> Self {
        self.hierarchy = self.hierarchy.with_cli_args(args);
        self
    }

    /// Build final configuration
    ///
    /// # Errors
    ///
    /// Propagates errors from [`ConfigHierarchy::build`].
    pub fn build(self) -> ConfigResult<BearDogConfig> {
        self.hierarchy.build()
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "loader_tests.rs"]
mod loader_tests;
