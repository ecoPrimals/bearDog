// SPDX-License-Identifier: AGPL-3.0-only

//! Architecture detection and Rust target mapping
//!
//! Provides compile-time architecture detection with zero runtime overhead.
//! Uses `std::env::consts::ARCH` for compile-time guarantees.
//!
//! # Philosophy
//! - Zero hardcoding: Uses Rust's compile-time constants
//! - Type-safe: Strong enum types, not strings
//! - Complete: All supported architectures
//! - Modern: Rust 2021 edition idioms

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Supported CPU architectures
///
/// Represents all architectures supported by biomeOS genomeBins.
/// Detection happens at compile time using `env::consts::ARCH`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    /// x86_64 (AMD64, Intel 64-bit)
    X86_64,
    /// ARM64 (AArch64, Apple Silicon, Android 64-bit)
    Aarch64,
    /// RISC-V 64-bit
    Riscv64,
    /// WebAssembly 32-bit
    Wasm32,
}

impl Architecture {
    /// Detect architecture at compile time
    ///
    /// Uses `std::env::consts::ARCH` for zero runtime overhead.
    /// Architecture is known at compile time, this just validates and converts.
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::arch::Architecture;
    ///
    /// let arch = Architecture::detect().unwrap();
    /// println!("Running on: {}", arch);
    /// ```
    ///
    /// # Errors
    /// Returns `ArchError::Unsupported` if architecture is not in supported list.
    pub fn detect() -> Result<Self, ArchError> {
        match std::env::consts::ARCH {
            "x86_64" => Ok(Self::X86_64),
            "aarch64" => Ok(Self::Aarch64),
            "riscv64" => Ok(Self::Riscv64),
            "wasm32" => Ok(Self::Wasm32),
            arch => Err(ArchError::Unsupported {
                arch: arch.to_string(),
                help: "Please report this at https://github.com/ecoPrimals/beardog/issues"
                    .to_string(),
            }),
        }
    }

    /// Convert to Rust target triple
    ///
    /// Maps architecture + OS to Rust's target triple convention.
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::arch::Architecture;
    /// use beardog_installer::platform::OperatingSystem;
    ///
    /// let arch = Architecture::X86_64;
    /// let os = OperatingSystem::Linux;
    /// assert_eq!(arch.to_rust_target(&os), "x86_64-unknown-linux-gnu");
    /// ```
    pub fn to_rust_target(&self, os: &crate::platform::OperatingSystem) -> String {
        use crate::platform::OperatingSystem::*;

        match (self, os) {
            // Linux (GNU)
            (Self::X86_64, Linux) => "x86_64-unknown-linux-gnu",
            (Self::Aarch64, Linux) => "aarch64-unknown-linux-gnu",
            (Self::Riscv64, Linux) => "riscv64gc-unknown-linux-gnu",

            // Linux (musl - static linking)
            (Self::X86_64, LinuxMusl) => "x86_64-unknown-linux-musl",
            (Self::Aarch64, LinuxMusl) => "aarch64-unknown-linux-musl",
            (Self::Riscv64, LinuxMusl) => "riscv64gc-unknown-linux-musl",

            // Android
            (Self::Aarch64, Android) => "aarch64-linux-android",
            (Self::X86_64, Android) => "x86_64-linux-android",

            // macOS (Darwin)
            (Self::X86_64, MacOS) => "x86_64-apple-darwin",
            (Self::Aarch64, MacOS) => "aarch64-apple-darwin",

            // iOS
            (Self::Aarch64, Ios) => "aarch64-apple-ios",

            // Windows (GNU toolchain)
            (Self::X86_64, Windows) => "x86_64-pc-windows-gnu",
            (Self::Aarch64, Windows) => "aarch64-pc-windows-gnullvm",

            // WebAssembly (platform-independent)
            (Self::Wasm32, _) => "wasm32-unknown-unknown",

            // Unsupported combinations
            _ => "unknown",
        }
        .to_string()
    }

    /// Binary file extension for this architecture/OS
    ///
    /// # Examples
    /// ```
    /// use beardog_installer::arch::Architecture;
    /// use beardog_installer::platform::OperatingSystem;
    ///
    /// let arch = Architecture::X86_64;
    /// assert_eq!(arch.binary_extension(&OperatingSystem::Windows), ".exe");
    /// assert_eq!(arch.binary_extension(&OperatingSystem::Linux), "");
    /// ```
    pub fn binary_extension(&self, os: &crate::platform::OperatingSystem) -> &'static str {
        match os {
            crate::platform::OperatingSystem::Windows => ".exe",
            _ => "",
        }
    }

    /// All supported architectures
    pub fn all() -> Vec<Self> {
        vec![Self::X86_64, Self::Aarch64, Self::Riscv64, Self::Wasm32]
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X86_64 => write!(f, "x86_64"),
            Self::Aarch64 => write!(f, "aarch64"),
            Self::Riscv64 => write!(f, "riscv64"),
            Self::Wasm32 => write!(f, "wasm32"),
        }
    }
}

