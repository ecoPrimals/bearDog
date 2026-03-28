// SPDX-License-Identifier: AGPL-3.0-only
//! Focused tests for graph security types and doctor helpers (coverage wave 3).

#![allow(clippy::expect_used, clippy::unwrap_used)]

use crate::graph_security::types::{GraphEdge, GraphNode, RiskLevel, ThreatCategory};

#[test]
fn risk_level_json_roundtrip_all_variants() {
    for level in [
        RiskLevel::Low,
        RiskLevel::Medium,
        RiskLevel::High,
        RiskLevel::Critical,
    ] {
        let j = serde_json::to_string(&level).expect("ser");
        let back: RiskLevel = serde_json::from_str(&j).expect("de");
        assert_eq!(level, back);
    }
}

#[test]
fn threat_category_json_roundtrip() {
    for cat in [
        ThreatCategory::Structure,
        ThreatCategory::Signature,
        ThreatCategory::CodeInjection,
        ThreatCategory::Privilege,
        ThreatCategory::Anomaly,
        ThreatCategory::ResourceAbuse,
    ] {
        let j = serde_json::to_string(&cat).expect("ser");
        let back: ThreatCategory = serde_json::from_str(&j).expect("de");
        assert_eq!(format!("{cat:?}"), format!("{back:?}"));
    }
}

#[test]
fn graph_node_serde_minimal() {
    let mut cfg = std::collections::HashMap::new();
    cfg.insert("k".to_string(), serde_json::json!(1));
    let n = GraphNode {
        id: "n1".to_string(),
        node_type: "compute".to_string(),
        handler_ref: "h1".to_string(),
        config: cfg,
    };
    let j = serde_json::to_string(&n).expect("ser");
    let back: GraphNode = serde_json::from_str(&j).expect("de");
    assert_eq!(back.id, "n1");
    assert_eq!(back.handler_ref, "h1");
}

#[test]
fn graph_edge_serde_optional_type() {
    let e = GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: Some("flow".to_string()),
    };
    let j = serde_json::to_string(&e).expect("ser");
    assert!(j.contains("flow"));
    let back: GraphEdge = serde_json::from_str(&j).expect("de");
    assert_eq!(back.from, "a");
    assert_eq!(back.to, "b");
}
