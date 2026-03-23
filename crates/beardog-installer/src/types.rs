// SPDX-License-Identifier: AGPL-3.0-only

//! Core types for genomeBin deployment
//!
//! Defines the fundamental types used throughout the installer:
//! - Primal identifiers (capability-based, not fixed enum)
//! - Deployment status
//! - Progress tracking
//! - Reports

use serde::{Deserialize, Serialize};
use std::fmt;

/// Bundled default genome artifact keys (manifest/registry metadata, not runtime peer discovery).
const DEFAULT_GENOME_TARGETS_RAW: &str = include_str!("../data/default_genome_targets.txt");

/// Primal name - capability-based identifier
///
/// A newtype wrapper around `String` that accepts any genome artifact key at runtime.
/// Default bundle lists live in `DEFAULT_GENOME_TARGETS_RAW` (manifest metadata).
/// Runtime peer discovery uses capabilities, not these labels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrimalName(
    /// Canonical artifact key string (lowercase slug).
    pub String,
);

impl PrimalName {
    /// Create from any string (typically a genome manifest key).
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Primal name (lowercase)
    pub fn name(&self) -> &str {
        &self.0
    }

    /// Human-readable label (generic title-case; no special-casing by ecosystem member name).
    pub fn display_name(&self) -> String {
        title_case_slug(&self.0)
    }

    /// Default genome bundle targets from manifest data, optionally overridden by
    /// `ECOPRIMALS_GENOME_TARGETS` (comma-separated slugs).
    pub fn genome_bundle_defaults() -> Vec<Self> {
        if let Ok(s) = beardog_errors::process_env::var("ECOPRIMALS_GENOME_TARGETS") {
            let parsed: Vec<Self> = s
                .split(',')
                .filter_map(|p| Self::parse_name(p.trim()))
                .collect();
            if !parsed.is_empty() {
                return parsed;
            }
        }
        parse_manifest_lines(DEFAULT_GENOME_TARGETS_RAW)
    }

    /// Backwards-compatible alias for [`Self::genome_bundle_defaults`].
    pub fn well_known() -> Vec<Self> {
        Self::genome_bundle_defaults()
    }

    /// Parse a genome artifact slug: lowercase letters, digits, `-`, `_`.
    pub fn parse_name(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        let lower = s.to_lowercase();
        if !lower
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
        {
            return None;
        }
        if !lower
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        {
            return None;
        }
        Some(Self(lower))
    }
}

fn parse_manifest_lines(raw: &str) -> Vec<PrimalName> {
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(PrimalName::parse_name)
        .collect()
}

fn title_case_slug(s: &str) -> String {
    let parts: Vec<&str> = s.split(['-', '_']).filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        return String::new();
    }
    parts
        .into_iter()
        .map(|word| {
            let mut c = word.chars();
            let first = c.next().unwrap_or_default();
            first
                .to_uppercase()
                .chain(c.flat_map(char::to_lowercase))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl fmt::Display for PrimalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Deployment status
///
/// Tracks the current state of a primal's deployment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum DeploymentStatus {
    /// Queued, not yet started
    Pending,
    /// Locating and preparing binary
    Downloading,
    /// Copying binary to installation directory
    Installing,
    /// Validating installation (checksum, execution, health)
    Validating,
    /// Deployment completed successfully
    Complete,
    /// Deployment failed
    Failed {
        /// Error message
        reason: String,
    },
    /// Deployment was rolled back
    RolledBack,
}

impl fmt::Display for DeploymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Downloading => write!(f, "Downloading"),
            Self::Installing => write!(f, "Installing"),
            Self::Validating => write!(f, "Validating"),
            Self::Complete => write!(f, "Complete"),
            Self::Failed { reason } => write!(f, "Failed: {reason}"),
            Self::RolledBack => write!(f, "Rolled Back"),
        }
    }
}

/// Deployment progress (real-time updates)
///
/// Used for progress reporting and UI updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentProgress {
    /// Which primal this progress is for
    pub primal: PrimalName,
    /// Current deployment status
    pub status: DeploymentStatus,
    /// Progress percentage (0-100)
    pub percent: u8,
    /// Human-readable message
    pub message: String,
}

