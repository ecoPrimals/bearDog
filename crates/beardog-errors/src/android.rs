// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android-specific structured errors
//!
//! These errors provide clear, actionable information about Android platform limitations
//! and PHASE-2 work.

use std::fmt::{self, Write as _};

/// Android-specific errors with clear context
#[derive(Debug, Clone, thiserror::Error)]
pub enum AndroidError {
    /// Feature requires PHASE-2 Binder IPC implementation
    #[error("{}", format_phase2_not_implemented(*.phase, .feature, *.tracking_issue, *.workaround, .implementation_notes))]
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
    #[error("{}", format_unsupported_platform(.platform, .feature, .alternatives))]
    UnsupportedPlatform {
        /// Current platform
        platform: String,
        /// Feature that requires Android
        feature: &'static str,
        /// Alternative approaches
        alternatives: Vec<&'static str>,
    },

    /// `StrongBox` not available on this device
    #[error("{}", format_strongbox_not_available(.manufacturer, .model, .android_version, *.tee_fallback_available))]
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

fn format_phase2_not_implemented(
    phase: Phase,
    feature: &str,
    tracking_issue: Option<&str>,
    workaround: Option<&str>,
    implementation_notes: &str,
) -> String {
    let mut output = format!("❌ Feature not yet implemented: {feature}\n\n");
    let _ = writeln!(output, "📋 Status: Planned for {phase}");
    if let Some(issue) = tracking_issue {
        let _ = writeln!(output, "🔗 Tracking: {issue}");
    }
    output.push('\n');
    output.push_str("📝 Implementation Notes:\n");
    output.push_str(implementation_notes);
    output.push('\n');
    if let Some(work) = workaround {
        output.push('\n');
        output.push_str("💡 Workaround:\n");
        output.push_str(work);
        output.push('\n');
    }
    output
}

fn format_unsupported_platform(
    platform: &str,
    feature: &str,
    alternatives: &[&str],
) -> String {
    let mut output = format!("❌ Platform not supported: {platform}\n\n");
    let _ = writeln!(output, "Feature '{feature}' requires Android platform");
    if !alternatives.is_empty() {
        output.push('\n');
        let _ = writeln!(output, "💡 Alternatives for {platform}:");
        for alt in alternatives {
            let _ = writeln!(output, "  • {alt}");
        }
    }
    output
}

fn format_strongbox_not_available(
    manufacturer: &str,
    model: &str,
    android_version: &str,
    tee_fallback_available: bool,
) -> String {
    let mut output = String::from("❌ StrongBox not available on this device\n\n");
    let _ = writeln!(output, "Device: {manufacturer} {model}");
    let _ = writeln!(output, "Android: {android_version}\n");
    if tee_fallback_available {
        output.push_str("💡 TEE (Trusted Execution Environment) fallback available\n");
        output.push_str("   (less secure than StrongBox but still hardware-backed)\n");
    } else {
        output.push_str("⚠️  No hardware-backed keystore available\n");
        output.push_str("   Software HSM will be used (less secure)\n");
    }
    output
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
