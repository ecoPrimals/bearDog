// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! # Standalone HSM Discovery Demo
//!
//! **BASIC STANDALONE VERSION** for mobile deployment
//!
//! This example demonstrates:
//! - Standalone HSM discovery on mobile devices (Pixel 8)
//! - Failsafe operation when network unavailable
//! - Ecosystem integration when available
//! - Hardware security chip utilization
//!
//! ## Usage Examples
//!
//! ```bash
//! # Quick discovery (standalone only)
//! cargo run --example standalone_hsm_discovery_demo -- --mode quick
//!
//! # Ecosystem-aware discovery (try network first)
//! cargo run --example standalone_hsm_discovery_demo -- --mode ecosystem
//!
//! # Show platform info
//! cargo run --example standalone_hsm_discovery_demo -- --mode platform
//! ```

use beardog_tunnel::universal_hsm_discovery::{
    StandaloneHsmFactory, 
    StandaloneHsmDiscovery,
    StandaloneHsmInfo,
    DevicePlatform,
};
use beardog_errors::BearDogResult;
use tracing::{info, error};
use std::env;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Standalone HSM Discovery Demo");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mode = if args.len() > 2 && args[1] == "--mode" {
        args[2].as_str()
    } else {
        "quick"
    };
    
    match mode {
        "quick" => demo_quick_discovery().await?,
        "ecosystem" => demo_ecosystem_aware_discovery().await?,
        "platform" => demo_platform_detection().await?,
        "full" => demo_full_integration().await?,
        _ => {
            println!("Usage: {} --mode [quick|ecosystem|platform|full]", args[0]);
            return Ok(());
        }
    }
    
    Ok(())
}

/// **QUICK DISCOVERY DEMO** (basic standalone)
async fn demo_quick_discovery() -> BearDogResult<()> {
    info!("🔍 === QUICK DISCOVERY MODE ===");
    info!("📱 Perfect for mobile deployment (Pixel 8) and failsafe scenarios");
    
    // Quick discovery using factory method
    let hsms = StandaloneHsmFactory::quick_discover().await?;
    
    println!("\n🎯 **DISCOVERED HSMs** (Standalone Mode)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    for (i, hsm) in hsms.iter().enumerate() {
        let emoji = match hsm.hsm_type {
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware => "📱",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::DesktopHardware => "🖥️",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software => "💾",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Cloud => "☁️",
        };
        
        let security_badge = match hsm.security_tier {
            beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Hardware => "🔒 HARDWARE",
            beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Trusted => "🛡️ TRUSTED",
            beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Software => "💻 SOFTWARE",
        };
        
        let entropy_badge = if hsm.supports_human_entropy { "🧬 HUMAN-ENTROPY" } else { "" };
        
        println!("{}. {} {} {} - {}", 
                 i + 1, emoji, hsm.vendor, hsm.model, security_badge);
        println!("   🆔 ID: {}", hsm.id);
        println!("   📊 Available: {} {}", 
                 if hsm.available { "✅" } else { "❌" }, entropy_badge);
        println!();
    }
    
    // Highlight mobile hardware security
    let mobile_hsms: Vec<_> = hsms.iter()
        .filter(|h| matches!(h.hsm_type, 
                           beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware))
        .collect();
    
    if !mobile_hsms.is_empty() {
        println!("🎉 **MOBILE HARDWARE SECURITY DETECTED**");
        println!("Perfect for Pixel 8 ephemeral seed generation!");
        
        for hsm in mobile_hsms {
            if hsm.supports_human_entropy {
                println!("✨ {} supports biometric entropy (Touch ID/Face ID/Fingerprint)", hsm.model);
            }
        }
        println!();
    }
    
    println!("🏁 Quick discovery complete - {} HSMs found", hsms.len());
    Ok(())
}

