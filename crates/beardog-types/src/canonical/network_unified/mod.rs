// Unified Network Configuration

use serde::{Deserialize, Serialize};

/// Core module
/// Core functionality
/// Core functionality
pub mod core;
pub use core::*;

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

pub type NetworkConfig = CanonicalNetworkConfig;
