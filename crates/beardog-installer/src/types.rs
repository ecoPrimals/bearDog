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

/// Primal name - capability-based identifier
///
/// A newtype wrapper around `String` that accepts any primal name at runtime.
/// Well-known constants are provided for convenience when deploying known primals.
/// Primals are discovered at runtime via capability-based discovery, not hardcoded.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrimalName(
    /// Canonical primal name string (typically lowercase, e.g. `beardog`).
    pub String,
);

impl PrimalName {
    /// BearDog genomeBin primal name.
    pub const BEARDOG: &'static str = "beardog";
    /// Songbird genomeBin primal name.
    pub const SONGBIRD: &'static str = "songbird";
    /// Squirrel genomeBin
    pub const SQUIRREL: &'static str = "squirrel";
    /// ToadStool genomeBin
    pub const TOADSTOOL: &'static str = "toadstool";
    /// NestGate genomeBin
    pub const NESTGATE: &'static str = "nestgate";

    /// Create from any string (capability-based)
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Primal name (lowercase)
    pub fn name(&self) -> &str {
        &self.0
    }

    /// Display name (proper case for well-known, else capitalized)
    pub fn display_name(&self) -> String {
        match self.0.to_lowercase().as_str() {
            Self::BEARDOG => "BearDog".to_string(),
            Self::SONGBIRD => "Songbird".to_string(),
            Self::SQUIRREL => "Squirrel".to_string(),
            Self::TOADSTOOL => "Toadstool".to_string(),
            Self::NESTGATE => "NestGate".to_string(),
            other => {
                let mut chars = other.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            }
        }
    }

    /// Well-known primals (for default "deploy all" - not exhaustive)
    pub fn well_known() -> Vec<Self> {
        vec![
            Self(Self::BEARDOG.to_string()),
            Self(Self::SONGBIRD.to_string()),
            Self(Self::SQUIRREL.to_string()),
            Self(Self::TOADSTOOL.to_string()),
            Self(Self::NESTGATE.to_string()),
        ]
    }

    /// Parse primal name from string (validates against well-known)
    pub fn parse_name(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        match s.as_str() {
            Self::BEARDOG | Self::SONGBIRD | Self::SQUIRREL | Self::TOADSTOOL | Self::NESTGATE => {
                Some(Self(s))
            }
            _ => None,
        }
    }
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
        assert_eq!(PrimalName::new(PrimalName::BEARDOG).name(), "beardog");
        assert_eq!(PrimalName::new(PrimalName::SONGBIRD).name(), "songbird");
        assert_eq!(PrimalName::new(PrimalName::SQUIRREL).name(), "squirrel");
        assert_eq!(PrimalName::new(PrimalName::TOADSTOOL).name(), "toadstool");
        assert_eq!(PrimalName::new(PrimalName::NESTGATE).name(), "nestgate");
    }

    #[test]
    fn test_primal_display_name() {
        assert_eq!(
            PrimalName::new(PrimalName::BEARDOG).display_name(),
            "BearDog"
        );
        assert_eq!(
            PrimalName::new(PrimalName::SONGBIRD).display_name(),
            "Songbird"
        );
    }

    #[test]
    fn test_primal_well_known() {
        let all = PrimalName::well_known();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&PrimalName::new(PrimalName::BEARDOG)));
        assert!(all.contains(&PrimalName::new(PrimalName::SONGBIRD)));
        assert!(all.contains(&PrimalName::new(PrimalName::SQUIRREL)));
        assert!(all.contains(&PrimalName::new(PrimalName::TOADSTOOL)));
        assert!(all.contains(&PrimalName::new(PrimalName::NESTGATE)));
    }

    #[test]
    fn test_primal_from_str() {
        assert_eq!(
            PrimalName::parse_name("beardog"),
            Some(PrimalName::new(PrimalName::BEARDOG))
        );
        assert_eq!(
            PrimalName::parse_name("BearDog"),
            Some(PrimalName::new(PrimalName::BEARDOG))
        );
        assert_eq!(
            PrimalName::parse_name("BEARDOG"),
            Some(PrimalName::new(PrimalName::BEARDOG))
        );
        assert_eq!(
            PrimalName::parse_name("songbird"),
            Some(PrimalName::new(PrimalName::SONGBIRD))
        );
        assert_eq!(PrimalName::parse_name("invalid"), None);
    }

    #[test]
    fn test_primal_display() {
        assert_eq!(PrimalName::new(PrimalName::BEARDOG).to_string(), "BearDog");
        assert_eq!(
            PrimalName::new(PrimalName::SONGBIRD).to_string(),
            "Songbird"
        );
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
        let progress = DeploymentProgress::pending(PrimalName::new(PrimalName::BEARDOG));
        assert_eq!(progress.primal, PrimalName::new(PrimalName::BEARDOG));
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
            failures: vec![(
                PrimalName::new(PrimalName::SQUIRREL),
                "test failure".to_string(),
            )],
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
        let primal = PrimalName::new(PrimalName::BEARDOG);
        let json = serde_json::to_string(&primal).expect("serialize primal");
        assert_eq!(json, "\"beardog\"");

        let deserialized: PrimalName = serde_json::from_str(&json).expect("deserialize primal");
        assert_eq!(deserialized, PrimalName::new(PrimalName::BEARDOG));
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
}