/// Architecture detection errors
#[derive(Debug, Error)]
pub enum ArchError {
    /// Unsupported architecture
    #[error("Unsupported architecture: {arch}. {help}")]
    Unsupported {
        /// Architecture name from `std::env::consts::ARCH`
        arch: String,
        /// Help message for user
        help: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_architecture() {
        let arch = Architecture::detect();
        assert!(arch.is_ok(), "Should detect current architecture");

        // Verify it's one of the supported architectures
        let arch = arch.unwrap();
        assert!(Architecture::all().contains(&arch));
    }

    #[test]
    fn test_architecture_display() {
        assert_eq!(Architecture::X86_64.to_string(), "x86_64");
        assert_eq!(Architecture::Aarch64.to_string(), "aarch64");
        assert_eq!(Architecture::Riscv64.to_string(), "riscv64");
        assert_eq!(Architecture::Wasm32.to_string(), "wasm32");
    }

    #[test]
    fn test_rust_target_linux_gnu() {
        use crate::platform::OperatingSystem;

        let os = OperatingSystem::Linux;
        assert_eq!(
            Architecture::X86_64.to_rust_target(&os),
            "x86_64-unknown-linux-gnu"
        );
        assert_eq!(
            Architecture::Aarch64.to_rust_target(&os),
            "aarch64-unknown-linux-gnu"
        );
        assert_eq!(
            Architecture::Riscv64.to_rust_target(&os),
            "riscv64gc-unknown-linux-gnu"
        );
    }

    #[test]
    fn test_rust_target_linux_musl() {
        use crate::platform::OperatingSystem;

        let os = OperatingSystem::LinuxMusl;
        assert_eq!(
            Architecture::X86_64.to_rust_target(&os),
            "x86_64-unknown-linux-musl"
        );
        assert_eq!(
            Architecture::Aarch64.to_rust_target(&os),
            "aarch64-unknown-linux-musl"
        );
    }

    #[test]
    fn test_rust_target_android() {
        use crate::platform::OperatingSystem;

        let os = OperatingSystem::Android;
        assert_eq!(
            Architecture::Aarch64.to_rust_target(&os),
            "aarch64-linux-android"
        );
        assert_eq!(
            Architecture::X86_64.to_rust_target(&os),
            "x86_64-linux-android"
        );
    }

    #[test]
    fn test_rust_target_macos() {
        use crate::platform::OperatingSystem;

        let os = OperatingSystem::MacOS;
        assert_eq!(
            Architecture::X86_64.to_rust_target(&os),
            "x86_64-apple-darwin"
        );
        assert_eq!(
            Architecture::Aarch64.to_rust_target(&os),
            "aarch64-apple-darwin"
        );
    }

    #[test]
    fn test_rust_target_windows() {
        use crate::platform::OperatingSystem;

        let os = OperatingSystem::Windows;
        assert_eq!(
            Architecture::X86_64.to_rust_target(&os),
            "x86_64-pc-windows-gnu"
        );
    }

    #[test]
    fn test_rust_target_wasm() {
        use crate::platform::OperatingSystem;

        // WASM is platform-independent
        let os = OperatingSystem::Linux;
        assert_eq!(
            Architecture::Wasm32.to_rust_target(&os),
            "wasm32-unknown-unknown"
        );
    }

    #[test]
    fn test_binary_extension() {
        use crate::platform::OperatingSystem;

        let arch = Architecture::X86_64;
        assert_eq!(arch.binary_extension(&OperatingSystem::Windows), ".exe");
        assert_eq!(arch.binary_extension(&OperatingSystem::Linux), "");
        assert_eq!(arch.binary_extension(&OperatingSystem::MacOS), "");
        assert_eq!(arch.binary_extension(&OperatingSystem::Android), "");
    }

    #[test]
    fn test_all_architectures() {
        let all = Architecture::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&Architecture::X86_64));
        assert!(all.contains(&Architecture::Aarch64));
        assert!(all.contains(&Architecture::Riscv64));
        assert!(all.contains(&Architecture::Wasm32));
    }

    #[test]
    fn test_serialization() {
        let arch = Architecture::X86_64;
        let json = serde_json::to_string(&arch).unwrap();
        assert_eq!(json, "\"x86_64\"");

        let deserialized: Architecture = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Architecture::X86_64);
    }
}
