// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};

/// Health status of a system component or service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HealthStatus {
    /// Healthy variant
    Healthy,

    /// Degraded variant
    Degraded,

    /// Unhealthy variant
    Unhealthy,

    /// Unknown variant
    #[default]
    Unknown,

    /// Starting variant
    Starting,

    /// Stopping variant
    Stopping,

    /// Unavailable variant
    Unavailable,

    /// Critical variant
    Critical,

    /// Warning variant
    Warning,
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

/// Status of an individual system component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum ComponentStatus {
    /// Starting variant
    Starting,
    /// Running variant
    Running,
    /// Stopping variant
    Stopping,
    /// Active variant
    Active,
    /// Inactive variant
    #[default]
    Inactive,
    /// Failed variant
    Failed,
    /// Maintenance variant
    Maintenance,
    /// Error state with error message
    Error(String),
}

impl ComponentStatus {
    /// Check if component is in a healthy state
    #[must_use]
    pub const fn healthy(&self) -> bool {
        matches!(self, Self::Running | Self::Active)
    }
}

/// Status of a system operation or task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum OperationStatus {
    /// Pending variant
    #[default]
    Pending,
    /// `InProgress` variant
    InProgress,
    /// Completed variant
    Completed,
    /// Success variant
    Success,
    /// Cancelled variant
    Cancelled,
}

/// Status of a workflow or process chain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum WorkflowStatus {
    /// Created variant
    #[default]
    Created,
    /// Running variant
    Running,
    /// Paused variant
    Paused,
    /// `PendingApprovals` variant
    PendingApprovals,
    /// Approved variant
    Approved,
    /// Rejected variant
    Rejected,
    /// Expired variant
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Status of cryptographic keys in the system
#[derive(Default)]
pub enum KeyStatus {
    /// Compromised variant
    Compromised,
    /// Revoked variant
    Revoked,
    /// `PendingActivation` variant
    #[default]
    PendingActivation,
}
