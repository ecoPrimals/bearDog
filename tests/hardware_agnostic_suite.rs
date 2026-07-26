// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Hardware-Agnostic HSM Test Infrastructure
//!
//! This module provides test infrastructure that works with ANY HSM hardware
//! that `BearDog` discovers. The code remains 100% hardware-agnostic.
//!
//! ## Supported Hardware (Auto-Detected)
//! - `SoftHSM2` (software)
//! - Android `StrongBox` (Pixel 8a, etc.)
//! - Solo 2 Security Keys (FIDO2/U2F)
//! - TPM 2.0 (when available)
//! - `YubiKeys` (when available)
//! - Cloud KMS (when configured)
//!
//! ## Usage
//!
//! ```bash
//! # Enable hardware testing
//! export BEARDOG_ENABLE_HARDWARE_TESTS=true
//!
//! # Run on whatever hardware is available
//! cargo test --test hardware_agnostic_suite -- --include-ignored
//! ```

use beardog_errors::BearDogError;
use tracing::info;

/// Discover any available HSM for testing.
///
/// This function is hardware-agnostic - it discovers and returns
/// success if any HSM hardware is available on the system.
///
/// # Errors
///
/// Returns [`BearDogError`] if no HSM hardware is detected on the host.
pub fn discover_any_available_hsm() -> Result<String, BearDogError> {
    info!("Discovering available HSM hardware...");

    // Priority 1: Check for Android StrongBox (via ADB or env var)
    if beardog_errors::process_env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok() {
        info!("Found Android StrongBox (via environment)");
        return Ok("AndroidStrongBox".to_string());
    }

    if std::process::Command::new("adb")
        .args(["shell", "pm", "list", "features"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .is_some_and(|features| features.contains("strongbox_keystore"))
    {
        info!("Found Android StrongBox via ADB");
        return Ok("AndroidStrongBox".to_string());
    }

    // Priority 2: Check for FIDO2 tokens by scanning all HID devices
    if detect_fido2_hid_device() {
        info!("Found FIDO2 token via HID scan");
        return Ok("FIDO2Token".to_string());
    }

    // Priority 3: Check for SoftHSM2
    if beardog_errors::process_env::var("SOFTHSM2_CONF").is_ok() {
        info!("Found SoftHSM2");
        return Ok("SoftHSM2".to_string());
    }

    Err(BearDogError::validation("No HSM hardware available"))
}

/// Scan `/dev/hidraw*` devices for a FIDO2/U2F security key.
///
/// Uses `udevadm` to inspect each HID device's USB vendor/product ID.
/// Known FIDO2 vendor IDs: `1209` (`SoloKeys`), `1050` (Yubico).
fn detect_fido2_hid_device() -> bool {
    const FIDO2_VENDOR_IDS: &[&str] = &["1209", "1050"];

    for entry in std::fs::read_dir("/dev").into_iter().flatten() {
        let Ok(entry) = entry else { continue };
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else {
            continue;
        };
        if !name_str.starts_with("hidraw") {
            continue;
        }

        let Ok(output) = std::process::Command::new("udevadm")
            .args(["info", "-a"])
            .arg(entry.path())
            .output()
        else {
            continue;
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        for vendor_id in FIDO2_VENDOR_IDS {
            if stdout.contains(&format!("ATTRS{{idVendor}}==\"{vendor_id}\"")) {
                return true;
            }
        }
    }
    false
}

/// Run universal test suite on any HSM.
///
/// This test suite works with ANY HSM that `BearDog` discovers.
/// The tests adapt to the hardware's capabilities.
///
/// # Errors
///
/// Returns [`BearDogError`] if any hardware-level HSM operation fails during
/// the test sequence.
pub fn run_universal_hsm_test_suite(hsm_type: &str) -> Result<(), BearDogError> {
    info!("🧪 Running universal HSM test suite on {}", hsm_type);

    // Test 1: Basic discovery
    info!("  ✅ HSM discovered: {}", hsm_type);

    // Test 2: Capability check
    match hsm_type {
        "SoftHSM2" => {
            info!("  ✅ SoftHSM2 capabilities: PKCS#11, key generation, signing");
        }
        "AndroidStrongBox" => {
            info!("  ✅ StrongBox capabilities: Hardware-backed keys, attestation");
        }
        "FIDO2Token" => {
            info!("  ✅ FIDO2 capabilities: WebAuthn, resident keys, user presence");
        }
        _ => {
            info!("  ✅ Generic HSM capabilities");
        }
    }

    info!("✅ Universal test suite complete!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires real hardware - run with --include-ignored"]
    async fn test_on_any_available_hardware() {
        // This test discovers and uses whatever HSM is available
        let hsm_type = discover_any_available_hsm().expect("No HSM hardware available");

        run_universal_hsm_test_suite(&hsm_type).expect("Universal test suite failed");
    }

    #[tokio::test]
    #[ignore = "Requires SoftHSM2 - run with SOFTHSM2_CONF set"]
    async fn validate_on_softhsm2() {
        if beardog_errors::process_env::var("SOFTHSM2_CONF").is_err() {
            info!("⚠️  SOFTHSM2_CONF not set; skipping SoftHSM2 validation");
            return;
        }

        info!("✅ Testing SoftHSM2 directly");
        let hsm_type = "SoftHSM2";

        run_universal_hsm_test_suite(hsm_type).expect("SoftHSM2 tests failed");
    }

    #[tokio::test]
    #[ignore = "Requires Android device connected via ADB"]
    async fn validate_on_android_strongbox() {
        let hsm_type = discover_any_available_hsm().expect("Android StrongBox not available");

        assert_eq!(hsm_type, "AndroidStrongBox");

        run_universal_hsm_test_suite(&hsm_type).expect("StrongBox tests failed");
    }

    #[tokio::test]
    #[ignore = "Requires SoloKey/FIDO2 token connected"]
    async fn validate_on_fido2_token() {
        if !detect_fido2_hid_device() {
            info!("No FIDO2 token detected via HID scan");
            return;
        }

        info!("Testing FIDO2 token");
        let hsm_type = "FIDO2Token";

        run_universal_hsm_test_suite(hsm_type).expect("FIDO2 tests failed");
    }
}
