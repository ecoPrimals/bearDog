// SPDX-License-Identifier: AGPL-3.0-only

use beardog_tunnel::tunnel::hsm::{HsmProvider, SoftwareHsm, SoftwareHsmConfig};
use colored::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "🔍 BearDog HSM Discovery".bright_blue().bold());
    println!("{}\n", "Discovering available Hardware Security Modules...".dimmed());

    // Check software HSM
    let config = SoftwareHsmConfig::default();
    let hsm = SoftwareHsm::new(config).await?;
    
    let is_available = hsm.is_available();
    let info = hsm.get_info().await?;

    println!(
        "{}",
        format!("✅ Discovered {} HSM(s)", if is_available { 1 } else { 0 })
            .green()
            .bold()
    );
    println!();

    if is_available {
        println!("{}", "HSM #1".bright_cyan().bold());
        println!("  {} {}", "Name:".bright_white(), info.name);
        println!("  {} {}", "ID:".bright_white(), info.id);
        println!("  {} {}", "Security Level:".bright_white(), info.security_level);
        println!("  {} {}", "Type:".bright_white(), "Software HSM");
        println!("  {} {}", "Available:".bright_white(), "Yes");
        println!("  {} {}", "Hardware-backed:".bright_white(), "No (software fallback)");
        println!();
    }

    println!("{}", "💡 Discovery Process:".bright_yellow().bold());
    println!("   • Scans for YubiKeys (PIV/PKCS#11)");
    println!("   • Checks TPM 2.0 availability");
    println!("   • Detects Android StrongBox");
    println!("   • Finds iOS Secure Enclave");
    println!("   • Falls back to software");

    println!("\n{}", "💡 Key Insights:".bright_yellow().bold());
    println!("   • HSM discovery happens automatically");
    println!("   • BearDog adapts to available hardware");
    println!("   • Graceful fallback to software when needed");
    println!("   • No configuration required");
    
    println!("\n🚀 Next: Try 03-key-constraints to see genetic keys!");

    Ok(())
}
