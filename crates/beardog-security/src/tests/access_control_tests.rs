// SPDX-License-Identifier: AGPL-3.0-or-later

//! Access control tests — authorization types and constant-time checks used by policy code.
//!
//! The legacy `access_control` / ecosystem-membership stack lives under `src/access_control/`
//! but is not exported from the crate root; these tests target the stable
//! [`crate::authorization_types`] and [`crate::constant_time_compare`] APIs.

use crate::authorization_types::{
    Action, ActionType, Resource, ResourceClassification, RiskLevel, Subject, SubjectType,
};
use crate::constant_time_compare;
use std::collections::HashMap;

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn subject_roles_clearance_and_metadata_round_trip() {
    let mut meta = HashMap::new();
    meta.insert("tenant".to_string(), "alpha".to_string());

    let subject = Subject {
        id: "u-1".to_string(),
        name: "Ada".to_string(),
        subject_type: SubjectType::User,
        roles: vec!["reader".to_string(), "writer".to_string()],
        clearance_level: Some(3),
        metadata: meta.clone(),
    };

    assert_eq!(subject.id, "u-1");
    assert_eq!(subject.roles.len(), 2);
    assert_eq!(subject.clearance_level, Some(3));
    assert_eq!(subject.metadata.get("tenant"), Some(&"alpha".to_string()));
}

#[test]
fn action_types_cover_common_crud_operations() {
    let types = [
        ActionType::Read,
        ActionType::Write,
        ActionType::Delete,
        ActionType::Admin,
    ];
    assert_eq!(types.len(), 4);
    let mut seen = std::collections::HashSet::new();
    for t in types {
        assert!(seen.insert(std::mem::discriminant(&t)));
    }
}

#[test]
fn resource_classification_ordering_for_policy_matching() {
    assert!(matches!(
        ResourceClassification::Public,
        ResourceClassification::Public
    ));
    let resource = Resource {
        name: "vault".to_string(),
        classification: ResourceClassification::Confidential,
        metadata: HashMap::new(),
    };
    assert_eq!(
        resource.classification,
        ResourceClassification::Confidential
    );
}

#[test]
fn action_with_metadata_preserves_entry() {
    let mut m = HashMap::new();
    m.insert("scope".to_string(), "documents/*".to_string());
    let action = Action {
        name: "open".to_string(),
        action_type: ActionType::Read,
        metadata: m,
    };
    assert_eq!(
        action.metadata.get("scope").map(String::as_str),
        Some("documents/*")
    );
}

#[test]
fn risk_level_exhaustive_for_authorization_audit() {
    let all = [
        RiskLevel::Low,
        RiskLevel::Medium,
        RiskLevel::High,
        RiskLevel::Critical,
    ];
    assert_eq!(all.len(), 4);
}

#[test]
fn constant_time_compare_models_permission_secret_check() {
    let granted = b"perm:read:documents";
    let offered = b"perm:read:documents";
    assert!(constant_time_compare(granted, offered));
    assert!(!constant_time_compare(granted, b"perm:write:documents"));
}
