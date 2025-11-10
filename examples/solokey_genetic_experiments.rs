//! SoloKey Genetic Experiments
//!
//! Experiments with:
//! 1. Multi-credential storage (multiple roles on one key)
//! 2. Genetic key hierarchies (parent-child relationships)
//! 3. Deterministic key derivation (same key on multiple devices)
//!
//! Run with:
//! ```bash
//! cargo run --example solokey_genetic_experiments --features fido2
//! ```

use beardog_security::hsm::fido2::discovery::discover_fido2_devices;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║       SoloKey Genetic Experiments - November 9, 2025          ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    // Discover devices
    println!("🔍 Discovering FIDO2 devices...");
    let devices = discover_fido2_devices().await?;
    
    if devices.is_empty() {
        println!("❌ No FIDO2 devices found!");
        return Ok(());
    }
    
    println!("✅ Found {} device(s)\n", devices.len());
    
    // Display menu
    loop {
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║                    Experiment Menu                             ║");
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║  1. Concept: Multi-Role Credentials                            ║");
        println!("║     (Multiple roles on ONE device)                             ║");
        println!("║                                                                ║");
        println!("║  2. Concept: Genetic Key Hierarchy                             ║");
        println!("║     (Parent-child credential relationships)                    ║");
        println!("║                                                                ║");
        println!("║  3. Concept: Cross-Device Replication                          ║");
        println!("║     (Same credentials on BOTH devices)                         ║");
        println!("║                                                                ║");
        println!("║  4. Implementation Status                                      ║");
        println!("║     (What's working vs. planned)                               ║");
        println!("║                                                                ║");
        println!("║  5. Exit                                                       ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        
        print!("\n   Select experiment (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => experiment_multi_role().await?,
            "2" => experiment_genetic_hierarchy().await?,
            "3" => experiment_cross_device(devices.len()).await?,
            "4" => show_implementation_status().await?,
            "5" => break,
            _ => println!("\n   ❌ Invalid selection\n"),
        }
    }
    
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                   Experiments Complete!                        ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    
    Ok(())
}

async fn experiment_multi_role() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║          Experiment 1: Multi-Role Credentials                  ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    
    println!("\n📋 Concept:");
    println!("   Store multiple credentials on ONE SoloKey, each with different");
    println!("   roles and permissions.");
    println!();
    println!("   Example credentials:");
    println!("   ┌─────────────────────────────────────────────────────────┐");
    println!("   │  Credential 1: \"admin\"                                  │");
    println!("   │    Role: SystemAdmin                                    │");
    println!("   │    Permissions: ALL                                     │");
    println!("   │    Can: Deploy, Configure, Manage Users                 │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    println!("   │  Credential 2: \"security\"                              │");
    println!("   │    Role: SecurityOfficer                                │");
    println!("   │    Permissions: Security operations only                │");
    println!("   │    Can: Audit logs, Manage keys, Review alerts          │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    println!("   │  Credential 3: \"operator\"                              │");
    println!("   │    Role: Operator                                       │");
    println!("   │    Permissions: Day-to-day operations                   │");
    println!("   │    Can: Start/stop services, View metrics               │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    println!("   │  Credential 4: \"auditor\"                               │");
    println!("   │    Role: Auditor                                        │");
    println!("   │    Permissions: Read-only + audit                       │");
    println!("   │    Can: View logs, Generate reports, No changes         │");
    println!("   └─────────────────────────────────────────────────────────┘");
    
    println!("\n💡 Benefits:");
    println!("   ✅ Principle of least privilege");
    println!("   ✅ One device, multiple roles");
    println!("   ✅ Easy role switching (user picks via button/PIN)");
    println!("   ✅ Time-limited credentials possible");
    println!("   ✅ Audit trail per role");
    
    println!("\n📊 Storage Capacity:");
    println!("   Solo 2: Up to 50+ resident credentials");
    println!("   You could have dozens of roles on one device!");
    
    println!("\n🔧 Implementation:");
    println!("   Status: ⏳ Requires CTAP2 MakeCredential (Phase 2)");
    println!("   ETA: After GetInfo is implemented");
    
    println!("\n   Press Enter to return to menu...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    
    Ok(())
}

