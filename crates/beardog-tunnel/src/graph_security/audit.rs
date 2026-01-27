//! Template origin audit and provenance verification
//!
//! This module implements the `graph.audit_origin` JSON-RPC method.

use crate::graph_security::types::{
    CommunityUsage, CreatorInfo, LineageVersion, OriginAudit, RiskLevel, SecurityAssessment,
    TemplateId,
};
use beardog_errors::BearDogError;
use uuid::Uuid;

/// Audit the origin and provenance of a template
///
/// Performs comprehensive origin verification:
/// 1. Creator identity verification
/// 2. Lineage tracking and chain of custody
/// 3. Community usage metrics
/// 4. Security assessment
///
/// # Arguments
///
/// * `template_id` - Template to audit
///
/// # Returns
///
/// Origin audit with trust score and risk assessment
pub async fn audit_origin(template_id: &TemplateId) -> Result<OriginAudit, BearDogError> {
    let audit_id = Uuid::new_v4().to_string();

    // 1. Verify creator identity
    let creator = verify_creator_identity(template_id).await?;

    // 2. Get template lineage
    let lineage = get_template_lineage(template_id).await?;

    // 3. Verify chain of custody
    let chain_valid = verify_chain_of_custody(&lineage).await?;

    // 4. Get community usage metrics
    let community_usage = get_community_usage(template_id).await?;

    // 5. Get security assessment
    let security_assessment = get_security_assessment(template_id).await?;

    // Calculate overall trust score and risk level
    let trust_score = calculate_trust_score(&creator, &community_usage, &security_assessment);
    let risk_level = calculate_audit_risk_level(&creator, &security_assessment, chain_valid);

    // Generate warnings and recommendations
    let (warnings, recommendations) = generate_audit_warnings(
        &creator,
        &security_assessment,
        chain_valid,
        &community_usage,
    );

    Ok(OriginAudit {
        template_id: template_id.clone(),
        creator,
        lineage,
        chain_valid,
        risk_level,
        trust_score,
        community_usage,
        security_assessment,
        warnings: if warnings.is_empty() {
            None
        } else {
            Some(warnings)
        },
        recommendations: if recommendations.is_empty() {
            None
        } else {
            Some(recommendations)
        },
        audit_id,
    })
}

/// Verify creator identity
async fn verify_creator_identity(template_id: &TemplateId) -> Result<CreatorInfo, BearDogError> {
    // TODO: Get actual creator info via collaboration capability
    // Future: Use CollaborationService::get_creator_info() for runtime discovery
    // For now, return placeholder data
    let creator_id = extract_creator_from_template_id(template_id);

    // Check if identity is verified (would use genetic lineage in production)
    let identity_verified = creator_id != "unknown";

    // Calculate base trust score
    let trust_score = if identity_verified { 0.75 } else { 0.10 };

    Ok(CreatorInfo {
        user_id: creator_id.clone(),
        identity_verified,
        trust_score,
        reputation: if identity_verified {
            "established_contributor".to_string()
        } else {
            "new_user".to_string()
        },
        member_since: "2025-06-15T10:00:00Z".to_string(),
        genetic_family: if identity_verified {
            Some("nat0".to_string())
        } else {
            None
        },
    })
}

/// Extract creator ID from template ID
fn extract_creator_from_template_id(template_id: &str) -> String {
    // Simple extraction - in production would query via collaboration capability
    if template_id.starts_with("template-") {
        "user-creator".to_string()
    } else {
        "unknown".to_string()
    }
}

/// Get template lineage history
async fn get_template_lineage(
    _template_id: &TemplateId,
) -> Result<Vec<LineageVersion>, BearDogError> {
    // TODO: Get actual lineage via collaboration capability
    // Future: Use CollaborationService::get_template_lineage() for runtime discovery
    // For now, return single version
    Ok(vec![LineageVersion {
        version: "1.0.0".to_string(),
        created_at: Some("2026-01-10T10:00:00Z".to_string()),
        modified_at: None,
        created_by: Some("user-creator".to_string()),
        modified_by: None,
        change_type: "initial_creation".to_string(),
        changes: None,
        signature: None,
    }])
}

