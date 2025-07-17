//! Incident response types
//!
//! This module contains types for incident response management,
//! including incident structures and status tracking.

pub mod metrics;
pub mod response;
pub mod status;
pub mod team;
pub mod timeline;

// Re-export all types for backward compatibility
pub use metrics::*;
pub use response::*;
pub use status::*;
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
            "analyst-001".to_string(),
            "John Doe".to_string(),
            IncidentRole::LeadAnalyst,
            "john.doe@company.com".to_string(),
        );

        assert_eq!(member.assigned_incidents.len(), 0);
        assert!(!member.is_overloaded());

        member.assign_incident("INC-2024-001".to_string());
        assert_eq!(member.assigned_incidents.len(), 1);

        // Test overload detection
        for i in 2..=5 {
            member.assign_incident(format!("INC-2024-{i:03}"));
        }
        assert!(member.is_overloaded());
    }

    #[test]
    fn test_timeline_entry_creation() {
        let entry = IncidentTimelineEntry::new(
            "Malware detected".to_string(),
            "antivirus-system".to_string(),
            TimelineEntryType::Detection,
        );

        assert_eq!(entry.action, "Malware detected");
        assert_eq!(entry.actor, "antivirus-system");
        assert_eq!(entry.entry_type, TimelineEntryType::Detection);
        assert!(entry.details.is_none());
    }

    #[test]
    fn test_incident_metrics() {
        let mut metrics = IncidentMetrics::new();
        assert!(!metrics.is_complete());
        assert_eq!(metrics.calculate_total_cost(), 0.0);

        metrics.estimated_cost = Some(5000.0);
        assert_eq!(metrics.calculate_total_cost(), 5000.0);

        metrics.time_to_detection = Some(30);
        metrics.time_to_containment = Some(120);
        metrics.time_to_resolution = Some(480);
        metrics.total_duration = Some(510);
        assert!(metrics.is_complete());
    }

    #[test]
    fn test_incident_response_methods() {
        let mut incident = IncidentResponse::default();

        // Test status update
        incident.update_status(IncidentStatus::InProgress);
        assert_eq!(incident.status, IncidentStatus::InProgress);

        // Test assignment
        incident.assign_to("analyst-001".to_string());
        assert_eq!(incident.assigned_to, Some("analyst-001".to_string()));

        // Test actions
        incident.add_containment_action("Isolated system".to_string());
        assert_eq!(incident.containment_actions.len(), 1);

        incident.add_remediation_action("Malware removed".to_string());
        assert_eq!(incident.remediation_actions.len(), 1);

        incident.add_lesson_learned("Update signatures".to_string());
        assert_eq!(incident.lessons_learned.len(), 1);

        // Test status checks
        assert!(incident.is_active());

        incident.update_status(IncidentStatus::Closed);
        assert!(!incident.is_active());
    }
}
