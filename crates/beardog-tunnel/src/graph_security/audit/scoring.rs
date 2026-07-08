// SPDX-License-Identifier: AGPL-3.0-or-later

//! Trust scoring, risk assessment, and audit warning generation.

use crate::graph_security::types::{
    CommunityUsage, CreatorInfo, RiskLevel, SecurityAssessment, VerificationStatus,
};

/// Calculate overall trust score from creator, community, and security signals.
pub(super) fn calculate_trust_score(
    creator: &CreatorInfo,
    community: &CommunityUsage,
    security: &SecurityAssessment,
) -> f64 {
    let mut score = creator.trust_score;

    if community.deployments > 100 {
        score += 0.10;
    }

    if let Some(success_rate) = community.success_rate
        && success_rate > 0.90
    {
        score += 0.05;
    }

    if security.vulnerabilities_found > 0 {
        score -= 0.20 * (f64::from(security.vulnerabilities_found) / 10.0);
    }

    score.clamp(0.0, 1.0)
}

/// Calculate risk level for an audit.
pub(super) fn calculate_audit_risk_level(
    creator: &CreatorInfo,
    security: &SecurityAssessment,
    chain_valid: bool,
    verification_status: VerificationStatus,
) -> RiskLevel {
    if !creator.identity_verified
        || !chain_valid
        || verification_status != VerificationStatus::Verified
    {
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

/// Generate warnings and recommendations based on audit signals.
pub(super) fn generate_audit_warnings(
    creator: &CreatorInfo,
    security: &SecurityAssessment,
    chain_valid: bool,
    verification_status: VerificationStatus,
    community: &CommunityUsage,
) -> (Vec<String>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut recommendations = Vec::new();

    if !creator.identity_verified {
        warnings.push("Creator identity not verified".to_string());
        recommendations.push("Verify creator identity before using template".to_string());
    }

    match verification_status {
        VerificationStatus::Verified => {}
        VerificationStatus::Unverified => {
            warnings.push(
                "Chain of custody signatures are unverified (public keys unavailable)".to_string(),
            );
            recommendations.push(
                "Obtain verified lineage signatures through the collaboration network".to_string(),
            );
        }
        VerificationStatus::Failed => {
            warnings.push("Chain of custody verification failed".to_string());
            recommendations
                .push("Reject template until lineage signatures can be validated".to_string());
        }
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

    fn test_creator(verified: bool, trust: f64) -> CreatorInfo {
        CreatorInfo {
            user_id: "u".to_string(),
            identity_verified: verified,
            trust_score: trust,
            reputation: "ok".to_string(),
            member_since: "2025-01-01T00:00:00Z".to_string(),
            genetic_family: None,
        }
    }

    fn test_security(vulns: u32) -> SecurityAssessment {
        SecurityAssessment {
            last_scan: "2026-01-01T00:00:00Z".to_string(),
            vulnerabilities_found: vulns,
            threat_level: if vulns > 5 {
                "high"
            } else if vulns > 0 {
                "medium"
            } else {
                "none"
            }
            .to_string(),
        }
    }

    #[test]
    fn risk_level_critical_when_many_vulnerabilities() {
        assert_eq!(
            calculate_audit_risk_level(
                &test_creator(true, 0.9),
                &test_security(10),
                true,
                VerificationStatus::Verified
            ),
            RiskLevel::Critical
        );
    }

    #[test]
    fn risk_level_medium_when_some_vulnerabilities() {
        assert_eq!(
            calculate_audit_risk_level(
                &test_creator(true, 0.9),
                &test_security(2),
                true,
                VerificationStatus::Verified
            ),
            RiskLevel::Medium
        );
    }

    #[test]
    fn risk_level_high_when_unverified() {
        assert_eq!(
            calculate_audit_risk_level(
                &test_creator(true, 0.9),
                &test_security(0),
                false,
                VerificationStatus::Unverified
            ),
            RiskLevel::High
        );
    }

    #[test]
    fn trust_score_applies_vulnerability_penalty() {
        let creator = test_creator(true, 0.9);
        let community = CommunityUsage {
            deployments: 0,
            success_rate: None,
            avg_rating: None,
            total_ratings: 0,
        };
        let clean = calculate_trust_score(&creator, &community, &test_security(0));
        let dirty = calculate_trust_score(&creator, &community, &test_security(10));
        assert!(dirty < clean);
    }
}