impl DeploymentProgress {
    /// Create new progress entry
    pub const fn new(
        primal: PrimalName,
        status: DeploymentStatus,
        percent: u8,
        message: String,
    ) -> Self {
        Self {
            primal,
            status,
            percent,
            message,
        }
    }

    /// Create pending entry
    pub fn pending(primal: PrimalName) -> Self {
        Self::new(
            primal,
            DeploymentStatus::Pending,
            0,
            "Queued for deployment".to_string(),
        )
    }
}

/// Deployment report (results summary)
///
/// Generated after all deployments complete (success or failure).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentReport {
    /// Total primals attempted
    pub total: usize,
    /// Number of successful deployments
    pub successes: usize,
    /// Failed deployments with reasons
    pub failures: Vec<(PrimalName, String)>,
    /// Architecture deployed to
    pub arch: crate::arch::Architecture,
    /// Operating system deployed to
    pub os: crate::platform::OperatingSystem,
}

impl DeploymentReport {
    /// Check if all deployments succeeded
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }

    /// Calculate success rate (percentage)
    #[expect(
        clippy::cast_precision_loss,
        reason = "Deployment success ratio; acceptable f64 precision for percentage"
    )]
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.successes as f64 / self.total as f64) * 100.0
        }
    }
}

impl fmt::Display for DeploymentReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Deployment Report:")?;
        writeln!(f, "  Total:     {}", self.total)?;
        writeln!(f, "  Successes: {}", self.successes)?;
        writeln!(f, "  Failures:  {}", self.failures.len())?;
        writeln!(f, "  Success Rate: {:.1}%", self.success_rate())?;
        writeln!(f, "  Architecture: {}", self.arch)?;
        writeln!(f, "  OS: {:?}", self.os)?;

        if !self.failures.is_empty() {
            writeln!(f, "\nFailures:")?;
            for (primal, reason) in &self.failures {
                writeln!(f, "  - {}: {}", primal.display_name(), reason)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primal_name() {
        assert_eq!(PrimalName::new("beardog").name(), "beardog");
        assert_eq!(PrimalName::new("songbird").name(), "songbird");
        assert_eq!(PrimalName::new("squirrel").name(), "squirrel");
        assert_eq!(PrimalName::new("toadstool").name(), "toadstool");
        assert_eq!(PrimalName::new("nestgate").name(), "nestgate");
    }

    #[test]
    fn test_primal_display_name() {
        assert_eq!(PrimalName::new("beardog").display_name(), "Beardog");
        assert_eq!(PrimalName::new("song-bird").display_name(), "Song Bird");
    }

    #[test]
    fn test_primal_well_known() {
        let all = PrimalName::well_known();
        assert!(!all.is_empty());
        assert!(all.contains(&PrimalName::new("beardog")));
    }

    #[test]
    fn test_primal_from_str() {
        assert_eq!(
            PrimalName::parse_name("beardog"),
            Some(PrimalName::new("beardog"))
        );
        assert_eq!(
            PrimalName::parse_name("BearDog"),
            Some(PrimalName::new("beardog"))
        );
        assert_eq!(
            PrimalName::parse_name("BEARDOG"),
            Some(PrimalName::new("beardog"))
        );
        assert_eq!(
            PrimalName::parse_name("songbird"),
            Some(PrimalName::new("songbird"))
        );
        assert_eq!(PrimalName::parse_name(""), None);
        assert_eq!(PrimalName::parse_name("bad name"), None);
    }

    #[test]
    fn test_primal_display() {
        assert_eq!(PrimalName::new("beardog").to_string(), "Beardog");
        assert_eq!(PrimalName::new("songbird").to_string(), "Songbird");
    }

    #[test]
    fn test_deployment_status_display() {
        assert_eq!(DeploymentStatus::Pending.to_string(), "Pending");
        assert_eq!(DeploymentStatus::Complete.to_string(), "Complete");
        assert_eq!(
            DeploymentStatus::Failed {
                reason: "test error".to_string()
            }
            .to_string(),
            "Failed: test error"
        );
    }

    #[test]
    fn test_deployment_progress() {
        let progress = DeploymentProgress::pending(PrimalName::new("beardog"));
        assert_eq!(progress.primal, PrimalName::new("beardog"));
        assert_eq!(progress.status, DeploymentStatus::Pending);
        assert_eq!(progress.percent, 0);
    }

    #[test]
    fn test_deployment_report() {
        use crate::arch::Architecture;
        use crate::platform::OperatingSystem;

        let report = DeploymentReport {
            total: 5,
            successes: 4,
            failures: vec![(PrimalName::new("squirrel"), "test failure".to_string())],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };

        assert!(!report.is_success());
        assert_eq!(report.success_rate(), 80.0);
    }

    #[test]
    fn test_deployment_report_all_success() {
        use crate::arch::Architecture;
        use crate::platform::OperatingSystem;

        let report = DeploymentReport {
            total: 5,
            successes: 5,
            failures: vec![],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };

        assert!(report.is_success());
        assert_eq!(report.success_rate(), 100.0);
    }

    #[test]
    fn test_serialization() {
        let primal = PrimalName::new("beardog");
        let json = serde_json::to_string(&primal).expect("serialize primal");
        assert_eq!(json, "\"beardog\"");

        let deserialized: PrimalName = serde_json::from_str(&json).expect("deserialize primal");
        assert_eq!(deserialized, PrimalName::new("beardog"));
    }

    #[test]
    fn test_deployment_status_serialization() {
        let status = DeploymentStatus::Complete;
        let json = serde_json::to_string(&status).expect("serialize status");
        assert!(json.contains("\"status\":\"complete\""));

        let status = DeploymentStatus::Failed {
            reason: "test".to_string(),
        };
        let json = serde_json::to_string(&status).expect("serialize failed status");
        assert!(json.contains("\"status\":\"failed\""));
        assert!(json.contains("\"reason\":\"test\""));
    }

    #[test]
    fn test_deployment_report_debug_nonempty() {
        use crate::arch::Architecture;
        use crate::platform::OperatingSystem;

        let report = DeploymentReport {
            total: 0,
            successes: 0,
            failures: vec![],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };
        let s = format!("{report:?}");
        assert!(!s.is_empty());
    }

    #[test]
    fn test_success_rate_zero_total() {
        use crate::arch::Architecture;
        use crate::platform::OperatingSystem;

        let report = DeploymentReport {
            total: 0,
            successes: 0,
            failures: vec![],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };
        assert_eq!(report.success_rate(), 0.0);
    }

    #[test]
    fn test_deployment_status_display_all_variants() {
        assert_eq!(DeploymentStatus::Downloading.to_string(), "Downloading");
        assert_eq!(DeploymentStatus::Installing.to_string(), "Installing");
        assert_eq!(DeploymentStatus::Validating.to_string(), "Validating");
        assert_eq!(DeploymentStatus::RolledBack.to_string(), "Rolled Back");
    }

    #[test]
    fn test_parse_name_rejects_leading_hyphen() {
        assert_eq!(PrimalName::parse_name("-beardog"), None);
        assert_eq!(PrimalName::parse_name("_start"), None);
    }

    #[test]
    fn test_parse_name_rejects_invalid_characters() {
        assert_eq!(PrimalName::parse_name("bad name"), None);
        assert_eq!(PrimalName::parse_name("a@b"), None);
    }

    #[test]
    fn test_primal_display_name_empty_slug() {
        assert_eq!(PrimalName::new("").display_name(), "");
    }

    #[test]
    fn test_genome_bundle_defaults_falls_back_when_env_override_has_no_valid_tokens() {
        beardog_errors::process_env::set_var("ECOPRIMALS_GENOME_TARGETS", ", , ");
        let defaults = PrimalName::genome_bundle_defaults();
        beardog_errors::process_env::remove_var("ECOPRIMALS_GENOME_TARGETS");
        assert!(
            !defaults.is_empty(),
            "should fall back to manifest when parsed override is empty"
        );
    }

    #[test]
    fn test_deployment_report_display_zero_total() {
        use crate::arch::Architecture;
        use crate::platform::OperatingSystem;

        let report = DeploymentReport {
            total: 0,
            successes: 0,
            failures: vec![],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };
        let s = format!("{report}");
        assert!(s.contains("Total:     0"));
        assert!(s.contains("0.0%"));
    }
}
