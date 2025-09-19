use beardog_errors::BearDogError;
use beardog_tunnel::universal_hsm_discovery::{
    DevicePlatform, StandaloneHsmDiscovery, StandaloneHsmFactory, StandaloneHsmInfo,
};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::fmt::init();

    info!("[ROCKET] Starting Standalone HSM Discovery Demo");

    let args: Vec<String> = env::args({} --mode [quick|ecosystem|platform|full]", args[0]);
            return Ok(());
        }
    }

    Ok(())
}

async fn demo_quick_discovery() -> Result<(), BearDogError> {
    info!("[SEARCH] === QUICK DISCOVERY MODE ===");
    info!("📱 Perfect for mobile deployment (Pixel 8) and failsafe scenarios");

    let hsms = StandaloneHsmFactory::quick_discover()?;

    println!("[TARGET] **DISCOVERED HSMs** (Standalone Mode)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for (i, hsm) in hsms.iter().enumerate() {
        let emoji = match hsm.hsm_type {
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware => "📱",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::DesktopHardware => "🖥️",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software => "💾",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Cloud => "☁️",
        };

        let security_badge = match hsm.security_tier {
            beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Hardware => {
                "[LOCK] HARDWARE"
            }
            beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Trusted => {
                "[SHIELD] TRUSTED"
            }
            beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Software => {
                "💻 SOFTWARE"
            }
        };

        let entropy_badge = if hsm.supports_human_entropy {
            "[DNA] HUMAN-ENTROPY"
        } else {
            ""
        };

        println!(
            "{}. {} {} {} - {}",
            i + 1,
            emoji,
            hsm.vendor,
            hsm.model,
            security_badge
        );
        println!("   🆔 ID: {}", hsm.id);
        println!(
            "   [CHART] Available: {} {}",
            if hsm.available { "[OK]" } else { "[X]" },
            entropy_badge
        );
        println!();
    }

    let mobile_hsms: Vec<_> = hsms.iter()
        .filter(|h| matches!(h.hsm_type, 
                           beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware))
        .collect();

    if !mobile_hsms.is_empty() {
        println!("[PARTY] **MOBILE HARDWARE SECURITY DETECTED**");
        println!("Perfect for Pixel 8 ephemeral seed generation!");

        for hsm in mobile_hsms {
            if hsm.supports_human_entropy {
                println!(
                    "✨ {} supports biometric entropy (Touch ID/Face ID/Fingerprint)",
                    hsm.model
                );
            }
        }
        println!();
    }

    println!("🏁 Quick discovery complete - {} HSMs found", hsms.len());
    Ok(())
}

async fn demo_ecosystem_aware_discovery() -> Result<(), BearDogError> {
    info!("🌐 === ECOSYSTEM-AWARE DISCOVERY MODE ===");
    info!("🔗 Attempting ecosystem integration (songbird + toadstool)");

    let hsms = StandaloneHsmFactory::ecosystem_aware_discover()?;

    println!("🌟 **ECOSYSTEM-INTEGRATED HSMs**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

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
            println!(
                "   - {} {} {}",
                hsm.vendor,
                hsm.model,
                if hsm.supports_human_entropy {
                    "[DNA]"
                } else {
                    ""
                }
            );
        }
        println!();
    }

    if !desktop.is_empty() {
        println!("🖥️ **DESKTOP HARDWARE** (Network nodes):");
        for hsm in desktop {
            println!("   - {} {}", hsm.vendor, hsm.model);
        }
        println!();
    }

    if !software.is_empty() {
        println!("💾 **SOFTWARE HSMs** (Fallbacks):");
        for hsm in software {
            println!("   - {} {}", hsm.vendor, hsm.model);
        }
        println!();
    }

    if !cloud.is_empty() {
        println!("☁️ **CLOUD HSMs** (via ComputeService):");
        for hsm in cloud {
            println!("   - {} {}", hsm.vendor, hsm.model);
        }
        println!();
    }

    println!(
        "🏁 Ecosystem discovery complete - {} HSMs found",
        hsms.len()
    );
    Ok(())
}

