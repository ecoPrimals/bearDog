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


/// # Adapter Integration Configuration - Canonical
///
/// **UNIFIED ADAPTER CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Adapter Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub name: String,
    pub enabled: bool,
    pub connection_timeout: Duration,
    pub retry_attempts: u32,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            connection_timeout: Duration::from_secs(30),
            retry_attempts: 3,
        }
    }
}

/// **CANONICAL** Adapter Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterPerformanceConfig {
    pub max_connections: u32,
    pub connection_pool_size: u32,
    pub request_timeout: Duration,
}

impl Default for AdapterPerformanceConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            connection_pool_size: 10,
            request_timeout: Duration::from_secs(60),
        }
    }
}

/// **CANONICAL** Adapter Connection Pooling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConnectionPooling {
    pub enabled: bool,
    pub min_connections: u32,
    pub max_connections: u32,
}

impl Default for AdapterConnectionPooling {
    fn default() -> Self {
        Self {
            enabled: true,
            min_connections: 1,
            max_connections: 10,
        }
    }
}

/// **CANONICAL** Adapter Authentication Method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdapterAuthMethod {
    None,
    Token,
    ApiKey,
}

impl Default for AdapterAuthMethod {
    fn default() -> Self {
        Self::None
    }
}

/// **CANONICAL** Adapter Security Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterSecurityPolicy {
    pub auth_method: AdapterAuthMethod,
    pub allowed_hosts: Vec<String>,
}

