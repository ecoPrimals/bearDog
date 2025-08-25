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


/// # Network Types - Canonical
///
/// **UNIFIED NETWORK TYPES** for the BearDog ecosystem

use serde::{Deserialize, Serialize};

/// **CANONICAL** Endpoint Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub address: String,
    pub port: u16,
    pub timeout_ms: u32,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 8080,
            timeout_ms: 5000,
        }
    }
}

/// **CANONICAL** Network Protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    WebSocket,
}

impl Default for NetworkProtocol {
    fn default() -> Self {
        Self::Http
    }
}

/// **CANONICAL** Service Endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub protocol: NetworkProtocol,
    pub config: EndpointConfig,
    pub enabled: bool,
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self {
            protocol: NetworkProtocol::default(),
            config: EndpointConfig::default(),
            enabled: true,
        }
    }
}

