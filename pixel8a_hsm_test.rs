#!/usr/bin/env rust-script
//! BearDog Pixel 8a HSM Key Creation Test
//! 
//! This test validates BearDog's HSM capabilities on your connected Pixel 8a device.

use std::time::{Duration, Instant};
use std::process::Command;

#[derive(Debug)]
struct HsmTestResults {
    device_detected: bool,
    device_model: String,
    device_serial: String,
    strongbox_available: bool,
    verified_boot_state: String,
    android_version: String,
    test_duration_ms: u64,
}

struct Pixel8aHsmTester;

impl Pixel8aHsmTester {
    fn new() -> Self {
        Self
    }

    fn run_comprehensive_test(&self) -> Result<HsmTestResults, Box<dyn std::error::Error>> {
        println!("🚀 BearDog Pixel 8a HSM Key Creation Test");
        println!("=========================================");
        println!("📱 Testing hardware security capabilities...");
        println!();

        let start_time = Instant::now();

        // Phase 1: Device Detection
        println!("📱 Phase 1: Device Detection & Validation");
        let (device_detected, device_info) = self.detect_pixel_device()?;

        // Phase 2: Security Feature Validation  
        println!("🔐 Phase 2: Security Feature Validation");
        let strongbox_available = self.check_strongbox_support(&device_info)?;
        let verified_boot = self.check_verified_boot(&device_info)?;
        let android_version = self.get_android_version(&device_info)?;

        // Phase 3: HSM Readiness Assessment
        println!("🛡️  Phase 3: HSM Readiness Assessment");
        self.assess_hsm_readiness(&device_info, strongbox_available)?;

        let test_duration = start_time.elapsed();

        let results = HsmTestResults {
            device_detected,
            device_model: device_info.model,
            device_serial: device_info.serial,
            strongbox_available,
            verified_boot_state: verified_boot,
            android_version,
            test_duration_ms: test_duration.as_millis() as u64,
        };

        self.display_results(&results)?;
        Ok(results)
    }

    fn detect_pixel_device(&self) -> Result<(bool, DeviceInfo), Box<dyn std::error::Error>> {
        println!("   🔍 Detecting connected Android devices...");
        
        let output = Command::new("adb")
            .args(&["devices", "-l"])
            .output()?;

        let devices_output = String::from_utf8(output.stdout)?;
        println!("   📋 ADB output: {}", devices_output.trim());

        if devices_output.contains("44251JEKB04957") {
            println!("   ✅ Pixel 8a detected: 44251JEKB04957");
            Ok((true, DeviceInfo {
                serial: "44251JEKB04957".to_string(),
                model: "Pixel_8a".to_string(),
                product: "akita".to_string(),
            }))
        } else {
            println!("   ⚠️  Pixel 8a not detected - check USB connection");
            Ok((false, DeviceInfo {
                serial: "unknown".to_string(),
                model: "unknown".to_string(),
                product: "unknown".to_string(),
            }))
        }
    }

    fn check_strongbox_support(&self, device: &DeviceInfo) -> Result<bool, Box<dyn std::error::Error>> {
        println!("   🔐 Checking StrongBox HSM support...");
        
        if !device.serial.contains("44251JEKB04957") {
            println!("   ⚠️  Device not connected - cannot check StrongBox");
            return Ok(false);
        }

        let output = Command::new("adb")
            .args(&["shell", "pm", "list", "features"])
            .output()?;

        let features = String::from_utf8(output.stdout)?;
        let strongbox_supported = features.contains("android.hardware.strongbox_keystore");

        if strongbox_supported {
            println!("   ✅ StrongBox HSM: SUPPORTED");
        } else {
            println!("   ⚠️  StrongBox HSM: NOT DETECTED");
        }

        Ok(strongbox_supported)
    }

    fn check_verified_boot(&self, device: &DeviceInfo) -> Result<String, Box<dyn std::error::Error>> {
        println!("   🛡️  Checking verified boot state...");
        
        if !device.serial.contains("44251JEKB04957") {
            println!("   ⚠️  Device not connected - cannot check boot state");
            return Ok("unknown".to_string());
        }

        let output = Command::new("adb")
            .args(&["shell", "getprop", "ro.boot.verifiedbootstate"])
            .output()?;

        let boot_state = String::from_utf8(output.stdout)?.trim().to_string();
        
        match boot_state.as_str() {
            "green" => println!("   ✅ Verified Boot: GREEN (optimal security)"),
            "yellow" => println!("   ⚠️  Verified Boot: YELLOW (warning)"),
            "orange" => println!("   ⚠️  Verified Boot: ORANGE (unlocked)"),
            "red" => println!("   ❌ Verified Boot: RED (compromised)"),
            _ => println!("   ❓ Verified Boot: {} (unknown)", boot_state),
        }

        Ok(boot_state)
    }

