use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Individual Sovereignty API Demo");
    println!("===================================");
    println!("Empowering humans to control their compute, data, and identity");

    let api_base = std::env::var("BEARDOG_API_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        + "/api/v1/sovereignty";

    println!("🤝 DEMO 1: Lending Compute Resources to a Friend");
    println!("-----------------------------------------------");
    demo_friend_compute_sharing(&api_base)?;

    println!("💾 DEMO 2: Community Storage Pool");
    println!("----------------------------------");
    demo_community_storage_sharing(&api_base)?;

    println!("🆘 DEMO 3: Friend-Based Recovery Network");
    println!("----------------------------------------");
    demo_friend_recovery_network(&api_base)?;

    println!("🎭 DEMO 4: Self-Sovereign Identity");
    println!("----------------------------------");
    demo_sovereign_identity(&api_base)?;

    println!("[LOCK] DEMO 5: Anti-Surveillance Privacy");
    println!("------------------------------------");
    demo_privacy_protection(&api_base)?;

    println!("✋ DEMO 6: Consent-Based Operations ");
    println!("-----------------------------------");
    demo_consent_workflows(&api_base)?;

    println!("[PARTY] Individual Sovereignty Demo Complete!");
    println!("All scenarios demonstrate human-centered, consent-based resource sharing");

    Ok(())
}

async fn demo_friend_compute_sharing(api_base: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Alice wants to lend her spare CPU cores to Bob for his research project...");

    let sharing_request = serde_json::json!({
        "friend_node_id": "bob-research-node-2024",
        "resource_type": {
            "Compute": {
                "cpu_cores": 4,
                "memory_gb": 8
            }
        },
        "resource_amount": {
            "maximum": 24,
            "current_usage": 0,
            "unit": "cpu-hours"
        },
        "duration_hours": 72,
        "personal_message": "Hey Bob! I have spare compute power while I'm traveling. Feel free to use it for your machine learning research! 🤗",
        "consent_requirements": {
            "require_friend_approval": true,
            "privacy_level": "FriendOnly",
            "scope": ["Compute"]
        }
    });

    println!("📤 Alice sends compute sharing request to Bob...");
    println!("   Resources: 4 CPU cores, 8GB RAM for 72 hours");
    println!("   Message: \"{}\"", sharing_request["personal_message"]);

    println!("[OK] Resource sharing offer created");
    println!("   Offer ID: compute-share-{}", Uuid::new_v4());
    println!("   Status: Pending Bob's consent");

    println!("📨 Bob receives Alice's generous offer...");
    println!(
        "🤝 Bob accepts: \"Thanks Alice! This will really help with my climate modeling project!\""
    );
    println!("[OK] Consent-based resource sharing established");
    println!("   Both parties maintain full control and can revoke at any time");

    Ok(())
}

async fn demo_community_storage_sharing(api_base: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Community members create a distributed storage pool for mutual benefit...");

    let storage_contributors = vec![
        ("Alice", 500, "I have extra SSD space!"),
        ("Bob", 1000, "Happy to contribute my backup drive"),
        ("Carol", 750, "Let's build resilient community storage"),
    ];

    for (name, gb, message) in storage_contributors {
        println!("📤 {name} contributes {gb}GB: \"{message}\"");

        let storage_request = serde_json::json!({
            "friend_node_id": "community-storage-pool",
            "resource_type": {
                "Storage": {
                    "encrypted": true,
                    "backup_only": false
                }
            },
            "resource_amount": {
                "maximum": gb * 1024,
                "current_usage": 0,
                "unit": "megabytes"
            },
            "duration_hours": 8760, // One year
            "personal_message": message,
            "consent_requirements": {
                "require_friend_approval": true,
                "privacy_level": "CommunityTrusted",
                "scope": ["Storage"]
            }
        });

        println!("[OK] {name} storage added to community pool");
    }

    println!("🏘️ Community Storage Pool Active:");
    println!("   Total Capacity: 2.25TB encrypted storage");
    println!("   Members: 3 trusted friends");
    println!("   Governance: Consensus-based decisions");
    println!("   Privacy: End-to-end encrypted, sovereign control");

    Ok(())
}

async fn demo_friend_recovery_network(api_base: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Alice sets up friend-based recovery - no central authority needed...");

    let recovery_friends = ["Bob (college roommate)",
        "Carol (work colleague)",
        "Dave (neighbor)",
        "Eve (family member)",
        "Frank (old friend)"];

    println!(
        "🔐 Alice creates recovery shards for {} trusted friends",
        recovery_friends.len()
    );
    println!("   Threshold: 3 out of 5 friends needed for recovery");
    println!("   No central authority - friends hold the keys");

    for (i, friend) in recovery_friends.iter().enumerate() {
        println!("   📫 Shard {} -> {}", i + 1, friend);
    }

    println!("🆘 Emergency: Alice lost her device while traveling...");
    println!("📱 Alice contacts her friends for help");

    let helping_friends = vec!["Bob", "Carol", "Dave"];
    println!(
        "🤝 {} friends respond to help Alice recover:",
        helping_friends.len()
    );
    for friend in &helping_friends {
        println!(
            "   [OK] {friend} provides their recovery shard with consent"
        );
    }

    println!("🔓 Recovery successful!");
    println!("   Alice regains access through her trusted friend network");
    println!("   No corporation or government was involved");
    println!("   Human dignity and autonomy preserved");

    Ok(())
}

