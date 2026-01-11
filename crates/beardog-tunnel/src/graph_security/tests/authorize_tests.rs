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
    assert!(result.checks_performed.contains(&"permission_check".to_string()));
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
    assert_eq!(result.blocked_reason.as_ref().expect("Should have reason"), "insufficient_permissions");
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

