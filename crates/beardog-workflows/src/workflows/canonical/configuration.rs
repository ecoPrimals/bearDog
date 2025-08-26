

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub use beardog_types::canonical::configuration::workflows::WorkflowEngineConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPolicyConfig {

    pub max_concurrent_workflows: u32,

    pub default_timeout: Duration,

    pub approval_timeout: Duration,

    pub auto_cleanup_enabled: bool,

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

    pub fn determine_approval_requirements(
        &self,
        _workflow_type: &beardog_types::canonical::workflow::WorkflowType,
        priority: &beardog_types::canonical::workflow::WorkflowPriority,
    ) -> Result<Vec<String>, beardog_errors::BearDogError> {

        let approvers = match priority {
            beardog_types::canonical::workflow::WorkflowPriority::Low => vec!["team_lead".to_string()],
            beardog_types::canonical::workflow::WorkflowPriority::Normal => vec!["team_lead".to_string(), "manager".to_string()],
            beardog_types::canonical::workflow::WorkflowPriority::High => vec!["team_lead".to_string(), "manager".to_string(), "director".to_string()],
            beardog_types::canonical::workflow::WorkflowPriority::Critical => vec!["team_lead".to_string(), "manager".to_string(), "director".to_string(), "ceo".to_string()],
        };
        
        Ok(approvers)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalTier {
    Basic,
    Enhanced,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalDecision {

    Approved,

    Rejected,

    Submitted,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {

    pub minimum_approvals: u32,

    pub approval_timeout: Option<Duration>,

    pub requires_unanimous: bool,

    pub required_tiers: Vec<ApprovalTier>,

    pub additional_requirements: HashMap<String, serde_json::Value>,
}

impl Default for ApprovalRequirements {
    fn default() -> Self {
        Self {
            minimum_approvals: 1,
            approval_timeout: Some(Duration::from_secs(86400)), // 24 hours
            requires_unanimous: false,
            required_tiers: vec![ApprovalTier::Basic],
            additional_requirements: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {

    pub response_id: String,

    pub approval_id: String,

    pub status: ApprovalDecision,

    pub workflow_status: beardog_types::canonical::workflow::WorkflowStatus,

    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSubmission {

    pub submission_id: String,

    pub approver_id: String,

    pub decision: ApprovalDecision,

    pub comments: Option<String>,

    pub reason: Option<String>,

    pub signature: Option<String>,

    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {

    pub approval_id: String,

    pub workflow_id: String,

    pub required_by: chrono::DateTime<chrono::Utc>,

    pub tier: ApprovalTier,

    pub description: String,
} 