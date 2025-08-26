

pub mod metrics;
pub mod response;
pub mod team;
pub mod timeline;

pub use metrics::{IncidentMetrics, IncidentRole, IncidentTeamMember};
pub use response::{IncidentResponse, IncidentStatus}; // Use response::IncidentStatus as the primary one
pub use team::*;
pub use timeline::*;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::threat::types::core::ThreatSeverity;
    #[test]
    fn test_incident_response_creation() {
        let incident = IncidentResponse::new(
            "INC-2024-001".to_string(),
            "threat-001".to_string(),
            ThreatSeverity::High,
            "Test incident".to_string(),
        );
        assert_eq!(incident.incident_id, "INC-2024-001");
        assert_eq!(incident.threat_id, "threat-001");
        assert_eq!(incident.severity, ThreatSeverity::High);
        assert_eq!(incident.status, IncidentStatus::Open);
        assert_eq!(incident.description, "Test incident");
    }
    fn test_incident_status_transitions() {
        let status = IncidentStatus::Open;
        let valid_next = status.valid_next_statuses();
        assert!(valid_next.contains(&IncidentStatus::InProgress));
        assert!(valid_next.contains(&IncidentStatus::Escalated));
        assert!(status.can_transition_to(&IncidentStatus::InProgress));
        assert!(!status.can_transition_to(&IncidentStatus::Closed));}

    #[test]
    fn test_team_member_functionality() {
        let mut member = IncidentTeamMember::new(
            "analyst-001".to_string(),
            "John Doe".to_string(),
            IncidentRole::LeadAnalyst,
            "john.doe@company.com".to_string(),
        );
        assert_eq!(member.assigned_incidents.len(), 0);
        assert!(!member.is_overloaded());
        member.assign_incident("INC-2024-001".to_string());
        assert_eq!(member.assigned_incidents.len(), 1);

        for i in 2..=5 {
            member.assign_incident(format!("INC-2024-{i:03}"));
        }
        assert!(member.is_overloaded());
    }

    #[test]
    fn test_incident_timeline_entry() {
        let entry = IncidentTimelineEntry::new(
            "Initial Detection".to_string(),
            "Suspicious activity detected by IDS".to_string(),
            "analyst@company.com".to_string(),
        );
        
        assert_eq!(entry.title, "Initial Detection");
        assert!(entry.details.is_none());
    }

    #[test]
    fn test_incident_metrics() {
        let metrics = IncidentMetrics::new();
        assert_eq!(metrics.total_incidents, 0);
        assert_eq!(metrics.open_incidents, 0);
        assert_eq!(metrics.closed_incidents, 0);
        assert_eq!(metrics.mean_time_to_detection_minutes, 0.0);
        assert_eq!(metrics.mean_time_to_response_minutes, 0.0);
        assert_eq!(metrics.mean_time_to_resolution_minutes, 0.0);
    }

    #[test]
    fn test_incident_response_methods() {
        let mut incident = SecurityIncident::new(
            "Malware Detection".to_string(),
            "Malware detected on endpoint".to_string(),
            IncidentSeverity::High,
            IncidentType::Malware,
        );

        incident.start_investigation();
        assert_eq!(incident.status, IncidentStatus::InProgress);
        
        incident.escalate();
        assert_eq!(incident.severity, IncidentSeverity::Critical);
        
        incident.resolve("Malware removed and system cleaned".to_string());
        assert_eq!(incident.status, IncidentStatus::Resolved);
    }
}
