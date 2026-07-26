// SPDX-License-Identifier: AGPL-3.0-or-later

//! Heuristic commercial-extraction vs human-use classification for adapter unlock flow.

use super::context::RequestContext;
use beardog_errors::BearDogError;
use beardog_types::adapters::{CertificateClassification, ExtractionRisk};

/// Classifies HTTP-style request metadata as human, team, commercial, or uncertain.
///
/// This is intentionally lightweight (no network I/O). Callers may replace or augment
/// this with richer signals later without changing certificate types.
#[derive(Debug, Clone, Default)]
pub struct CommercialExtractionDetector;

impl CommercialExtractionDetector {
    /// Classify the given request context.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future classification failures.
    pub fn classify(
        &self,
        ctx: &RequestContext,
    ) -> Result<CertificateClassification, BearDogError> {
        Ok(Self::classify_sync(ctx))
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Automation percent 0–100 after round fits u8 for display"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "Scaled automation score is non-negative before percent conversion"
    )]
    fn classify_sync(ctx: &RequestContext) -> CertificateClassification {
        let automation = ctx.automation_score.clamp(0.0, 1.0);
        let pattern = ctx.pattern_consistency.clamp(0.0, 1.0);

        if automation >= 0.85 && pattern >= 0.95 && ctx.request_rate >= 200 {
            return CertificateClassification::Commercial {
                risk_level: ExtractionRisk::High,
                confidence: automation,
                automation_percent: (automation * 100.0).round() as u8,
            };
        }

        if automation >= 0.55 {
            let risk = if pattern >= 0.9 && ctx.request_rate >= 80 {
                ExtractionRisk::Medium
            } else {
                ExtractionRisk::Low
            };
            return CertificateClassification::Commercial {
                risk_level: risk,
                confidence: automation,
                automation_percent: (automation * 100.0).round() as u8,
            };
        }

        if automation < 0.35 && ctx.request_rate < 120 {
            return CertificateClassification::Human {
                confidence: (1.0 - automation).min(1.0),
            };
        }

        CertificateClassification::Uncertain {
            likely_classification: Box::new(CertificateClassification::Human { confidence: 0.75 }),
            confidence: 0.55,
        }
    }
}
