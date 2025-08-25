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


/// # Canonical Health Types
///
/// **ELIMINATES FRAGMENTATION**: Unifies health status definitions across modules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// **CANONICAL HealthStatus** - Single source of truth
pub use crate::canonical::HealthStatus;
/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Current status
    pub status: HealthStatus,
    /// Optional status message
    pub message: Option<String>,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Health check duration in milliseconds
    pub check_duration_ms: u64,
}
/// System-wide health information
pub struct SystemHealth {
    /// Overall system status
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// `BearDog` version
    pub version: String,
    /// Individual component health
    pub components: Vec<ComponentHealth>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,}


impl Default for ComponentHealth {}


    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            status: HealthStatus::Unknown,
            message: None,
            last_check: Utc::now(),
            check_duration_ms: 0,
        }
    }
