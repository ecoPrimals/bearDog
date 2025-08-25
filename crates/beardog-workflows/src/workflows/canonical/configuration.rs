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


/// Workflow Configuration Types - Configuration and Policy Settings
///
/// This module contains configuration types for the workflow system,
/// including engine configuration, policy settings, and approval tiers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ✅ CONFIGURATION UNIFICATION: Use canonical WorkflowEngineConfig
// This duplicate definition has been eliminated - use beardog_types::canonical::configuration::workflows::WorkflowEngineConfig
pub use beardog_types::canonical::configuration::workflows::WorkflowEngineConfig;

/// Workflow policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPolicyConfig {
    /// Maximum concurrent workflows
    pub max_concurrent_workflows: u32,
    /// Default timeout duration
    pub default_timeout: Duration,
    /// Approval timeout duration
    pub approval_timeout: Duration,
    /// Auto cleanup enabled flag
    pub auto_cleanup_enabled: bool,
    /// Retention period
    pub retention_period: Duration,
}

impl Default for WorkflowPolicyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_workflows: 100,
            default_timeout: Duration::from_secs(3600), // 1 hour
            approval_timeout: Duration::from_secs(86400), // 24 hours
            auto_cleanup_enabled: true,
            retention_period: Duration::from_secs(2592000), // 30 days
        }
    }
}

impl WorkflowPolicyConfig {
    /// Determine approval requirements based on workflow type and priority
    pub fn determine_approval_requirements(
        &self,
        _workflow_type: &beardog_types::canonical::workflow::WorkflowType,
        priority: &beardog_types::canonical::workflow::WorkflowPriority,
    ) -> Result<Vec<String>, beardog_errors::BearDogError> {
        // Simple approval logic based on priority
        let approvers = match priority {
            beardog_types::canonical::workflow::WorkflowPriority::Low => vec!["team_lead".to_string()],
            beardog_types::canonical::workflow::WorkflowPriority::Normal => vec!["team_lead".to_string(), "manager".to_string()],
            beardog_types::canonical::workflow::WorkflowPriority::High => vec!["team_lead".to_string(), "manager".to_string(), "director".to_string()],
            beardog_types::canonical::workflow::WorkflowPriority::Critical => vec!["team_lead".to_string(), "manager".to_string(), "director".to_string(), "ceo".to_string()],
        };
        
        Ok(approvers)
    }
}

/// Approval tier levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalTier {
    Basic,
    Enhanced,
    Critical,
}

/// Approval decision enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalDecision {
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Submitted for review
    Submitted,
    /// Pending review
    Pending,
}

impl std::fmt::Display for ApprovalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Approved => write!(f, "Approved"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Pending => write!(f, "Pending"),
            Self::Submitted => write!(f, "Submitted"),
        }
    }
}

/// Approval requirements for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {
    /// Minimum number of approvals required
    pub minimum_approvals: u32,
    /// Approval timeout duration
    pub approval_timeout: Option<Duration>,
    /// Legacy timeout field for compatibility
    pub timeout: Option<Duration>,
    /// Whether unanimous approval is required
    pub requires_unanimous: bool,
    /// Required approval tiers
    pub required_tiers: Vec<ApprovalTier>,
    /// Additional requirements
    pub additional_requirements: HashMap<String, serde_json::Value>,
}

impl Default for ApprovalRequirements {
    fn default() -> Self {
        Self {
            minimum_approvals: 1,
            approval_timeout: Some(Duration::from_secs(86400)), // 24 hours
            timeout: Some(Duration::from_secs(86400)), // Legacy field
            requires_unanimous: false,
            required_tiers: vec![ApprovalTier::Basic],
            additional_requirements: HashMap::new(),
        }
    }
}

/// Approval response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    /// Response ID
    pub response_id: String,
    /// Approval ID
    pub approval_id: String,
    /// Decision made
    pub status: ApprovalDecision,
    /// Updated workflow status
    pub workflow_status: beardog_types::canonical::workflow::WorkflowStatus,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Approval submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSubmission {
    /// Submission ID
    pub submission_id: String,
    /// Approver ID
    pub approver_id: String,
    /// Approval decision
    pub decision: ApprovalDecision,
    /// Comments from approver
    pub comments: Option<String>,
    /// Reason for the decision
    pub reason: Option<String>,
    /// Digital signature
    pub signature: Option<String>,
    /// Submission timestamp
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

/// Pending approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    /// Approval ID
    pub approval_id: String,
    /// Workflow ID
    pub workflow_id: String,
    /// Required by timestamp
    pub required_by: chrono::DateTime<chrono::Utc>,
    /// Approval tier required
    pub tier: ApprovalTier,
    /// Description of what needs approval
    pub description: String,
} 