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


/// # Discovery Configuration - Canonical
///
/// **UNIFIED DISCOVERY CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Unified Discovery Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedDiscoveryConfig {
    pub service_discovery: ServiceDiscoveryConfig,
    pub node_discovery: NodeDiscoveryConfig,
    pub health_checks: DiscoveryHealthConfig,
    pub load_balancing: DiscoveryLoadBalancingConfig,
}


/// **CANONICAL** Service Discovery Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub discovery_method: String,
    pub refresh_interval: Duration,
    pub timeout: Duration,
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_method: "dns".to_string(),
            refresh_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
        }
    }
}

/// **CANONICAL** Node Discovery Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDiscoveryConfig {
    pub enabled: bool,
    pub discovery_port: u16,
    pub announcement_interval: Duration,
    pub node_timeout: Duration,
}

impl Default for NodeDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_port: 8080,
            announcement_interval: Duration::from_secs(60),
            node_timeout: Duration::from_secs(300),
        }
    }
}

/// **CANONICAL** Discovery Health Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryHealthConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
}

impl Default for DiscoveryHealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(10),
            healthy_threshold: 2,
            unhealthy_threshold: 3,
        }
    }
}

/// **CANONICAL** Discovery Load Balancing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryLoadBalancingConfig {
    pub enabled: bool,
    pub strategy: String,
    pub weight_adjustment: bool,
    pub sticky_sessions: bool,
}

impl Default for DiscoveryLoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "round_robin".to_string(),
            weight_adjustment: true,
            sticky_sessions: false,
        }
    }
}
