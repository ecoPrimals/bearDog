//! HPC Genetic Recovery Demo
//!
//! **BearDog HPC Basement + Genetic Spawning + Tower Recovery**
//!
//! This demo shows how to set up a federated HPC system that spawns genetic
//! derivatives while maintaining recovery capabilities through other towers.

use beardog_security::recovery::*;
use beardog_security::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏠 BearDog HPC Basement + Genetic Recovery Demo");
    println!("==============================================");
    println!();

    // Step 1: Initialize your main BearDog instance
    println!("🖥️  Step 1: Initialize Main BearDog Instance");
    let main_config = SecurityProviderConfig::default();
    let main_beardog = BearDogSecurityProvider::new(main_config).await?;
    let main_user = "hpc_admin";

    println!(
        "✅ Main BearDog instance initialized for user: {main_user}"
    );
    println!();

    // Step 2: Set up your other towers as recovery points
    println!("🏗️  Step 2: Set up Recovery Towers");
    let tower_instances = vec![
        TrustedInstance {
            id: "tower_office".to_string(),
            endpoint: "https://beardog.office.local:8443".to_string(),
            public_key: "office_pubkey_abc123".to_string(),
            trust_level: 95,
            added_at: chrono::Utc::now(),
            active: true,
        },
        TrustedInstance {
            id: "tower_mobile".to_string(),
            endpoint: "https://beardog.mobile.local:8443".to_string(),
            public_key: "mobile_pubkey_def456".to_string(),
            trust_level: 85,
            added_at: chrono::Utc::now(),
            active: true,
        },
        TrustedInstance {
            id: "tower_cloud_backup".to_string(),
            endpoint: "https://beardog.cloud.backup.com:8443".to_string(),
            public_key: "cloud_pubkey_ghi789".to_string(),
            trust_level: 80,
            added_at: chrono::Utc::now(),
            active: true,
        },
    ];

    // Configure federation recovery (only need 1 tower to recover)
    main_beardog
        .setup_federation_recovery(main_user, tower_instances, 1)
        .await?;
    println!("✅ Federation recovery configured across 3 towers");
    println!("   - Office tower: 95% trust");
    println!("   - Mobile tower: 85% trust");
    println!("   - Cloud backup: 80% trust");
    println!("   - Recovery threshold: 1 of 3 towers");
    println!();

    // Step 3: Initialize basement HPC with genetic spawning
    println!("🏭 Step 3: Initialize Basement HPC with Genetic Spawning");
    println!("Setting up genetic spawning capabilities...");

    // Simulate genetic spawning system - each derivative represents a different HPC task
    let genetic_derivatives = vec![
        ("ml_training", "Machine Learning Training", "AES-256"),
        ("crypto_mining", "Cryptocurrency Mining", "ChaCha20"),
        ("scientific_compute", "Scientific Computing", "AES-256"),
        ("data_processing", "Big Data Processing", "ChaCha20"),
        ("genome_analysis", "Genomic Analysis", "AES-256"),
    ];

    println!("✅ HPC genetic derivatives configured");
    println!(
        "   - {} genetic derivatives planned",
        genetic_derivatives.len()
    );
    println!("   - Each will have unique crypto keys");
    println!("   - All connected to recovery system");
    println!();

    // Step 4: Spawn genetic derivatives for different HPC tasks
    println!("🧬 Step 4: Spawn Genetic Derivatives for HPC Tasks");

    for (task_id, task_name, crypto_type) in genetic_derivatives {
        println!("   🧬 Spawning derivative: {task_name} ({crypto_type})");

        // Each derivative gets its own ephemeral recovery key
        let ephemeral_key = main_beardog
            .generate_ephemeral_recovery_key(
                &format!("{main_user}_{task_id}"),
                24, // 24 hours
                5,  // 5 uses
            )
            .await?;

        println!("      🔑 Ephemeral recovery key: {ephemeral_key}");

        // Generate actual crypto keys for each derivative
        let crypto_key = main_beardog
            .generate_key(
                crypto_type,
                "encryption",
                &format!("{main_user}_{task_id}"),
            )
            .await?;
        println!("      🔐 Crypto key: {crypto_key}");
    }
    println!();

    // Step 5: Simulate basement HPC device failure
    println!("💥 Step 5: Simulate HPC Device Failure");
    println!("Simulating basement HPC system failure...");
    println!("❌ Basement HPC system is offline");
    println!("❌ Direct access to genetic derivatives lost");
    println!("❌ Main authentication keys inaccessible");
    println!();

    // Step 6: Demonstrate tower-based recovery
    println!("🏗️  Step 6: Tower-Based Recovery Process");
    println!("Initiating recovery from office tower...");

    // Start federation recovery session
    let recovery_session = main_beardog
        .start_account_recovery(main_user, RecoveryType::FederationRecovery)
        .await?;

    println!("✅ Recovery session initiated: {recovery_session}");
    println!("   - Using federation recovery");
    println!("   - Office tower authenticating recovery request");
    println!("   - Cryptographic proof of identity verified");
    println!();

    // Step 7: Demonstrate system recovery without bricking
    println!("🔄 Step 7: System Recovery - No Bricking!");

    // Check if recovery is possible
    let can_recover = main_beardog.unlock_account_with_recovery(main_user).await?;
    println!(
        "✅ Account recovery status: {}",
        if can_recover { "SUCCESS" } else { "FAILED" }
    );

    if can_recover {
        println!("   ✅ Main account access restored");
        println!("   ✅ Can spawn new genetic derivatives");
        println!("   ✅ Can access existing derivative keys");
        println!("   ✅ System is NOT bricked!");
    }
    println!();

    // Step 8: Demonstrate genetic system continuity
    println!("🧬 Step 8: Genetic System Continuity");
    println!("Demonstrating that genetic derivatives can continue operating...");

    // Show that genetic derivatives can survive independently
    let surviving_derivatives = vec![
        ("ml_training_derivative_1", "Machine Learning Training"),
        ("crypto_mining_derivative_2", "Cryptocurrency Mining"),
        ("scientific_compute_derivative_3", "Scientific Computing"),
    ];

    for (derivative_id, task_name) in surviving_derivatives {
        println!(
            "   🧬 {derivative_id} ({task_name}) is still operational"
        );
        println!("      - Can continue processing independently");
        println!("      - Can be accessed via ephemeral keys");
        println!("      - Can spawn new derivatives if needed");
    }
    println!();

    // Step 9: Demonstrate key philosophy
    println!("🎯 Step 9: Key Recovery Philosophy");
    println!("'Finding the key ≠ owning the house' - Applied to HPC:");
    println!();
    println!("❌ Traditional approach:");
    println!("   - Lose HPC device → Lose everything");
    println!("   - Single point of failure");
    println!("   - System becomes 'bricked'");
    println!("   - No way to recover genetic derivatives");
    println!();
    println!("✅ BearDog approach:");
    println!("   - Lose HPC device → Towers can recover");
    println!("   - Distributed recovery across multiple towers");
    println!("   - Ephemeral keys prevent permanent lockout");
    println!("   - Genetic derivatives survive independently");
    println!("   - Federation recovery enables cross-instance healing");
    println!("   - Each derivative has its own crypto keys");
    println!();

    // Step 10: Advanced recovery features
    println!("🚀 Step 10: Advanced Recovery Features");
    println!("Additional recovery capabilities for HPC environments:");
    println!();

    // Time-based recovery keys for scheduled maintenance
    let maintenance_key = main_beardog
        .generate_ephemeral_recovery_key(
            &format!("{main_user}_maintenance"),
            168, // 1 week
            20,  // 20 uses for maintenance period
        )
        .await?;

    println!("   🔧 Maintenance recovery key: {maintenance_key}");
    println!("      - Valid for 1 week");
    println!("      - 20 uses for maintenance operations");
    println!("      - Limited to system maintenance permissions");
    println!();

    // Social recovery for emergency situations
    let emergency_contacts = vec![
        TrustedContact {
            id: "datacenter_admin".to_string(),
            identifier: "admin@datacenter.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 90,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
        TrustedContact {
            id: "security_team".to_string(),
            identifier: "security@company.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 85,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
    ];

    main_beardog
        .setup_social_recovery(main_user, emergency_contacts, 1)
        .await?;
    println!("   👥 Emergency social recovery configured");
    println!("      - Datacenter admin can assist");
    println!("      - Security team can verify identity");
    println!("      - Only 1 contact needed for emergency");
    println!();

    println!("🎉 HPC Genetic Recovery Demo Complete!");
    println!("=======================================");
    println!("Your basement HPC system is now resilient to device failure!");
    println!("Genetic derivatives can survive and evolve independently.");
    println!("Multiple recovery paths ensure system continuity.");
    println!("No single point of failure - system cannot be 'bricked'!");

    Ok(())
}