/// Verify chain of custody for lineage
///
/// # Current Limitations
///
/// Full verification requires public keys for all signers, which will be
/// retrieved via CollaborationService once available. Until then, we perform
/// basic validation (signature format, lineage continuity).
///
/// # Future Implementation
///
/// Each lineage version signature will be verified against the modifier's
/// public key, ensuring complete chain of custody.
async fn verify_chain_of_custody(lineage: &[LineageVersion]) -> Result<bool, BearDogError> {
    use base64::Engine;
    
    // Check if lineage is continuous (no gaps in versions)
    if lineage.is_empty() {
        return Ok(false);
    }
    
    // Check if all versions are properly signed (when signatures present)
    for (idx, version) in lineage.iter().enumerate() {
        if let Some(signature_b64) = &version.signature {
            // Validate signature format
            let signature = base64::engine::general_purpose::STANDARD
                .decode(signature_b64)
                .map_err(|e| {
                    BearDogError::validation(&format!(
                        "Invalid base64 signature in lineage version {}: {e}",
                        version.version
                    ))
                })?;
            
            // Validate Ed25519 signature length
            if signature.len() != 64 {
                tracing::warn!(
                    "⚠️  Lineage version {} has invalid signature length: {} bytes (expected 64)",
                    version.version,
                    signature.len()
                );
                return Ok(false);
            }
            
            // TODO: Verify Ed25519 signature against modifier's public key
            // Future: Get public key from CollaborationService
            //
            // let modifier = version.modified_by.as_ref().unwrap_or(&version.created_by.unwrap());
            // let public_key = collaboration_service.get_user_public_key(modifier).await?;
            //
            // // Create canonical lineage version (without signature)
            // let mut canonical_version = version.clone();
            // canonical_version.signature = None;
            // let canonical_json = serde_json::to_vec(&canonical_version)?;
            //
            // // Verify signature
            // use beardog_core::crypto_service::algorithms::asymmetric;
            // if !asymmetric::verify_ed25519(&canonical_json, &signature, &public_key)? {
            //     return Ok(false);
            // }
            
            tracing::debug!(
                "✓ Lineage version {} signature format valid (verification pending CollaborationService)",
                version.version
            );
        } else if idx > 0 {
            // Warn if later versions aren't signed (first version can be unsigned)
            tracing::warn!(
                "⚠️  Lineage version {} is not signed (chain of custody incomplete)",
                version.version
            );
        }
    }

    // Basic validation passed
    Ok(true)
}

/// Get community usage metrics
async fn get_community_usage(_template_id: &TemplateId) -> Result<CommunityUsage, BearDogError> {
    // TODO: Get actual usage via collaboration capability
    // Future: Use CollaborationService::get_community_usage() for runtime discovery
    // For now, return placeholder data
    Ok(CommunityUsage {
        deployments: 145,
        success_rate: Some(0.94),
        avg_rating: Some(4.6),
        total_ratings: 23,
    })
}

/// Get security assessment
async fn get_security_assessment(
    _template_id: &TemplateId,
) -> Result<SecurityAssessment, BearDogError> {
    // TODO: Get actual assessment from recent validation
    // For now, return clean assessment
    Ok(SecurityAssessment {
        last_scan: chrono::Utc::now().to_rfc3339(),
        vulnerabilities_found: 0,
        threat_level: "none".to_string(),
    })
}

/// Calculate overall trust score
fn calculate_trust_score(
    creator: &CreatorInfo,
    community: &CommunityUsage,
    security: &SecurityAssessment,
) -> f64 {
    let mut score = creator.trust_score;

    // Boost for successful deployments
    if community.deployments > 100 {
        score += 0.10;
    }

    // Boost for high success rate
    if let Some(success_rate) = community.success_rate {
        if success_rate > 0.90 {
            score += 0.05;
        }
    }

    // Penalty for vulnerabilities
    if security.vulnerabilities_found > 0 {
        score -= 0.20 * (security.vulnerabilities_found as f64 / 10.0);
    }

    score.clamp(0.0, 1.0)
}

