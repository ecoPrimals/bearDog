// SPDX-License-Identifier: AGPL-3.0-only

// ========================================================================
// authorization_types: 0% → test all Default impls and type construction
// ========================================================================

mod authorization_types_tests {
    use crate::authorization_types::*;
    use std::collections::HashMap;

    #[test]
    fn test_subject_default() {
        let subject = Subject::default();
        assert!(subject.id.is_empty());
        assert!(subject.name.is_empty());
        assert!(matches!(subject.subject_type, SubjectType::User));
        assert!(subject.roles.is_empty());
        assert!(subject.clearance_level.is_none());
        assert!(subject.metadata.is_empty());
    }

    #[test]
    fn test_subject_custom() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());
        let subject = Subject {
            id: "sub-001".to_string(),
            name: "Test Subject".to_string(),
            subject_type: SubjectType::Service,
            roles: vec!["admin".to_string()],
            clearance_level: Some(5),
            metadata,
        };
        assert_eq!(subject.id, "sub-001");
        assert_eq!(subject.name, "Test Subject");
        assert!(matches!(subject.subject_type, SubjectType::Service));
        assert_eq!(subject.roles.len(), 1);
        assert_eq!(subject.clearance_level, Some(5));
    }

    #[test]
    fn test_subject_type_variants() {
        let _user = SubjectType::User;
        let _service = SubjectType::Service;
        let _node = SubjectType::Node;
        let _system = SubjectType::System;
        // Verify Debug
        assert!(!format!("{:?}", SubjectType::User).is_empty());
        assert!(!format!("{:?}", SubjectType::Node).is_empty());
        assert!(!format!("{:?}", SubjectType::System).is_empty());
    }

    #[test]
    fn test_action_default() {
        let action = Action::default();
        assert!(action.name.is_empty());
        assert!(matches!(action.action_type, ActionType::Read));
        assert!(action.metadata.is_empty());
    }

    #[test]
    fn test_action_custom() {
        let action = Action {
            name: "deploy".to_string(),
            action_type: ActionType::Execute,
            metadata: HashMap::new(),
        };
        assert_eq!(action.name, "deploy");
        assert_eq!(action.action_type, ActionType::Execute);
    }

    #[test]
    fn test_action_type_variants() {
        assert_eq!(ActionType::Read, ActionType::Read);
        assert_ne!(ActionType::Read, ActionType::Write);
        assert_ne!(ActionType::Execute, ActionType::Delete);
        assert_ne!(ActionType::Admin, ActionType::Approve);
        assert_ne!(ActionType::Create, ActionType::Update);
        // Debug
        assert!(!format!("{:?}", ActionType::Write).is_empty());
        assert!(!format!("{:?}", ActionType::Admin).is_empty());
        assert!(!format!("{:?}", ActionType::Approve).is_empty());
        assert!(!format!("{:?}", ActionType::Create).is_empty());
        assert!(!format!("{:?}", ActionType::Update).is_empty());
    }

    #[test]
    fn test_resource_default() {
        let resource = Resource::default();
        assert!(resource.name.is_empty());
        assert!(matches!(
            resource.classification,
            ResourceClassification::Internal
        ));
        assert!(resource.metadata.is_empty());
    }

    #[test]
    fn test_resource_custom() {
        let resource = Resource {
            name: "secret-doc".to_string(),
            classification: ResourceClassification::Secret,
            metadata: HashMap::new(),
        };
        assert_eq!(resource.name, "secret-doc");
        assert_eq!(resource.classification, ResourceClassification::Secret);
    }

    #[test]
    fn test_resource_classification_variants() {
        assert_eq!(
            ResourceClassification::Public,
            ResourceClassification::Public
        );
        assert_ne!(
            ResourceClassification::Confidential,
            ResourceClassification::TopSecret
        );
        assert!(!format!("{:?}", ResourceClassification::Internal).is_empty());
        assert!(!format!("{:?}", ResourceClassification::TopSecret).is_empty());
    }

    #[test]
    fn test_risk_level_variants() {
        assert_eq!(RiskLevel::Low, RiskLevel::Low);
        assert_ne!(RiskLevel::Medium, RiskLevel::High);
        assert_ne!(RiskLevel::High, RiskLevel::Critical);
        assert!(!format!("{:?}", RiskLevel::Critical).is_empty());
    }

    #[test]
    fn test_authorization_result_construction() {
        let result = AuthorizationResult {
            authorized: true,
            reason: "Access granted".to_string(),
            risk_level: RiskLevel::Low,
            additional_requirements: vec![],
            expires_at: None,
            audit_id: "audit-001".to_string(),
        };
        assert!(result.authorized);
        assert_eq!(result.reason, "Access granted");
        assert_eq!(result.risk_level, RiskLevel::Low);
        assert!(result.additional_requirements.is_empty());
        assert!(result.expires_at.is_none());
    }

    #[test]
    fn test_authorization_result_with_requirements() {
        let result = AuthorizationResult {
            authorized: false,
            reason: "Needs approval".to_string(),
            risk_level: RiskLevel::High,
            additional_requirements: vec!["mfa".to_string(), "manager_approval".to_string()],
            expires_at: Some(chrono::Utc::now()),
            audit_id: "audit-002".to_string(),
        };
        assert!(!result.authorized);
        assert_eq!(result.additional_requirements.len(), 2);
        assert!(result.expires_at.is_some());
    }

    #[test]
    fn test_subject_clone() {
        let subject = Subject {
            id: "clone-test".to_string(),
            name: "Clone".to_string(),
            subject_type: SubjectType::Node,
            roles: vec!["reader".to_string()],
            clearance_level: Some(3),
            metadata: HashMap::new(),
        };
        let cloned = subject.clone();
        assert_eq!(cloned.id, subject.id);
        assert_eq!(cloned.name, subject.name);
    }
}
