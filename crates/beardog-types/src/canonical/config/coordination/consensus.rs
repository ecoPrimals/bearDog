// SPDX-License-Identifier: AGPL-3.0-only

//! Consensus mechanisms and decision protocols
//!
//! This module contains the decision-making and collaboration structures
//! that enable democratic coordination patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Decision protocol for collaborative coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionProtocol {
    /// Protocol identifier
    pub protocol_id: String,
    /// Protocol name
    pub protocol_name: String,
    /// Decision-making steps
    pub decision_steps: Vec<DecisionStep>,
    /// Advancement requirements between steps
    pub advancement_requirements: Vec<AdvancementRequirement>,
}

/// Individual step in decision protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionStep {
    /// Step identifier
    pub step_id: String,
    /// Step description
    pub description: String,
    /// Required participants for this step
    pub required_participants: Vec<String>,
    /// Optional participants for this step
    pub optional_participants: Vec<String>,
    /// Minimum participation rate required
    pub min_participation: f64,
}

/// Requirements to advance between decision steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvancementRequirement {
    /// Time-based advancement
    TimeElapsed { duration: Duration },
    /// Participation threshold met
    ParticipationThreshold { threshold: f64 },
    /// Consensus reached
    ConsensusReached { consensus_type: String },
    /// Expert approval received
    ExpertApproval { required_experts: Vec<String> },
    /// Custom condition met
    CustomCondition { condition_id: String, parameters: HashMap<String, String> },
}

/// Framework for collaborative interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationFramework {
    /// Framework identifier
    pub framework_id: String,
    /// Type of collaboration
    pub collaboration_type: String,
    /// Participant roles in collaboration
    pub roles: HashMap<String, Role>,
    /// Communication protocols
    pub communication_protocols: Vec<CommunicationProtocol>,
    /// Success metrics for collaboration
    pub success_metrics: Vec<SuccessMetric>,
}

/// Role definition within collaboration framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role identifier
    pub role_id: String,
    /// Role name
    pub role_name: String,
    /// Role responsibilities
    pub responsibilities: Vec<String>,
    /// Required capabilities for role
    pub required_capabilities: Vec<String>,
    /// Role permissions
    pub permissions: Vec<String>,
}

/// Communication protocol for collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationProtocol {
    /// Protocol identifier
    pub protocol_id: String,
    /// Communication method (sync/async/hybrid)
    pub communication_method: String,
    /// Response requirements
    pub response_requirements: ResponseRequirements,
    /// Escalation procedures
    pub escalation_procedures: Vec<String>,
}

/// Requirements for communication responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRequirements {
    /// Maximum response time
    pub max_response_time: Duration,
    /// Required response format
    pub response_format: String,
    /// Acknowledgment requirements
    pub acknowledgment_required: bool,
}

/// Metric for measuring collaboration success
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetric {
    /// Metric identifier
    pub metric_id: String,
    /// Metric name
    pub metric_name: String,
    /// Target value for success
    pub target_value: f64,
    /// Current metric value
    pub current_value: Option<f64>,
    /// Measurement frequency
    pub measurement_frequency: Duration,
}

/// Mutual accountability framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualAccountability {
    /// Accountability agreements
    pub accountability_agreements: Vec<AccountabilityAgreement>,
    /// Shared responsibility areas
    pub shared_responsibilities: HashMap<String, Vec<String>>,
    /// Accountability review schedule
    pub review_schedule: Duration,
}

/// Individual accountability agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountabilityAgreement {
    /// Agreement identifier
    pub agreement_id: String,
    /// Participants in agreement
    pub participants: Vec<String>,
    /// Mutual commitments
    pub commitments: Vec<String>,
    /// Success criteria
    pub success_criteria: Vec<String>,
    /// Review and adjustment procedures
    pub review_procedures: Vec<String>,
}

impl Default for DecisionProtocol {
    fn default() -> Self {
        Self {
            protocol_id: "default_collaborative".to_string(),
            protocol_name: "Default Collaborative Decision Protocol".to_string(),
            decision_steps: vec![
                DecisionStep {
                    step_id: "proposal".to_string(),
                    description: "Propose decision for collaborative review".to_string(),
                    required_participants: vec![],
                    optional_participants: vec![],
                    min_participation: std::env::var("BEARDOG_CONSENSUS_MIN_PARTICIPATION")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.5),
                }
            ],
            advancement_requirements: vec![
                AdvancementRequirement::ParticipationThreshold { threshold: 0.6 }
            ],
        }
    }
}

impl Default for MutualAccountability {
    fn default() -> Self {
        Self {
            accountability_agreements: vec![],
            shared_responsibilities: HashMap::new(),
            review_schedule: Duration::from_secs(
                std::env::var("BEARDOG_REVIEW_SCHEDULE_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(86400 * 7) // Weekly
            ),
        }
    }
} 