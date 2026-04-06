// SPDX-License-Identifier: AGPL-3.0-or-later

// Unified Network Configuration

use serde::{Deserialize, Serialize};

/// Core module
/// Core functionality
pub mod core;
pub use core::*;

/// Canonical network listener settings (port, timeout, enabled).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalNetworkConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Port
    /// Number of port
    pub port: u16,
    /// Timeout
    pub timeout: u64,
}

/// Alias for [`CanonicalNetworkConfig`] for backward-compatible imports.
pub type NetworkConfig = CanonicalNetworkConfig;
