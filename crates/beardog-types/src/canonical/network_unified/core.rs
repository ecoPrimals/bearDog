// SPDX-License-Identifier: AGPL-3.0-only

// Core Network Types

use serde::{Deserialize, Serialize};

/// Core network configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkCoreConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Protocol
    /// The protocol value
    pub protocol: NetworkProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum NetworkProtocol {
    /// HTTP protocol
    #[default]
    /// Represents http variant
    Http,
    /// Https variant
    Https,
    /// Tcp variant
    Tcp,
    /// Udp variant
    Udp,
}