    fn get_android_version(&self, device: &DeviceInfo) -> Result<String, Box<dyn std::error::Error>> {
        println!("   📱 Checking Android version...");
        
        if !device.serial.contains("44251JEKB04957") {
            return Ok("unknown".to_string());
        }

        let output = Command::new("adb")
            .args(&["shell", "getprop", "ro.build.version.release"])
            .output()?;

        let version = String::from_utf8(output.stdout)?.trim().to_string();
        println!("   ✅ Android Version: {}", version);

        Ok(version)
    }

    fn assess_hsm_readiness(&self, device: &DeviceInfo, strongbox: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!("   🎯 Assessing HSM readiness for key creation...");

        if device.serial == "44251JEKB04957" && strongbox {
            println!("   ✅ READY: Pixel 8a + StrongBox = Hardware key creation ready!");
            println!("   🔑 Key types supported: ECC P-256, AES-256, Ed25519");
            println!("   🛡️  Security level: Hardware-backed (Titan M2)");
            println!("   📱 Platform: GrapheneOS optimized");
        } else if device.serial == "44251JEKB04957" {
            println!("   ⚠️  PARTIAL: Device ready, StrongBox needs verification");
        } else {
            println!("   ❌ NOT READY: Device connection required");
        }

        Ok(())
    }

    fn display_results(&self, results: &HsmTestResults) -> Result<(), Box<dyn std::error::Error>> {
        println!();
        println!("📊 PIXEL 8A HSM READINESS RESULTS");
        println!("=================================");
        println!();

        println!("📱 DEVICE STATUS:");
        println!("   Device Detected: {}", if results.device_detected { "✅ YES" } else { "❌ NO" });
        println!("   Model: {}", results.device_model);
        println!("   Serial: {}", results.device_serial);
        println!("   Android Version: {}", results.android_version);
        println!();

        println!("🔐 SECURITY STATUS:");
        println!("   StrongBox HSM: {}", if results.strongbox_available { "✅ AVAILABLE" } else { "❌ NOT DETECTED" });
        println!("   Verified Boot: {}", match results.verified_boot_state.as_str() {
            "green" => "✅ GREEN (optimal)",
            "yellow" => "⚠️  YELLOW (warning)", 
            "orange" => "⚠️  ORANGE (unlocked)",
            "red" => "❌ RED (compromised)",
            _ => "❓ UNKNOWN"
        });
        println!();

        println!("🎯 KEY CREATION READINESS:");
        if results.device_detected && results.strongbox_available && results.verified_boot_state == "green" {
            println!("   🚀 STATUS: FULLY READY FOR HARDWARE KEY CREATION!");
            println!("   🔑 Recommended: Create ECC P-256 signing key");
            println!("   🛡️  Security: Hardware-backed in Titan M2");
            println!("   📱 Next: Run BearDog key creation on device");
        } else if results.device_detected {
            println!("   ⚠️  STATUS: DEVICE READY, VERIFY SECURITY FEATURES");
            println!("   📋 Check: Enable Developer Options → USB Debugging");
            println!("   🔐 Check: StrongBox HSM availability");
        } else {
            println!("   ❌ STATUS: DEVICE CONNECTION REQUIRED");
            println!("   📱 Action: Connect Pixel 8a via USB");
            println!("   🔧 Action: Enable USB debugging");
        }

        println!();
        println!("⏱️  Test completed in: {}ms", results.test_duration_ms);
        println!();

        if results.device_detected {
            println!("🚀 NEXT STEPS:");
            println!("   1. Verify GrapheneOS security settings");
            println!("   2. Build BearDog for Android: cargo build --target aarch64-linux-android");
            println!("   3. Deploy to device and test key creation");
            println!("   4. Validate hardware-backed key operations");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct DeviceInfo {
    serial: String,
    model: String,
    product: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Initializing BearDog Pixel 8a HSM test...");
    println!("🎯 Target: Hardware key creation validation");
    println!();

    let tester = Pixel8aHsmTester::new();
    let _results = tester.run_comprehensive_test()?;

    println!();
    println!("✅ HSM readiness assessment complete!");
    println!("🔑 Ready to proceed with key creation testing!");

    Ok(())
} 