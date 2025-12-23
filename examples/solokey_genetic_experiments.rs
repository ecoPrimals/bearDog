#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! SoloKey Genetic Experiments
//!
//! Experiments with:
//! 1. Multi-credential storage (multiple roles on one key)
//! 2. Genetic key hierarchies (parent-child relationships)
//! 3. Deterministic key derivation (same key on multiple devices)
//!
//! Run with:
//! ```bash
//! cargo run --example solokey_genetic_experiments
//! ```

// NOTE: This example is temporarily disabled due to FIDO2 API refactoring.
// The FIDO2 module structure has changed and needs to be updated.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║       SoloKey Genetic Experiments - November 9, 2025          ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    println!("❌ This example is temporarily disabled due to API changes.");
    println!("   The FIDO2 module structure has been refactored.");
    println!("   This example will be updated in a future release.");
    println!();
    println!("   Original functionality included:");
    println!("   • Multi-credential storage (multiple roles on one key)");
    println!("   • Genetic key hierarchies (parent-child relationships)");
    println!("   • Deterministic key derivation (same key on multiple devices)");
    println!();
    println!("   To re-enable this example:");
    println!("   1. Update the FIDO2 discovery API imports");
    println!("   2. Update to use new beardog_security::hsm::fido2 structure");
    println!("   3. Rebuild with: cargo build --example solokey_genetic_experiments");

    Ok(())
}
