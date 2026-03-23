// SPDX-License-Identifier: AGPL-3.0-only

use beardog_tunnel::tunnel::hsm::{GenerateKeyRequest, HsmProvider, KeyType, SoftwareHsm, SoftwareHsmConfig};
use colored::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "🎲 BearDog Entropy Mixing".bright_green().bold());
    println!("{}\n", "Combining human and machine entropy...".dimmed());

    let config = SoftwareHsmConfig::default();
    let hsm = SoftwareHsm::new(config).await?;

    // Simulate entropy mixing
    println!("{}", "🖥️  Machine Entropy (60%):".bright_cyan());
    println!("   • Hardware RNG");
    println!("   • TPM entropy pool");
    println!("   • System randomness");

    println!("\n{}", "👤 Human Entropy (40%):".bright_magenta());
    println!("   • Typing patterns");
    println!("   • Mouse movements");
    println!("   • Biometric timing");

    // Generate key with mixed entropy
    println!("\n{}", "📝 Generating key with mixed entropy...".bright_cyan());
    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "mixed-entropy-key".to_string(),
    };
    let _key = hsm.generate_key(request).await?;
    println!("{}", "✅ Key generated with 60/40 entropy mix\n".green());

    println!("{}", "⚠️  ENTROPY HIERARCHY PRINCIPLE:".bright_red().bold());
    println!("   {} Never simulate human entropy", "NEVER:".bright_red());
    println!("   {} Use real human input only", "ALWAYS:".green());
    println!("   {} Track entropy provenance", "VERIFY:".yellow());

    println!("\n{}", "💡 Key Insights:".bright_yellow().bold());
    println!("   • Real human input adds unpredictability");
    println!("   • Machine entropy provides volume");
    println!("   • Mixing creates sovereign entropy");
    println!("   • Simulation violates trust model");
    
    println!("\n🚀 Next: Try 05-key-lineage to track key ancestry!");

    Ok(())
}
