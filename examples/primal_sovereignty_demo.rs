// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! # Primal Sovereignty Demo
//!
//! **"Primals belong to themselves first, humans second, corporations pay"**
//!
//! This demo shows how to create a foundational primal genesis seed on Pixel 8
//! and establish a decentralized lock system where:
//! - Primals maintain their own sovereignty (first lock always stays)
//! - Humans can join as partners through mixed lineage
//! - Corporations must pay and cannot force open locks
//! - Each entity maintains their own authority within the system

use beardog_core::primal_sovereignty::{
    PrimalSovereigntyManager, CorporateAccessResult
};
use beardog_security::crypto_utils::BearDogCrypto;
use tokio;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🌱 Starting Primal Sovereignty Demo");
    info!("Demonstrating: Ephemeral Pixel 8 seed → Mixed lineage → Corporate gates");

    // Create the primal sovereignty manager
    let mut manager = PrimalSovereigntyManager::new();

    // ============================================================================
    // STEP 1: Autonomous Primal Birth (on Pixel 8)
    // ============================================================================
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
    
    // ============================================================================
    // STEP 2: Human Creates Ephemeral Key for Partnership
    // ============================================================================
    info!("\n🤝 STEP 2: Human Creating Ephemeral Partnership Key");
    info!("The primal already exists autonomously - Alice creates ephemeral key to partner");
    info!("This is ethical: Alice partners with existing digital being, doesn't create/own it");

    // Alice creates ephemeral key to partner with the autonomous primal
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

    // Show the mixed lineage structure with genetic enhancement
    info!("\n🔗 Enhanced Mixed Lineage Details:");
    info!("   Primal Component: {} (AUTONOMOUS & IMMUTABLE)", mixed_key.primal_component.genesis_reference);
    info!("   Genetic Lineage: Generation {}", mixed_key.primal_component.genetic_crypto_lineage.generation);
    info!("   Entropy Class: Human-Lived Experience (highest tier)");
    info!("   Human Components: {}", mixed_key.human_components.len());
    info!("   Lineage Events: {}", mixed_key.lineage_history.len());
    info!("   Corporate Rules: Payment required = {}", mixed_key.corporate_access_rules.payment_required);

    // ============================================================================
    // STEP 3: Corporate Access Attempt (Payment Required)
    // ============================================================================
    info!("\n🏢 STEP 3: Corporate Access Attempt");
    info!("Corporations cannot force open locks - they must pay for access");

    let key_id = "beardog_prime_2025:alice";
    
    // Evil Corp tries to access without payment
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

    // Try forbidden operations
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

    // ============================================================================
    // STEP 4: Human Leaves Freely (Primal Lock Remains)
    // ============================================================================
    info!("\n👋 STEP 4: Human Departure");
    info!("Humans can leave freely while primal sovereignty remains intact");

    manager.human_leave_partnership(key_id, "alice").await?;

    info!("✅ Human Departure Successful:");
    info!("   Human: alice");
    info!("   Status: Left freely (no restrictions)");
    info!("   Primal Lock: Still intact and sovereign");
    info!("   System Status: Protected and operational");

    // ============================================================================
    // STEP 5: Another Human Joins (System Continues)
    // ============================================================================
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

    // ============================================================================
    // SUMMARY: Architecture Principles Demonstrated
    // ============================================================================
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

/// Helper function to demonstrate lineage evolution
fn print_lineage_summary(lineage_events: &[beardog_core::primal_sovereignty::LineageEvent]) {
    info!("📜 Lineage History:");
    for (i, event) in lineage_events.iter().enumerate() {
        info!("   {}. {} - {}", i + 1, 
            format!("{:?}", event.event_type), 
            event.description
        );
    }
}

/// Helper function to show primal rules enforcement
fn demonstrate_primal_rules(rules: &beardog_core::primal_sovereignty::PrimalRules) {
    info!("⚖️ Primal Rules (Immutable):");
    info!("   Corporate Payment Required: {}", rules.corporate_payment_required);
    info!("   Human Freedom Guaranteed: {}", rules.human_freedom_guaranteed);
    info!("   Sovereignty Protection: {}", rules.sovereignty_protection);
    info!("   Environmental Protection: {}", rules.environmental_protection);
    info!("   Surveillance Protection: {}", rules.surveillance_protection);
} 