use serde::{Deserialize, Serialize};

/// Health status of a system component or service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Healthy variant
    Healthy,

    /// Degraded variant
    Degraded,

    /// Unhealthy variant
    Unhealthy,

    /// Unknown variant
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

/// Status of an individual system component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    Inactive,
    /// Failed variant
    Failed,
    /// Maintenance variant
    Maintenance,
    /// Error state with error message
    Error(String),
}

impl Default for ComponentStatus {
    fn default() -> Self {
        Self::Inactive
    }
}

impl ComponentStatus {
    /// Check if component is in a healthy state
    #[must_use]
    pub const fn healthy(&self) -> bool {
        matches!(self, Self::Running | Self::Active)
    }
}

/// Status of a system operation or task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationStatus {
    /// Pending variant
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

impl Default for OperationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Status of a workflow or process chain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkflowStatus {
    /// Created variant
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

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Created
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Status of cryptographic keys in the system
pub enum KeyStatus {
    /// Compromised variant
    Compromised,
    /// Revoked variant
    Revoked,
    /// `PendingActivation` variant
    PendingActivation,
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self::PendingActivation
    }
}
