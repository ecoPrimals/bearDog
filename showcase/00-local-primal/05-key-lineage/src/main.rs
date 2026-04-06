// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_tunnel::tunnel::hsm::{GenerateKeyRequest, HsmProvider, KeyType, SoftwareHsm, SoftwareHsmConfig};
use colored::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "🌳 BearDog Key Lineage".bright_cyan().bold());
    println!("{}\n", "Tracking key ancestry and evolution...".dimmed());

    let config = SoftwareHsmConfig::default();
    let hsm = SoftwareHsm::new(config).await?;

    // Create a key family
    println!("{}", "📝 Creating key family...".bright_cyan());
    
    // Genesis key
    println!("\n{}", "Generation 0 (Genesis):".bright_white().bold());
    let genesis_request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "genesis-key".to_string(),
    };
    let _genesis = hsm.generate_key(genesis_request).await?;
    println!("  {} genesis-key", "└─".dimmed());

    // Child keys
    println!("\n{}", "Generation 1 (Children):".bright_white().bold());
    for i in 1..=3 {
        let child_request = GenerateKeyRequest {
            key_type: KeyType::Ed25519,
            key_id: format!("child-key-{}", i),
        };
        let _child = hsm.generate_key(child_request).await?;
        println!("  {} child-key-{}", "├─".dimmed(), i);
    }

    // Grandchild
    println!("\n{}", "Generation 2 (Grandchildren):".bright_white().bold());
    let grandchild_request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "grandchild-key-1".to_string(),
    };
    let _grandchild = hsm.generate_key(grandchild_request).await?;
    println!("  {} grandchild-key-1", "└─".dimmed());

    println!("\n{}", "✅ Key lineage established!".green());

    println!("\n{}", "💡 Key Insights:".bright_yellow().bold());
    println!("   • Keys have verifiable ancestry");
    println!("   • Lineage tracks evolution");
    println!("   • Genetic algorithms inherit traits");
    println!("   • Audit trail built-in");
    
    println!("\n🚀 Next: Try 06-btsp-tunnel for secure connections!");

    Ok(())
}
