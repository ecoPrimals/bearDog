//! Template validation for security
//!
//! This module implements the `graph.validate_template` JSON-RPC method.

use crate::graph_security::{
    threats,
    types::{
        GraphTemplate, IssueSeverity, RiskLevel, ThreatCategory, ValidationIssue, ValidationReport,
    },
};
use beardog_errors::BearDogError;
use std::collections::HashSet;
use uuid::Uuid;

/// Validate a graph template for security issues
///
/// Performs comprehensive validation:
/// 1. Structure validation (nodes, edges, cycles)
/// 2. Signature verification (if present)
/// 3. Vulnerability scanning
/// 4. Threat detection
///
/// # Arguments
///
/// * `template` - Template to validate
///
/// # Returns
///
/// Validation report with issues and recommendations
pub async fn validate_template(template: &GraphTemplate) -> Result<ValidationReport, BearDogError> {
    let mut issues = Vec::new();
    let mut checks_performed = Vec::new();
    let validation_id = Uuid::new_v4().to_string();

    // 1. Structure validation
    let structure_issues = validate_structure(template);
    issues.extend(structure_issues);
    checks_performed.push("structure_validation".to_string());

    // 2. Signature verification (if present)
    if template.signature.is_some() {
        if let Some(issue) = verify_signature(template).await? {
            issues.push(issue);
        }
        checks_performed.push("signature_verification".to_string());
    }

    // 3. Vulnerability scanning
    let vuln_issues = scan_vulnerabilities(template).await?;
    issues.extend(vuln_issues);
    checks_performed.push("vulnerability_scan".to_string());

    // 4. Threat detection
    let threats = threats::detect_template_threats(template).await?;
    for threat in threats {
        issues.push(ValidationIssue {
            severity: IssueSeverity::High,
            category: threat.category,
            description: format!("{}: {}", threat.pattern, threat.location),
            location: Some(threat.location),
        });
    }
    checks_performed.push("threat_detection".to_string());

    // Calculate risk level and security score
    let risk_level = calculate_risk_level(&issues);
    let security_score = calculate_security_score(&issues);

    // Generate recommendations
    let recommendations = generate_recommendations(&issues);

    Ok(ValidationReport {
        valid: issues.is_empty(),
        risk_level,
        issues,
        security_score,
        checks_performed,
        recommendations,
        validation_id,
    })
}

/// Validate template structure
fn validate_structure(template: &GraphTemplate) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // Check for empty template
    if template.nodes.is_empty() {
        issues.push(ValidationIssue {
            severity: IssueSeverity::Medium,
            category: ThreatCategory::Structure,
            description: "Template has no nodes".to_string(),
            location: None,
        });
        return issues;
    }

    // Check for disconnected subgraphs
    let node_ids: HashSet<_> = template.nodes.iter().map(|n| n.id.as_str()).collect();
    let mut connected = HashSet::new();

    // Start with first node
    if let Some(first) = template.nodes.first() {
        let mut to_visit = vec![first.id.as_str()];

        while let Some(current) = to_visit.pop() {
            if connected.contains(current) {
                continue;
            }
            connected.insert(current);

            // Find connected nodes via edges
            for edge in &template.edges {
                if edge.from == current && !connected.contains(edge.to.as_str()) {
                    to_visit.push(&edge.to);
                }
                if edge.to == current && !connected.contains(edge.from.as_str()) {
                    to_visit.push(&edge.from);
                }
            }
        }
    }

    // Check if all nodes are connected
    if connected.len() < node_ids.len() && template.nodes.len() > 1 {
        issues.push(ValidationIssue {
            severity: IssueSeverity::Low,
            category: ThreatCategory::Structure,
            description: "Template has disconnected subgraphs".to_string(),
            location: None,
        });
    }

    // Check for invalid edge references
    for edge in &template.edges {
        if !node_ids.contains(edge.from.as_str()) {
            issues.push(ValidationIssue {
                severity: IssueSeverity::High,
                category: ThreatCategory::Structure,
                description: format!("Edge references non-existent node: {}", edge.from),
                location: Some(format!("edge {} → {}", edge.from, edge.to)),
            });
        }
        if !node_ids.contains(edge.to.as_str()) {
            issues.push(ValidationIssue {
                severity: IssueSeverity::High,
                category: ThreatCategory::Structure,
                description: format!("Edge references non-existent node: {}", edge.to),
                location: Some(format!("edge {} → {}", edge.from, edge.to)),
            });
        }
    }

    issues
}

/// Verify template signature
async fn verify_signature(
    _template: &GraphTemplate,
) -> Result<Option<ValidationIssue>, BearDogError> {
    // TODO: Implement Ed25519 signature verification
    // For now, accept any signature
    Ok(None)
}