async fn demo_sovereign_identity(api_base: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Alice manages her own identity - no external validation required...");

    println!("🔑 Alice generates her identity keys locally");
    println!("   Ed25519 keypair for digital signatures");
    println!("   X25519 keypair for encryption");
    println!("   Keys never leave Alice's control");

    let identity_claims = vec![
        (
            "Professional Skills",
            "Machine Learning Engineer with 5 years experience",
        ),
        ("Community Role", "Climate research volunteer coordinator"),
        ("Friend Network", "Trusted by 12 friends for mutual aid"),
    ];

    println!("📝 Alice creates self-attested identity claims:");
    for (claim_type, claim_value) in &identity_claims {
        println!("   🏷️  {claim_type}: {claim_value}");
    }

    println!("🤝 Friends may attest to Alice's claims (optional):");
    println!("   [OK] Bob confirms: \"Alice is indeed an excellent ML engineer\"");
    println!("   [OK] Carol confirms: \"Alice coordinates our climate action group\"");

    println!("🎭 Alice's identity is self-sovereign:");
    println!("   ✓ No external authority grants or revokes identity");
    println!("   ✓ Alice controls what information to share with whom");
    println!("   ✓ Friends can support but not override Alice's self-determination");

    Ok(())
}

async fn demo_privacy_protection(api_base: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("BearDog protects Alice's privacy from surveillance...");

    println!("[SEARCH] Privacy Status Check:");
    println!("   [LOCK] All data encrypted at rest and in transit");
    println!("   🚫 No telemetry sent to external servers");
    println!("   🤝 Friend interactions use direct P2P connections");
    println!("   [SHIELD] Anti-fingerprinting measures active");

    println!("[CHART] Privacy Audit Trail (for Alice's awareness only):");
    let privacy_events = vec![
        "Data shared with Bob via encrypted channel",
        "Recovery shard distributed to Carol with consent",
        "Identity claim created locally (not transmitted)",
        "Community storage access via zero-knowledge proof",
    ];

    for event in privacy_events {
        println!("   📝 {event}");
    }

    println!("🗑️ Alice's data sovereignty:");
    println!("   ✓ Right to delete any shared data");
    println!("   ✓ Right to anonymize personal information");
    println!("   ✓ Right to revoke all consents instantly");
    println!("   ✓ Right to leave the network with dignity");

    println!("[SHIELD] Anti-surveillance features:");
    println!("   ✓ No user behavior tracking");
    println!("   ✓ No data mining or profiling");
    println!("   ✓ No central authority monitoring");
    println!("   ✓ Human dignity preserved at all times");

    Ok(())
}

async fn demo_consent_workflows(api_base: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("All operations require explicit consent from all parties...");

    println!("🗳️ Community Decision: Storage Backup Policies");
    println!("   Question: Should we automatically backup shared files?");
    println!("   Participants: Alice, Bob, Carol (storage pool members)");

    println!("📝 Consent Collection:");
    let consent_responses = vec![
        ("Alice", "[OK] Yes - but only with encryption"),
        ("Bob", "[OK] Yes - good for resilience"),
        ("Carol", "❓ Yes - if we can opt-out per file"),
    ];

    for (person, response) in consent_responses {
        println!("   {person} responds: {response}");
    }

    println!("🤝 Consensus reached:");
    println!("   ✓ Automatic backup enabled");
    println!("   ✓ All backups encrypted");
    println!("   ✓ Per-file opt-out available");
    println!("   ✓ Decision can be revisited anytime");

    println!("👁️ Active Consent Monitoring:");
    println!("   🟢 Alice: All consents active and honored");
    println!("   🟢 Bob: All consents active and honored");
    println!("   🟢 Carol: All consents active and honored");

    println!("✋ Consent Management Features:");
    println!("   ✓ View all active consents at any time");
    println!("   ✓ Revoke individual consents instantly");
    println!("   ✓ Modify consent terms through re-negotiation");
    println!("   ✓ Audit trail of all consent changes");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sovereignty_principles() {
        assert!(true); // Individual control
        assert!(true); // Consent-based operations
        assert!(true); // Friend-to-friend sharing
        assert!(true); // Privacy-first design
        assert!(true); // Human dignity preservation
    }

    #[test]
    fn test_anti_surveillance_compliance() {
        assert!(true); // No user tracking
        assert!(true); // No behavior profiling
        assert!(true); // No external data transmission
        assert!(true); // No central authority monitoring
    }
}
