//! Core types for genomeBin deployment
//!
//! Defines the fundamental types used throughout the installer:
//! - Primal identifiers
//! - Deployment status
//! - Progress tracking
//! - Reports

use serde::{Deserialize, Serialize};
use std::fmt;

/// Primal identifiers
///
/// Represents the five core primals in the biomeOS ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Primal {
    /// BearDog - Security foundation (HSM, crypto, trust)
    BearDog,
    /// Songbird - Discovery & federation (mDNS, federation, neural API)
    Songbird,
    /// Squirrel - AI coordination (LLM, RAG, multi-provider)
    Squirrel,
    /// Toadstool - GPU compute (barraCUDA, Flash Attention, GNN)
    Toadstool,
    /// NestGate - Storage & persistence (RocksDB, SQLite, adapters)
    NestGate,
}

impl Primal {
    /// Primal name (lowercase)
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::types::Primal;
    ///
    /// assert_eq!(Primal::BearDog.name(), "beardog");
    /// assert_eq!(Primal::Songbird.name(), "songbird");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            Self::BearDog => "beardog",
            Self::Songbird => "songbird",
            Self::Squirrel => "squirrel",
            Self::Toadstool => "toadstool",
            Self::NestGate => "nestgate",
        }
    }

    /// Display name (proper case)
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::BearDog => "BearDog",
            Self::Songbird => "Songbird",
            Self::Squirrel => "Squirrel",
            Self::Toadstool => "Toadstool",
            Self::NestGate => "NestGate",
        }
    }

    /// All primals
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::types::Primal;
    ///
    /// let all = Primal::all();
    /// assert_eq!(all.len(), 5);
    /// ```
    pub fn all() -> Vec<Self> {
        vec![
            Self::BearDog,
            Self::Songbird,
            Self::Squirrel,
            Self::Toadstool,
            Self::NestGate,
        ]
    }

    /// Parse from string
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::types::Primal;
    ///
    /// assert_eq!(Primal::from_str("beardog"), Some(Primal::BearDog));
    /// assert_eq!(Primal::from_str("invalid"), None);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "beardog" => Some(Self::BearDog),
            "songbird" => Some(Self::Songbird),
            "squirrel" => Some(Self::Squirrel),
            "toadstool" => Some(Self::Toadstool),
            "nestgate" => Some(Self::NestGate),
            _ => None,
        }
    }
}

impl fmt::Display for Primal {
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
            Self::Failed { reason } => write!(f, "Failed: {}", reason),
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
    pub primal: Primal,
    /// Current deployment status
    pub status: DeploymentStatus,
    /// Progress percentage (0-100)
    pub percent: u8,
    /// Human-readable message
    pub message: String,
}

impl DeploymentProgress {
    /// Create new progress entry
    pub fn new(primal: Primal, status: DeploymentStatus, percent: u8, message: String) -> Self {
        Self {
            primal,
            status,
            percent,
            message,
        }
    }

    /// Create pending entry
    pub fn pending(primal: Primal) -> Self {
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
    pub failures: Vec<(Primal, String)>,
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
        assert_eq!(Primal::BearDog.name(), "beardog");
        assert_eq!(Primal::Songbird.name(), "songbird");
        assert_eq!(Primal::Squirrel.name(), "squirrel");
        assert_eq!(Primal::Toadstool.name(), "toadstool");
        assert_eq!(Primal::NestGate.name(), "nestgate");
    }

    #[test]
    fn test_primal_display_name() {
        assert_eq!(Primal::BearDog.display_name(), "BearDog");
        assert_eq!(Primal::Songbird.display_name(), "Songbird");
    }

    #[test]
    fn test_primal_all() {
        let all = Primal::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&Primal::BearDog));
        assert!(all.contains(&Primal::Songbird));
        assert!(all.contains(&Primal::Squirrel));
        assert!(all.contains(&Primal::Toadstool));
        assert!(all.contains(&Primal::NestGate));
    }

    #[test]
    fn test_primal_from_str() {
        assert_eq!(Primal::from_str("beardog"), Some(Primal::BearDog));
        assert_eq!(Primal::from_str("BearDog"), Some(Primal::BearDog));
        assert_eq!(Primal::from_str("BEARDOG"), Some(Primal::BearDog));
        assert_eq!(Primal::from_str("songbird"), Some(Primal::Songbird));
        assert_eq!(Primal::from_str("invalid"), None);
    }

    #[test]
    fn test_primal_display() {
        assert_eq!(Primal::BearDog.to_string(), "BearDog");
        assert_eq!(Primal::Songbird.to_string(), "Songbird");
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
        let progress = DeploymentProgress::pending(Primal::BearDog);
        assert_eq!(progress.primal, Primal::BearDog);
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
            failures: vec![(Primal::Squirrel, "test failure".to_string())],
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
        let primal = Primal::BearDog;
        let json = serde_json::to_string(&primal).unwrap();
        assert_eq!(json, "\"beardog\"");

        let deserialized: Primal = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Primal::BearDog);
    }

    #[test]
    fn test_deployment_status_serialization() {
        let status = DeploymentStatus::Complete;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"complete\""));

        let status = DeploymentStatus::Failed {
            reason: "test".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"failed\""));
        assert!(json.contains("\"reason\":\"test\""));
    }
}
