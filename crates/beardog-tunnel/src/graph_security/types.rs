// SPDX-License-Identifier: AGPL-3.0-only

//! Common types for graph security operations
//!
//! This module defines the core data structures used across all graph security APIs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Graph identifier
pub type GraphId = String;

/// User identifier
pub type UserId = String;

/// Template identifier
pub type TemplateId = String;

/// Node identifier
pub type NodeId = String;

/// Risk level for security assessments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    /// Low risk - safe to proceed
    Low,
    /// Medium risk - caution advised
    Medium,
    /// High risk - review required
    High,
    /// Critical risk - block immediately
    Critical,
}

/// Threat category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatCategory {
    /// Structural issues (cycles, invalid refs)
    Structure,
    /// Signature verification failures
    Signature,
    /// Code injection attempts
    CodeInjection,
    /// Privilege escalation attempts
    Privilege,
    /// Anomalous patterns
    Anomaly,
    /// Resource abuse
    ResourceAbuse,
}

/// A node in the execution graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique node identifier
    pub id: NodeId,
    /// Node type (compute, storage, ai, etc.)
    #[serde(rename = "type")]
    pub node_type: String,
    /// Execution binding for this node: capability tag, discovered provider id, or opaque template reference.
    /// Resolved at runtime via capability discovery—not a compile-time ecosystem product name.
    #[serde(rename = "handler_ref", alias = "primal")]
    pub handler_ref: String,
    /// Node-specific configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// An edge connecting nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source node ID
    pub from: NodeId,
    /// Target node ID
    pub to: NodeId,
    /// Optional edge type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub edge_type: Option<String>,
}

/// Complete graph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    /// Graph identifier
    pub id: GraphId,
    /// Graph owner
    pub owner: UserId,
    /// Nodes in the graph
    pub nodes: Vec<GraphNode>,
    /// Edges connecting nodes
    pub edges: Vec<GraphEdge>,
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Type of modification action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModificationAction {
    /// Add a new node
    AddNode,
    /// Remove an existing node
    RemoveNode,
    /// Modify an existing node
    ModifyNode,
    /// Add a new edge
    AddEdge,
    /// Remove an existing edge
    RemoveEdge,
}

/// A modification to a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphModification {
    /// Type of modification
    #[serde(alias = "type")]
    pub action: ModificationAction,
    /// Node to add/modify (for node operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<GraphNode>,
    /// Node ID to remove/modify
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,
    /// Edge to add (for edge operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge: Option<GraphEdge>,
    /// Changes for modify operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<HashMap<String, serde_json::Value>>,
}

/// Authorization result for a modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    /// Whether the modification is authorized
    pub authorized: bool,
    /// Human-readable reasoning
    pub reasoning: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Risk level assessment
    pub risk_level: RiskLevel,
    /// List of checks performed
    pub checks_performed: Vec<String>,
    /// Audit trail ID
    pub audit_id: String,
    /// Optional: Reason for denial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    /// Optional: Threat details if detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_details: Option<ThreatDetails>,
    /// Optional: Recommendations for fixing issues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,
}

/// Details about a detected threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetails {
    /// Category of threat
    pub category: ThreatCategory,
    /// Location in graph (node ID, edge, etc.)
    pub location: String,
    /// Pattern or signature matched
    pub pattern: String,
}

/// A graph template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphTemplate {
    /// Template identifier
    pub id: TemplateId,
    /// Template name
    pub name: String,
    /// Template creator
    pub creator: UserId,
    /// Nodes in the template
    pub nodes: Vec<GraphNode>,
    /// Edges in the template
    pub edges: Vec<GraphEdge>,
    /// Optional Ed25519 signature (base64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Template metadata
    pub metadata: TemplateMetadata,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Template version
    pub version: String,
    /// Creation timestamp (RFC3339)
    pub created_at: String,
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Validation issue found in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Severity of the issue
    pub severity: IssueSeverity,
    /// Category of issue
    pub category: ThreatCategory,
    /// Human-readable description
    pub description: String,
    /// Optional location reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - should fix
    Medium,
    /// High severity - must fix
    High,
    /// Critical severity - blocking
    Critical,
}

/// Validation report for a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether the template is valid
    pub valid: bool,
    /// Risk level assessment
    pub risk_level: RiskLevel,
    /// List of issues found
    pub issues: Vec<ValidationIssue>,
    /// Security score (0.0 - 1.0)
    pub security_score: f64,
    /// Checks performed
    pub checks_performed: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Validation ID
    pub validation_id: String,
}

/// Creator information for origin audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// User ID
    pub user_id: UserId,
    /// Whether identity is verified
    pub identity_verified: bool,
    /// Trust score (0.0 - 1.0)
    pub trust_score: f64,
    /// Reputation level
    pub reputation: String,
    /// Member since timestamp (RFC3339)
    pub member_since: String,
    /// Optional genetic family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_family: Option<String>,
}

/// A version in the template lineage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageVersion {
    /// Version string
    pub version: String,
    /// Creation/modification timestamp (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Modification timestamp (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// Creator/modifier user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserId>,
    /// Modifier user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<UserId>,
    /// Type of change
    pub change_type: String,
    /// Optional list of changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<String>>,
    /// Optional signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Community usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityUsage {
    /// Number of deployments
    pub deployments: u64,
    /// Success rate (0.0 - 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_rate: Option<f64>,
    /// Average rating (1.0 - 5.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_rating: Option<f64>,
    /// Total number of ratings
    pub total_ratings: u64,
}

/// Security assessment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAssessment {
    /// Last scan timestamp (RFC3339)
    pub last_scan: String,
    /// Number of vulnerabilities found
    pub vulnerabilities_found: u32,
    /// Overall threat level
    pub threat_level: String,
}

/// Origin audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginAudit {
    /// Template ID
    pub template_id: TemplateId,
    /// Creator information
    pub creator: CreatorInfo,
    /// Version lineage
    pub lineage: Vec<LineageVersion>,
    /// Whether the chain of custody is valid
    pub chain_valid: bool,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Trust score (0.0 - 1.0)
    pub trust_score: f64,
    /// Community usage metrics
    pub community_usage: CommunityUsage,
    /// Security assessment
    pub security_assessment: SecurityAssessment,
    /// Optional warnings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    /// Optional recommendations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,
    /// Audit ID
    pub audit_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_level_serialization() {
        let low = RiskLevel::Low;
        let json = serde_json::to_string(&low).unwrap();
        assert_eq!(json, "\"low\"");
    }

    #[test]
    fn test_graph_node_serialization() {
        let mut config = HashMap::new();
        config.insert("cpu".to_string(), serde_json::json!(4));

        let node = GraphNode {
            id: "node-1".to_string(),
            node_type: "compute".to_string(),
            handler_ref: "compute.workload.example".to_string(),
            config,
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-1"));
        assert!(json.contains("compute"));
        assert!(json.contains("handler_ref"));
    }

    #[test]
    fn test_graph_node_accepts_legacy_primal_json_key() {
        let json = r#"{"id":"n1","type":"compute","primal":"opaque-binding-ref","config":{}}"#;
        let node: GraphNode = serde_json::from_str(json).expect("deserialize legacy key");
        assert_eq!(node.handler_ref, "opaque-binding-ref");
    }

    #[test]
    fn test_modification_action_types() {
        let actions = vec![
            ModificationAction::AddNode,
            ModificationAction::RemoveNode,
            ModificationAction::ModifyNode,
            ModificationAction::AddEdge,
            ModificationAction::RemoveEdge,
        ];

        for action in actions {
            let json = serde_json::to_string(&action).unwrap();
            assert!(!json.is_empty());
        }
    }
}
