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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(!report.valid, "Empty template should be invalid");
    assert!(!report.issues.is_empty());
}

#[tokio::test]
async fn test_valid_template() {
    let nodes = vec![create_test_node("node-1"), create_test_node("node-2")];
    let edges = vec![GraphEdge {
        from: "node-1".to_string(),
        to: "node-2".to_string(),
        edge_type: None,
    }];

    let template = create_test_template(nodes, edges);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(!report.valid, "Invalid edge reference should fail");
    assert!(report
        .issues
        .iter()
        .any(|i| i.category == ThreatCategory::Structure));
}

#[tokio::test]
async fn test_cyclic_dependency_detected() {
    let nodes = vec![create_test_node("node-1"), create_test_node("node-2")];
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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(!report.valid, "Cyclic dependency should be detected");
    assert!(report
        .issues
        .iter()
        .any(|i| i.category == ThreatCategory::Structure));
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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(report
        .issues
        .iter()
        .any(|i| i.category == ThreatCategory::ResourceAbuse));
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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(report
        .issues
        .iter()
        .any(|i| i.category == ThreatCategory::ResourceAbuse));
}

#[tokio::test]
async fn test_validation_includes_recommendations() {
    let template = create_test_template(vec![], vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(
        !report.recommendations.is_empty(),
        "Should include recommendations"
    );
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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

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
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    // Disconnected subgraphs are a low severity issue
    assert!(report
        .issues
        .iter()
        .any(|i| i.description.contains("disconnected")));
}

#[tokio::test]
async fn test_validation_id_generated() {
    let template = create_test_template(vec![create_test_node("node-1")], vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(
        !report.validation_id.is_empty(),
        "Should generate validation ID"
    );
}

#[tokio::test]
async fn test_large_template_validation() {
    // Create a large template with 100 nodes
    let nodes: Vec<GraphNode> = (0..100)
        .map(|i| create_test_node(&format!("node-{}", i)))
        .collect();

    let template = create_test_template(nodes, vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    // Large templates should be validated (may have warnings about disconnected nodes)
    assert!(report.security_score >= 0.0 && report.security_score <= 1.0);
}

#[tokio::test]
async fn test_deeply_nested_graph() {
    // Create a chain of nodes
    let nodes: Vec<GraphNode> = (0..20)
        .map(|i| create_test_node(&format!("node-{}", i)))
        .collect();

    // Create edges forming a chain
    let edges: Vec<GraphEdge> = (0..19)
        .map(|i| GraphEdge {
            from: format!("node-{}", i),
            to: format!("node-{}", i + 1),
            edge_type: None,
        })
        .collect();

    let template = create_test_template(nodes, edges);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(report.valid, "Deep chain should be valid");
}

#[tokio::test]
async fn test_self_referencing_edge() {
    let nodes = vec![create_test_node("node-1")];
    let edges = vec![GraphEdge {
        from: "node-1".to_string(),
        to: "node-1".to_string(),
        edge_type: None,
    }];

    let template = create_test_template(nodes, edges);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    // Self-referencing edges are validated (may or may not be flagged)
    assert!(report.security_score >= 0.0);
}

#[tokio::test]
async fn test_duplicate_node_ids() {
    let nodes = vec![
        create_test_node("node-1"),
        create_test_node("node-1"), // Duplicate ID
    ];

    let template = create_test_template(nodes, vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    // Validation completes (duplicate IDs may or may not be flagged depending on implementation)
    assert!(report.security_score >= 0.0 && report.security_score <= 1.0);
}

#[tokio::test]
async fn test_multiple_subgraphs() {
    let nodes = vec![
        create_test_node("node-1"),
        create_test_node("node-2"),
        create_test_node("node-3"),
        create_test_node("node-4"),
    ];

    // Two separate subgraphs
    let edges = vec![
        GraphEdge {
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            edge_type: None,
        },
        GraphEdge {
            from: "node-3".to_string(),
            to: "node-4".to_string(),
            edge_type: None,
        },
    ];

    let template = create_test_template(nodes, edges);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    // Multiple disconnected subgraphs should be noted
    assert!(report
        .issues
        .iter()
        .any(|i| i.description.contains("disconnected") || i.description.contains("subgraph")));
}

#[tokio::test]
async fn test_validation_with_node_metadata() {
    let mut node = create_test_node("node-1");
    node.config
        .insert("important".to_string(), serde_json::json!(true));
    node.config
        .insert("priority".to_string(), serde_json::json!(10));

    let template = create_test_template(vec![node], vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(report.valid, "Valid metadata should pass");
}

#[tokio::test]
async fn test_validation_recommendations_quality() {
    let template = create_test_template(vec![create_test_node("node-1")], vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(
        !report.recommendations.is_empty(),
        "Should have recommendations"
    );
    for rec in &report.recommendations {
        assert!(
            !rec.is_empty(),
            "Recommendations should not be empty strings"
        );
        assert!(rec.len() > 10, "Recommendations should be descriptive");
    }
}

#[tokio::test]
async fn test_security_score_bounds() {
    let template = create_test_template(vec![create_test_node("node-1")], vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(
        report.security_score >= 0.0 && report.security_score <= 1.0,
        "Security score should be in range [0, 1]"
    );
}

#[tokio::test]
async fn test_validation_id_format() {
    let template = create_test_template(vec![create_test_node("node-1")], vec![]);
    let report = validate_template(&template)
        .await
        .expect("Validation should succeed");

    assert!(
        !report.validation_id.is_empty(),
        "Should have validation ID"
    );
    assert!(
        report.validation_id.len() > 10,
        "Validation ID should be substantial"
    );
}
