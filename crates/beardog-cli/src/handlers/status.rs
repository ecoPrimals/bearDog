// SPDX-License-Identifier: AGPL-3.0-only

//! Status, version, and capability summary for the CLI.

use beardog_errors::BearDogError;

/// Build information structure
pub struct BuildInfo {
    /// Crate version string (with `v` prefix via [`format_version`])
    pub version: String,
    /// Rust `TARGET` triple when available
    pub target: String,
    /// `debug` or `release`
    pub profile: String,
}

/// System information structure
pub struct SystemInfo {
    /// Operating system name (`std::env::consts::OS`)
    pub os: String,
    /// CPU architecture (`std::env::consts::ARCH`)
    pub arch: String,
}

/// Format version string with 'v' prefix
pub fn format_version(version: &str) -> String {
    format!("v{version}")
}

/// Get build information
pub fn get_build_info() -> BuildInfo {
    BuildInfo {
        version: format_version(env!("CARGO_PKG_VERSION")),
        target: std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
        profile: if cfg!(debug_assertions) {
            "debug".to_string()
        } else {
            "release".to_string()
        },
    }
}

/// Get system information
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
    }
}

/// Handle status command
pub async fn handle_status(verbose: bool) -> Result<(), BearDogError> {
    let build_info = get_build_info();
    let system_info = get_system_info();

    println!("🐻 BearDog Status");
    println!("================");
    println!();

    println!("📋 System Information:");
    println!("   Version: {}", build_info.version);
    println!("   Platform: {} ({})", system_info.os, system_info.arch);
    println!("   Build: {}", build_info.profile);
    if verbose {
        println!("   Target: {}", build_info.target);
    }
    println!();

    println!("🔐 Security Features:");
    println!("   ✅ Universal HSM Integration");
    println!("   ✅ Genetic Cryptography");
    println!("   ✅ Human Entropy Collection");
    println!("   ✅ Vendor-Agnostic Design");
    println!("   ✅ Algorithm-Agnostic Encryption");
    println!("   ✅ Transport-Agnostic Security");
    println!("   ✅ Sovereignty Compliance");
    println!();

    if verbose {
        println!("🛠️  Supported HSMs:");
        println!("   • SoftHSM2 (software)");
        println!("   • Android StrongBox (mobile)");
        println!("   • USB security token (FIDO2/CTAP2)");
        println!("   • Solo 2 (FIDO2 token)");
        println!("   • TPM (platform)");
        println!("   • Any PKCS#11 device");
        println!();

        println!("🎯 Supported Algorithms:");
        println!("   • AES-256-GCM");
        println!("   • ChaCha20-Poly1305");
        println!("   • Ed25519");
        println!("   • RSA-4096");
        println!("   • Genetic variants (adaptive)");
        println!();
    }

    println!("💡 Quick Start:");
    println!("   1. Discover HSMs: beardog hsm discover");
    println!("   2. Collect entropy: beardog entropy collect --human-input --output seed.json");
    println!("   3. Generate key: beardog key generate --key-id my-key --algorithm aes256-gcm");
    println!("   4. Encrypt file: beardog encrypt --key my-key --input data.txt --output data.enc");
    println!();

    if verbose {
        println!("📚 Documentation:");
        println!("   • User Guide: docs/USER_GUIDE_CLI.md");
        println!("   • Architecture: specs/current/integration/");
        println!("   • GitHub: https://github.com/eastgate-software/beardog");
        println!();
    }

    Ok(())
}

/// Handle version command
#[allow(dead_code)] // Used in tests, planned for CLI subcommand
pub async fn handle_version() -> Result<(), BearDogError> {
    let build_info = get_build_info();
    println!("BearDog {}", build_info.version);
    Ok(())
}

/// Legacy function for compatibility
pub async fn show_status() -> Result<(), BearDogError> {
    handle_status(true).await
}
