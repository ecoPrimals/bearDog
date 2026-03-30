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

    /// `StrongBox` not available on this device
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

/// Create a PHASE-2 not implemented error with context.
#[must_use]
pub const fn phase2_not_implemented(
    feature: &'static str,
    implementation_notes: &'static str,
    workaround: Option<&'static str>,
) -> AndroidError {
    AndroidError::Phase2NotImplemented {
        feature,
        phase: Phase::Two,
        tracking_issue: Some("https://github.com/ecoPrimals/beardog/issues"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_display_all_variants() {
        assert_eq!(format!("{}", Phase::One), "Phase 1");
        assert_eq!(format!("{}", Phase::Two), "Phase 2");
        assert_eq!(format!("{}", Phase::Three), "Phase 3");
    }

    #[test]
    fn phase2_not_implemented_builder_formats() {
        let err = phase2_not_implemented(
            "binder-ipc",
            "Wire up AIDL/Binder stubs.",
            Some("Use local socket until Phase 2."),
        );
        let s = err.to_string();
        assert!(s.contains("binder-ipc"), "display should name feature: {s}");
        assert!(s.contains("Phase 2"), "display should mention phase: {s}");
        let wrapped: crate::BearDogError = err.into();
        let msg = wrapped.to_string();
        assert!(
            msg.contains("binder-ipc"),
            "BearDogError should preserve context: {msg}"
        );
    }

    #[test]
    fn unsupported_platform_display_lists_alternatives() {
        let err = AndroidError::UnsupportedPlatform {
            platform: "linux-desktop".to_string(),
            feature: "strongbox",
            alternatives: vec!["software-hsm", "pkcs11"],
        };
        let s = err.to_string();
        assert!(s.contains("linux-desktop"), "{s}");
        assert!(s.contains("strongbox"), "{s}");
        assert!(s.contains("software-hsm"), "{s}");
    }

    #[test]
    fn strongbox_not_available_tee_branch() {
        let err = AndroidError::StrongBoxNotAvailable {
            manufacturer: "Acme".to_string(),
            model: "Phone X".to_string(),
            android_version: "14".to_string(),
            tee_fallback_available: true,
        };
        let s = err.to_string();
        assert!(s.contains("StrongBox"), "{s}");
        assert!(s.contains("TEE"), "{s}");
    }

    #[test]
    fn strongbox_not_available_no_tee_branch() {
        let err = AndroidError::StrongBoxNotAvailable {
            manufacturer: "Acme".to_string(),
            model: "Phone Y".to_string(),
            android_version: "13".to_string(),
            tee_fallback_available: false,
        };
        let s = err.to_string();
        assert!(s.contains("Software HSM"), "{s}");
    }

    #[test]
    fn phase2_with_tracking_and_without_workaround() {
        let err = AndroidError::Phase2NotImplemented {
            feature: "nfc-hsm",
            phase: Phase::Three,
            tracking_issue: None,
            workaround: None,
            implementation_notes: "Notes only.",
        };
        let s = err.to_string();
        assert!(s.contains("nfc-hsm"), "{s}");
        assert!(s.contains("Phase 3"), "{s}");
        assert!(s.contains("Notes only."), "{s}");
    }
}
