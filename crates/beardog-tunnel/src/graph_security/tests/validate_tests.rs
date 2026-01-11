//! Unit tests for template validation

use crate::graph_security::{
    types::{GraphEdge, GraphNode, GraphTemplate, RiskLevel, TemplateMetadata, ThreatCategory},
    validate::validate_template,
};
use std::collections::HashMap;

fn create_test_template(nodes: Vec<GraphNode>, edges: Vec<GraphEdge>) -> GraphTemplate {
    GraphTemplate {
        id: "template-1".to_string(),
        name: "Test Template".to_string(),
        creator: "alice".to_string(),
        nodes,
        edges,
        signature: None,
        metadata: TemplateMetadata {
            version: "1.0.0".to_string(),
            created_at: "2026-01-11T12:00:00Z".to_string(),
            description: None,
        },
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
async fn test_empty_template_invalid() {
    let template = create_test_template(vec![], vec![]);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(!report.valid, "Empty template should be invalid");
    assert!(!report.issues.is_empty());
}

#[tokio::test]
async fn test_valid_template() {
    let nodes = vec![
        create_test_node("node-1"),
        create_test_node("node-2"),
    ];
    let edges = vec![GraphEdge {
        from: "node-1".to_string(),
        to: "node-2".to_string(),
        edge_type: None,
    }];
    
    let template = create_test_template(nodes, edges);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(report.valid, "Valid template should pass");
    assert_eq!(report.risk_level, RiskLevel::Low);
    assert!(report.security_score > 0.9);
}

#[tokio::test]
async fn test_invalid_edge_reference() {
    let nodes = vec![create_test_node("node-1")];
    let edges = vec![GraphEdge {
        from: "node-1".to_string(),
        to: "non-existent".to_string(),
        edge_type: None,
    }];
    
    let template = create_test_template(nodes, edges);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(!report.valid, "Invalid edge reference should fail");
    assert!(report.issues.iter().any(|i| i.category == ThreatCategory::Structure));
}

#[tokio::test]
async fn test_cyclic_dependency_detected() {
    let nodes = vec![
        create_test_node("node-1"),
        create_test_node("node-2"),
    ];
    let edges = vec![
        GraphEdge {
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            edge_type: None,
        },
        GraphEdge {
            from: "node-2".to_string(),
            to: "node-1".to_string(),
            edge_type: None,
        },
    ];
    
    let template = create_test_template(nodes, edges);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(!report.valid, "Cyclic dependency should be detected");
    assert!(report.issues.iter().any(|i| i.category == ThreatCategory::Structure));
}

#[tokio::test]
async fn test_excessive_cpu_request() {
    let mut config = HashMap::new();
    config.insert("cpu".to_string(), serde_json::json!(128));
    
    let node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };
    
    let template = create_test_template(vec![node], vec![]);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(report.issues.iter().any(|i| i.category == ThreatCategory::ResourceAbuse));
}

#[tokio::test]
async fn test_excessive_memory_request() {
    let mut config = HashMap::new();
    config.insert("memory".to_string(), serde_json::json!("10TB"));
    
    let node = GraphNode {
        id: "node-1".to_string(),
        node_type: "compute".to_string(),
        primal: "ToadStool".to_string(),
        config,
    };
    
    let template = create_test_template(vec![node], vec![]);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(report.issues.iter().any(|i| i.category == ThreatCategory::ResourceAbuse));
}

#[tokio::test]
async fn test_validation_includes_recommendations() {
    let template = create_test_template(vec![], vec![]);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(!report.recommendations.is_empty(), "Should include recommendations");
}

#[tokio::test]
async fn test_validation_security_score() {
    let nodes = vec![create_test_node("node-1"), create_test_node("node-2")];
    let edges = vec![GraphEdge {
        from: "node-1".to_string(),
        to: "node-2".to_string(),
        edge_type: None,
    }];
    
    let template = create_test_template(nodes, edges);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(report.security_score >= 0.0 && report.security_score <= 1.0);
}

#[tokio::test]
async fn test_disconnected_subgraphs_detected() {
    let nodes = vec![
        create_test_node("node-1"),
        create_test_node("node-2"),
        create_test_node("node-3"),
    ];
    // node-1 and node-2 connected, node-3 disconnected
    let edges = vec![GraphEdge {
        from: "node-1".to_string(),
        to: "node-2".to_string(),
        edge_type: None,
    }];
    
    let template = create_test_template(nodes, edges);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    // Disconnected subgraphs are a low severity issue
    assert!(report.issues.iter().any(|i| i.description.contains("disconnected")));
}

#[tokio::test]
async fn test_validation_id_generated() {
    let template = create_test_template(vec![create_test_node("node-1")], vec![]);
    let report = validate_template(&template).await.expect("Validation should succeed");
    
    assert!(!report.validation_id.is_empty(), "Should generate validation ID");
}

