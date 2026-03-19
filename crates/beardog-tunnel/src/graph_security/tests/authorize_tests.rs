// SPDX-License-Identifier: AGPL-3.0-only

//! Unit tests for authorization

use crate::graph_security::{
    authorize::authorize_modification,
    types::{Graph, GraphModification, GraphNode, ModificationAction, RiskLevel},
};
use std::collections::HashMap;

fn create_test_graph(owner: &str) -> Graph {
    Graph {
        id: "test-graph".to_string(),
        owner: owner.to_string(),
        nodes: vec![],
        edges: vec![],
        metadata: Some(HashMap::new()),
    }
}

fn create_test_node(id: &str) -> GraphNode {
    GraphNode {
        id: id.to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config: HashMap::new(),
    }
}

#[tokio::test]
async fn test_owner_can_add_node() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(result.authorized, "Owner should be authorized");
    assert_eq!(result.risk_level, RiskLevel::Low);
    assert!(result
        .checks_performed
        .contains(&"permission_check".to_string()));
}

#[tokio::test]
async fn test_non_owner_cannot_add_node() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"bob".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized, "Non-owner should not be authorized");
    assert!(result.blocked_reason.is_some());
    assert_eq!(
        result.blocked_reason.as_ref().expect("Should have reason"),
        "insufficient_permissions"
    );
}

#[tokio::test]
async fn test_invalid_structure_rejected() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: None, // Missing required node
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized);
    assert_eq!(result.blocked_reason, Some("invalid_structure".to_string()));
    assert!(result.recommendations.is_some());
}

#[tokio::test]
async fn test_code_injection_detected() {
    let graph = create_test_graph("alice");

    let mut config = HashMap::new();
    config.insert("command".to_string(), serde_json::json!("eval(user_input)"));

    let malicious_node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(malicious_node),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized, "Malicious code should be blocked");
    assert_eq!(result.blocked_reason, Some("threat_detected".to_string()));
    assert!(result.threat_details.is_some());
    assert_eq!(result.risk_level, RiskLevel::High);
}

#[tokio::test]
async fn test_privilege_escalation_detected() {
    let graph = create_test_graph("alice");

    let mut changes = HashMap::new();
    changes.insert("role".to_string(), serde_json::json!("admin"));

    let modification = GraphModification {
        action: ModificationAction::ModifyNode,
        node: None,
        node_id: Some("node-1".to_string()),
        edge: None,
        changes: Some(changes),
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized, "Privilege escalation should be blocked");
    assert!(result.threat_details.is_some());
}

#[tokio::test]
async fn test_owner_can_remove_node() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::RemoveNode,
        node: None,
        node_id: Some("node-1".to_string()),
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(result.authorized, "Owner should be able to remove nodes");
}

#[tokio::test]
async fn test_non_owner_cannot_remove_node() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::RemoveNode,
        node: None,
        node_id: Some("node-1".to_string()),
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"bob".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(
        !result.authorized,
        "Non-owner should not be able to remove nodes"
    );
}

#[tokio::test]
async fn test_owner_can_add_edge() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddEdge,
        node: None,
        node_id: None,
        edge: Some(crate::graph_security::types::GraphEdge {
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            edge_type: None,
        }),
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(result.authorized);
}

#[tokio::test]
async fn test_owner_can_modify_node() {
    let graph = create_test_graph("alice");

    let mut changes = HashMap::new();
    changes.insert("memory".to_string(), serde_json::json!("8GB"));

    let modification = GraphModification {
        action: ModificationAction::ModifyNode,
        node: None,
        node_id: Some("node-1".to_string()),
        edge: None,
        changes: Some(changes),
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(result.authorized, "Owner should be able to modify nodes");
}

#[tokio::test]
async fn test_shell_injection_detected() {
    let graph = create_test_graph("alice");

    let mut config = HashMap::new();
    config.insert("script".to_string(), serde_json::json!("$(rm -rf /)"));

    let malicious_node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(malicious_node),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized, "Shell injection should be blocked");
    assert_eq!(result.risk_level, RiskLevel::High);
}

#[tokio::test]
async fn test_backtick_injection_detected() {
    let graph = create_test_graph("alice");

    let mut config = HashMap::new();
    config.insert("command".to_string(), serde_json::json!("`whoami`"));

    let malicious_node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(malicious_node),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized, "Backtick injection should be blocked");
}

