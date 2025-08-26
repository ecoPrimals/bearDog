

use beardog_security::recovery::*;
use beardog_security::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐻 BearDog Recovery System Demo");
    println!("================================");
    println!();

    let provider = BearDogSecurityProvider::new(SecurityProviderConfig::default()).await?;

    println!("📞 Demo 1: Social Recovery Setup");
    println!("Setting up trusted contacts for user 'alice'...");

    let trusted_contacts = vec![
        TrustedContact {
            id: "bob".to_string(),
            identifier: "bob@example.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 90,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
        TrustedContact {
            id: "charlie".to_string(),
            identifier: "charlie@example.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 85,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
        TrustedContact {
            id: "device1".to_string(),
            identifier: "hardware_token_123".to_string(),
            contact_type: ContactType::HardwareDevice,
            public_key: Some("device_pubkey".to_string()),
            trust_level: 95,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
    ];

    provider
        .setup_social_recovery("alice", trusted_contacts, 2)
        .await?;
    println!("✅ Social recovery configured: requires 2 of 3 trusted contacts");
    println!();

    println!("🌐 Demo 2: Federation Recovery Setup");
    println!("Setting up trusted BearDog instances for cross-instance recovery...");

    let trusted_instances = vec![
        TrustedInstance {
            id: "beardog_corp".to_string(),
            endpoint: "https://beardog.corp.example.com".to_string(),
            public_key: "corp_pubkey_123".to_string(),
            trust_level: 90,
            added_at: chrono::Utc::now(),
            active: true,
        },
        TrustedInstance {
            id: "beardog_university".to_string(),
            endpoint: "https://beardog.university.example.com".to_string(),
            public_key: "uni_pubkey_456".to_string(),
            trust_level: 85,
            added_at: chrono::Utc::now(),
            active: true,
        },
    ];

    provider
        .setup_federation_recovery("alice", trusted_instances, 1)
        .await?;
    println!("✅ Federation recovery configured: requires 1 of 2 trusted instances");
    println!();

    println!("🔑 Demo 3: Ephemeral Recovery Keys");
    println!("Generating time-bound, limited-use recovery keys...");

    let ephemeral_key = provider
        .generate_ephemeral_recovery_key("alice", 24, 3)
        .await?;
    println!("✅ Ephemeral key generated: {ephemeral_key}");
    println!("   - Expires in: 24 hours");
    println!("   - Max uses: 3");
    println!("   - Can unlock account: Yes");
    println!("   - Can reset password: No (limited scope)");
    println!();

    println!("🔒 Demo 4: Account Lockout Scenario");
    println!("Simulating account lockout due to failed attempts...");

    let mut lockout_config = SecurityProviderConfig::default();
    lockout_config.max_failed_attempts = 2;
    lockout_config.lockout_duration_minutes = 30;

    let lockout_provider = BearDogSecurityProvider::new(lockout_config).await?;

    let recovery_contacts = vec![TrustedContact {
        id: "emergency_contact".to_string(),
        identifier: "emergency@example.com".to_string(),
        contact_type: ContactType::Email,
        public_key: None,
        trust_level: 100,
        added_at: chrono::Utc::now(),
        last_used: None,
        active: true,
    }];

    lockout_provider
        .setup_social_recovery("locked_user", recovery_contacts, 1)
        .await?;

    for i in 1..=3 {
        let result = lockout_provider
            .authenticate("locked_user", "wrong_password")
            .await?;
        println!(
            "   Attempt {}: {}",
            i,
            if result.success { "SUCCESS" } else { "FAILED" }
        );
    }

    let result = lockout_provider
        .authenticate("locked_user", "correct_password")
        .await?;
    println!(
        "   Account status: {}",
        if result.success { "UNLOCKED" } else { "LOCKED" }
    );
    println!("   Reason: {}", result.reason);

    let recovery_session = lockout_provider
        .start_account_recovery("locked_user", RecoveryType::SocialRecovery)
        .await?;
    println!("   Recovery session started: {recovery_session}");
    println!();

    println!("🏠 Demo 5: 'Finding the key ≠ owning the house' Philosophy");
    println!("Demonstrating limited-scope recovery keys...");

    let recovery_manager = RecoveryManager::new().await?;

    let mut unlock_permissions = EphemeralPermissions::default();
    unlock_permissions.can_unlock_account = true;
    unlock_permissions.can_reset_password = false;

    let unlock_key = recovery_manager
        .generate_ephemeral_recovery_key("demo_user", unlock_permissions, 1, 1)
        .await?;

    let unlock_result = recovery_manager
        .use_ephemeral_recovery_key(&unlock_key, "demo_user", "unlock_account")
        .await?;
    println!("   ✅ Key can unlock account: {unlock_result}");

    let reset_result = recovery_manager
        .use_ephemeral_recovery_key(&unlock_key, "demo_user", "reset_password")
        .await;
    println!("   ❌ Key cannot reset password: {}", reset_result.is_err());

    println!();
    println!("🔐 Demo 6: Multi-Party Recovery");
    println!("Demonstrating that no single party can recover alone...");

    let multi_contacts = vec![
        TrustedContact {
            id: "friend1".to_string(),
            identifier: "friend1@example.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 80,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
        TrustedContact {
            id: "friend2".to_string(),
            identifier: "friend2@example.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 85,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
        TrustedContact {
            id: "family".to_string(),
            identifier: "family@example.com".to_string(),
            contact_type: ContactType::Email,
            public_key: None,
            trust_level: 90,
            added_at: chrono::Utc::now(),
            last_used: None,
            active: true,
        },
    ];

    let policy = RecoveryPolicy::default();
    recovery_manager
        .setup_social_recovery("secure_user", multi_contacts, 2, policy)
        .await?;

    let multi_session = recovery_manager
        .start_account_recovery("secure_user", RecoveryType::SocialRecovery, HashMap::with_capacity(16))
        .await?;
    println!("   Multi-party recovery session: {multi_session}");
    println!("   Requires 2 of 3 contacts to complete recovery");
    println!("   ✅ No single contact can recover the account alone");

    println!();
    println!("🎯 Key Principles Demonstrated:");
    println!("   • Distributed Trust: No single point of failure");
    println!("   • Time-Bounded: Recovery keys expire automatically");
    println!("   • Limited Scope: Keys have specific, limited permissions");
    println!("   • Multi-Party: Requires multiple trusted parties");
    println!("   • Ephemeral: Keys can only be used limited times");
    println!("   • Federated: Cross-instance recovery for network effects");
    println!();
    println!("💡 'Finding the key ≠ owning the house'");
    println!("   Recovery helpers don't gain ownership of your data");
    println!("   Keys have limited scope and expire automatically");
    println!("   Multiple parties required for sensitive operations");

    Ok(())
}