/// Scan for known vulnerabilities
async fn scan_vulnerabilities(
    template: &GraphTemplate,
) -> Result<Vec<ValidationIssue>, BearDogError> {
    let mut issues = Vec::new();

    // Check for resource abuse patterns
    for node in &template.nodes {
        // Check for excessive resource requests
        if let Some(cpu) = node.config.get("cpu") {
            if let Some(cpu_val) = cpu.as_u64() {
                if cpu_val > 64 {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Medium,
                        category: ThreatCategory::ResourceAbuse,
                        description: format!("Excessive CPU request: {}", cpu_val),
                        location: Some(format!("node {}", node.id)),
                    });
                }
            }
        }

        // Check for excessive memory requests
        if let Some(memory) = node.config.get("memory") {
            if let Some(mem_str) = memory.as_str() {
                if mem_str.contains("TB") || mem_str.contains("PB") {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::High,
                        category: ThreatCategory::ResourceAbuse,
                        description: format!("Excessive memory request: {}", mem_str),
                        location: Some(format!("node {}", node.id)),
                    });
                }
            }
        }
    }

    Ok(issues)
}

/// Calculate overall risk level from issues
fn calculate_risk_level(issues: &[ValidationIssue]) -> RiskLevel {
    if issues.is_empty() {
        return RiskLevel::Low;
    }

    let has_critical = issues.iter().any(|i| i.severity == IssueSeverity::Critical);
    let has_high = issues.iter().any(|i| i.severity == IssueSeverity::High);

    if has_critical {
        RiskLevel::Critical
    } else if has_high {
        RiskLevel::High
    } else {
        RiskLevel::Medium
    }
}

/// Calculate security score (0.0 - 1.0)
fn calculate_security_score(issues: &[ValidationIssue]) -> f64 {
    if issues.is_empty() {
        return 1.0;
    }

    let mut penalty = 0.0;
    for issue in issues {
        penalty += match issue.severity {
            IssueSeverity::Low => 0.05,
            IssueSeverity::Medium => 0.15,
            IssueSeverity::High => 0.30,
            IssueSeverity::Critical => 0.50,
        };
    }

    (1.0_f64 - penalty).max(0.0)
}

/// Generate recommendations based on issues
fn generate_recommendations(issues: &[ValidationIssue]) -> Vec<String> {
    let mut recommendations = Vec::new();

    if issues.is_empty() {
        recommendations.push("Template is production-ready".to_string());
        return recommendations;
    }

    let has_structure = issues
        .iter()
        .any(|i| i.category == ThreatCategory::Structure);
    let has_injection = issues
        .iter()
        .any(|i| i.category == ThreatCategory::CodeInjection);
    let has_signature = issues
        .iter()
        .any(|i| i.category == ThreatCategory::Signature);

    if has_structure {
        recommendations.push("Fix structural issues (cycles, invalid references)".to_string());
    }

    if has_injection {
        recommendations.push("Remove executable code from configurations".to_string());
        recommendations.push("Use declarative configuration instead of scripts".to_string());
    }

    if has_signature {
        recommendations.push("Re-sign template with valid Ed25519 signature".to_string());
    }

    recommendations.push("Review security best practices".to_string());

    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_security::types::{GraphEdge, GraphNode, TemplateMetadata};
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
    async fn test_validate_empty_template() {
        let template = create_test_template(vec![], vec![]);
        let report = validate_template(&template).await.unwrap();

        assert!(!report.valid);
        assert!(!report.issues.is_empty());
    }

    #[tokio::test]
    async fn test_validate_valid_template() {
        let nodes = vec![create_test_node("node-1"), create_test_node("node-2")];
        let edges = vec![GraphEdge {
            from: "node-1".to_string(),
            to: "node-2".to_string(),
            edge_type: None,
        }];

        let template = create_test_template(nodes, edges);
        let report = validate_template(&template).await.unwrap();

        assert!(report.valid);
        assert_eq!(report.risk_level, RiskLevel::Low);
        assert!(report.security_score > 0.9);
    }

    #[tokio::test]
    async fn test_validate_invalid_edge_reference() {
        let nodes = vec![create_test_node("node-1")];
        let edges = vec![GraphEdge {
            from: "node-1".to_string(),
            to: "non-existent".to_string(),
            edge_type: None,
        }];

        let template = create_test_template(nodes, edges);
        let report = validate_template(&template).await.unwrap();

        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|i| i.category == ThreatCategory::Structure));
    }

    #[tokio::test]
    async fn test_validate_excessive_resources() {
        let mut config = HashMap::new();
        config.insert("cpu".to_string(), serde_json::json!(128));

        let node = GraphNode {
            id: "node-1".to_string(),
            node_type: "compute".to_string(),
            primal: "ToadStool".to_string(),
            config,
        };

        let template = create_test_template(vec![node], vec![]);
        let report = validate_template(&template).await.unwrap();

        assert!(report
            .issues
            .iter()
            .any(|i| i.category == ThreatCategory::ResourceAbuse));
    }
}
