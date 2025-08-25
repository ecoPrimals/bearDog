// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # External Systems Configuration - Canonical
///
/// **UNIFIED EXTERNAL SYSTEMS CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** External System Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSystemConfig {
    pub name: String,
    pub enabled: bool,
    pub endpoint: String,
    pub timeout: Duration,
}

impl Default for ExternalSystemConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            endpoint: "http://localhost:8080".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// **CANONICAL** Protocol Support Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSupportConfig {
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub websocket_enabled: bool,
}

impl Default for ProtocolSupportConfig {
    fn default() -> Self {
        Self {
            http_enabled: true,
            https_enabled: true,
            websocket_enabled: false,
        }
    }
}

/// **CANONICAL** Message Queuing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueuingConfig {
    pub enabled: bool,
    pub queue_size: u32,
    pub batch_size: u32,
    pub flush_interval: Duration,
}

impl Default for MessageQueuingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            queue_size: 1000,
            batch_size: 100,
            flush_interval: Duration::from_secs(5),
        }
    }
}
