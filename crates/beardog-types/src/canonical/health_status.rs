

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {

    Healthy,

    Degraded,

    Unhealthy,

    Unknown,

    Starting,

    Stopping,

    Unavailable,

    Critical,

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

