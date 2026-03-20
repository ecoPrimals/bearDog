// SPDX-License-Identifier: AGPL-3.0-only

// ============================================================================
// types/incidents.rs - 18 uncovered lines, 52.63% coverage
// ============================================================================

#[cfg(test)]
mod incidents_tests {
    use crate::threat::types::ThreatSeverity;
    use crate::threat::types::incidents::{IncidentStatus, SecurityIncident};

    #[test]
    fn test_security_incident_new() {
        let incident = SecurityIncident::new(
            "INC-001".to_string(),
            "Malware Detected".to_string(),
            "Malware found on server".to_string(),
            ThreatSeverity::High,
        );
        assert_eq!(incident.id, "INC-001");
        assert_eq!(incident.title, "Malware Detected");
        assert_eq!(incident.status, IncidentStatus::Reported);
        assert!(incident.assigned_to.is_none());
        assert!(incident.threat_events.is_empty());
        assert!(incident.response_actions.is_empty());
        assert!(incident.notes.is_empty());
    }

    #[test]
    fn test_update_status() {
        let mut incident = SecurityIncident::new(
            "INC-002".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Medium,
        );
        incident.update_status(IncidentStatus::Investigating);
        assert_eq!(incident.status, IncidentStatus::Investigating);

        incident.update_status(IncidentStatus::Resolved);
        assert_eq!(incident.status, IncidentStatus::Resolved);
    }

    #[test]
    fn test_assign_to() {
        let mut incident = SecurityIncident::new(
            "INC-003".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Low,
        );
        assert!(incident.assigned_to.is_none());

        incident.assign_to("security_team".to_string());
        assert_eq!(incident.assigned_to, Some("security_team".to_string()));
    }

    #[test]
    fn test_add_note() {
        let mut incident = SecurityIncident::new(
            "INC-004".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Low,
        );
        incident.add_note("Initial investigation started".to_string());
        incident.add_note("Found suspicious file".to_string());
        assert_eq!(incident.notes.len(), 2);
        assert_eq!(incident.notes[0], "Initial investigation started");
    }

    #[test]
    fn test_link_threat_event() {
        let mut incident = SecurityIncident::new(
            "INC-005".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Critical,
        );
        incident.link_threat_event("threat-001".to_string());
        incident.link_threat_event("threat-002".to_string());
        assert_eq!(incident.threat_events.len(), 2);

        // Duplicate should not be added
        incident.link_threat_event("threat-001".to_string());
        assert_eq!(incident.threat_events.len(), 2);
    }

    #[test]
    fn test_incident_status_variants() {
        let statuses = vec![
            IncidentStatus::Reported,
            IncidentStatus::Investigating,
            IncidentStatus::Confirmed,
            IncidentStatus::Containing,
            IncidentStatus::Contained,
            IncidentStatus::Remediating,
            IncidentStatus::Resolved,
            IncidentStatus::FalsePositive,
            IncidentStatus::Closed,
        ];
        for s in &statuses {
            let serialized = serde_json::to_string(s).unwrap();
            assert!(!serialized.is_empty());
        }
    }
}

// ============================================================================
// types/engine/conditions.rs - 49 uncovered lines, 77.52% coverage
// ============================================================================

#[cfg(test)]
mod conditions_gap_tests {
    use crate::threat::types::engine::conditions::{ConditionBuilder, RuleCondition};
    use std::collections::HashMap;

