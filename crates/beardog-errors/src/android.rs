// SPDX-License-Identifier: AGPL-3.0-only

//! Android-specific structured errors
//!
//! These errors provide clear, actionable information about Android platform limitations
//! and PHASE-2 work.

use std::fmt;

/// Android-specific errors with clear context
#[derive(Debug, Clone)]
pub enum AndroidError {
    /// Feature requires PHASE-2 Binder IPC implementation
    Phase2NotImplemented {
        /// Name of the feature
        feature: &'static str,
        /// What phase will implement it
        phase: Phase,
        /// GitHub tracking issue (if any)
        tracking_issue: Option<&'static str>,
        /// Workaround suggestion
        workaround: Option<&'static str>,
        /// Technical details for implementers
        implementation_notes: &'static str,
    },

    /// Platform not supported (non-Android)
    UnsupportedPlatform {
        /// Current platform
        platform: String,
        /// Feature that requires Android
        feature: &'static str,
        /// Alternative approaches
        alternatives: Vec<&'static str>,
    },

    /// StrongBox not available on this device
    StrongBoxNotAvailable {
        /// Device manufacturer
        manufacturer: String,
        /// Device model
        model: String,
        /// Android version
        android_version: String,
        /// Whether TEE fallback is available
        tee_fallback_available: bool,
    },
}

/// Development phase indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Phase 1: Pure Rust infrastructure (COMPLETE)
    One,
    /// Phase 2: Binder IPC implementation (FUTURE)
    Two,
    /// Phase 3: Advanced features (FUTURE)
    Three,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "Phase 1"),
            Self::Two => write!(f, "Phase 2"),
            Self::Three => write!(f, "Phase 3"),
        }
    }
}

impl fmt::Display for AndroidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Phase2NotImplemented {
                feature,
                phase,
                tracking_issue,
                workaround,
                implementation_notes,
            } => {
                writeln!(f, "❌ Feature not yet implemented: {feature}")?;
                writeln!(f)?;
                writeln!(f, "📋 Status: Planned for {phase}")?;
                if let Some(issue) = tracking_issue {
                    writeln!(f, "🔗 Tracking: {issue}")?;
                }
                writeln!(f)?;
                writeln!(f, "📝 Implementation Notes:")?;
                writeln!(f, "{implementation_notes}")?;

                if let Some(work) = workaround {
                    writeln!(f)?;
                    writeln!(f, "💡 Workaround:")?;
                    writeln!(f, "{work}")?;
                }

                Ok(())
            }
            Self::UnsupportedPlatform {
                platform,
                feature,
                alternatives,
            } => {
                writeln!(f, "❌ Platform not supported: {platform}")?;
                writeln!(f)?;
                writeln!(f, "Feature '{feature}' requires Android platform")?;
                if !alternatives.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "💡 Alternatives for {platform}:")?;
                    for alt in alternatives {
                        writeln!(f, "  • {alt}")?;
                    }
                }
                Ok(())
            }
            Self::StrongBoxNotAvailable {
                manufacturer,
                model,
                android_version,
                tee_fallback_available,
            } => {
                writeln!(f, "❌ StrongBox not available on this device")?;
                writeln!(f)?;
                writeln!(f, "Device: {manufacturer} {model}")?;
                writeln!(f, "Android: {android_version}")?;
                writeln!(f)?;
                if *tee_fallback_available {
                    writeln!(
                        f,
                        "💡 TEE (Trusted Execution Environment) fallback available"
                    )?;
                    writeln!(
                        f,
                        "   (less secure than StrongBox but still hardware-backed)"
                    )?;
                } else {
                    writeln!(f, "⚠️  No hardware-backed keystore available")?;
                    writeln!(f, "   Software HSM will be used (less secure)")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for AndroidError {}

/// Create a PHASE-2 not implemented error with context
pub fn phase2_not_implemented(
    feature: &'static str,
    implementation_notes: &'static str,
    workaround: Option<&'static str>,
) -> AndroidError {
    AndroidError::Phase2NotImplemented {
        feature,
        phase: Phase::Two,
        tracking_issue: Some("https://github.com/ecoPrimals/bearDog/issues/TBD"),
        workaround,
        implementation_notes,
    }
}

/// Converts a platform-specific [`AndroidError`] into a [`crate::BearDogError`].
///
/// The detailed Android message is kept in the resulting error string while classifying the
/// failure under the shared system error category for cross-platform callers.
impl From<AndroidError> for crate::BearDogError {
    /// Wraps `err` using [`crate::BearDogError::system`] and its `Display` text.
    fn from(err: AndroidError) -> Self {
        Self::system(err.to_string())
    }
}
