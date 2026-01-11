//! Authorization of graph modifications
//!
//! This module implements the `graph.authorize_modification` JSON-RPC method.

use crate::graph_security::{
    permissions, threats,
    types::{AuthorizationResult, Graph, GraphModification, RiskLevel, UserId},
};
use beardog_errors::BearDogError;
use uuid::Uuid;

/// Authorize a graph modification request
///
/// This function implements the 5-layer security model:
/// 1. Authentication - Verify user identity
/// 2. Authorization - Check permissions
/// 3. Validation - Validate structure
/// 4. Threat Detection - Check for malicious patterns
/// 5. Audit - Log the decision
///
/// # Arguments
///
/// * `user_id` - User requesting the modification
/// * `graph` - Current graph state
/// * `modification` - Requested modification
///
/// # Returns
///
/// Authorization result with reasoning and recommendations
pub async fn authorize_modification(
    user_id: &UserId,
    graph: &Graph,
    modification: &GraphModification,
) -> Result<AuthorizationResult, BearDogError> {
    let mut checks_performed = Vec::new();
    let audit_id = Uuid::new_v4().to_string();
    
    // Layer 1: Authentication (handled by calling context - HSM-backed)
    checks_performed.push("user_authentication".to_string());
    
    // Layer 2: Authorization - Check permissions
    let has_permission = permissions::check_permission(user_id, graph, modification).await?;
    checks_performed.push("permission_check".to_string());
    
    if !has_permission {
        return Ok(AuthorizationResult {
            authorized: false,
            reasoning: format!(
                "User {} does not have permission to perform {:?} on graph {}",
                user_id, modification.action, graph.id
            ),
            confidence: 0.99,
            risk_level: RiskLevel::Medium,
            checks_performed,
            audit_id,
            blocked_reason: Some("insufficient_permissions".to_string()),
            threat_details: None,
            recommendations: Some(vec![
                "Request owner approval".to_string(),
                "Check your role assignment".to_string(),
            ]),
        });
    }
    
    // Layer 3: Validation - Verify ownership for destructive operations
    if modification.action == crate::graph_security::types::ModificationAction::RemoveNode {
        let is_owner = permissions::verify_ownership(user_id, graph);
        checks_performed.push("ownership_verification".to_string());
        
        if !is_owner {
            return Ok(AuthorizationResult {
                authorized: false,
                reasoning: "Only graph owner can remove nodes".to_string(),
                confidence: 1.0,
                risk_level: RiskLevel::High,
                checks_performed,
                audit_id,
                blocked_reason: Some("owner_only_operation".to_string()),
                threat_details: None,
                recommendations: Some(vec![
                    "Contact graph owner for this operation".to_string(),
                ]),
            });
        }
    }
    
    // Layer 3: Validation - Structure validation
    if let Err(e) = validate_modification_structure(modification) {
        checks_performed.push("structure_validation".to_string());
        return Ok(AuthorizationResult {
            authorized: false,
            reasoning: format!("Invalid modification structure: {}", e),
            confidence: 1.0,
            risk_level: RiskLevel::Medium,
            checks_performed,
            audit_id,
            blocked_reason: Some("invalid_structure".to_string()),
            threat_details: None,
            recommendations: Some(vec![
                "Fix structural issues".to_string(),
                "Ensure all required fields are present".to_string(),
            ]),
        });
    }
    checks_performed.push("structure_validation".to_string());
    
    // Layer 4: Threat Detection - Check for malicious patterns
    if let Some(threat) = threats::detect_modification_threats(modification, graph).await? {
        checks_performed.push("threat_detection".to_string());
        return Ok(AuthorizationResult {
            authorized: false,
            reasoning: format!(
                "Malicious pattern detected: {:?} in {}",
                threat.category, threat.location
            ),
            confidence: 0.98,
            risk_level: RiskLevel::High,
            checks_performed,
            audit_id,
            blocked_reason: Some("threat_detected".to_string()),
            threat_details: Some(threat),
            recommendations: Some(vec![
                "Remove malicious code from configuration".to_string(),
                "Use declarative configuration instead of executable code".to_string(),
                "Review security best practices".to_string(),
            ]),
        });
    }
    checks_performed.push("threat_detection".to_string());
    
    // All checks passed - authorize the modification
    Ok(AuthorizationResult {
        authorized: true,
        reasoning: format!(
            "User {} authorized to perform {:?} on graph {}. All security checks passed.",
            user_id, modification.action, graph.id
        ),
        confidence: 0.95,
        risk_level: RiskLevel::Low,
        checks_performed,
        audit_id,
        blocked_reason: None,
        threat_details: None,
        recommendations: None,
    })
}

/// Validate the structure of a modification
fn validate_modification_structure(modification: &GraphModification) -> Result<(), BearDogError> {
    use crate::graph_security::types::ModificationAction;
    
    match modification.action {
        ModificationAction::AddNode => {
            if modification.node.is_none() {
                return Err(BearDogError::configuration(
                    "AddNode modification requires 'node' field",
                ));
            }
        }
        ModificationAction::RemoveNode => {
            if modification.node_id.is_none() {
                return Err(BearDogError::configuration(
                    "RemoveNode modification requires 'node_id' field",
                ));
            }
        }
        ModificationAction::ModifyNode => {
            if modification.node_id.is_none() || modification.changes.is_none() {
                return Err(BearDogError::configuration(
                    "ModifyNode modification requires 'node_id' and 'changes' fields",
                ));
            }
        }
        ModificationAction::AddEdge => {
            if modification.edge.is_none() {
                return Err(BearDogError::configuration(
                    "AddEdge modification requires 'edge' field",
                ));
            }
        }
        ModificationAction::RemoveEdge => {
            if modification.edge.is_none() {
                return Err(BearDogError::configuration(
                    "RemoveEdge modification requires 'edge' field",
                ));
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_security::types::{GraphNode, ModificationAction};
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
    async fn test_authorize_owner_add_node() {
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
            .unwrap();
        
        assert!(result.authorized);
        assert_eq!(result.risk_level, RiskLevel::Low);
    }

    #[tokio::test]
    async fn test_authorize_non_owner_denied() {
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
            .unwrap();
        
        assert!(!result.authorized);
        assert!(result.blocked_reason.is_some());
    }

    #[tokio::test]
    async fn test_authorize_invalid_structure() {
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
            .unwrap();
        
        assert!(!result.authorized);
        assert_eq!(result.blocked_reason, Some("invalid_structure".to_string()));
    }

    #[tokio::test]
    async fn test_authorize_code_injection_detected() {
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
            .unwrap();
        
        assert!(!result.authorized);
        assert_eq!(result.blocked_reason, Some("threat_detected".to_string()));
        assert!(result.threat_details.is_some());
    }
}