async fn experiment_genetic_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║        Experiment 2: Genetic Key Hierarchy                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    
    println!("\n🧬 Concept:");
    println!("   Create a hierarchy of credentials where each 'child' is derived");
    println!("   from a 'parent' with progressively restricted permissions.");
    println!();
    println!("   Genetic Tree:");
    println!("   ┌─────────────────────────────────────────────────────────┐");
    println!("   │  Generation 0: ROOT                                     │");
    println!("   │    DNA: 0x1a2b3c4d...                                   │");
    println!("   │    Traits: can_spawn, can_admin, can_sign, can_encrypt  │");
    println!("   │    Sovereignty: 100%                                    │");
    println!("   │                                                         │");
    println!("   │    ├── Generation 1: ADMIN (child of ROOT)              │");
    println!("   │    │     DNA: 0x5e6f7g8h... (derived from parent)       │");
    println!("   │    │     Traits: can_spawn, can_admin, can_sign         │");
    println!("   │    │     Sovereignty: 90%                               │");
    println!("   │    │                                                    │");
    println!("   │    │   ├── Generation 2: OPERATOR (child of ADMIN)      │");
    println!("   │    │   │     DNA: 0x9i0j1k2l... (derived from admin)    │");
    println!("   │    │   │     Traits: can_sign, can_encrypt              │");
    println!("   │    │   │     Sovereignty: 50%                           │");
    println!("   │    │   │                                                │");
    println!("   │    │   │   └── Generation 3: READONLY                   │");
    println!("   │    │   │         DNA: 0x3m4n5o6p...                     │");
    println!("   │    │   │         Traits: (none - read only)             │");
    println!("   │    │   │         Sovereignty: 10%                       │");
    println!("   └─────────────────────────────────────────────────────────┘");
    
    println!("\n🎯 Key Features:");
    println!("   ✅ Hierarchical permissions (parent > child)");
    println!("   ✅ Traceable lineage (know parent/grandparent)");
    println!("   ✅ Deterministic derivation (reproducible)");
    println!("   ✅ Sovereignty levels (quantified power)");
    println!("   ✅ 'Genetic' metadata (BearDog-specific)");
    
    println!("\n💡 Use Cases:");
    println!("   - Delegated administration (admin creates operator keys)");
    println!("   - Temporary access (short-lived child credentials)");
    println!("   - Audit trails (track credential lineage)");
    println!("   - Progressive restrictions (each gen has less power)");
    
    println!("\n🔬 BearDog Genetics:");
    println!("   Each credential has 'DNA' - a unique identifier derived from:");
    println!("   - Parent DNA (if child)");
    println!("   - Role name");
    println!("   - Creation timestamp");
    println!("   - Random salt");
    println!();
    println!("   This creates a verifiable chain of custody!");
    
    println!("\n🔧 Implementation:");
    println!("   Status: ⏳ Requires CTAP2 MakeCredential + custom metadata");
    println!("   Complexity: Medium (need CBOR encoding for metadata)");
    println!("   ETA: Phase 2 + Phase 3");
    
    println!("\n   Press Enter to return to menu...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    
    Ok(())
}

