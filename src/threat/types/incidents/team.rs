//! Incident response team management
//!
//! This module contains types and functionality for managing incident response
//! team members and their roles.

use serde::{Deserialize, Serialize};

/// Incident response team member
///
/// Represents a team member involved in incident response
/// with their role and contact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTeamMember {
    /// Team member identifier
    pub member_id: String,

    /// Team member name
    pub name: String,

    /// Role in incident response
    pub role: IncidentRole,

    /// Contact information
    pub contact_info: String,

    /// Availability status
    pub available: bool,

    /// Assigned incidents
    pub assigned_incidents: Vec<String>,
}

/// Incident response role enumeration
///
/// Defines different roles in incident response
/// with specific responsibilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentRole {
    /// Incident commander - overall coordination
    IncidentCommander,

    /// Lead analyst - technical investigation
    LeadAnalyst,

    /// Forensics analyst - evidence collection
    ForensicsAnalyst,

    /// Communications lead - stakeholder communication
    CommunicationsLead,

    /// Legal counsel - legal and compliance matters
    LegalCounsel,

    /// Management liaison - business coordination
    ManagementLiaison,

    /// External consultant - specialized expertise
    ExternalConsultant,
}

impl Default for IncidentTeamMember {
    fn default() -> Self {
        Self {
            member_id: String::new(),
            name: String::new(),
            role: IncidentRole::LeadAnalyst,
            contact_info: String::new(),
            available: true,
            assigned_incidents: vec![],
        }
    }
}

impl IncidentTeamMember {
    /// Create a new team member
    ///
    /// # Arguments
    /// * `member_id` - Unique member identifier
    /// * `name` - Member name
    /// * `role` - Member role
    /// * `contact_info` - Contact information
    ///
    /// # Returns
    /// A new `IncidentTeamMember` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentTeamMember, IncidentRole};
    ///
    /// let member = IncidentTeamMember::new(
    ///     "analyst-001".to_string(),
    ///     "John Doe".to_string(),
    ///     IncidentRole::LeadAnalyst,
    ///     "john.doe@company.com".to_string()
    /// );
    /// ```
    pub fn new(member_id: String, name: String, role: IncidentRole, contact_info: String) -> Self {
        Self {
            member_id,
            name,
            role,
            contact_info,
            available: true,
            assigned_incidents: vec![],
        }
    }

    /// Assign incident to member
    ///
    /// # Arguments
    /// * `incident_id` - Incident identifier to assign
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentTeamMember, IncidentRole};
    ///
    /// let mut member = IncidentTeamMember::new(
    ///     "analyst-001".to_string(),
    ///     "John Doe".to_string(),
    ///     IncidentRole::LeadAnalyst,
    ///     "john.doe@company.com".to_string()
    /// );
    ///
    /// member.assign_incident("INC-2024-001".to_string());
    /// assert_eq!(member.assigned_incidents.len(), 1);
    /// ```
    pub fn assign_incident(&mut self, incident_id: String) {
        if !self.assigned_incidents.contains(&incident_id) {
            self.assigned_incidents.push(incident_id);
        }
    }

    /// Remove incident assignment
    ///
    /// # Arguments
    /// * `incident_id` - Incident identifier to remove
    ///
    /// # Returns
    /// `true` if incident was found and removed
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentTeamMember, IncidentRole};
    ///
    /// let mut member = IncidentTeamMember::new(
    ///     "analyst-001".to_string(),
    ///     "John Doe".to_string(),
    ///     IncidentRole::LeadAnalyst,
    ///     "john.doe@company.com".to_string()
    /// );
    ///
    /// member.assign_incident("INC-2024-001".to_string());
    /// assert!(member.remove_incident("INC-2024-001"));
    /// assert_eq!(member.assigned_incidents.len(), 0);
    /// ```
    pub fn remove_incident(&mut self, incident_id: &str) -> bool {
        let original_len = self.assigned_incidents.len();
        self.assigned_incidents.retain(|id| id != incident_id);
        self.assigned_incidents.len() != original_len
    }

    /// Check if member is overloaded
    ///
    /// # Returns
    /// `true` if member has more than 3 assigned incidents
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentTeamMember, IncidentRole};
    ///
    /// let mut member = IncidentTeamMember::new(
    ///     "analyst-001".to_string(),
    ///     "John Doe".to_string(),
    ///     IncidentRole::LeadAnalyst,
    ///     "john.doe@company.com".to_string()
    /// );
    ///
    /// // Assign multiple incidents
    /// for i in 1..=5 {
    ///     member.assign_incident(format!("INC-2024-{:03}", i));
    /// }
    ///
    /// assert!(member.is_overloaded());
    /// ```
    pub fn is_overloaded(&self) -> bool {
        self.assigned_incidents.len() > 3
    }
}

impl std::fmt::Display for IncidentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncidentRole::IncidentCommander => write!(f, "Incident Commander"),
            IncidentRole::LeadAnalyst => write!(f, "Lead Analyst"),
            IncidentRole::ForensicsAnalyst => write!(f, "Forensics Analyst"),
            IncidentRole::CommunicationsLead => write!(f, "Communications Lead"),
            IncidentRole::LegalCounsel => write!(f, "Legal Counsel"),
            IncidentRole::ManagementLiaison => write!(f, "Management Liaison"),
            IncidentRole::ExternalConsultant => write!(f, "External Consultant"),
        }
    }
} 