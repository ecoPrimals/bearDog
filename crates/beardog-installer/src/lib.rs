//! # beardog-installer
//!
//! **Universal genomeBin Installer** - Pure Rust, async, platform-agnostic
//!
//! Reference implementation of genomeBin deployment machinery.
//! Demonstrates modern idiomatic Rust for universal, isomorphic deployment.
//!
//! ## Features
//!
//! - ✅ **Pure Rust** - Zero shell scripts, zero external commands
//! - ✅ **Fully Async** - Tokio-based concurrent deployment
//! - ✅ **Universal** - Works on x86_64, ARM64, RISC-V, WASM
//! - ✅ **Platform-Agnostic** - Linux, macOS, Windows, Android, iOS
//! - ✅ **Isomorphic** - Single binary, auto-detects platform/arch
//! - ✅ **Zero Hardcoding** - XDG Base Directory compliant
//! - ✅ **Type-Safe** - Strong types, compile-time guarantees
//!
//! ## Philosophy
//!
//! Follows BearDog deep debt principles:
//! - Modern idiomatic Rust
//! - Capability-based discovery (zero hardcoding)
//! - Complete implementation (not wrappers)
//! - Async/concurrent by default
//! - Universal & agnostic

// Forbid unsafe code (100% Pure Rust)
#![forbid(unsafe_code)]
// Warn on missing docs
#![warn(missing_docs)]

pub mod arch;
pub mod deployment;
pub mod installer;
pub mod platform;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use arch::{ArchError, Architecture};
pub use deployment::{DeploymentError, DeploymentManager};
pub use installer::{BinaryInstaller, InstallerError};
pub use platform::{BiomeOSPaths, OperatingSystem, PlatformError};
pub use types::{DeploymentProgress, DeploymentReport, DeploymentStatus, Primal};
pub use validator::{BinaryValidator, ValidationError, ValidationReport};

/// Installer version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Installer name
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(NAME, "beardog-installer");
    }

    #[test]
    fn test_detect_current_platform() {
        // Should be able to detect current platform
        let arch = Architecture::detect();
        assert!(arch.is_ok());

        let os = OperatingSystem::detect();
        assert!(os.is_ok());
    }

    #[test]
    fn test_discover_paths() {
        // Should be able to discover paths on current platform
        let paths = BiomeOSPaths::discover();
        assert!(paths.is_ok());
    }
}