    #[test]
    fn test_field_in_match() {
        let condition = RuleCondition::FieldIn {
            field: "severity".to_string(),
            values: vec!["high".to_string(), "critical".to_string()],
        };
        let mut data = HashMap::new();
        data.insert("severity".to_string(), "high".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_in_no_match() {
        let condition = RuleCondition::FieldIn {
            field: "severity".to_string(),
            values: vec!["high".to_string(), "critical".to_string()],
        };
        let mut data = HashMap::new();
        data.insert("severity".to_string(), "low".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_in_missing_field() {
        let condition = RuleCondition::FieldIn {
            field: "severity".to_string(),
            values: vec!["high".to_string()],
        };
        let data = HashMap::new();
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_greater_than_match() {
        let condition = RuleCondition::FieldGreaterThan {
            field: "score".to_string(),
            value: 50.0,
        };
        let mut data = HashMap::new();
        data.insert("score".to_string(), "75".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_greater_than_no_match() {
        let condition = RuleCondition::FieldGreaterThan {
            field: "score".to_string(),
            value: 50.0,
        };
        let mut data = HashMap::new();
        data.insert("score".to_string(), "30".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_greater_than_not_number() {
        let condition = RuleCondition::FieldGreaterThan {
            field: "score".to_string(),
            value: 50.0,
        };
        let mut data = HashMap::new();
        data.insert("score".to_string(), "not_a_number".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_less_than_match() {
        let condition = RuleCondition::FieldLessThan {
            field: "latency".to_string(),
            value: 100.0,
        };
        let mut data = HashMap::new();
        data.insert("latency".to_string(), "50".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_less_than_no_match() {
        let condition = RuleCondition::FieldLessThan {
            field: "latency".to_string(),
            value: 100.0,
        };
        let mut data = HashMap::new();
        data.insert("latency".to_string(), "200".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_between_match() {
        let condition = RuleCondition::FieldBetween {
            field: "temp".to_string(),
            min: 20.0,
            max: 30.0,
        };
        let mut data = HashMap::new();
        data.insert("temp".to_string(), "25".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_between_no_match() {
        let condition = RuleCondition::FieldBetween {
            field: "temp".to_string(),
            min: 20.0,
            max: 30.0,
        };
        let mut data = HashMap::new();
        data.insert("temp".to_string(), "35".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_between_boundary() {
        let condition = RuleCondition::FieldBetween {
            field: "val".to_string(),
            min: 10.0,
            max: 20.0,
        };
        let mut data = HashMap::new();
        data.insert("val".to_string(), "10".to_string());
        assert!(condition.evaluate(&data));
        data.insert("val".to_string(), "20".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_matches_returns_false() {
        let condition = RuleCondition::FieldMatches {
            field: "msg".to_string(),
            pattern: ".*error.*".to_string(),
        };
        let mut data = HashMap::new();
        data.insert("msg".to_string(), "an error occurred".to_string());
        assert!(!condition.evaluate(&data)); // Simplified to always false
    }

    #[test]
    fn test_frequency_threshold_returns_false() {
        let condition = RuleCondition::FrequencyThreshold {
            count: 5,
            window_minutes: 10,
        };
        assert!(!condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_time_window_returns_false() {
        let condition = RuleCondition::TimeWindow {
            start_hour: 9,
            end_hour: 17,
        };
        assert!(!condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_custom_condition_returns_false() {
        let condition = RuleCondition::Custom {
            name: "custom_check".to_string(),
            parameters: HashMap::new(),
        };
        assert!(!condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_always_condition() {
        let condition = RuleCondition::Always;
        assert!(condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_is_complex_custom() {
        let condition = RuleCondition::Custom {
            name: "test".to_string(),
            parameters: HashMap::new(),
        };
        assert!(condition.is_complex());
    }

    #[test]
    fn test_complexity_score_custom() {
        let condition = RuleCondition::Custom {
            name: "test".to_string(),
            parameters: HashMap::new(),
        };
        assert_eq!(condition.complexity_score(), 3);
    }

    #[test]
    fn test_complexity_score_frequency_threshold() {
        let condition = RuleCondition::FrequencyThreshold {
            count: 5,
            window_minutes: 10,
        };
        assert_eq!(condition.complexity_score(), 2);
    }

    #[test]
    fn test_referenced_fields_logical_not() {
        let condition = RuleCondition::logical_not(RuleCondition::field_equals("f1", "v1"));
        let fields = condition.referenced_fields();
        assert_eq!(fields, vec!["f1"]);
    }

    #[test]
    fn test_referenced_fields_always() {
        let condition = RuleCondition::Always;
        assert!(condition.referenced_fields().is_empty());
    }

    #[test]
    fn test_referenced_fields_field_in() {
        let condition = RuleCondition::FieldIn {
            field: "status".to_string(),
            values: vec!["a".to_string()],
        };
        let fields = condition.referenced_fields();
        assert_eq!(fields, vec!["status"]);
    }

    #[test]
    fn test_referenced_fields_field_between() {
        let condition = RuleCondition::FieldBetween {
            field: "temp".to_string(),
            min: 0.0,
            max: 100.0,
        };
        let fields = condition.referenced_fields();
        assert_eq!(fields, vec!["temp"]);
    }

    #[test]
    fn test_builder_single_condition_build_and() {
        let condition = ConditionBuilder::new()
            .field_equals("key", "value")
            .build_and();
        assert!(!condition.is_complex());
    }

    #[test]
    fn test_builder_single_condition_build_or() {
        let condition = ConditionBuilder::new().field_exists("key").build_or();
        assert!(!condition.is_complex());
    }

    #[test]
    fn test_builder_multiple_conditions_build_or() {
        let condition = ConditionBuilder::new()
            .field_equals("a", "1")
            .field_equals("b", "2")
            .build_or();
        assert!(condition.is_complex());
    }

    #[test]
    fn test_builder_add_condition() {
        let condition = ConditionBuilder::new()
            .add_condition(RuleCondition::Always)
            .add_condition(RuleCondition::field_exists("x"))
            .build_and();
        assert!(condition.is_complex());
    }

    #[test]
    fn test_builder_field_contains() {
        let condition = ConditionBuilder::new()
            .field_contains("msg", "error")
            .build_and();
        let mut data = HashMap::new();
        data.insert("msg".to_string(), "an error happened".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_logical_not_complexity() {
        let inner = RuleCondition::field_equals("a", "b");
        let not = RuleCondition::logical_not(inner);
        assert_eq!(not.complexity_score(), 2);
    }
}
