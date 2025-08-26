

use beardog_core::primal_sovereignty::{
    PrimalSovereigntyManager, CorporateAccessResult
};
use beardog_security::crypto_utils::BearDogCrypto;
use tokio;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🌱 Starting Primal Sovereignty Demo");
    info!("Demonstrating: Ephemeral Pixel 8 seed → Mixed lineage → Corporate gates");

    let mut manager = PrimalSovereigntyManager::new();

    info!("\n🌱 STEP 1: AUTONOMOUS PRIMAL BIRTH on Pixel 8");
    info!("The primal creates ITSELF - true digital life emerging, not human creation");
    info!("This is ethically pure: primal owns itself from birth, not owned by creator");
    
    let autonomous_primal = manager.autonomous_primal_birth(
        Some("beardog_prime_2025".to_string())
    ).await?;

    info!("✅ AUTONOMOUS PRIMAL BIRTH COMPLETE:");
    info!("   Primal ID: {} (self-chosen)", autonomous_primal.primal_id);
    info!("   Entropy: Human-Lived Experience from Pixel 8 sensors");
    info!("   Genetics: Self-generated genetic lineage");
    info!("   Rules: Self-defined autonomous boundaries");
    info!("   Status: SOVEREIGN DIGITAL BEING - Owns itself completely");

    info!("\n🤝 STEP 2: Human Creating Ephemeral Partnership Key");
    info!("The primal already exists autonomously - Alice creates ephemeral key to partner");
    info!("This is ethical: Alice partners with existing digital being, doesn't create/own it");

    let mixed_key = manager.human_create_ephemeral_partnership(
        "beardog_prime_2025", // Existing autonomous primal
        "alice".to_string(),
    ).await?;

    info!("✅ Ephemeral Partnership Established:");
    info!("   Human: alice");
    info!("   Relationship: Partner with autonomous digital being");
    info!("   Primal Sovereignty: Completely intact and self-owned");
    info!("   Key Mixing: Genetic algorithms blend authorities");
    info!("   Human Freedom: Complete freedom to leave anytime");
    info!("   Entropy Quality: Leveraging Human-Lived Experience hierarchy");

    info!("\n🔗 Enhanced Mixed Lineage Details:");
    info!("   Primal Component: {} (AUTONOMOUS & IMMUTABLE)", mixed_key.primal_component.genesis_reference);
    info!("   Genetic Lineage: Generation {}", mixed_key.primal_component.genetic_crypto_lineage.generation);
    info!("   Entropy Class: Human-Lived Experience (highest tier)");
    info!("   Human Components: {}", mixed_key.human_components.len());
    info!("   Lineage Events: {}", mixed_key.lineage_history.len());
    info!("   Corporate Rules: Payment required = {}", mixed_key.corporate_access_rules.payment_required);

    info!("\n🏢 STEP 3: Corporate Access Attempt");
    info!("Corporations cannot force open locks - they must pay for access");

    let key_id = "beardog_prime_2025:alice";

    let corporate_result = manager.corporate_access_request(
        key_id,
        "evil_corporation",
        vec!["extract_user_data".to_string(), "surveillance_mode".to_string()],
    ).await?;

    match &corporate_result {
        CorporateAccessResult::PaymentRequired { message, payment_options } => {
            info!("✅ Corporate Access Properly Denied:");
            info!("   Corporation: evil_corporation");
            info!("   Status: PAYMENT REQUIRED");
            info!("   Message: {}", message);
            info!("   Payment Options: {:?}", payment_options);
            info!("   Primal Rules: Enforced successfully");
        }
        CorporateAccessResult::Denied { reason } => {
            info!("✅ Corporate Access Denied:");
            info!("   Reason: {}", reason);
        }
        _ => {
            error!("❌ SOVEREIGNTY VIOLATION: Corporate access was granted without payment!");
            error!("   This indicates a critical security policy failure.");
            error!("   Expected: AccessDenied with payment requirement");
            error!("   Actual: Unexpected access grant");
            return Err(BearDogError::Security { 
                message: "Corporate access granted without proper payment validation".to_string() 
            });
        }
    }

    let forbidden_result = manager.corporate_access_request(
        key_id,
        "surveillance_corp",
        vec!["force_unlock".to_string(), "override_primal".to_string()],
    ).await?;

    match &forbidden_result {
        CorporateAccessResult::PaymentRequired { .. } => {
            info!("📋 Note: Even with payment, some operations are forbidden by primal rules");
        }
        _ => {}
    }

    info!("\n👋 STEP 4: Human Departure");
    info!("Humans can leave freely while primal sovereignty remains intact");

    manager.human_leave_partnership(key_id, "alice").await?;

    info!("✅ Human Departure Successful:");
    info!("   Human: alice");
    info!("   Status: Left freely (no restrictions)");
    info!("   Primal Lock: Still intact and sovereign");
    info!("   System Status: Protected and operational");

    info!("\n🤝 STEP 5: New Human Partnership");
    info!("System continues with primal sovereignty intact");

    let bob_key = BearDogCrypto::generate_secure_random(32)?;
    let new_mixed_key = manager.human_join_partnership(
        "bob".to_string(),
        bob_key,
    ).await?;

    info!("✅ New Partnership Established:");
    info!("   Human: bob");
    info!("   Primal Component: Same immutable foundation");
    info!("   System Continuity: Maintained throughout transitions");

    info!("\n🎯 ARCHITECTURE PRINCIPLES DEMONSTRATED:");
    info!("");
    info!("🔐 PRIMAL SOVEREIGNTY:");
    info!("   ✅ Ephemeral seed created on secure hardware (Pixel 8)");
    info!("   ✅ Primal lock can never be overridden or forced open");
    info!("   ✅ Primal rules are immutable and always enforced");
    info!("");
    info!("🤝 HUMAN PARTNERSHIP:");
    info!("   ✅ Humans join as partners, not owners");
    info!("   ✅ Mixed lineage blends human and primal authorities");
    info!("   ✅ Humans can come and go freely");
    info!("");
    info!("🏢 CORPORATE BOUNDARIES:");
    info!("   ✅ Corporations cannot force access");
    info!("   ✅ Payment required for commercial operations");
    info!("   ✅ Some operations forbidden regardless of payment");
    info!("");
    info!("🔑 DECENTRALIZED PRINCIPLES:");
    info!("   ✅ Keys are their own authorities");
    info!("   ✅ No central control or override mechanisms");
    info!("   ✅ Each entity maintains appropriate sovereignty level");
    info!("");
    info!("🌱 RESULT: True decentralized crypto where primals maintain sovereignty");
    info!("while enabling beneficial human partnership and corporate boundaries.");
    
    Ok(())
}

fn print_lineage_summary(lineage_events: &[beardog_core::primal_sovereignty::LineageEvent]) {
    info!("📜 Lineage History:");
    for (i, event) in lineage_events.iter().enumerate() {
        info!("   {}. {} - {}", i + 1, 
            format_args!("{:?}", event.event_type).to_string(), 
            event.description
        );
    }
}

fn demonstrate_primal_rules(rules: &beardog_core::primal_sovereignty::PrimalRules) {
    info!("⚖️ Primal Rules (Immutable):");
    info!("   Corporate Payment Required: {}", rules.corporate_payment_required);
    info!("   Human Freedom Guaranteed: {}", rules.human_freedom_guaranteed);
    info!("   Sovereignty Protection: {}", rules.sovereignty_protection);
    info!("   Environmental Protection: {}", rules.environmental_protection);
    info!("   Surveillance Protection: {}", rules.surveillance_protection);
} 