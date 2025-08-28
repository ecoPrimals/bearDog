pub mod metrics;
pub mod response;
pub mod team;
pub mod timeline;

pub use metrics::IncidentMetrics;
pub use response::{IncidentResponse, IncidentStatus}; // Use response::IncidentStatus as the primary one
pub use team::*;
pub use timeline::*;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::threat::types::core::ThreatSeverity;
    use crate::threat::types::incidents::metrics::{IncidentSeverity, SecurityIncident};
    // use crate::threat::types::incidents::response::IncidentType; // Unused import removed
    #[test]
    fn test_incident_response_creation() {
        let incident = IncidentResponse::new("INC-2024-001", ThreatSeverity::High, "Test incident");
        assert_eq!(incident.incident_id, "INC-2024-001");
        assert_eq!(incident.severity, ThreatSeverity::High);
        assert_eq!(incident.status, IncidentStatus::Open);
    }
    #[test]
    fn test_incident_status_transitions() {
        let status = IncidentStatus::Open;
        let valid_next = status.valid_next_statuses();
        assert!(valid_next.contains(&IncidentStatus::InProgress));
        assert!(valid_next.contains(&IncidentStatus::Escalated));
        assert!(status.can_transition_to(&IncidentStatus::InProgress));
        assert!(!status.can_transition_to(&IncidentStatus::Closed));
    }

    #[test]
    fn test_team_member_functionality() {
        let mut member = IncidentTeamMember::new(
            "analyst-001",
            "John Doe",
            IncidentRole::LeadAnalyst,
            "john.doe@company.com",
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
        let entry = IncidentTimelineEntry::new("Initial Detection", "analyst@company.com");

        assert_eq!(entry.action, "Initial Detection");
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
            IncidentSeverity::High,
            metrics::IncidentStatus::Open,
        );

        incident.start_investigation();
        assert_eq!(incident.status, metrics::IncidentStatus::InProgress);

        incident.escalate();
        assert_eq!(incident.severity, IncidentSeverity::Critical);

        incident.resolve("Malware removed and system cleaned".to_string());
        assert_eq!(incident.status, metrics::IncidentStatus::Resolved);
    }
}