async fn experiment_cross_device(device_count: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║         Experiment 3: Cross-Device Replication                 ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    
    println!("\n🔄 Concept:");
    println!("   Create the SAME credentials on BOTH SoloKeys without copying");
    println!("   private keys between devices.");
    println!();
    
    if device_count >= 2 {
        println!("   Your Setup: ✅ {} devices detected", device_count);
    } else {
        println!("   Your Setup: ⚠️  Only {} device detected", device_count);
        println!("                (Need 2 for this experiment)");
    }
    
    println!("\n   ┌─────────────────────────────────────────────────────────┐");
    println!("   │  Master Seed: 0xdeadbeef... (shared secret)              │");
    println!("   │                                                          │");
    println!("   │  Device 1 (/dev/hidraw5)      Device 2 (/dev/hidraw6)   │");
    println!("   │  ┌──────────────────────┐    ┌──────────────────────┐  │");
    println!("   │  │ Derive(seed, \"admin\") │    │ Derive(seed, \"admin\") │  │");
    println!("   │  │ → Private Key A      │    │ → Private Key A      │  │");
    println!("   │  │   (SAME key!)        │    │   (SAME key!)        │  │");
    println!("   │  └──────────────────────┘    └──────────────────────┘  │");
    println!("   │                                                          │");
    println!("   │  Both devices independently derive the same key!         │");
    println!("   │  Private key NEVER left either device!                   │");
    println!("   └─────────────────────────────────────────────────────────┘");
    
    println!("\n🔑 How It Works:");
    println!("   1. Generate master seed (or use existing)");
    println!("   2. For each role (admin, operator, etc.):");
    println!("      a. Derive role-specific seed: HMAC(master_seed, role)");
    println!("      b. Use hmac-secret extension on BOTH devices");
    println!("      c. Create credential from derived seed");
    println!("   3. Result: Both devices have 'the same' credentials");
    println!("      (But private keys never left hardware!)");
    
    println!("\n✅ Security Properties:");
    println!("   ✅ Private keys never exported");
    println!("   ✅ Deterministic and reproducible");
    println!("   ✅ Master seed can be backed up securely");
    println!("   ✅ Can recreate on new device if one is lost");
    println!("   ✅ Transparent to applications (same public keys)");
    
    println!("\n💡 Use Cases:");
    println!("   - Backup device (same keys on both)");
    println!("   - Load balancing (distribute across devices)");
    println!("   - Redundancy (if one fails, use the other)");
    println!("   - Disaster recovery (recreate from master seed)");
    
    println!("\n⚠️  Important:");
    println!("   Master seed must be:");
    println!("   - Generated securely (high entropy)");
    println!("   - Stored securely (encrypted backup)");
    println!("   - Never shared (compromise = both devices compromised)");
    
    println!("\n🔧 Implementation:");
    println!("   Status: ⏳ Requires CTAP2 MakeCredential + hmac-secret");
    println!("   Dependencies:");
    println!("   - GetInfo (to query hmac-secret support)");
    println!("   - MakeCredential (to create credentials)");
    println!("   - GetAssertion with hmac-secret extension");
    println!("   ETA: Phase 2 (hmac-secret is high priority)");
    
    println!("\n   Press Enter to return to menu...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    
    Ok(())
}

async fn show_implementation_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║              Implementation Status                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    
    println!("\n📊 Current Status (November 9, 2025):");
    println!();
    println!("   Phase 1: Device Discovery");
    println!("   ✅ hidapi integration");
    println!("   ✅ Device enumeration");
    println!("   ✅ Basic device info");
    println!("   ✅ Protocol detection (CTAP2)");
    println!("   ✅ Multi-device support");
    println!("   Status: COMPLETE");
    println!();
    println!("   Phase 2: CTAP2 Commands (IN PROGRESS)");
    println!("   ⏳ GetInfo (query capabilities)");
    println!("   ⏳ MakeCredential (create resident keys)");
    println!("   ⏳ GetAssertion (authenticate/sign)");
    println!("   ⏳ hmac-secret extension (entropy)");
    println!("   ⏳ User presence detection");
    println!("   Status: 0% - Starting soon");
    println!();
    println!("   Phase 3: Multi-Credential (PLANNED)");
    println!("   ⏳ Multiple credentials per device");
    println!("   ⏳ Role-based access");
    println!("   ⏳ Credential management");
    println!("   ⏳ Permission system");
    println!("   Status: 0% - Depends on Phase 2");
    println!();
    println!("   Phase 4: Genetic System (PLANNED)");
    println!("   ⏳ Hierarchical credentials");
    println!("   ⏳ Parent-child relationships");
    println!("   ⏳ Lineage tracking");
    println!("   ⏳ Sovereignty levels");
    println!("   Status: 0% - Depends on Phase 3");
    println!();
    println!("   Phase 5: Cross-Device (PLANNED)");
    println!("   ⏳ Deterministic derivation");
    println!("   ⏳ Multi-device orchestration");
    println!("   ⏳ Load balancing");
    println!("   ⏳ Failover support");
    println!("   Status: 0% - Depends on Phase 2");
    
    println!("\n🎯 Immediate Next Steps:");
    println!("   1. Implement CTAP2 GetInfo");
    println!("      → Query actual device capabilities");
    println!("      → Detect hmac-secret support");
    println!("      → Discover max credential storage");
    println!();
    println!("   2. Implement hmac-secret extension");
    println!("      → Generate hardware entropy");
    println!("      → Enable deterministic derivation");
    println!("      → Foundation for cross-device");
    println!();
    println!("   3. Implement MakeCredential");
    println!("      → Create resident keys");
    println!("      → Enable multi-credential");
    println!("      → Enable genetic hierarchies");
    
    println!("\n📅 Estimated Timeline:");
    println!("   Phase 2: 2-3 days (CTAP2 basics)");
    println!("   Phase 3: 1-2 days (Multi-credential)");
    println!("   Phase 4: 2-3 days (Genetic system)");
    println!("   Phase 5: 1-2 days (Cross-device)");
    println!("   Total: ~1-2 weeks for full implementation");
    
    println!("\n💡 What You Can Do Now:");
    println!("   ✅ Run device discovery");
    println!("   ✅ Test hardware detection");
    println!("   ✅ Verify both devices work");
    println!("   ✅ Plan your role structure");
    println!("   ✅ Design permission system");
    
    println!("\n   Press Enter to return to menu...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    
    Ok(())
}

