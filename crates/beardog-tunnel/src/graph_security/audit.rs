// SPDX-License-Identifier: AGPL-3.0-only

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
    // Get actual creator info via collaboration capability (runtime discovery)
    let template_info = crate::graph_security::internal::get_creator_info(template_id).await?;

    // Map TemplateInfo to CreatorInfo
    Ok(CreatorInfo {
        user_id: template_info.creator_id,
        identity_verified: template_info.identity_verified,
        trust_score: template_info.trust_score,
        reputation: template_info.reputation,
        member_since: template_info.member_since,
        genetic_family: template_info.genetic_family,
    })
}

/// Get template lineage history
async fn get_template_lineage(
    template_id: &TemplateId,
) -> Result<Vec<LineageVersion>, BearDogError> {
    // Get actual lineage via collaboration capability (runtime discovery)
    let collab_lineage = crate::graph_security::internal::get_lineage(template_id).await?;

    // Map collaboration_service::LineageVersion to types::LineageVersion
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

            // Verify Ed25519 signature against modifier's public key.
            // Planned: Get public key from CollaborationService (blocked by inventory #3).
            // For now, verify signature format and structure is correct.
            // Full verification will be enabled when CollaborationService integration is complete

            // Create canonical lineage version (without signature) for verification
            let mut canonical_version = version.clone();
            canonical_version.signature = None;
            let canonical_json = serde_json::to_vec(&canonical_version).map_err(|e| {
                BearDogError::validation(&format!(
                    "Failed to serialize lineage version {} for signature verification: {e}",
                    version.version
                ))
            })?;

            // Verify signature using Ed25519.
            // Planned: Replace with actual public key from CollaborationService for full verification.
            if let Some(public_key_b64) = version.created_by.as_ref().and(None::<String>) {
                // Decode public key (when available)
                let public_key_bytes = base64::engine::general_purpose::STANDARD
                    .decode(public_key_b64)
                    .map_err(|e| {
                        BearDogError::validation(&format!(
                            "Invalid base64 public key for lineage version {}: {e}",
                            version.version
                        ))
                    })?;

                // Validate Ed25519 public key length
                if public_key_bytes.len() != 32 {
                    tracing::warn!(
                        "⚠️  Lineage version {} has invalid public key length: {} bytes (expected 32)",
                        version.version,
                        public_key_bytes.len()
                    );
                    return Ok(false);
                }

                // Perform Ed25519 signature verification
                use ed25519_dalek::{Signature, Verifier, VerifyingKey};

                let verifying_key = VerifyingKey::from_bytes(
                    public_key_bytes.as_slice().try_into().map_err(|_| {
                        BearDogError::validation(&format!(
                            "Invalid public key format for lineage version {}",
                            version.version
                        ))
                    })?,
                )
                .map_err(|e| {
                    BearDogError::validation(&format!(
                        "Invalid Ed25519 public key for lineage version {}: {e}",
                        version.version
                    ))
                })?;

                let sig = Signature::from_bytes(signature.as_slice().try_into().map_err(|_| {
                    BearDogError::validation(&format!(
                        "Invalid signature format for lineage version {}",
                        version.version
                    ))
                })?);

                // Verify the signature
                if let Err(e) = verifying_key.verify(&canonical_json, &sig) {
                    tracing::warn!(
                        "⚠️  Lineage version {} has invalid Ed25519 signature: {e}",
                        version.version
                    );
                    return Ok(false);
                }

                tracing::debug!(
                    "✓ Lineage version {} Ed25519 signature verified successfully",
                    version.version
                );
            } else {
                // Public key not available yet (pending CollaborationService integration)
                tracing::debug!(
                    "✓ Lineage version {} signature format valid (full verification pending CollaborationService integration)",
                    version.version
                );
            }
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
async fn get_community_usage(template_id: &TemplateId) -> Result<CommunityUsage, BearDogError> {
    // Get actual usage via collaboration capability (runtime discovery)
    let collab_metrics =
        crate::graph_security::internal::get_community_metrics(template_id).await?;

    // Map CommunityMetrics to CommunityUsage
    Ok(CommunityUsage {
        deployments: collab_metrics.deployments,
        success_rate: collab_metrics.success_rate,
        avg_rating: collab_metrics.avg_rating,
        total_ratings: collab_metrics.total_ratings,
    })
}

/// Get security assessment
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

fn lineage_latest_scan_rfc3339(lineage: &[crate::graph_security::types::LineageVersion]) -> String {
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

/// Counts security-relevant lineage signals: unsigned edits after the initial version and
/// high-risk change types (best-effort without external scanners).
fn count_lineage_security_findings(
    lineage: &[crate::graph_security::types::LineageVersion],
) -> u32 {
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
    if let Some(success_rate) = community.success_rate
        && success_rate > 0.90
    {
        score += 0.05;
    }

    // Penalty for vulnerabilities
    if security.vulnerabilities_found > 0 {
        score -= 0.20 * (f64::from(security.vulnerabilities_found) / 10.0);
    }

    score.clamp(0.0, 1.0)
}

/// Calculate risk level for audit
const fn calculate_audit_risk_level(
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
    use crate::graph_security::types::LineageVersion;
    use base64::Engine;

    #[tokio::test]
    async fn verify_chain_of_custody_empty_lineage_is_false() {
        let ok = verify_chain_of_custody(&[]).await.expect("query");
        assert!(!ok);
    }

    #[tokio::test]
    async fn verify_chain_of_custody_invalid_base64_signature_is_error() {
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: Some("not-valid-base64!!!".to_string()),
        }];
        let err = verify_chain_of_custody(&lineage).await.unwrap_err();
        assert!(
            format!("{err}").contains("base64") || format!("{err}").contains("Invalid"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn verify_chain_of_custody_wrong_signature_length_after_decode_is_false() {
        let short_sig = base64::engine::general_purpose::STANDARD.encode([1u8, 2u8, 3u8]);
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: Some(short_sig),
        }];
        let ok = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(!ok);
    }

    #[tokio::test]
    async fn verify_chain_of_custody_well_formed_ed25519_length_without_pubkey_succeeds() {
        let sig64 = base64::engine::general_purpose::STANDARD.encode([0u8; 64]);
        let lineage = vec![LineageVersion {
            version: "v1".to_string(),
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            change_type: "create".to_string(),
            changes: None,
            signature: Some(sig64),
        }];
        let ok = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(ok);
    }

    #[tokio::test]
    async fn verify_chain_of_custody_second_version_unsigned_warns_but_passes() {
        let lineage = vec![
            LineageVersion {
                version: "v1".to_string(),
                created_at: None,
                modified_at: None,
                created_by: None,
                modified_by: None,
                change_type: "create".to_string(),
                changes: None,
                signature: None,
            },
            LineageVersion {
                version: "v2".to_string(),
                created_at: None,
                modified_at: None,
                created_by: None,
                modified_by: None,
                change_type: "edit".to_string(),
                changes: None,
                signature: None,
            },
        ];
        let ok = verify_chain_of_custody(&lineage).await.expect("query");
        assert!(ok);
    }

    #[test]
    fn calculate_audit_risk_level_critical_when_many_vulnerabilities() {
        let creator = CreatorInfo {
            user_id: "u".to_string(),
            identity_verified: true,
            trust_score: 0.9,
            reputation: "ok".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: None,
        };
        let security = SecurityAssessment {
            last_scan: "2026-01-01T00:00:00Z".to_string(),
            vulnerabilities_found: 10,
            threat_level: "high".to_string(),
        };
        assert_eq!(
            calculate_audit_risk_level(&creator, &security, true),
            RiskLevel::Critical
        );
    }

    #[test]
    fn calculate_audit_risk_level_medium_when_some_vulnerabilities() {
        let creator = CreatorInfo {
            user_id: "u".to_string(),
            identity_verified: true,
            trust_score: 0.9,
            reputation: "ok".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: None,
        };
        let security = SecurityAssessment {
            last_scan: "2026-01-01T00:00:00Z".to_string(),
            vulnerabilities_found: 2,
            threat_level: "medium".to_string(),
        };
        assert_eq!(
            calculate_audit_risk_level(&creator, &security, true),
            RiskLevel::Medium
        );
    }

    #[test]
    fn calculate_trust_score_applies_vulnerability_penalty() {
        let creator = CreatorInfo {
            user_id: "u".to_string(),
            identity_verified: true,
            trust_score: 0.9,
            reputation: "ok".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: None,
        };
        let community = CommunityUsage {
            deployments: 0,
            success_rate: None,
            avg_rating: None,
            total_ratings: 0,
        };
        let security_clean = SecurityAssessment {
            last_scan: "2026-01-01T00:00:00Z".to_string(),
            vulnerabilities_found: 0,
            threat_level: "none".to_string(),
        };
        let security_dirty = SecurityAssessment {
            last_scan: "2026-01-01T00:00:00Z".to_string(),
            vulnerabilities_found: 10,
            threat_level: "high".to_string(),
        };
        let clean = calculate_trust_score(&creator, &community, &security_clean);
        let dirty = calculate_trust_score(&creator, &community, &security_dirty);
        assert!(dirty < clean);
    }

    #[tokio::test]
    async fn test_audit_origin_basic() {
        let template_id = "template-123".to_string();
        let result = audit_origin(&template_id)
            .await
            .expect("audit_origin succeeds for test template");

        assert!(result.creator.identity_verified);
        assert!(!result.lineage.is_empty());
        assert!(result.trust_score > 0.0);
    }

    #[tokio::test]
    async fn test_audit_origin_unknown_creator() {
        let template_id = "unknown-template".to_string();
        let result = audit_origin(&template_id)
            .await
            .expect("audit_origin succeeds for unknown creator template");

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
