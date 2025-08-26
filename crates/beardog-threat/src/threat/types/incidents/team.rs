

use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTeamMember {

    pub member_id: String,

    pub name: String,

    pub role: IncidentRole,

    pub contact_info: String,

    pub available: bool,

    pub assigned_incidents: Vec<String>,

    pub availability: MemberAvailability,

    pub last_active: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentRole {
    IncidentCommander,
    LeadAnalyst,
    TechnicalLead,
    ForensicsAnalyst,
    CommunicationsLead,
    LegalCounsel,
    ManagementLiaison,
    ExternalConsultant,
    SecurityAnalyst,
    ForensicsExpert,
    NetworkAnalyst,
    MalwareAnalyst,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemberAvailability {

    Available,

    Busy,

    Unavailable,

    OnCall,
}

impl Default for MemberAvailability {
    fn default() -> Self {
        MemberAvailability::Available
    }
}

impl Default for IncidentTeamMember {
    fn default() -> Self {
        Self {
            member_id: String::new(),
            name: String::new(),
            role: IncidentRole::LeadAnalyst,
            contact_info: String::new(),
            available: true,
            assigned_incidents: Vec::new(),
            availability: MemberAvailability::Available,
            last_active: chrono::Utc::now(),
        }
    }
}

impl IncidentTeamMember {

    pub fn new(member_id: &str, name: &str, role: IncidentRole, contact_info: &str) -> Self {
        Self {
            member_id: member_id.to_string(),
            name: name.to_string(),
            role,
            contact_info: contact_info.to_string(),
            available: true,
            assigned_incidents: Vec::new(),
            availability: MemberAvailability::Available,
            last_active: Utc::now(),
        }
    }

    pub fn with_availability(member_id: &str, name: &str, role: IncidentRole, contact_info: &str, available: bool) -> Self {
        Self {
            member_id: member_id.to_string(),
            name: name.to_string(),
            role,
            contact_info: contact_info.to_string(),
            available,
            assigned_incidents: Vec::new(),
            availability: if available { MemberAvailability::Available } else { MemberAvailability::Busy },
            last_active: Utc::now(),
        }
    }

    pub fn assign_to_incident(&mut self, incident_id: &str) {
        let incident_id_string = incident_id.to_string();
        if !self.assigned_incidents.contains(&incident_id_string) {
            self.assigned_incidents.push(incident_id_string);
        }
    }

    pub fn unassign_incident(&mut self, incident_id: &str) -> bool {
        if let Some(pos) = self.assigned_incidents.iter().position(|x| x == incident_id) {
            self.assigned_incidents.remove(pos);
            self.last_active = Utc::now();
            true
        } else {
            false
        }
    }

    pub fn is_overloaded(&self) -> bool {
        self.assigned_incidents.len() > 3
    }

    pub fn update_role(&mut self, new_role: &str) {

        self.last_active = Utc::now();
    }

    pub fn set_availability(&mut self, availability: MemberAvailability) {
        self.availability = availability;
        self.last_active = Utc::now();
    }

    pub fn incident_count(&self) -> usize {
        self.assigned_incidents.len()
    }
}

impl std::fmt::Display for IncidentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncidentRole::IncidentCommander => write!(f, "Incident Commander"),
            IncidentRole::LeadAnalyst => write!(f, "Lead Analyst"),
            IncidentRole::TechnicalLead => write!(f, "Technical Lead"),
            IncidentRole::ForensicsAnalyst => write!(f, "Forensics Analyst"),
            IncidentRole::CommunicationsLead => write!(f, "Communications Lead"),
            IncidentRole::LegalCounsel => write!(f, "Legal Counsel"),
            IncidentRole::ManagementLiaison => write!(f, "Management Liaison"),
            IncidentRole::ExternalConsultant => write!(f, "External Consultant"),
            IncidentRole::SecurityAnalyst => write!(f, "Security Analyst"),
            IncidentRole::ForensicsExpert => write!(f, "Forensics Expert"),
            IncidentRole::NetworkAnalyst => write!(f, "Network Analyst"),
            IncidentRole::MalwareAnalyst => write!(f, "Malware Analyst"),
        }
    }
}
