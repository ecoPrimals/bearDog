// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core types for canonical workflow examples

use crate::workflows::canonical_traits::{Workflow, WorkflowId, WorkflowStatus};
use serde::{Deserialize, Serialize};

/// Newtype wrapper for example workflow ids (string-backed, hashable, serde-friendly).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExampleWorkflowId(pub String);

impl std::fmt::Display for ExampleWorkflowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl WorkflowId for ExampleWorkflowId {
    /// Returns as str
    fn as_str(&self) -> &str {
        &self.0
    }
}

/// Example lifecycle states from creation through terminal success, failure, or cancel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExampleWorkflowStatus {
    /// State indicating created
    Created,
    /// State indicating started
    Started,
    /// Currently processing
    Processing,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed(String),
    /// State indicating cancelled
    Cancelled,
}

/// Reference workflow aggregate used in tests and samples: id, display name, status, optional JSON payload, timestamps.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExampleWorkflow {
    /// Stable instance identifier ([`ExampleWorkflowId`]).
    pub id: ExampleWorkflowId,
    /// Name of the item
    pub name: String,
    /// Current status of the component
    pub status: ExampleWorkflowStatus,
    /// Optional data
    pub data: Option<serde_json::Value>,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The updated at value
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl WorkflowStatus for ExampleWorkflowStatus {
    /// Checks if terminal
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed(_) | Self::Cancelled)
    }

    /// Checks if active
    fn is_active(&self) -> bool {
        matches!(self, Self::Started | Self::Processing)
    }
}

impl Workflow for ExampleWorkflow {
    type Id = ExampleWorkflowId;
    type Status = ExampleWorkflowStatus;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn status(&self) -> &Self::Status {
        &self.status
    }

    /// Creates `itemd_at`
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

impl ExampleWorkflow {
    /// New operation.
    /// Creates a new instance
    #[must_use]
    pub fn new(id: &str, name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: ExampleWorkflowId(id.to_string()),
            name: name.to_string(),
            status: ExampleWorkflowStatus::Created,
            data: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// With Data operation.
    /// Creates instance with data
    #[must_use]
    pub fn with_data(mut self, workflow_data: serde_json::Value) -> Self {
        self.data = Some(workflow_data);
        self
    }

    /// Set Status operation.
    /// Sets status
    #[must_use]
    pub fn set_status(mut self, status: ExampleWorkflowStatus) -> Self {
        self.status = status;
        self.updated_at = chrono::Utc::now();
        self
    }
}

/// Inputs passed into the example processor: acting principal, deadline hint, and retry budget.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessingContext {
    /// Logical user or service account driving this run (`"system"` when unspecified).
    pub user_id: String,
    /// Soft timeout hint for the processor or wrappers, in seconds.
    pub timeout_seconds: u64,
    /// Number of retry
    pub retry_count: u32,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        Self {
            user_id: "system".to_string(),
            timeout_seconds: 30,
            retry_count: 3,
        }
    }
}
