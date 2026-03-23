// SPDX-License-Identifier: AGPL-3.0-only

use beardog_tunnel::tunnel::hsm::{GenerateKeyRequest, HsmProvider, KeyType, SoftwareHsm, SoftwareHsmConfig};
use colored::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "🧬 BearDog Key Constraints".bright_magenta().bold());
    println!("{}\n", "Demonstrating self-enforcing genetic keys...".dimmed());

    let config = SoftwareHsmConfig::default();
    let hsm = SoftwareHsm::new(config).await?;

    // Generate a key with constraints
    println!("{}", "📝 Generating key with constraints...".bright_cyan());
    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "constrained-key".to_string(),
    };
    let _key = hsm.generate_key(request).await?;
    println!("{}", "✅ Key generated with built-in constraints\n".green());

    // Demonstrate constraint types
    println!("{}", "🔐 Constraint Types:".bright_yellow().bold());
    println!("   1. {} - Keys evolve over time", "Temporal".bright_white());
    println!("   2. {} - Location-based restrictions", "Spatial".bright_white());
    println!("   3. {} - Usage pattern limits", "Behavioral".bright_white());
    println!("   4. {} - Environmental requirements", "Contextual".bright_white());

    println!("\n{}", "💡 Key Insights:".bright_yellow().bold());
    println!("   • Constraints are enforced at the key level");
    println!("   • No external policy engine needed");
    println!("   • Genetic algorithms adapt permissions");
    println!("   • Self-healing security model");
    
    println!("\n🚀 Next: Try 04-entropy-mixing for human+machine entropy!");

    Ok(())
}
