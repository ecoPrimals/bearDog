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


/// # Network Configuration
///
/// **CANONICAL NETWORK CONFIGURATION TYPES**
/// This module contains all network-related configuration structures.
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL NETWORK CONFIGURATION** - Main network settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    /// Communication mesh configuration
    pub communication_mesh: CommunicationMeshConfig,
    /// Network security configuration
    pub security: NetworkSecurityConfig,
    /// Port configuration
    pub ports: PortConfig,
}
/// **CANONICAL COMMUNICATION MESH CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMeshConfig {
    /// Mesh enabled
    pub enabled: bool,
    /// Mesh nodes
    pub nodes: Vec<String>,
    /// Mesh discovery interval
    pub discovery_interval: Duration,}


impl Default for CommunicationMeshConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            nodes: vec!["localhost:8080".to_string(), "localhost:8081".to_string()],
            discovery_interval: Duration::from_secs(30),
        }
    }
}
/// **CANONICAL NETWORK SECURITY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSecurityConfig {
    /// TLS enabled
    pub tls_enabled: bool,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Key path
    pub key_path: Option<String>,
}

/// **CANONICAL PORT CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    /// HTTP port
    pub http: u16,
    /// HTTPS port
    pub https: u16,
    /// gRPC port
    pub grpc: u16,
}

impl Default for PortConfig {
    fn default() -> Self {
        Self {
            http: 8080,
            https: 8443,
            grpc: 9090,
        }
    }
}
