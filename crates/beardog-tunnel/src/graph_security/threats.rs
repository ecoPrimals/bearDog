// SPDX-License-Identifier: AGPL-3.0-only

//! Threat detection for graph security
//!
//! This module detects malicious patterns and anomalies in graphs and modifications.

use crate::graph_security::types::{
    Graph, GraphModification, GraphNode, GraphTemplate, ThreatCategory, ThreatDetails,
};
use beardog_errors::BearDogError;
use std::collections::{HashMap, HashSet};

/// Detect threats in a modification
pub async fn detect_modification_threats(
    modification: &GraphModification,
    _graph: &Graph,
) -> Result<Option<ThreatDetails>, BearDogError> {
    // Check for code injection in node config
    if let Some(node) = &modification.node
        && let Some(threat) = check_code_injection(node).await?
    {
        return Ok(Some(threat));
    }

    // Check for suspicious changes
    if let Some(changes) = &modification.changes
        && let Some(threat) = check_suspicious_changes(changes).await?
    {
        return Ok(Some(threat));
    }

    Ok(None)
}

/// Detect threats in a template
pub async fn detect_template_threats(
    template: &GraphTemplate,
) -> Result<Vec<ThreatDetails>, BearDogError> {
    let mut threats = Vec::new();

    // Check each node for threats
    for node in &template.nodes {
        if let Some(threat) = check_code_injection(node).await? {
            threats.push(threat);
        }
    }

    // Check for cyclic dependencies
    if has_cycles(&template.nodes, &template.edges) {
        threats.push(ThreatDetails {
            category: ThreatCategory::Structure,
            location: "graph edges".to_string(),
            pattern: "Cyclic dependency detected".to_string(),
        });
    }

    Ok(threats)
}

/// Check for code injection in node configuration
async fn check_code_injection(node: &GraphNode) -> Result<Option<ThreatDetails>, BearDogError> {
    // Patterns that indicate code injection attempts
    let dangerous_patterns = vec![
        "eval(",
        "exec(",
        "system(",
        "__import__",
        "require(",
        "import(",
        "$(", // Shell command substitution
        "`",  // Backticks
        ";",  // Command chaining
    ];

    // Check config values for dangerous patterns
    for (key, value) in &node.config {
        if let Some(value_str) = value.as_str() {
            for pattern in &dangerous_patterns {
                if value_str.contains(pattern) {
                    return Ok(Some(ThreatDetails {
                        category: ThreatCategory::CodeInjection,
                        location: format!("node {}, config.{}", node.id, key),
                        pattern: format!("{pattern} call detected"),
                    }));
                }
            }
        }
    }

    Ok(None)
}

/// Check for suspicious changes that might indicate an attack
async fn check_suspicious_changes(
    changes: &HashMap<String, serde_json::Value>,
) -> Result<Option<ThreatDetails>, BearDogError> {
    // Check for privilege escalation attempts
    if changes.contains_key("role") || changes.contains_key("permissions") {
        return Ok(Some(ThreatDetails {
            category: ThreatCategory::Privilege,
            location: "modification changes".to_string(),
            pattern: "Privilege escalation attempt detected".to_string(),
        }));
    }

    Ok(None)
}

/// Check if a graph has cyclic dependencies
fn has_cycles(nodes: &[GraphNode], edges: &[crate::graph_security::types::GraphEdge]) -> bool {
    // Build adjacency list
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes {
        adj.insert(node.id.clone(), Vec::new());
    }

    for edge in edges {
        if let Some(neighbors) = adj.get_mut(&edge.from) {
            neighbors.push(edge.to.clone());
        }
    }

    // DFS to detect cycles
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for node in nodes {
        if !visited.contains(&node.id)
            && has_cycle_dfs(&node.id, &adj, &mut visited, &mut rec_stack)
        {
            return true;
        }
    }

    false
}

/// DFS helper to detect cycles
fn has_cycle_dfs(
    node: &str,
    adj: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
) -> bool {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());

    if let Some(neighbors) = adj.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                if has_cycle_dfs(neighbor, adj, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(neighbor) {
                return true;
            }
        }
    }

    rec_stack.remove(node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_security::types::GraphEdge;
    use std::collections::HashMap;

    fn create_test_node(id: &str, config: HashMap<String, serde_json::Value>) -> GraphNode {
        GraphNode {
            id: id.to_string(),
            node_type: "compute".to_string(),
            handler_ref: "compute.workload.example".to_string(),
            config,
        }
    }

    #[tokio::test]
    async fn test_detect_code_injection() {
        let mut config = HashMap::new();
        config.insert("command".to_string(), serde_json::json!("eval(user_input)"));

        let node = create_test_node("node-1", config);
        let result = check_code_injection(&node).await.unwrap();

        assert!(result.is_some());
        let threat = result.unwrap();
        assert_eq!(threat.category, ThreatCategory::CodeInjection);
    }

    #[tokio::test]
    async fn test_no_code_injection() {
        let mut config = HashMap::new();
        config.insert("memory".to_string(), serde_json::json!("4GB"));

        let node = create_test_node("node-1", config);
        let result = check_code_injection(&node).await.unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_has_cycles_simple() {
        let nodes = vec![
            create_test_node("node-1", HashMap::new()),
            create_test_node("node-2", HashMap::new()),
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

        assert!(has_cycles(&nodes, &edges));
    }

    #[test]
    fn test_no_cycles() {
        let nodes = vec![
            create_test_node("node-1", HashMap::new()),
            create_test_node("node-2", HashMap::new()),
            create_test_node("node-3", HashMap::new()),
        ];

        let edges = vec![
            GraphEdge {
                from: "node-1".to_string(),
                to: "node-2".to_string(),
                edge_type: None,
            },
            GraphEdge {
                from: "node-2".to_string(),
                to: "node-3".to_string(),
                edge_type: None,
            },
        ];

        assert!(!has_cycles(&nodes, &edges));
    }
}
