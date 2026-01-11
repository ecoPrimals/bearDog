//! Unit tests for origin audit

use crate::graph_security::{audit::audit_origin, types::RiskLevel};

#[tokio::test]
async fn test_audit_known_template() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(result.creator.identity_verified, "Creator should be verified");
    assert!(!result.lineage.is_empty(), "Should have lineage");
    assert!(result.trust_score > 0.0, "Should have trust score");
    assert!(result.chain_valid, "Chain should be valid");
}

#[tokio::test]
async fn test_audit_unknown_template() {
    let template_id = "unknown-template".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(!result.creator.identity_verified, "Unknown creator should not be verified");
    assert_eq!(result.risk_level, RiskLevel::High, "Unknown creator should be high risk");
    assert!(result.warnings.is_some(), "Should have warnings");
}

#[tokio::test]
async fn test_audit_includes_community_metrics() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(result.community_usage.deployments > 0, "Should have deployment count");
}

#[tokio::test]
async fn test_audit_includes_security_assessment() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(!result.security_assessment.last_scan.is_empty(), "Should have scan timestamp");
}

#[tokio::test]
async fn test_audit_generates_recommendations() {
    let template_id = "unknown-template".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(result.recommendations.is_some(), "Should have recommendations");
    let recommendations = result.recommendations.expect("Should have recommendations");
    assert!(!recommendations.is_empty(), "Recommendations should not be empty");
}

#[tokio::test]
async fn test_audit_chain_valid() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(result.chain_valid, "Known template should have valid chain");
}

#[tokio::test]
async fn test_audit_id_generated() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(!result.audit_id.is_empty(), "Should generate audit ID");
}

#[tokio::test]
async fn test_audit_trust_score_range() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(result.trust_score >= 0.0 && result.trust_score <= 1.0, "Trust score should be in range [0, 1]");
}

#[tokio::test]
async fn test_audit_creator_info_complete() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(!result.creator.user_id.is_empty(), "Should have creator user ID");
    assert!(!result.creator.reputation.is_empty(), "Should have reputation");
    assert!(!result.creator.member_since.is_empty(), "Should have member_since");
}

#[tokio::test]
async fn test_audit_security_assessment_complete() {
    let template_id = "template-123".to_string();
    let result = audit_origin(&template_id).await.expect("Audit should succeed");
    
    assert!(!result.security_assessment.last_scan.is_empty(), "Should have last scan timestamp");
    assert!(!result.security_assessment.threat_level.is_empty(), "Should have threat level");
}

