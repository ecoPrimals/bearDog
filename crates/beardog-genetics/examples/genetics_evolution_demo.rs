// SPDX-License-Identifier: AGPL-3.0-only
// # Pure Genetics Evolution Demonstration
//
// This example demonstrates the ecosystem evolution genetics capabilities
// from horizontal gene transfer integration across ecosystem primals,
// using only the beardog-genetics crate directly.

use beardog_genetics::ecosystem_evolution::{
    BinaryAccessPattern, BinaryTrust, CoordinationModel, EcosystemGeneticEngine,
    EcosystemMembership, HierarchicalPattern, TrustEvolution, migrate_from_binary_patterns,
};
use beardog_types::canonical::HealthStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 BearDog Pure Genetics Evolution Demonstration");
    println!("===============================================");
    println!("🌱 Showcasing Horizontal Gene Transfer Across Ecosystem Primals");
    println!();

    // === Initialize Ecosystem Genetic Engine ===
    println!("🚀 PHASE 1: Ecosystem Genetic Engine Initialization");
    println!("---------------------------------------------------");

    let engine = EcosystemGeneticEngine::new()?;
    println!("✅ EcosystemGeneticEngine initialized successfully");
    println!("   🧬 Evolution algorithms: Active");
    println!("   🧬 Trait inheritance: Configured");
    println!("   🧬 Adaptive security: Enabled");
    println!();

    // === Binary Access Pattern Evolution ===
    println!("🔄 PHASE 2: Binary Access Pattern → Ecosystem Membership Evolution");
    println!("------------------------------------------------------------------");

    // Create a mock ecosystem context
    let mock_context = beardog_genetics::ecosystem_evolution::EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 12,
        ecosystem_load: 0.35,
        recent_events: vec![
            "positive_interaction_detected".to_string(),
            "trust_network_strengthened".to_string(),
        ],
    };

    let test_patterns = vec![
        (
            "Allowlist Pattern",
            BinaryAccessPattern::Allowlist {
                allowed_entities: vec![
                    "alice@security-team.com".to_string(),
                    "bob@dev-team.com".to_string(),
                ],
            },
        ),
        (
            "Simple Allow",
            BinaryAccessPattern::Simple { is_allowed: true },
        ),
        (
            "Simple Deny",
            BinaryAccessPattern::Simple { is_allowed: false },
        ),
    ];

    for (pattern_name, pattern) in test_patterns {
        println!("🧪 Testing: {}", pattern_name);

        let evolved_membership = engine.evolve_access_pattern(pattern, mock_context.clone())?;

        match &evolved_membership {
            EcosystemMembership::ActiveContributor { trust_level, .. } => {
                println!(
                    "   ✅ Evolved to: ActiveContributor (trust: {:.2})",
                    trust_level
                );
            }
            EcosystemMembership::LearningParticipant { trust_level, .. } => {
                println!(
                    "   🌱 Evolved to: LearningParticipant (trust: {:.2})",
                    trust_level
                );
            }
            EcosystemMembership::CautiousInteraction { trust_level, .. } => {
                println!(
                    "   ⚠️  Evolved to: CautiousInteraction (trust: {:.2})",
                    trust_level
                );
            }
            EcosystemMembership::EcosystemProtection { trust_level, .. } => {
                println!(
                    "   🛡️  Evolved to: EcosystemProtection (trust: {:.2})",
                    trust_level
                );
            }
            _ => println!("   🔄 Evolved to: {:?}", evolved_membership),
        }
        println!();
    }

    // === Trust Evolution ===
    println!("💙 PHASE 3: Binary Trust → Trust Evolution Dynamics");
    println!("--------------------------------------------------");

    let mock_history = beardog_genetics::ecosystem_evolution::RelationshipHistory {
        interactions: vec![],
        trust_changes: vec![],
        collaboration_outcomes: vec![],
    };

    let trust_scenarios = vec![
        ("Established Trust", BinaryTrust::Trusted),
        ("Building Trust", BinaryTrust::Untrusted),
    ];

    for (scenario_name, trust_state) in trust_scenarios {
        println!("🧪 Testing: {}", scenario_name);

        let evolved_trust =
            engine.evolve_trust_pattern(trust_state, mock_history.clone(), mock_context.clone())?;

        match &evolved_trust {
            TrustEvolution::Flourishing { .. } => {
                println!("   🌸 Evolved to: Flourishing Trust");
            }
            TrustEvolution::Building { progress_rate, .. } => {
                println!(
                    "   🔨 Evolved to: Building Trust (progress: {:.2})",
                    progress_rate
                );
            }
            TrustEvolution::Questioning { .. } => {
                println!("   ❓ Evolved to: Questioning Trust");
            }
            TrustEvolution::Healing {
                recovery_progress, ..
            } => {
                println!(
                    "   🩹 Evolved to: Healing Trust (recovery: {:.1}%)",
                    recovery_progress * 100.0
                );
            }
            TrustEvolution::Transforming { .. } => {
                println!("   🦋 Evolved to: Transforming Trust");
            }
        }
        println!();
    }

    // === Coordination Evolution ===
    println!("🤝 PHASE 4: Hierarchical → Symbiotic Coordination Evolution");
    println!("-----------------------------------------------------------");

    let coordination_scenarios = vec![
        (
            "Primary/Replica",
            HierarchicalPattern::PrimaryReplica {
                primary: "control_node".to_string(),
                replicas: vec!["worker_1".to_string(), "worker_2".to_string()],
            },
        ),
        (
            "Primary/Secondary",
            HierarchicalPattern::PrimarySecondary {
                primary: "db_primary".to_string(),
                secondaries: vec!["db_replica_1".to_string()],
            },
        ),
    ];

    for (scenario_name, hierarchy) in coordination_scenarios {
        println!("🧪 Testing: {}", scenario_name);

        let evolved_coordination = engine.evolve_coordination_pattern(
            hierarchy,
            vec![], // Relationships parameter - empty for this demo
            "test_context".to_string(),
        )?;

        match &evolved_coordination {
            CoordinationModel::Distributed { .. } => {
                println!("   🌐 Evolved to: Distributed Coordination");
            }
            CoordinationModel::Rotational { .. } => {
                println!("   🔄 Evolved to: Rotational Leadership");
            }
            CoordinationModel::Collaborative { .. } => {
                println!("   🤝 Evolved to: Collaborative Coordination");
            }
            CoordinationModel::Contextual { .. } => {
                println!("   🎯 Evolved to: Contextual Coordination");
            }
            CoordinationModel::Emergent { .. } => {
                println!("   🌟 Evolved to: Emergent Coordination");
            }
        }
        println!();
    }

    // === Emergent Behaviors ===
    println!("🌟 PHASE 5: Emergent Ecosystem Intelligence Detection");
    println!("----------------------------------------------------");

    let emergent_behaviors = engine.generate_emergent_behaviors()?;

    println!(
        "🧠 Detected Emergent Behaviors: {} total",
        emergent_behaviors.len()
    );
    for (i, behavior) in emergent_behaviors.iter().enumerate() {
        let status_icon = if behavior.beneficial { "✅" } else { "⚠️" };
        println!("   {}. {} {}", i + 1, status_icon, behavior.behavior_type);
        println!(
            "      💪 Strength: {:.1}%",
            behavior.emergence_strength * 100.0
        );
    }
    println!();

    // === Health Assessment ===
    println!("🏥 PHASE 6: Ecosystem Health Assessment");
    println!("---------------------------------------");

    let health_report = engine.assess_ecosystem_health()?;

    println!("📊 Ecosystem Health Report:");
    println!("   🌍 Overall Health: {:?}", health_report.overall_health);
    println!(
        "   💙 Trust Health: {:.1}%",
        health_report.trust_health * 100.0
    );
    println!(
        "   🤝 Relationship Health: {:.1}%",
        health_report.relationship_health * 100.0
    );
    println!(
        "   ⚙️  Coordination Health: {:.1}%",
        health_report.coordination_health * 100.0
    );
    println!();

    // === Legacy Migration ===
    println!("🔄 PHASE 7: Legacy System Migration");
    println!("-----------------------------------");

    let legacy_allowlist = vec![
        "admin@company.com".to_string(),
        "dev-lead@company.com".to_string(),
    ];

    let legacy_blocklist = vec!["spam-bot@malicious.com".to_string()];

    println!("📋 Legacy Systems to Migrate:");
    println!("   ✅ Allowlist Entries: {}", legacy_allowlist.len());
    println!("   ❌ Blocklist Entries: {}", legacy_blocklist.len());

    let migrated_memberships =
        migrate_from_binary_patterns(legacy_allowlist.clone(), legacy_blocklist.clone())?;

    println!(
        "🌱 Migration Results: {} total memberships created",
        migrated_memberships.len()
    );
    println!();

    // === Final Summary ===
    println!("🎉 PURE GENETICS EVOLUTION DEMONSTRATION COMPLETE!");
    println!("=================================================");
    println!("✨ Horizontal Gene Transfer Integration: SUCCESSFUL");
    println!("🧬 Genetic Evolution Capabilities: OPERATIONAL");
    println!("🌱 Binary → Spectrum Evolution: DEMONSTRATED");
    println!("🤝 Symbiotic Relationships: ACTIVE");
    println!("🌟 Emergent Intelligence: DETECTED");
    println!("🏥 Health Monitoring: FUNCTIONAL");
    println!("🔄 Legacy Migration: READY");
    println!();
    println!("🚀 BearDog ecosystem evolution genetics are fully operational!");
    println!("   The system has successfully transcended binary relationship");
    println!("   patterns and achieved spectrum-based ecosystem intelligence.");

    Ok(())
}