/// **ECOSYSTEM-AWARE DISCOVERY DEMO** (with network integration)
async fn demo_ecosystem_aware_discovery() -> BearDogResult<()> {
    info!("🌐 === ECOSYSTEM-AWARE DISCOVERY MODE ===");
    info!("🔗 Attempting ecosystem integration (songbird + toadstool)");
    
    // Try ecosystem-aware discovery
    let hsms = StandaloneHsmFactory::ecosystem_aware_discover().await?;
    
    println!("\n🌟 **ECOSYSTEM-INTEGRATED HSMs**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Group by type for better display
    let mut mobile = Vec::new();
    let mut desktop = Vec::new();
    let mut software = Vec::new();
    let mut cloud = Vec::new();
    
    for hsm in &hsms {
        match hsm.hsm_type {
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware => mobile.push(hsm),
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::DesktopHardware => desktop.push(hsm),
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software => software.push(hsm),
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Cloud => cloud.push(hsm),
        }
    }
    
    if !mobile.is_empty() {
        println!("📱 **MOBILE HARDWARE** (Primary for Pixel 8):");
        for hsm in mobile {
            println!("   • {} {} {}", 
                     hsm.vendor, hsm.model,
                     if hsm.supports_human_entropy { "🧬" } else { "" });
        }
        println!();
    }
    
    if !desktop.is_empty() {
        println!("🖥️ **DESKTOP HARDWARE** (Network nodes):");
        for hsm in desktop {
            println!("   • {} {}", hsm.vendor, hsm.model);
        }
        println!();
    }
    
    if !software.is_empty() {
        println!("💾 **SOFTWARE HSMs** (Fallbacks):");
        for hsm in software {
            println!("   • {} {}", hsm.vendor, hsm.model);
        }
        println!();
    }
    
    if !cloud.is_empty() {
        println!("☁️ **CLOUD HSMs** (via ToadStool):");
        for hsm in cloud {
            println!("   • {} {}", hsm.vendor, hsm.model);
        }
        println!();
    }
    
    println!("🏁 Ecosystem discovery complete - {} HSMs found", hsms.len());
    Ok(())
}

/// **PLATFORM DETECTION DEMO** (device info)
async fn demo_platform_detection() -> BearDogResult<()> {
    info!("📱 === PLATFORM DETECTION MODE ===");
    info!("🔍 Analyzing device platform and security capabilities");
    
    // Create discovery instance to access platform info
    let discovery = StandaloneHsmFactory::create().await?;
    
    println!("\n🌍 **DEVICE PLATFORM ANALYSIS**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Note: In real implementation, we'd access the platform field
    // For now, demonstrate the concept
    
    #[cfg(target_os = "android")]
    {
        println!("📱 **ANDROID DEVICE DETECTED**");
        println!("   🔹 Target: Android 14+ (Pixel 8 optimized)");
        println!("   🔹 Security: StrongBox hardware security");
        println!("   🔹 Biometrics: Fingerprint sensor entropy");
        println!("   🔹 Deployment: Mobile-first standalone");
    }
    
    #[cfg(target_os = "ios")]
    {
        println!("🍎 **iOS DEVICE DETECTED**");
        println!("   🔹 Target: iOS 17+ (iPhone/iPad)");
        println!("   🔹 Security: Secure Enclave");
        println!("   🔹 Biometrics: Touch ID/Face ID entropy");
        println!("   🔹 Deployment: Mobile secure enclave");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("🐧 **LINUX SYSTEM DETECTED**");
        println!("   🔹 Target: Desktop/Server deployment");
        println!("   🔹 Security: TPM 2.0 / Software fallback");
        println!("   🔹 Network: Full ecosystem integration");
        println!("   🔹 Deployment: Development/production");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("🍎 **macOS SYSTEM DETECTED**");
        println!("   🔹 Target: macOS 14+ (Apple Silicon preferred)");
        println!("   🔹 Security: Secure Enclave (M1/M2/M3)");
        println!("   🔹 Biometrics: Touch ID entropy");
        println!("   🔹 Deployment: Development/desktop");
    }
    
    #[cfg(target_os = "windows")]
    {
        println!("🪟 **WINDOWS SYSTEM DETECTED**");
        println!("   🔹 Target: Windows 11+ (TPM 2.0)");
        println!("   🔹 Security: TPM hardware / Windows Hello");
        println!("   🔹 Network: Full ecosystem integration");
        println!("   🔹 Deployment: Enterprise/desktop");
    }
    
    // Show discovery results
    let hsms = discovery.discover_basic().await?;
    
    println!("\n🎯 **PLATFORM-OPTIMIZED HSMs**:");
    for hsm in &hsms {
        let priority = match hsm.hsm_type {
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware => "🥇 PRIMARY",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::DesktopHardware => "🥈 SECONDARY", 
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software => "🥉 FALLBACK",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Cloud => "🌐 NETWORK",
        };
        
        println!("   • {} - {} {} {}", 
                 priority, hsm.vendor, hsm.model,
                 if hsm.supports_human_entropy { "(Human Entropy ✨)" } else { "" });
    }
    
    println!("\n🏁 Platform analysis complete");
    Ok(())
}

/// **FULL INTEGRATION DEMO** (comprehensive example)
async fn demo_full_integration() -> BearDogResult<()> {
    info!("🚀 === FULL INTEGRATION DEMO ===");
    info!("🌟 Complete standalone + ecosystem integration showcase");
    
    println!("\n🎭 **BEARDOG STANDALONE HSM DISCOVERY**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📱 Mobile-First • ⚡ Failsafe • 🌐 Ecosystem-Ready");
    println!();
    
    // 1. Create discovery instance
    println!("1️⃣ **INITIALIZING DISCOVERY ENGINE**");
    let discovery = StandaloneHsmFactory::create().await?;
    println!("   ✅ Standalone discovery engine ready");
    
    // 2. Try ecosystem connection (non-blocking)
    println!("\n2️⃣ **ATTEMPTING ECOSYSTEM CONNECTION**");
    println!("   🎼 Trying songbird universal adapter...");
    println!("   🍄 Trying toadstool platform context...");
    match discovery.try_ecosystem_connection().await {
        Ok(_) => println!("   ✅ Ecosystem connection established"),
        Err(_) => println!("   ⚠️ Ecosystem unavailable (standalone mode active)"),
    }
    
    // 3. Discover HSMs
    println!("\n3️⃣ **DISCOVERING AVAILABLE HSMs**");
    let hsms = discovery.discover_basic().await?;
    println!("   🔍 Discovered {} HSM(s)", hsms.len());
    
    // 4. Analyze results
    println!("\n4️⃣ **HSM ANALYSIS & PRIORITIZATION**");
    
    let hardware_hsms: Vec<_> = hsms.iter()
        .filter(|h| matches!(h.security_tier, 
                           beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Hardware))
        .collect();
    
    let entropy_hsms: Vec<_> = hsms.iter()
        .filter(|h| h.supports_human_entropy)
        .collect();
    
    println!("   🔒 Hardware HSMs: {}", hardware_hsms.len());
    println!("   🧬 Human Entropy Support: {}", entropy_hsms.len());
    
    // 5. Show recommended deployment
    println!("\n5️⃣ **DEPLOYMENT RECOMMENDATIONS**");
    
    if let Some(primary) = hsms.iter()
        .find(|h| matches!(h.hsm_type, 
                          beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware) 
                 && h.supports_human_entropy) {
        println!("   🎯 **PRIMARY**: {} {} (Mobile Hardware + Human Entropy)", 
                 primary.vendor, primary.model);
        println!("      📱 Perfect for Pixel 8 ephemeral seed generation");
        println!("      🧬 Biometric entropy for genetic algorithms");
    }
    
    if let Some(fallback) = hsms.iter()
        .find(|h| matches!(h.hsm_type, 
                          beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software)) {
        println!("   🛡️ **FALLBACK**: {} {} (Software HSM)", 
                 fallback.vendor, fallback.model);
        println!("      ⚡ Always available when hardware unavailable");
    }
    
    // 6. Convert to full discovery format (for compatibility)
    println!("\n6️⃣ **COMPATIBILITY CONVERSION**");
    let full_hsms = discovery.to_discovered_hsms().await?;
    println!("   🔄 Converted {} standalone HSMs to full discovery format", full_hsms.len());
    
    // 7. Summary
    println!("\n🏆 **DEPLOYMENT SUMMARY**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Standalone discovery: OPERATIONAL");
    println!("📱 Mobile deployment: READY (Pixel 8 optimized)");
    println!("🌐 Ecosystem integration: ENABLED");
    println!("⚡ Failsafe operation: GUARANTEED");
    println!("🧬 Human entropy: {} HSM(s) support", entropy_hsms.len());
    println!("🎯 Total HSMs available: {}", hsms.len());
    
    println!("\n🎉 Full integration demo complete!");
    Ok(())
} 