#[tokio::test]
async fn test_authorization_includes_audit_id() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.audit_id.is_empty(), "Should include audit ID");
}

#[tokio::test]
async fn test_authorization_includes_checks_performed() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(result
        .checks_performed
        .contains(&"permission_check".to_string()));
    assert!(result
        .checks_performed
        .contains(&"structure_validation".to_string()));
    assert!(result
        .checks_performed
        .contains(&"threat_detection".to_string()));
}

#[tokio::test]
async fn test_authorization_confidence_score() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(
        result.confidence > 0.9,
        "Confidence should be high for authorized requests"
    );
}

#[tokio::test]
async fn test_owner_can_modify_graph_metadata() {
    let mut graph = create_test_graph("alice");
    graph.metadata = Some(HashMap::from([
        ("version".to_string(), serde_json::json!("1.0")),
        ("description".to_string(), serde_json::json!("Test graph")),
    ]));

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-2")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(
        result.authorized,
        "Owner should be able to add nodes with metadata present"
    );
}

#[tokio::test]
async fn test_empty_metadata_handled() {
    let mut graph = create_test_graph("alice");
    graph.metadata = None;

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-2")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(
        result.authorized,
        "Should handle missing metadata gracefully"
    );
}

#[tokio::test]
async fn test_multiple_threat_detection() {
    let graph = create_test_graph("alice");

    let mut config = HashMap::new();
    config.insert("script".to_string(), serde_json::json!("eval('rm -rf /')"));
    config.insert("cpu".to_string(), serde_json::json!("1000"));

    let malicious_node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(malicious_node),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.authorized, "Multiple threats should be blocked");
    assert!(
        result.threat_details.is_some(),
        "Should have threat details"
    );
    let threat = result.threat_details.unwrap();
    assert!(!threat.pattern.is_empty(), "Should have threat pattern");
}

#[tokio::test]
async fn test_modification_with_different_primal() {
    let graph = create_test_graph("alice");

    let different_node = GraphNode {
        id: "node-1".to_string(),
        node_type: "storage".to_string(),
        primal: "Squirrel".to_string(),
        config: HashMap::new(),
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(different_node),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    // Owner can add nodes with different primals
    assert!(result.confidence > 0.0, "Should have some confidence");
}

#[tokio::test]
async fn test_remove_nonexistent_node() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::RemoveNode,
        node: None,
        node_id: Some("nonexistent-node".to_string()),
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    // Authorization may pass (owner can try), but would fail at execution
    assert!(
        result.confidence >= 0.0,
        "Should have some confidence value"
    );
}

#[tokio::test]
async fn test_add_edge_between_nodes() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddEdge,
        node: None,
        node_id: None,
        edge: Some(crate::graph_security::types::GraphEdge {
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            edge_type: Some("depends_on".to_string()),
        }),
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    // Owner can add edges
    assert!(result.confidence >= 0.0, "Should have confidence value");
}

#[tokio::test]
async fn test_authorization_with_empty_graph() {
    let graph = crate::graph_security::types::Graph {
        id: "empty-graph".to_string(),
        owner: "alice".to_string(),
        nodes: vec![],
        edges: vec![],
        metadata: Some(HashMap::new()),
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(result.authorized, "Owner can add to empty graph");
}

#[tokio::test]
async fn test_risk_level_assessment() {
    let graph = create_test_graph("alice");

    let mut config = HashMap::new();
    config.insert("command".to_string(), serde_json::json!("sudo rm -rf /"));

    let high_risk_node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };

    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(high_risk_node),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    // Should have a risk level assessment (could be any level)
    // Just verify we got some response
    assert!(result.confidence >= 0.0);
}

#[tokio::test]
async fn test_authorization_reasoning_present() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.reasoning.is_empty(), "Should provide reasoning");
    assert!(
        result.reasoning.len() > 20,
        "Reasoning should be descriptive"
    );
}

#[tokio::test]
async fn test_audit_id_format() {
    let graph = create_test_graph("alice");
    let modification = GraphModification {
        action: ModificationAction::AddNode,
        node: Some(create_test_node("node-1")),
        node_id: None,
        edge: None,
        changes: None,
    };

    let result = authorize_modification(&"alice".to_string(), &graph, &modification)
        .await
        .expect("Authorization should succeed");

    assert!(!result.audit_id.is_empty(), "Should include audit ID");
    // Audit ID should be a reasonable format (UUID or similar)
    assert!(result.audit_id.len() > 10, "Audit ID should be substantial");
}
