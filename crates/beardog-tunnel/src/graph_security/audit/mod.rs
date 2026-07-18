// SPDX-License-Identifier: AGPL-3.0-or-later

//! Template origin audit and provenance verification.
//!
//! This module implements the `graph.audit_origin` JSON-RPC method.

mod chain_of_custody;
mod scoring;

use crate::graph_security::types::{
    CommunityUsage, CreatorInfo, LineageVersion, OriginAudit, SecurityAssessment, TemplateId,
};
use beardog_errors::BearDogError;
use uuid::Uuid;

use chain_of_custody::verify_chain_of_custody;
use scoring::{calculate_audit_risk_level, calculate_trust_score, generate_audit_warnings};

/// Audit the origin and provenance of a template.
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
/// Origin audit with trust score and risk assessment.
///
/// # Errors
///
/// Returns an error if creator identity, lineage, community metrics, or security assessment
/// cannot be fetched, or if chain-of-custody verification fails due to invalid encoding or serialization.
pub async fn audit_origin(template_id: &TemplateId) -> Result<OriginAudit, BearDogError> {
    let audit_id = Uuid::new_v4().to_string();

    let creator = verify_creator_identity(template_id).await?;
    let lineage = get_template_lineage(template_id).await?;
    let chain_verification = verify_chain_of_custody(&lineage).await?;
    let community_usage = get_community_usage(template_id).await?;
    let security_assessment = get_security_assessment(template_id).await?;

    let trust_score = calculate_trust_score(&creator, &community_usage, &security_assessment);
    let risk_level = calculate_audit_risk_level(
        &creator,
        &security_assessment,
        chain_verification.chain_valid,
        chain_verification.verification_status,
    );

    let (warnings, recommendations) = generate_audit_warnings(
        &creator,
        &security_assessment,
        chain_verification.chain_valid,
        chain_verification.verification_status,
        &community_usage,
    );

    Ok(OriginAudit {
        template_id: template_id.clone(),
        creator,
        lineage,
        chain_valid: chain_verification.chain_valid,
        verification_status: chain_verification.verification_status,
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

async fn verify_creator_identity(template_id: &TemplateId) -> Result<CreatorInfo, BearDogError> {
    let template_info = crate::graph_security::internal::get_creator_info(template_id).await?;

    Ok(CreatorInfo {
        user_id: template_info.creator_id,
        identity_verified: template_info.identity_verified,
        trust_score: template_info.trust_score,
        reputation: template_info.reputation,
        member_since: template_info.member_since,
        genetic_family: template_info.genetic_family,
    })
}

async fn get_template_lineage(
    template_id: &TemplateId,
) -> Result<Vec<LineageVersion>, BearDogError> {
    let collab_lineage = crate::graph_security::internal::get_lineage(template_id).await?;

    Ok(collab_lineage
        .into_iter()
        .map(|v| LineageVersion {
            version: v.version,
            created_at: v.created_at,
            modified_at: v.modified_at,
            created_by: v.created_by,
            modified_by: v.modified_by,
            change_type: v.change_type,
            changes: v.changes,
            signature: v.signature,
        })
        .collect())
}

async fn get_community_usage(template_id: &TemplateId) -> Result<CommunityUsage, BearDogError> {
    let collab_metrics =
        crate::graph_security::internal::get_community_metrics(template_id).await?;

    Ok(CommunityUsage {
        deployments: collab_metrics.deployments,
        success_rate: collab_metrics.success_rate,
        avg_rating: collab_metrics.avg_rating,
        total_ratings: collab_metrics.total_ratings,
    })
}

async fn get_security_assessment(
    template_id: &TemplateId,
) -> Result<SecurityAssessment, BearDogError> {
    let lineage = get_template_lineage(template_id).await?;
    let community = get_community_usage(template_id).await?;

    let last_scan = lineage_latest_scan_rfc3339(&lineage);
    let vulnerabilities_found = count_lineage_security_findings(&lineage);
    let threat_level = threat_level_from_signals(
        vulnerabilities_found,
        community.success_rate,
        community.deployments,
    );

    tracing::debug!(
        template_id = %template_id,
        vulnerabilities_found,
        threat_level = %threat_level,
        "Security assessment derived from lineage and community metrics"
    );

    Ok(SecurityAssessment {
        last_scan,
        vulnerabilities_found,
        threat_level,
    })
}

fn lineage_latest_scan_rfc3339(lineage: &[LineageVersion]) -> String {
    let mut latest = chrono::Utc::now();
    for v in lineage {
        for ts in [v.modified_at.as_deref(), v.created_at.as_deref()] {
            let Some(s) = ts else {
                continue;
            };
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
                let utc = dt.with_timezone(&chrono::Utc);
                if utc > latest {
                    latest = utc;
                }
            }
        }
    }
    latest.to_rfc3339()
}

fn count_lineage_security_findings(lineage: &[LineageVersion]) -> u32 {
    let mut n = 0u32;
    for (idx, v) in lineage.iter().enumerate() {
        if idx > 0 && v.signature.is_none() {
            n = n.saturating_add(1);
        }
        let ct = v.change_type.to_lowercase();
        if ct.contains("security") && (ct.contains("issue") || ct.contains("vuln")) {
            n = n.saturating_add(1);
        }
    }
    n
}

fn threat_level_from_signals(
    vulnerabilities_found: u32,
    success_rate: Option<f64>,
    deployments: u64,
) -> String {
    if vulnerabilities_found > 5 {
        return "critical".to_string();
    }
    if vulnerabilities_found > 0 {
        return "medium".to_string();
    }
    if let Some(rate) = success_rate
        && rate < 0.75
    {
        return "medium".to_string();
    }
    if deployments < 3 {
        return "low".to_string();
    }
    if let Some(rate) = success_rate
        && rate < 0.9
    {
        return "low".to_string();
    }
    "none".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_security::types::{RiskLevel, VerificationStatus};

    #[tokio::test]
    async fn test_audit_origin_requires_live_collaboration_network() {
        let template_id = "template-123".to_string();
        let err = audit_origin(&template_id)
            .await
            .expect_err("audit_origin should fail without collaboration network");
        assert!(
            err.to_string().contains("collaboration network")
                || err.to_string().contains("Not yet available"),
            "{err}"
        );
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
        assert!(score > 0.85);
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
        let risk =
            calculate_audit_risk_level(&creator, &security, true, VerificationStatus::Verified);
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
        let risk =
            calculate_audit_risk_level(&creator, &security, false, VerificationStatus::Unverified);
        assert_eq!(risk, RiskLevel::High);
    }
}
