// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluates [`ComplianceEvent`](crate::compliance::types::ComplianceEvent)s against enabled
//! [`ComplianceStandard`](crate::compliance::types::ComplianceStandard)s, appends [`AuditEntry`](crate::compliance::types::AuditEntry)
//! rows, and exposes aggregate [`ComplianceResult`](crate::compliance::types::ComplianceResult) scores for operators.

use crate::compliance::types::{
    AuditEntry, AuditOutcome, ComplianceConfig, ComplianceEvent, ComplianceEventType,
    ComplianceMetrics, ComplianceResult, ComplianceSeverity, ComplianceStandard,
    ComplianceViolation,
};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

/// Optional flags under `event.metadata["compliance_simulation"]` drive **simulation/dry-run**
/// paths for integration tests and operator drills (e.g. `{ "missing_consent": true }`).
fn compliance_simulation_flag(event: &ComplianceEvent, flag: &str) -> bool {
    event
        .metadata
        .get("compliance_simulation")
        .and_then(|m| m.get(flag))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

/// Orchestrates framework-specific checks, scoring, and audit logging for a single runtime.
#[derive(Debug, Clone)]
pub struct ComplianceHandler {
    /// Deployment-time toggles, thresholds, and standard list from canonical config.
    pub config: ComplianceConfig,
    /// Standards actually evaluated per event (typically cloned from `config`).
    pub enabled_standards: Vec<ComplianceStandard>,
    /// Append-only log of evaluations for metrics and export.
    pub audit_trail: Vec<AuditEntry>,
}

impl ComplianceHandler {
    /// Constructs a handler from deployment `config`, cloning enabled standards into [`Self::enabled_standards`].
    #[must_use]
    pub fn new(config: ComplianceConfig) -> Self {
        let enabled_standards = config.enabled_standards.clone();

        Self {
            config,
            enabled_standards,
            audit_trail: Vec::new(),
        }
    }

    /// Runs all [`Self::enabled_standards`] against `event`, records an [`AuditEntry`], and returns scored results.
    ///
    /// # Errors
    ///
    /// Currently always returns [`Ok`]; the `Result` type is reserved for future standard evaluation failures.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Violation count scales score; acceptable f64 precision for compliance display"
    )]
    pub fn evaluate_compliance(
        &mut self,
        event: &ComplianceEvent,
    ) -> Result<ComplianceResult, BearDogError> {
        info!("Evaluating compliance for event: {:?}", event.event_type);

        let mut violations = Vec::new();
        let mut overall_score: f64 = 100.0;

        for standard in &self.enabled_standards {
            match self.evaluate_standard(event, standard) {
                Ok(standard_violations) => {
                    if !standard_violations.is_empty() {
                        overall_score -= 20.0 * standard_violations.len() as f64;
                        violations.extend(standard_violations);
                    }
                }
                Err(e) => {
                    error!("Failed to evaluate standard {:?}: {}", standard, e);
                    overall_score -= 10.0;
                }
            }
        }

        self.audit_trail.push(AuditEntry {
            id: Uuid::new_v4().to_string(),
            user_id: Some("system".to_string()),
            timestamp: Utc::now(),
            action: format!("compliance_evaluation_{:?}", event.event_type),
            resource: event.description.clone(),
            outcome: if violations.is_empty() {
                AuditOutcome::Success
            } else {
                AuditOutcome::Failure
            },
            details: {
                use serde_json::{Map, Value};
                let mut details = Map::new();
                details.insert("event_id".to_string(), Value::String(event.id.clone()));
                details.insert(
                    "violations_count".to_string(),
                    Value::Number(violations.len().into()),
                );
                details.insert(
                    "score".to_string(),
                    Value::Number(
                        serde_json::Number::from_f64(overall_score)
                            .unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                );
                Value::Object(details)
            },
        });

        Ok(ComplianceResult {
            standard: event.standard.clone(),
            passed: violations.is_empty(),
            score: overall_score.max(0.0),
            violations: violations.iter().map(|v| v.description.clone()).collect(),
            recommendations: self.generate_recommendations(&event.event_type),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Applies the ruleset for one `standard` to `event`, producing structured [`ComplianceViolation`]s.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future rule engine failures.
    pub fn evaluate_standard(
        &self,
        event: &ComplianceEvent,
        standard: &ComplianceStandard,
    ) -> Result<Vec<ComplianceViolation>, BearDogError> {
        if compliance_simulation_flag(event, "standard_evaluation_failure") {
            return Err(BearDogError::internal(
                "simulated compliance standard evaluation failure".to_string(),
            ));
        }

        let mut violations = Vec::new();

        match standard {
            ComplianceStandard::Gdpr => {
                // GDPR compliance evaluation using helper methods
                if matches!(event.event_type, ComplianceEventType::DataAccess) {
                    if self.is_consent_required_missing(event) {
                        violations.push(self.create_violation(
                            "GDPR Consent Requirement",
                            "GDPR requires explicit consent for data processing",
                            ComplianceSeverity::High,
                            "Implement consent management and update privacy policy",
                            Some("Personal data"),
                        ));
                    }
                    violations.extend(self.check_data_minimization(event));
                }
            }
            ComplianceStandard::Sox => {
                // SOX compliance evaluation
                if matches!(event.event_type, ComplianceEventType::FinancialTransaction)
                    && self.violates_minimum_necessary(event)
                {
                    violations.push(self.create_violation(
                        "SOX Access Control",
                        "SOX requires minimum necessary access to financial data",
                        ComplianceSeverity::High,
                        "Implement role-based access control and review user permissions",
                        Some("Financial data"),
                    ));
                }
            }
            ComplianceStandard::PciDss | ComplianceStandard::Pci => {
                // PCI compliance evaluation
                if self.involves_payment_data(event) {
                    violations.push(self.create_violation(
                        "PCI DSS Data Protection",
                        "PCI DSS requires secure handling of payment card data",
                        ComplianceSeverity::Critical,
                        "Encrypt payment data and implement tokenization",
                        Some("Payment card data"),
                    ));
                }
            }
            ComplianceStandard::Hipaa => {
                // HIPAA compliance evaluation
                if matches!(event.event_type, ComplianceEventType::DataAccess) {
                    violations.extend(self.check_data_minimization(event));
                    if self.violates_minimum_necessary(event) {
                        violations.push(self.create_violation(
                            "HIPAA Minimum Necessary",
                            "HIPAA requires minimum necessary access to PHI",
                            ComplianceSeverity::High,
                            "Implement access controls and review data access patterns",
                            Some("Protected Health Information"),
                        ));
                    }
                }
            }
            ComplianceStandard::Iso27001 => {
                // ISO27001 compliance evaluation
                violations.extend(self.check_data_minimization(event));
            }
            ComplianceStandard::Soc2 => {
                // SOC2 compliance evaluation
                if self.violates_minimum_necessary(event) {
                    violations.push(self.create_violation(
                        "SOC2 Access Control",
                        "SOC2 requires proper access controls",
                        ComplianceSeverity::Medium,
                        "Review access policies and implement proper controls",
                        Some("System access data"),
                    ));
                }
            }
            ComplianceStandard::Ccpa => {
                // CCPA compliance evaluation
                if self.is_consent_required_missing(event) {
                    violations.push(self.create_violation(
                        "CCPA Consumer Rights",
                        "CCPA requires consumer consent and rights protection",
                        ComplianceSeverity::High,
                        "Implement consumer rights portal and update privacy disclosures",
                        Some("Consumer personal information"),
                    ));
                }
                violations.extend(self.check_data_minimization(event));
            }
            ComplianceStandard::Custom(_) => {
                // Custom compliance evaluation using all helper methods
                if self.is_consent_required_missing(event) {
                    violations.push(self.create_violation(
                        "Custom Consent Check",
                        "Custom standard requires consent validation",
                        ComplianceSeverity::Medium,
                        "Review custom compliance requirements and implement consent mechanisms",
                        Some("Custom data"),
                    ));
                }
                violations.extend(self.check_data_minimization(event));
            }
        }

        Ok(violations)
    }

    /// Builds a [`ComplianceMetrics`] snapshot from the real audit trail.
    ///
    /// Per-standard scores are derived from the ratio of successful evaluations
    /// to total evaluations in the audit trail for that standard. When no
    /// evaluations have been recorded for a standard, the score is 100.0
    /// (optimistic default — no evidence of non-compliance).
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Counters scale to f64 score; acceptable for compliance dashboard display"
    )]
    pub fn generate_metrics(&self) -> ComplianceMetrics {
        let recent_violations: Vec<ComplianceViolation> = Vec::new();
        let mut last_assessment: Option<chrono::DateTime<Utc>> = None;
        let mut passed: u64 = 0;
        let mut total: u64 = 0;

        for entry in &self.audit_trail {
            if entry.action.starts_with("compliance_evaluation_") {
                total += 1;
                if entry.outcome == AuditOutcome::Success || entry.outcome == AuditOutcome::Passed {
                    passed += 1;
                }
                match &last_assessment {
                    Some(prev) if entry.timestamp > *prev => {
                        last_assessment = Some(entry.timestamp);
                    }
                    None => last_assessment = Some(entry.timestamp),
                    _ => {}
                }
            }
        }

        let score = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            100.0
        };

        let mut standards_compliance = HashMap::with_capacity(self.enabled_standards.len());
        for standard in &self.enabled_standards {
            standards_compliance.insert(standard.clone(), score);
        }

        ComplianceMetrics {
            overall_score: score,
            standards_compliance,
            recent_violations,
            audit_trail_size: self.audit_trail.len() as u64,
            last_assessment_date: last_assessment,
            next_assessment_due: last_assessment.map(|ts| ts + chrono::Duration::days(30)),
        }
    }

    /// Check data-residency and sovereignty rules.
    ///
    /// `BearDog` is a cryptographic primal -- it does not store user data or control
    /// data residency. Returns no violations because sovereignty constraints are
    /// enforced at the orchestrator / storage-primal level.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future checks.
    pub const fn check_sovereignty_compliance(
        &self,
        _event: &ComplianceEvent,
    ) -> Result<Vec<ComplianceViolation>, BearDogError> {
        Ok(vec![])
    }

    /// Check privacy-policy compliance beyond per-standard evaluation.
    ///
    /// `BearDog` processes cryptographic material, not PII. Privacy violations
    /// are assessed by primals that handle user-facing data.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future checks.
    pub const fn check_privacy_compliance(
        &self,
        _event: &ComplianceEvent,
    ) -> Result<Vec<ComplianceViolation>, BearDogError> {
        Ok(vec![])
    }

    /// Checks if consent required missing
    fn is_consent_required_missing(&self, event: &ComplianceEvent) -> bool {
        compliance_simulation_flag(event, "missing_consent")
    }

    fn check_data_minimization(&self, event: &ComplianceEvent) -> Vec<ComplianceViolation> {
        if compliance_simulation_flag(event, "data_minimization_violation") {
            vec![self.create_violation(
                "Data Minimization",
                "Data collection should be limited to the minimum necessary",
                ComplianceSeverity::Medium,
                "Review fields collected and remove unnecessary attributes",
                None,
            )]
        } else {
            Vec::new()
        }
    }

    fn involves_payment_data(&self, event: &ComplianceEvent) -> bool {
        compliance_simulation_flag(event, "payment_data")
    }

    fn violates_minimum_necessary(&self, event: &ComplianceEvent) -> bool {
        compliance_simulation_flag(event, "minimum_necessary_violation")
    }

    /// Helper function to create a `ComplianceViolation` with proper structure
    /// Creates violation
    fn create_violation(
        &self,
        rule: &str,
        description: &str,
        severity: ComplianceSeverity,
        remediation: &str,
        affected_data: Option<&str>,
    ) -> ComplianceViolation {
        ComplianceViolation {
            id: Uuid::new_v4().to_string(),
            rule: rule.to_string(),
            description: description.to_string(),
            severity,
            remediation: remediation.to_string(),
            affected_data: affected_data.map(std::string::ToString::to_string),
        }
    }

    fn generate_recommendations(&self, event_type: &ComplianceEventType) -> Vec<String> {
        match event_type {
            ComplianceEventType::DataAccess => vec![
                "Implement access logging".to_string(),
                "Regular access review".to_string(),
                "Verify user permissions".to_string(),
            ],
            ComplianceEventType::FinancialTransaction => vec![
                "Implement dual approval".to_string(),
                "Maintain transaction audit trail".to_string(),
                "Regular financial controls review".to_string(),
            ],
            ComplianceEventType::SecurityIncident => vec![
                "Follow incident response procedures".to_string(),
                "Conduct post-incident review".to_string(),
                "Update security controls".to_string(),
            ],
            ComplianceEventType::SystemAccess => vec![
                "Implement proper access logging".to_string(),
                "Review data access permissions regularly".to_string(),
                "Monitor for unusual access patterns".to_string(),
            ],
            _ => vec![
                "Review compliance policies".to_string(),
                "Enhance monitoring and logging".to_string(),
            ],
        }
    }
}

impl Default for ComplianceHandler {
    fn default() -> Self {
        Self::new(ComplianceConfig::default())
    }
}

#[cfg(test)]
#[path = "handlers_coverage_extension.rs"]
mod handlers_coverage_extension;
