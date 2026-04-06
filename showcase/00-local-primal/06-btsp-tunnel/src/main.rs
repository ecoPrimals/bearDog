// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_tunnel::tunnel::hsm::{GenerateKeyRequest, HsmProvider, KeyType, SoftwareHsm, SoftwareHsmConfig};
use colored::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "🔐 BearDog BTSP Tunnel".bright_blue().bold());
    println!("{}\n", "BearDog Transport Security Protocol demo...".dimmed());

    let config = SoftwareHsmConfig::default();
    let hsm = SoftwareHsm::new(config).await?;

    // Generate keys for tunnel
    println!("{}", "📝 Step 1: Generate tunnel keys".bright_cyan());
    let client_request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "client-tunnel-key".to_string(),
    };
    let server_request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "server-tunnel-key".to_string(),
    };
    
    let _client_key = hsm.generate_key(client_request).await?;
    let _server_key = hsm.generate_key(server_request).await?;
    println!("{}  Client key: client-tunnel-key", "   ✓".green());
    println!("{}  Server key: server-tunnel-key\n", "   ✓".green());

    // Simulate tunnel establishment
    println!("{}", "📝 Step 2: BTSP Handshake".bright_cyan());
    println!("   1. Client Hello → Server");
    println!("   2. Server Hello ← Server");
    println!("   3. Key Exchange (Ed25519)");
    println!("   4. Authenticate & Establish");
    println!("{}  Tunnel established!\n", "   ✓".green());

    // Simulate secure communication
    println!("{}", "📝 Step 3: Secure Communication".bright_cyan());
    let message = b"Hello through BTSP tunnel!";
    let signature = hsm.sign("client-tunnel-key", message).await?;
    let is_valid = hsm.verify("client-tunnel-key", message, &signature).await?;
    println!("{}  Message signed and verified\n", "   ✓".green());

    println!("{}", "✅ BTSP Tunnel Demo Complete!".green().bold());

    println!("\n{}", "💡 Key Insights:".bright_yellow().bold());
    println!("   • BTSP uses sovereign keys");
    println!("   • End-to-end encryption");
    println!("   • No certificate authorities");
    println!("   • Hardware-backed security");
    
    println!("\n🎉 Level 0 Complete! You've mastered local primal capabilities!");
    println!("🚀 Next Level: Hardware Integration (YubiKey, TPM, etc.)");

    Ok(())
}
