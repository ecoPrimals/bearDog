// SPDX-License-Identifier: AGPL-3.0-only

// Authorization types for BearDog Security
//
// This module provides authorization-related types for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Subject of an authorization request
#[derive(Debug, Clone)]
pub struct Subject {
    /// Unique identifier
    pub id: String,
    /// Name of the subject
    pub name: String,
    /// The subject type value
    pub subject_type: SubjectType,
    /// Collection of roles
    pub roles: Vec<String>,
    /// Optional clearance level
    pub clearance_level: Option<u32>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Type of subject
#[derive(Debug, Clone)]
pub enum SubjectType {
    /// Represents user variant
    User,
    /// Represents service variant
    Service,
    /// Represents node variant
    Node,
    /// Represents system variant
    System,
}

/// Authorization result
#[derive(Debug, Clone)]
pub struct AuthorizationResult {
    /// Whether authorized is enabled
    pub authorized: bool,
    /// The reason value
    pub reason: String,
    /// The risk level value
    pub risk_level: RiskLevel,
    /// Collection of additional requirements
    pub additional_requirements: Vec<String>,
    /// Optional expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// Audit ID for tracking
    pub audit_id: String,
}

/// Action being performed
#[derive(Debug, Clone)]
pub struct Action {
    /// Name of the action
    pub name: String,
    /// The action type value
    pub action_type: ActionType,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Types of action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    /// Represents read variant
    Read,
    /// Represents write variant
    Write,
    /// Represents execute variant
    Execute,
    /// Represents delete variant
    Delete,
    /// Represents admin variant
    Admin,
    /// Represents approve variant
    Approve,
    /// Represents create variant
    Create,
    /// Represents update variant
    Update,
}

/// Resource being accessed
#[derive(Debug, Clone)]
pub struct Resource {
    /// Name of the resource
    pub name: String,
    /// The classification value
    pub classification: ResourceClassification,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Resource classification level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceClassification {
    /// Public resource
    Public,
    /// Internal resource
    Internal,
    /// Confidential resource
    Confidential,
    /// Secret resource
    Secret,
    /// Top secret resource
    TopSecret,
}

/// Risk level for authorization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

impl Default for Subject {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            subject_type: SubjectType::User,
            roles: Vec::new(),
            clearance_level: None,
            metadata: HashMap::new(),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: String::new(),
            action_type: ActionType::Read,
            metadata: HashMap::new(),
        }
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            name: String::new(),
            classification: ResourceClassification::Internal,
            metadata: HashMap::new(),
        }
    }
}
