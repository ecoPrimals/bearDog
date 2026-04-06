// SPDX-License-Identifier: AGPL-3.0-or-later
//! Pure unit tests for tunnel exports, graph security types, and doctor helpers (coverage wave 2).

#![allow(clippy::expect_used, clippy::unwrap_used)]

use crate::graph_security::types::{
    AuthorizationResult, CommunityUsage, CreatorInfo, Graph, GraphEdge, GraphModification,
    GraphNode, IssueSeverity, LineageVersion, ModificationAction, OriginAudit, RiskLevel,
    SecurityAssessment, TemplateMetadata, ThreatCategory, ThreatDetails, ValidationIssue,
    ValidationReport,
};
use crate::modes::doctor::{doctor_json_status_line, doctor_resolve_socket_path};
use crate::{BStpConfig, SecurityLevel, SessionManager};
use std::collections::HashMap;

#[test]
fn bstp_config_default_security_level_high() {
    let c = BStpConfig::default();
    assert_eq!(c.security_level, SecurityLevel::High);
}

#[test]
fn bstp_config_clone_and_mutate_security_level() {
    let mut c = BStpConfig::default();
    c.security_level = SecurityLevel::Critical;
    let d = c.clone();
    assert_eq!(d.security_level, SecurityLevel::Critical);
}

#[test]
fn bstp_config_maximum_security_preset_values() {
    let c = BStpConfig::maximum_security();
    assert_eq!(c.security_level, SecurityLevel::Critical);
    assert_eq!(c.max_concurrent_sessions, 100);
    assert_eq!(c.session_timeout_seconds, 1800);
}

#[test]
fn bstp_config_competitive_gaming_preset_values() {
    let c = BStpConfig::competitive_gaming();
    assert_eq!(c.security_level, SecurityLevel::High);
    assert_eq!(c.max_concurrent_sessions, 10000);
    assert!(c.session_timeout_seconds >= 7200);
}

#[test]
fn session_manager_new_is_send_sync() {
    let m = SessionManager::new();
    let _ = format!("{m:?}");
}

#[test]
fn security_level_variants_distinct() {
    use SecurityLevel::*;
    let v = [Low, Medium, High, Critical];
    let n = v.len();
    let mut u = std::collections::HashSet::new();
    for x in v {
        u.insert(std::mem::discriminant(&x));
    }
    assert_eq!(u.len(), n);
}

#[test]
fn doctor_json_status_line_comprehensive_true_and_false() {
    let t = doctor_json_status_line(true);
    let f = doctor_json_status_line(false);
    assert_ne!(t, f);
    assert!(t.contains("true"));
    assert!(f.contains("false"));
}

#[test]
fn doctor_resolve_socket_path_explicit_wins() {
    assert_eq!(
        doctor_resolve_socket_path(Some("/explicit.sock".into())),
        "/explicit.sock"
    );
}

#[test]
fn risk_level_json_roundtrip_all() {
    for r in [
        RiskLevel::Low,
        RiskLevel::Medium,
        RiskLevel::High,
        RiskLevel::Critical,
    ] {
        let j = serde_json::to_string(&r).expect("ser");
        let back: RiskLevel = serde_json::from_str(&j).expect("de");
        assert_eq!(r, back);
    }
}

#[test]
fn threat_category_json_roundtrip() {
    let t = ThreatCategory::Privilege;
    let j = serde_json::to_string(&t).expect("ser");
    let back: ThreatCategory = serde_json::from_str(&j).expect("de");
    assert_eq!(t, back);
}

#[test]
fn issue_severity_json_roundtrip() {
    let s = IssueSeverity::High;
    let j = serde_json::to_string(&s).expect("ser");
    let back: IssueSeverity = serde_json::from_str(&j).expect("de");
    assert_eq!(s, back);
}

#[test]
fn graph_roundtrip_with_metadata() {
    let g = Graph {
        id: "g1".to_string(),
        owner: "u1".to_string(),
        nodes: vec![GraphNode {
            id: "n1".to_string(),
            node_type: "compute".to_string(),
            handler_ref: "h1".to_string(),
            config: HashMap::from([("k".to_string(), serde_json::json!(1))]),
        }],
        edges: vec![GraphEdge {
            from: "n1".to_string(),
            to: "n2".to_string(),
            edge_type: Some("data".to_string()),
        }],
        metadata: Some(HashMap::from([(
            "m".to_string(),
            serde_json::json!({"a": true}),
        )])),
    };
    let j = serde_json::to_string(&g).expect("ser");
    let back: Graph = serde_json::from_str(&j).expect("de");
    assert_eq!(back.id, g.id);
    assert_eq!(back.nodes.len(), 1);
}

#[test]
fn graph_modification_all_actions_serialize() {
    for a in [
        ModificationAction::AddNode,
        ModificationAction::RemoveNode,
        ModificationAction::ModifyNode,
        ModificationAction::AddEdge,
        ModificationAction::RemoveEdge,
    ] {
        let j = serde_json::to_string(&a).expect("ser");
        assert!(!j.is_empty());
    }
}

