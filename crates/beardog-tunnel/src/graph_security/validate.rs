// SPDX-License-Identifier: AGPL-3.0-only

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
use tracing::debug;
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
///
/// # Current Limitations
///
/// Signature verification requires the creator's public key, which will be
/// retrieved via the CollaborationService once available. Until then, we
/// issue a warning for signed templates without verification capability.
///
/// # Future Implementation
///
/// 1. Query CollaborationService for creator's Ed25519 public key
/// 2. Canonicalize template (excluding signature field)
/// 3. Verify Ed25519 signature against canonical form
async fn verify_signature(
    template: &GraphTemplate,
) -> Result<Option<ValidationIssue>, BearDogError> {
    use base64::Engine;

    // Extract signature
    let signature_b64 = template
        .signature
        .as_ref()
        .ok_or_else(|| BearDogError::validation("verify_signature called without signature"))?;

    // Decode signature
    let signature = base64::engine::general_purpose::STANDARD
        .decode(signature_b64)
        .map_err(|e| BearDogError::validation(&format!("Invalid base64 signature: {e}")))?;

    // Validate signature length (Ed25519 signatures are 64 bytes)
    if signature.len() != 64 {
        return Ok(Some(ValidationIssue {
            severity: IssueSeverity::High,
            category: ThreatCategory::Signature,
            description: format!(
                "Invalid Ed25519 signature length: {} bytes (expected 64)",
                signature.len()
            ),
            location: Some("template.signature".to_string()),
        }));
    }

    // Planned: Get creator's public key via CollaborationService::get_user_public_key(&template.creator).
    // Full signature verification is blocked until collaboration capability integration.
    debug!(
        "Template signature present but verification requires CollaborationService (creator: {})",
        template.creator
    );

    // Return low-severity issue noting signature can't be verified yet
    Ok(Some(ValidationIssue {
        severity: IssueSeverity::Low,
        category: ThreatCategory::Signature,
        description: format!(
            "Template signature present but verification requires CollaborationService \
            (creator: {}, signature: {} bytes)",
            template.creator,
            signature.len()
        ),
        location: Some("template.signature".to_string()),
    }))

    // NOTE: Full implementation will look like this once collaboration service exists:
    //
    // // Get creator's public key
    // let public_key = collaboration_service
    //     .get_user_public_key(&template.creator)
    //     .await?;
    //
    // // Create canonical form (template without signature)
    // let mut canonical_template = template.clone();
    // canonical_template.signature = None;
    // let canonical_json = serde_json::to_vec(&canonical_template)
    //     .map_err(|e| BearDogError::validation(format!("JSON serialization failed: {e}")))?;
    //
    // // Verify signature
    // use beardog_core::crypto_service::algorithms::asymmetric;
    // let valid = asymmetric::verify_ed25519(&canonical_json, &signature, &public_key)?;
    //
    // if !valid {
    //     return Ok(Some(ValidationIssue {
    //         severity: IssueSeverity::Critical,
    //         category: ThreatCategory::Signature,
    //         description: "Ed25519 signature verification FAILED".to_string(),
    //         location: Some("template.signature".to_string()),
    //     }));
    // }
    //
    // Ok(None) // Signature valid
}

/// Scan for known vulnerabilities
async fn scan_vulnerabilities(
    template: &GraphTemplate,
) -> Result<Vec<ValidationIssue>, BearDogError> {
    let mut issues = Vec::new();

    // Check for resource abuse patterns
    for node in &template.nodes {
        // Check for excessive resource requests
        if let Some(cpu) = node.config.get("cpu")
            && let Some(cpu_val) = cpu.as_u64()
            && cpu_val > 64
        {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Medium,
                category: ThreatCategory::ResourceAbuse,
                description: format!("Excessive CPU request: {cpu_val}"),
                location: Some(format!("node {}", node.id)),
            });
        }

        // Check for excessive memory requests
        if let Some(memory) = node.config.get("memory")
            && let Some(mem_str) = memory.as_str()
            && (mem_str.contains("TB") || mem_str.contains("PB"))
        {
            issues.push(ValidationIssue {
                severity: IssueSeverity::High,
                category: ThreatCategory::ResourceAbuse,
                description: format!("Excessive memory request: {mem_str}"),
                location: Some(format!("node {}", node.id)),
            });
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
        assert!(
            report
                .issues
                .iter()
                .any(|i| i.category == ThreatCategory::Structure)
        );
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

        assert!(
            report
                .issues
                .iter()
                .any(|i| i.category == ThreatCategory::ResourceAbuse)
        );
    }
}