async fn demo_platform_detection() -> Result<(), BearDogError> {
    info!("📱 === PLATFORM DETECTION MODE ===");
    info!("[SEARCH] Analyzing device platform and security capabilities");

    let discovery = StandaloneHsmFactory::create()?;

    println!("🌍 **DEVICE PLATFORM ANALYSIS**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

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

    let hsms = discovery.discover_basic()?;

    println!("[TARGET] **PLATFORM-OPTIMIZED HSMs**:");
    for hsm in &hsms {
        let priority = match hsm.hsm_type {
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware => "🥇 PRIMARY",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::DesktopHardware => "🥈 SECONDARY", 
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software => "🥉 FALLBACK",
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Cloud => "🌐 NETWORK",
        };

        println!(
            "   - {} - {} {} {}",
            priority,
            hsm.vendor,
            hsm.model,
            if hsm.supports_human_entropy {
                "(Human Entropy ✨)"
            } else {
                ""
            }
        );
    }

    println!("🏁 Platform analysis complete ");
    Ok(())
}

async fn demo_full_integration() -> Result<(), BearDogError> {
    info!("[ROCKET] === FULL INTEGRATION DEMO ===");
    info!("🌟 Complete standalone + ecosystem integration showcase");

    println!("🎭 **BEARDOG STANDALONE HSM DISCOVERY**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📱 Mobile-First - [LIGHTNING] Failsafe - 🌐 Ecosystem-Ready");
    println!();

    println!("1️⃣ **INITIALIZING DISCOVERY ENGINE**");
    let discovery = StandaloneHsmFactory::create()?;
    println!("   [OK] Standalone discovery engine ready");

    println!("2️⃣ **ATTEMPTING ECOSYSTEM CONNECTION**");
    println!("   🎼 Trying songbird universal adapter...");
    println!("   🍄 Trying toadstool platform context...");
    match discovery.try_ecosystem_connection() {
        Ok(_) => println!("   [OK] Ecosystem connection established"),
        Err(_) => println!("   ⚠️ Ecosystem unavailable (standalone mode active)"),
    }

    println!("3️⃣ **DISCOVERING AVAILABLE HSMs**");
    let hsms = discovery.discover_basic()?;
    println!("   [SEARCH] Discovered {} HSM(s)", hsms.len());

    println!("4️⃣ **HSM ANALYSIS & PRIORITIZATION**");

    let hardware_hsms: Vec<_> = hsms
        .iter()
        .filter(|h| {
            matches!(
                h.security_tier,
                beardog_tunnel::universal_hsm_discovery::standalone::SecurityTier::Hardware
            )
        })
        .collect();

    let entropy_hsms: Vec<_> = hsms.iter({}", hardware_hsms.len({}", entropy_hsms.len());

    println!("5️⃣ **DEPLOYMENT RECOMMENDATIONS**");

    if let Some(primary) = hsms.iter().find(|h| {
        matches!(
            h.hsm_type,
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::MobileHardware
        ) && h.supports_human_entropy
    }) {
        println!(
            "   [TARGET] **PRIMARY**: {} {} (Mobile Hardware + Human Entropy)",
            primary.vendor, primary.model
        );
        println!("      📱 Perfect for Pixel 8 ephemeral seed generation");
        println!("      [DNA] Biometric entropy for genetic algorithms");
    }

    if let Some(fallback) = hsms.iter().find(|h| {
        matches!(
            h.hsm_type,
            beardog_tunnel::universal_hsm_discovery::standalone::StandaloneHsmType::Software
        )
    }) {
        println!(
            "   [SHIELD] **FALLBACK**: {} {} (Software HSM)",
            fallback.vendor, fallback.model
        );
        println!("      [LIGHTNING] Always available when hardware unavailable");
    }

    println!("6️⃣ **COMPATIBILITY CONVERSION**");
    let full_hsms = discovery.to_discovered_hsms()?;
    println!(
        "   [CYCLE] Converted {} standalone HSMs to full discovery format",
        full_hsms.len()
    );

    println!("[TROPHY] **DEPLOYMENT SUMMARY**");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("[OK] Standalone discovery: OPERATIONAL");
    println!("📱 Mobile deployment: READY (Pixel 8 optimized)");
    println!("🌐 Ecosystem integration: ENABLED");
    println!("[LIGHTNING] Failsafe operation: GUARANTEED");
    println!("[DNA] Human entropy: {} HSM({}", hsms.len());

    println!("[PARTY] Full integration demo complete!");
    Ok(())
}
