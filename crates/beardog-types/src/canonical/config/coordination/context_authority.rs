// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    missing_docs,
    reason = "coordination config types — serde-derived structs with self-documenting field names"
)]

//! Context evaluation and authority delegation types.

use super::supporting::{ContextChange, DelegationEvent};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Context evaluation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEvaluation {
    /// Factors to consider in context evaluation
    /// Collection of evaluation factors
    pub evaluation_factors: Vec<ContextFactor>,
    /// Mapping of factor weights
    pub factor_weights: HashMap<String, f64>,
    /// Current context assessment
    /// Optional current context
    pub current_context: Option<ContextAssessment>,
    /// History of context changes
    /// Collection of context history
    pub context_history: Vec<ContextChange>,
}

/// Factors that influence context evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextFactor {
    SystemLoad,
    OperationType,
    /// Urgency level of current tasks
    UrgencyLevel,
    /// Available expertise
    AvailableExpertise,
    /// External environment factors
    ExternalEnvironment,
    /// Risk level of operations
    RiskLevel,
}

/// Current context assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAssessment {
    /// Overall context score
    /// The context score value
    pub context_score: f64,
    /// Individual factor scores
    /// Mapping of factor scores
    pub factor_scores: HashMap<Arc<str>, f64>,
    /// Recommended coordination model
    /// The recommended model value
    pub recommended_model: Arc<str>,
    /// Confidence in recommendation
    pub confidence: f64,
    /// Assessment timestamp
    /// The assessed at value
    pub assessed_at: DateTime<Utc>,
}

/// Authority delegation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityDelegation {
    /// Mapping of delegation rules
    pub delegation_rules: HashMap<Arc<str>, DelegationRule>,
    /// Current active delegations
    /// Collection of active delegations
    pub active_delegations: Vec<Delegation>,
    /// History of delegations
    /// Collection of delegation history
    pub delegation_history: Vec<DelegationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRule {
    /// Type of authority being delegated
    /// The authority type value
    pub authority_type: Arc<str>,
    /// Required expertise level
    /// The required expertise value
    pub required_expertise: f64,
    /// Maximum delegation duration
    /// The max duration value
    pub max_duration: Duration,
    /// Collection of auto delegate conditions
    pub auto_delegate_conditions: Vec<String>,
}

/// Active delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    /// Unique delegation ID
    pub delegation_id: Arc<str>,
    /// Who delegated the authority
    /// The delegator value
    pub delegator: Arc<str>,
    /// Who received the authority
    /// The delegate value
    pub delegate: Arc<str>,
    /// Type of authority delegated
    /// The authority type value
    pub authority_type: Arc<str>,
    /// When delegation started
    /// The started at value
    pub started_at: DateTime<Utc>,
    /// When delegation expires
    /// The expires at value
    pub expires_at: DateTime<Utc>,
    /// Current status
    /// Current status of the component
    pub status: DelegationStatus,
}

/// Status of delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelegationStatus {
    /// Active or enabled state
    Active,
    /// State indicating suspended
    Suspended,
    /// State indicating expired
    Expired,
    /// State indicating revoked
    Revoked,
}
