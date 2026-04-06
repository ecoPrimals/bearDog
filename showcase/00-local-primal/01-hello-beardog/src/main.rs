// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_tunnel::tunnel::hsm::{GenerateKeyRequest, HsmProvider, KeyType, SoftwareHsm, SoftwareHsmConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🐻 Hello BearDog! Generating your first sovereign key...\n");

    // Initialize software HSM
    let config = SoftwareHsmConfig::default();
    let hsm = SoftwareHsm::new(config).await?;

    // 1. Generate an Ed25519 key
    let key_id = "my-first-sovereign-key";
    println!("📝 Step 1: Generating Ed25519 key...");
    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: key_id.to_string(),
    };
    let _key = hsm.generate_key(request).await?;
    println!("✅ Key generated: {}\n", key_id);

    // 2. Sign a message
    let message = b"Hello, sovereign world!";
    println!("📝 Step 2: Signing message...");
    println!("   Message: {:?}", String::from_utf8_lossy(message));
    let signature = hsm.sign(key_id, message).await?;
    println!("✅ Signature created: {} bytes\n", signature.len());

    // 3. Verify the signature
    println!("📝 Step 3: Verifying signature...");
    let is_valid = hsm.verify(key_id, message, &signature).await?;
    if is_valid {
        println!("✅ Signature is VALID!\n");
    } else {
        println!("❌ Signature is INVALID!\n");
    }

    // 4. Try verifying with wrong message (should fail)
    println!("📝 Step 4: Testing with wrong message...");
    let wrong_message = b"Different message";
    let is_invalid = hsm.verify(key_id, wrong_message, &signature).await?;
    if !is_invalid {
        println!("✅ Correctly rejected invalid signature!\n");
    } else {
        println!("⚠️  Warning: Accepted invalid signature\n");
    }

    println!("🎉 BearDog Demo Complete!");
    println!("\n💡 Key Insights:");
    println!("   • Generated Ed25519 sovereign key");
    println!("   • Signed data with cryptographic proof");
    println!("   • Verified signature authenticity");
    println!("   • Rejected tampered signatures");
    println!("\n🚀 Next: Try 02-hsm-discovery to see what HSMs are available!");

    Ok(())
}
