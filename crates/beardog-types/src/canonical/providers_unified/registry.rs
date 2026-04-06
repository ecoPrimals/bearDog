// SPDX-License-Identifier: AGPL-3.0-or-later

// Provider Registry Configuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderRegistryEntry {
    /// Id
    pub id: String,
    /// Name
    /// Name of the item
    pub name: String,
    /// Status
    /// Current status of the component
    pub status: ProviderStatus,
    /// Health
    /// The health value
    pub health: ProviderHealth,
}

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderHealth {
    /// Healthy variant
    Healthy,
    /// Degraded variant
    Degraded,
    /// Unhealthy variant
    Unhealthy,
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self::Healthy
    }
}

/// Provider status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderStatus {
    /// Active variant
    Active,
    /// Inactive variant
    Inactive,
    /// Maintenance variant
    Maintenance,
}

impl Default for ProviderStatus {
    fn default() -> Self {
        Self::Active
    }
}
