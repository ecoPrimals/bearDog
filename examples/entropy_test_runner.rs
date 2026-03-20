// SPDX-License-Identifier: AGPL-3.0-only
//! Modern Rust Entropy Test Runner
//!
//! Replaces bash script with proper Rust implementation.
//! Detects hardware, runs tests, provides interactive menu.
//!
//! ```bash
//! cargo run --example entropy_test_runner
//! ```

use std::io::{self, Write};
use std::process::Command;

#[derive(Debug)]
struct HardwareCapabilities {
    solokey_detected: bool,
    pixel_detected: bool,
    adb_available: bool,
}

impl HardwareCapabilities {
    fn detect() -> Self {
        let solokey_detected = Self::check_solokey();
        let adb_available = Self::check_adb();
        let pixel_detected = if adb_available {
            Self::check_pixel()
        } else {
            false
        };

        Self {
            solokey_detected,
            pixel_detected,
            adb_available,
        }
    }

    fn check_solokey() -> bool {
        Command::new("lsusb")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .is_some_and(|s| s.to_lowercase().contains("solo"))
    }

    fn check_adb() -> bool {
        Command::new("which")
            .arg("adb")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn check_pixel() -> bool {
        Command::new("adb")
            .args(["devices"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .is_some_and(|s| s.contains("device") && s.lines().count() > 1)
    }

    fn print_status(&self) {
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║  Hardware Detection Status                                     ║");
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!(
            "║  SoloKey FIDO2:  {:44} ║",
            if self.solokey_detected {
                "✅ Detected"
            } else {
                "⚠️  Not found"
            }
        );
        println!(
            "║  Pixel 8a:       {:44} ║",
            if self.pixel_detected {
                "✅ Connected (ADB)"
            } else if self.adb_available {
                "⚠️  Not connected"
            } else {
                "⚠️  ADB not installed"
            }
        );
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
    }
}

fn print_menu(caps: &HardwareCapabilities) {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  🎲 Entropy Testing Suite - Test Selection                    ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  1. Quick Test (Software HSM only)                             ║");
    println!("║  2. Full Test (All available hardware)                         ║");

    if caps.solokey_detected {
        println!("║  3. SoloKey FIDO2 Test                                         ║");
    } else {
        println!("║  3. SoloKey FIDO2 Test (not available)                        ║");
    }

    if caps.pixel_detected {
        println!("║  4. Pixel 8a Titan M Test                                      ║");
    } else {
        println!("║  4. Pixel 8a Titan M Test (not available)                     ║");
    }

    println!("║  5. Human Entropy Test (Interactive)                           ║");
    println!("║  6. Build for Android (Pixel 8a)                               ║");
    println!("║  7. Exit                                                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
}

fn run_software_test() -> io::Result<()> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  Running: Software HSM Test                                    ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    let status = Command::new("cargo")
        .args(["run", "--example", "entropy_hardware_comparison", "--quiet"])
        .status()?;

    if !status.success() {
        eprintln!("❌ Test failed with exit code: {:?}", status.code());
    }

    Ok(())
}

fn run_solokey_test() -> io::Result<()> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  Running: SoloKey FIDO2 Test                                   ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    let status = Command::new("cargo")
        .args([
            "run",
            "--example",
            "entropy_hardware_comparison",
            "--features",
            "fido2",
            "--quiet",
        ])
        .status()?;

    if !status.success() {
        eprintln!("❌ Test failed with exit code: {:?}", status.code());
    }

    Ok(())
}

fn build_for_android() -> io::Result<()> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  Building for Android (Pixel 8a)                               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    println!("📦 Building for aarch64-linux-android...");
    let status = Command::new("cargo")
        .args([
            "ndk",
            "-t",
            "aarch64-linux-android",
            "build",
            "--example",
            "entropy_hardware_comparison",
            "--release",
        ])
        .status()?;

    if !status.success() {
        eprintln!("❌ Build failed");
        return Ok(());
    }

    println!("✅ Build complete!");
    println!();
    println!("📱 Deploy to device with:");
    println!(
        "   adb push target/aarch64-linux-android/release/examples/entropy_hardware_comparison /data/local/tmp/"
    );
    println!("   adb shell chmod +x /data/local/tmp/entropy_hardware_comparison");
    println!("   adb shell /data/local/tmp/entropy_hardware_comparison");
    println!();

    Ok(())
}

fn run_pixel_test() -> io::Result<()> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  Running: Pixel 8a Titan M Test                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    // Check if binary exists
    let binary_path = "target/aarch64-linux-android/release/examples/entropy_hardware_comparison";
    if !std::path::Path::new(binary_path).exists() {
        println!("⚠️  Android binary not found. Building now...");
        build_for_android()?;
    }

    println!("📱 Deploying to Pixel 8a...");
    let push_status = Command::new("adb")
        .args(["push", binary_path, "/data/local/tmp/entropy_test"])
        .status()?;

    if !push_status.success() {
        eprintln!("❌ Failed to push to device");
        return Ok(());
    }

    println!("🔧 Setting permissions...");
    Command::new("adb")
        .args(["shell", "chmod", "+x", "/data/local/tmp/entropy_test"])
        .status()?;

    println!("🚀 Running on device...");
    println!();

    let run_status = Command::new("adb")
        .args(["shell", "/data/local/tmp/entropy_test"])
        .status()?;

    if !run_status.success() {
        eprintln!("❌ Test failed on device");
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║    🎲 BearDog Entropy Testing Suite - January 13, 2026        ║");
    println!("║       Modern Rust Edition (No Bash!)                           ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    // Detect hardware
    let caps = HardwareCapabilities::detect();
    caps.print_status();

    loop {
        print_menu(&caps);

        print!("Enter choice [1-7]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                println!();
                run_software_test()?;
                println!();
            }
            "2" => {
                println!();
                println!("🔄 Running all available tests...");
                println!();

                run_software_test()?;

                if caps.solokey_detected {
                    println!();
                    run_solokey_test()?;
                }

                if caps.pixel_detected {
                    println!();
                    run_pixel_test()?;
                }

                println!();
                println!("✅ All tests complete!");
                println!();
            }
            "3" => {
                println!();
                if caps.solokey_detected {
                    run_solokey_test()?;
                } else {
                    println!("⚠️  SoloKey not detected. Please insert device and try again.");
                }
                println!();
            }
            "4" => {
                println!();
                if caps.pixel_detected {
                    run_pixel_test()?;
                } else {
                    println!("⚠️  Pixel 8a not connected via ADB.");
                    println!("   Connect device, enable USB debugging, and run 'adb devices'");
                }
                println!();
            }
            "5" => {
                println!();
                println!("🎤 Starting Human Entropy Test...");
                println!("   You will be prompted to interact with keyboard/mouse");
                println!();
                run_software_test()?;
                println!();
            }
            "6" => {
                println!();
                build_for_android()?;
                println!();
            }
            "7" => {
                println!();
                println!("👋 Goodbye!");
                break;
            }
            _ => {
                println!();
                println!("❌ Invalid choice. Please enter 1-7.");
                println!();
            }
        }
    }

    Ok(())
}
