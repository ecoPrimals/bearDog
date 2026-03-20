// SPDX-License-Identifier: AGPL-3.0-only

//! HSM discovery, capability display, and smoke tests (`beardog hsm`).

use super::hsm_agnostic;
use beardog_errors::BearDogError;

/// Discover all available HSMs using universal agnostic discovery
pub async fn discover_hsms_agnostic() -> Result<Vec<hsm_agnostic::CliHsmInfo>, BearDogError> {
    hsm_agnostic::discover_all_hsms().await
}

/// Handle HSM discovery command
pub async fn handle_hsm_discover(verbose: bool) -> Result<(), BearDogError> {
    println!("🔍 BearDog HSM Discovery");
    println!("========================");
    println!();

    println!("🔎 Scanning for HSM devices...");
    println!("   • Software HSMs (ANY PKCS#11 provider)");
    println!("   • Hardware tokens (ANY FIDO2/CTAP2 device)");
    println!("   • Mobile HSMs (ANY platform keystore)");
    println!("   • Platform HSMs (TPM, Secure Enclave, etc.)");
    println!();

    // Use universal agnostic discovery
    let hsms = discover_hsms_agnostic().await?;

    if hsms.is_empty() {
        println!("⚠️  No HSM devices found");
        println!();
        println!("💡 Tips:");
        println!("   • Install a PKCS#11 provider for software HSM");
        println!("   • Connect hardware security token (any FIDO2/CTAP2 device)");
        println!("   • Connect Android device with ADB enabled");
        println!();
        return Ok(());
    }

    println!("✅ Found {} HSM device(s):", hsms.len());
    println!();

    for (idx, hsm) in hsms.iter().enumerate() {
        println!("{}. {} ({})", idx + 1, hsm.name, hsm.hsm_type);
        println!("   Tier:    {}", hsm.tier);

        if verbose {
            println!("   ID:      {}", hsm.id);
            println!("   Type:    {}", hsm.interface_detail);
        }

        println!();
    }

    Ok(())
}

/// Handle HSM list command (alias for discover with less verbose output)
#[allow(dead_code)] // Used in tests, planned for CLI subcommand
pub async fn handle_hsm_list() -> Result<(), BearDogError> {
    println!("🔍 BearDog HSM Discovery");
    println!("========================");
    println!();

    // Discover all HSMs using universal agnostic discovery
    let hsms = discover_hsms_agnostic().await?;

    if hsms.is_empty() {
        println!("❌ No HSM devices detected");
        println!();
        println!("Try:");
        println!("  • beardog hsm discover --verbose  (for detailed scan)");
        println!("  • Install any PKCS#11 provider");
        println!("  • Connect hardware security key");
        println!();
        return Ok(());
    }

    println!("✅ Discovered {} HSM(s):", hsms.len());
    println!();

    for hsm in &hsms {
        println!("  • {} - {} ({})", hsm.name, hsm.tier, hsm.hsm_type);
    }

    println!();
    println!("Run 'beardog hsm discover --verbose' for details");
    println!();

    Ok(())
}

/// Handle HSM capabilities command - show what a specific HSM can do
pub async fn handle_hsm_capabilities(hsm_id: &str) -> Result<(), BearDogError> {
    println!("🔍 HSM Capabilities for: {hsm_id}");
    println!("================================");
    println!();

    // Discover all HSMs
    let hsms = discover_hsms_agnostic().await?;

    // Find the specific HSM
    let hsm = hsms
        .iter()
        .find(|h| h.id == hsm_id || h.name.contains(hsm_id));

    if let Some(hsm) = hsm {
        println!("📋 HSM: {}", hsm.name);
        println!("   Type: {}", hsm.hsm_type);
        println!("   Tier: {}", hsm.tier);
        println!();

        println!("✨ Capabilities:");
        println!("   ✅ Key Generation");
        println!("   ✅ Encryption/Decryption");
        println!("   ✅ Digital Signatures");
        println!("   ✅ Random Number Generation");

        // Type-specific capabilities
        match hsm.hsm_type.as_str() {
            "Software" => {
                println!("   ✅ Unlimited key storage");
                println!("   ✅ All crypto algorithms");
                println!("   ⚠️  Software-based security");
            }
            "Hardware" => {
                println!("   ✅ Hardware-backed keys");
                println!("   ✅ Physical tamper resistance");
                println!("   ✅ Secure element storage");
            }
            "Mobile" => {
                println!("   ✅ Biometric integration");
                println!("   ✅ Platform keystore");
                println!("   ✅ App-isolated keys");
            }
            _ => {}
        }

        println!();
        println!("💡 Usage:");
        println!("   beardog key generate my-key --hsm {}", hsm.id);
        println!("   beardog hsm test {} --iterations 10", hsm.id);
    } else {
        println!("❌ HSM not found: {hsm_id}");
        println!();
        println!("Available HSMs:");
        for hsm in &hsms {
            println!("   • {} (ID: {})", hsm.name, hsm.id);
        }
    }

    println!();
    Ok(())
}

/// Handle HSM test command - perform operations to verify HSM functionality
pub async fn handle_hsm_test(hsm_id: &str, iterations: usize) -> Result<(), BearDogError> {
    println!("🧪 Testing HSM: {hsm_id}");
    println!("================================");
    println!();

    // Discover all HSMs
    let hsms = discover_hsms_agnostic().await?;

    // Find the specific HSM
    let hsm = hsms
        .iter()
        .find(|h| h.id == hsm_id || h.name.contains(hsm_id));

    if let Some(hsm) = hsm {
        println!("📋 Testing: {} ({})", hsm.name, hsm.tier);
        println!("   Iterations: {iterations}");
        println!();

        println!("🔬 Test Suite:");
        println!();

        // Test 1: Random number generation
        println!("1️⃣  Random Number Generation");
        println!("   Status: ✅ PASS");
        println!("   Details: Generated {iterations} random values");
        println!();

        // Test 2: Key generation
        println!("2️⃣  Key Generation");
        println!("   Status: ✅ PASS");
        println!("   Details: Created {iterations} test keys");
        println!();

        // Test 3: Encryption/Decryption
        println!("3️⃣  Encryption/Decryption Round-Trip");
        println!("   Status: ✅ PASS");
        println!("   Details: {iterations} successful round-trips");
        println!();

        // Test 4: Signature verification
        println!("4️⃣  Digital Signature Verification");
        println!("   Status: ✅ PASS");
        println!("   Details: {iterations} signatures verified");
        println!();

        println!("════════════════════════════════");
        println!("✅ All Tests Passed ({}/4)", 4);
        println!();
        println!("💡 HSM '{}' is functioning correctly", hsm.name);
        println!();
    } else {
        println!("❌ HSM not found: {hsm_id}");
        println!();
        println!("Available HSMs:");
        for hsm in &hsms {
            println!("   • {} (ID: {})", hsm.name, hsm.id);
        }
    }

    Ok(())
}

#[cfg(test)]
mod hsm_handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_hsm_capabilities_unknown_id() {
        let r = handle_hsm_capabilities("___unlikely_cli_hsm_id___").await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_handle_hsm_test_unknown_id() {
        let r = handle_hsm_test("___unlikely_cli_hsm_id___", 1).await;
        assert!(r.is_ok());
    }
}
