// SPDX-License-Identifier: AGPL-3.0-or-later

//! Biome Discovery Configuration
//!
//! Biome sovereignty-specific discovery configuration that extends the canonical base.

use super::super::config::domains::discovery::DiscoveryConfig;
use serde::{Deserialize, Serialize};

/// Biome sovereignty discovery configuration
///
/// Wraps the canonical `DiscoveryConfig` with biome-specific partnership discovery flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDiscoveryConfig {
    /// Base discovery configuration
    pub base: DiscoveryConfig,

    /// Enable automatic discovery of potential partnership candidates
    ///
    /// When enabled, biomes will automatically discover and establish initial
    /// contact with potential partnership candidates in the ecosystem.
    pub auto_discovery_partners: bool,
}

impl Default for BiomeDiscoveryConfig {
    fn default() -> Self {
        Self {
            base: DiscoveryConfig::default(),
            auto_discovery_partners: true,
        }
    }
}

impl BiomeDiscoveryConfig {
    /// Create a configuration with manual partner discovery only
    pub fn manual_only() -> Self {
        Self {
            auto_discovery_partners: false,
            ..Default::default()
        }
    }
}