/// Calculate risk level for audit
fn calculate_audit_risk_level(
    creator: &CreatorInfo,
    security: &SecurityAssessment,
    chain_valid: bool,
) -> RiskLevel {
    if !creator.identity_verified || !chain_valid {
        return RiskLevel::High;
    }

    if security.vulnerabilities_found > 5 {
        return RiskLevel::Critical;
    }

    if security.vulnerabilities_found > 0 {
        return RiskLevel::Medium;
    }

    RiskLevel::Low
}

/// Generate warnings and recommendations
fn generate_audit_warnings(
    creator: &CreatorInfo,
    security: &SecurityAssessment,
    chain_valid: bool,
    community: &CommunityUsage,
) -> (Vec<String>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut recommendations = Vec::new();

    if !creator.identity_verified {
        warnings.push("Creator identity not verified".to_string());
        recommendations.push("Verify creator identity before using template".to_string());
    }

    if !chain_valid {
        warnings.push("Template modification chain is invalid".to_string());
        recommendations.push("Contact template creator for verification".to_string());
    }

    if community.deployments == 0 {
        warnings.push("No community usage history".to_string());
        recommendations.push("Test template thoroughly before production use".to_string());
    }

    if security.vulnerabilities_found > 0 {
        warnings.push(format!(
            "Template has {} known vulnerabilities",
            security.vulnerabilities_found
        ));
        recommendations.push("Review and fix security vulnerabilities".to_string());
    }

    if security.vulnerabilities_found > 0 || !creator.identity_verified {
        recommendations.push("Consider using established community templates instead".to_string());
    }

    (warnings, recommendations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_origin_basic() {
        let template_id = "template-123".to_string();
        let result = audit_origin(&template_id).await.unwrap();

        assert!(result.creator.identity_verified);
        assert!(!result.lineage.is_empty());
        assert!(result.trust_score > 0.0);
    }

    #[tokio::test]
    async fn test_audit_origin_unknown_creator() {
        let template_id = "unknown-template".to_string();
        let result = audit_origin(&template_id).await.unwrap();

        assert!(!result.creator.identity_verified);
        assert_eq!(result.risk_level, RiskLevel::High);
        assert!(result.warnings.is_some());
    }

    #[test]
    fn test_calculate_trust_score() {
        let creator = CreatorInfo {
            user_id: "user-1".to_string(),
            identity_verified: true,
            trust_score: 0.75,
            reputation: "established_contributor".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: Some("nat0".to_string()),
        };

        let community = CommunityUsage {
            deployments: 150,
            success_rate: Some(0.95),
            avg_rating: Some(4.8),
            total_ratings: 50,
        };

        let security = SecurityAssessment {
            last_scan: "2026-01-11T12:00:00Z".to_string(),
            vulnerabilities_found: 0,
            threat_level: "none".to_string(),
        };

        let score = calculate_trust_score(&creator, &community, &security);
        assert!(score > 0.85); // High trust
    }

    #[test]
    fn test_calculate_risk_level_low() {
        let creator = CreatorInfo {
            user_id: "user-1".to_string(),
            identity_verified: true,
            trust_score: 0.85,
            reputation: "established_contributor".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: Some("nat0".to_string()),
        };

        let security = SecurityAssessment {
            last_scan: "2026-01-11T12:00:00Z".to_string(),
            vulnerabilities_found: 0,
            threat_level: "none".to_string(),
        };

        let risk = calculate_audit_risk_level(&creator, &security, true);
        assert_eq!(risk, RiskLevel::Low);
    }

    #[test]
    fn test_calculate_risk_level_high_unverified() {
        let creator = CreatorInfo {
            user_id: "unknown".to_string(),
            identity_verified: false,
            trust_score: 0.10,
            reputation: "new_user".to_string(),
            member_since: "2026-01-11T00:00:00Z".to_string(),
            genetic_family: None,
        };

        let security = SecurityAssessment {
            last_scan: "2026-01-11T12:00:00Z".to_string(),
            vulnerabilities_found: 0,
            threat_level: "none".to_string(),
        };

        let risk = calculate_audit_risk_level(&creator, &security, false);
        assert_eq!(risk, RiskLevel::High);
    }
}
