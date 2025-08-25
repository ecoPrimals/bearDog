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


/// # Health and Status Types
///
/// **CANONICAL HEALTH & STATUS TYPES** - Single source of truth for all health status enums
// DateTime imports removed - not used in this module
use serde::{Deserialize, Serialize};

/// **CANONICAL** Health Status - Single source of truth for ALL health status enums
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// System is operating normally
    Healthy,
    /// System is functional but degraded
    Degraded,
    /// System is not functional
    Unhealthy,
    /// System status is unknown
    Unknown,
    /// System is starting up
    Starting,
    /// System is shutting down
    Stopping,
    /// System is unavailable
    Unavailable,
    /// System is in critical state
    Critical,
    /// System has warnings
    Warning,
}
impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "Healthy"),
            Self::Degraded => write!(f, "Degraded"),
            Self::Unhealthy => write!(f, "Unhealthy"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Starting => write!(f, "Starting"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Unavailable => write!(f, "Unavailable"),
            Self::Critical => write!(f, "Critical"),
            Self::Warning => write!(f, "Warning"),
        }
    }
}

/// **CANONICAL** Component Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentStatus {
    Active,
    Inactive,
    Failed,
    Maintenance,
}

impl Default for ComponentStatus {
    fn default() -> Self {
        Self::Inactive
    }
}

/// **CANONICAL** Operation Status  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationStatus {
    Pending,
    InProgress,
    Completed,
    Success,
    Cancelled,
}

impl Default for OperationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// **CANONICAL** Workflow Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkflowStatus {
    Created,
    Running,
    Paused,
    PendingApprovals,
    Approved,
    Rejected,
    Expired,
}

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Created
    }
}

/// **CANONICAL** Key Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeyStatus {
    Compromised,
    Revoked,
    PendingActivation,
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self::PendingActivation
    }
}

// ProviderHealthStatus moved to canonical/providers.rs to avoid duplication