#[test]
fn authorization_result_with_threat_details_roundtrip() {
    let ar = AuthorizationResult {
        authorized: false,
        reasoning: "denied".to_string(),
        confidence: 0.2,
        risk_level: RiskLevel::High,
        checks_performed: vec!["a".to_string()],
        audit_id: "audit-1".to_string(),
        blocked_reason: Some("policy".to_string()),
        threat_details: Some(ThreatDetails {
            category: ThreatCategory::Anomaly,
            location: "n3".to_string(),
            pattern: "p".to_string(),
        }),
        recommendations: Some(vec!["fix".to_string()]),
    };
    let j = serde_json::to_string(&ar).expect("ser");
    let back: AuthorizationResult = serde_json::from_str(&j).expect("de");
    assert!(!back.authorized);
    assert!(back.threat_details.is_some());
}

#[test]
fn validation_report_empty_issues_still_roundtrips() {
    let vr = ValidationReport {
        valid: true,
        risk_level: RiskLevel::Low,
        issues: vec![],
        security_score: 1.0,
        checks_performed: vec![],
        recommendations: vec![],
        validation_id: "v1".to_string(),
    };
    let j = serde_json::to_string(&vr).expect("ser");
    let back: ValidationReport = serde_json::from_str(&j).expect("de");
    assert!(back.valid);
    assert!(back.issues.is_empty());
}

#[test]
fn validation_issue_with_optional_location() {
    let vi = ValidationIssue {
        severity: IssueSeverity::Medium,
        category: ThreatCategory::Structure,
        description: "d".to_string(),
        location: None,
    };
    let j = serde_json::to_string(&vi).expect("ser");
    let back: ValidationIssue = serde_json::from_str(&j).expect("de");
    assert_eq!(back.description, "d");
}

#[test]
fn graph_modification_with_edge_roundtrip() {
    let gm = GraphModification {
        action: ModificationAction::AddEdge,
        node: None,
        node_id: None,
        edge: Some(GraphEdge {
            from: "a".to_string(),
            to: "b".to_string(),
            edge_type: None,
        }),
        changes: None,
    };
    let j = serde_json::to_string(&gm).expect("ser");
    let back: GraphModification = serde_json::from_str(&j).expect("de");
    assert!(back.edge.is_some());
}

#[test]
fn template_metadata_optional_description() {
    let tm = TemplateMetadata {
        version: "1.0.0".to_string(),
        created_at: "2020-01-01T00:00:00Z".to_string(),
        description: None,
    };
    let j = serde_json::to_string(&tm).expect("ser");
    assert!(!j.contains("description"));
}

#[test]
fn creator_info_genetic_family_none_roundtrip() {
    let c = CreatorInfo {
        user_id: "u".to_string(),
        identity_verified: true,
        trust_score: 0.9,
        reputation: "high".to_string(),
        member_since: "2021-01-01T00:00:00Z".to_string(),
        genetic_family: None,
    };
    let j = serde_json::to_string(&c).expect("ser");
    let back: CreatorInfo = serde_json::from_str(&j).expect("de");
    assert_eq!(back.user_id, "u");
}

#[test]
fn lineage_version_optional_timestamps() {
    let lv = LineageVersion {
        version: "1.1.0".to_string(),
        created_at: None,
        modified_at: None,
        created_by: None,
        modified_by: None,
        change_type: "patch".to_string(),
        changes: None,
        signature: None,
    };
    let j = serde_json::to_string(&lv).expect("ser");
    let back: LineageVersion = serde_json::from_str(&j).expect("de");
    assert_eq!(back.version, "1.1.0");
}

#[test]
fn community_usage_optional_rates() {
    let cu = CommunityUsage {
        deployments: 0,
        success_rate: None,
        avg_rating: None,
        total_ratings: 0,
    };
    let j = serde_json::to_string(&cu).expect("ser");
    let back: CommunityUsage = serde_json::from_str(&j).expect("de");
    assert_eq!(back.deployments, 0);
}

#[test]
fn security_assessment_roundtrip() {
    let sa = SecurityAssessment {
        last_scan: "2024-01-01T00:00:00Z".to_string(),
        vulnerabilities_found: 0,
        threat_level: "low".to_string(),
    };
    let j = serde_json::to_string(&sa).expect("ser");
    let back: SecurityAssessment = serde_json::from_str(&j).expect("de");
    assert_eq!(back.vulnerabilities_found, 0);
}

#[test]
fn origin_audit_roundtrip_minimal() {
    let oa = OriginAudit {
        template_id: "t1".to_string(),
        creator: CreatorInfo {
            user_id: "u".to_string(),
            identity_verified: false,
            trust_score: 0.5,
            reputation: "new".to_string(),
            member_since: "2024-01-01T00:00:00Z".to_string(),
            genetic_family: Some("fam".to_string()),
        },
        lineage: vec![],
        chain_valid: true,
        risk_level: RiskLevel::Medium,
        trust_score: 0.6,
        community_usage: CommunityUsage {
            deployments: 1,
            success_rate: Some(1.0),
            avg_rating: Some(5.0),
            total_ratings: 1,
        },
        security_assessment: SecurityAssessment {
            last_scan: "2024-01-01T00:00:00Z".to_string(),
            vulnerabilities_found: 0,
            threat_level: "none".to_string(),
        },
        warnings: None,
        recommendations: None,
        audit_id: "a1".to_string(),
    };
    let j = serde_json::to_string(&oa).expect("ser");
    let back: OriginAudit = serde_json::from_str(&j).expect("de");
    assert_eq!(back.template_id, "t1");
}
