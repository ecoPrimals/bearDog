// SPDX-License-Identifier: AGPL-3.0-only
#![allow(missing_docs)]

//! Collaborative decision protocols, frameworks, and mutual accountability.

use super::supporting::{FeedbackSystem, MonitoringMechanism};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionProtocol {
    /// Steps in the decision making process
    /// Collection of decision steps
    pub decision_steps: Vec<DecisionStep>,
    pub step_timeouts: HashMap<String, Duration>,
    /// Mapping of advancement requirements
    pub advancement_requirements: HashMap<String, AdvancementRequirement>,
}

/// Step in collaborative decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionStep {
    /// Step identifier
    pub step_id: Arc<str>,
    /// Description of what happens in this step
    /// The description value
    pub description: Arc<str>,
    /// Required participants
    /// Collection of required participants
    pub required_participants: Vec<String>,
    /// Optional participants
    /// Collection of optional participants
    pub optional_participants: Vec<String>,
    /// Minimum participation level
    /// The min participation value
    pub min_participation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvancementRequirement {
    /// All required participants must agree
    UnanimousAgreement,
    /// Majority of participants must agree
    MajorityAgreement { threshold: f64 },
    /// Specific time must pass
    TimeElapsed { duration: Duration },
    /// Certain conditions must be met
    ConditionsMet { conditions: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationFramework {
    /// Framework identifier
    pub framework_id: Arc<str>,
    /// Type of collaboration this framework supports
    /// The collaboration type value
    pub collaboration_type: Arc<str>,
    /// Roles and responsibilities
    /// Mapping of roles
    pub roles: HashMap<Arc<str>, Role>,
    /// Communication protocols
    /// Collection of communication protocols
    pub communication_protocols: Vec<CommunicationProtocol>,
    /// Success metrics
    /// Collection of success metrics
    pub success_metrics: Vec<SuccessMetric>,
}

/// Role in collaboration framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role identifier
    pub role_id: Arc<str>,
    /// Role description
    /// The description value
    pub description: Arc<str>,
    /// Responsibilities
    /// Collection of responsibilities
    pub responsibilities: Vec<String>,
    /// Required expertise
    /// Collection of required expertise
    pub required_expertise: Vec<String>,
    /// Authority level
    /// The authority level value
    pub authority_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationProtocol {
    /// Protocol identifier
    pub protocol_id: Arc<str>,
    /// When this protocol is used
    /// The usage context value
    pub usage_context: Arc<str>,
    /// Communication channels
    /// Collection of channels
    pub channels: Vec<String>,
    pub message_formats: Vec<String>,
    /// Response requirements
    /// The response requirements value
    pub response_requirements: ResponseRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRequirements {
    /// Maximum response time
    pub max_response_time: Duration,
    /// Required acknowledgment
    /// Whether `acknowledgment_required` is enabled
    pub acknowledgment_required: bool,
    /// Escalation procedures
    /// Collection of escalation procedures
    pub escalation_procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetric {
    /// Metric identifier
    pub metric_id: Arc<str>,
    /// Description of what is measured
    /// The description value
    pub description: Arc<str>,
    /// Target value
    /// The target value value
    pub target_value: f64,
    /// Current value
    /// The current value value
    pub current_value: f64,
    /// Measurement frequency
    /// The measurement frequency value
    pub measurement_frequency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualAccountability {
    /// Accountability agreements between participants
    /// Collection of agreements
    pub agreements: Vec<AccountabilityAgreement>,
    /// Monitoring mechanisms
    /// Collection of monitoring mechanisms
    pub monitoring_mechanisms: Vec<MonitoringMechanism>,
    /// Feedback systems
    /// Collection of feedback systems
    pub feedback_systems: Vec<FeedbackSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountabilityAgreement {
    /// Agreement identifier
    pub agreement_id: Arc<str>,
    /// Participants in the agreement
    /// Collection of participants
    pub participants: Vec<String>,
    /// Commitments made by each participant
    /// Mapping of commitments
    pub commitments: HashMap<String, Vec<String>>,
    /// Collection of fulfillment metrics
    pub fulfillment_metrics: Vec<String>,
    /// Collection of consequences
    pub consequences: Vec<String>,
}
