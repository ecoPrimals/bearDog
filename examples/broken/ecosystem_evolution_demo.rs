// # Ecosystem Evolution Demonstration
//
// This example demonstrates the horizontal gene transfer integration from the Squirrel team
// and shows how BearDog's ecosystem evolves from binary relationship patterns to
// spectrum-based ecosystem intelligence.
//
// ## What This Demo Shows
//
// 1. **Binary → Spectrum Evolution**: Whitelist/blacklist → EcosystemMembership spectrum
// 2. **Trust Evolution**: Binary trust → Dynamic trust evolution states  
// 3. **Coordination Evolution**: Master/slave → Symbiotic coordination models
// 4. **Emergent Behaviors**: Collective intelligence and self-healing networks
// 5. **Genetic Integration**: How the evolution integrates with BearDog's genetic system
//
// ## Horizontal Gene Transfer
//
// This demonstrates the successful integration of genetic material from the Squirrel
// team's ecosystem evolution initiative, showing how biological relationship patterns
// can create more adaptive and resilient systems.

use beardog_genetics::ecosystem_evolution::{
    migrate_from_binary_patterns, BinaryAccessPattern, BinaryTrust, CollaborationOutcome,
    ContributionType, CoordinationModel, EcosystemContext, EcosystemGeneticEngine,
    EcosystemMembership, HierarchicalPattern, InteractionRecord, InteractionTrend, ProtectionLevel,
    ProtectionReason, RelationshipHistory, TrustChangeEvent, TrustEvolution,
};
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 BearDog Ecosystem Evolution Demonstration");
    println!("============================================");
    println!("🌱 Horizontal Gene Transfer Integration from Squirrel Team");
    println!();

    // Initialize the ecosystem genetic engine
    let engine = EcosystemGeneticEngine::new()?;
    println!("✅ EcosystemGeneticEngine initialized with horizontal gene transfer capabilities");

    // Create ecosystem context
    let ecosystem_context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 15,
        ecosystem_load: 0.4,
        recent_events: vec![
            "new_contributor_joined".to_string(),
            "successful_collaboration_completed".to_string(),
        ],
    };
    println!(
        "🌍 Ecosystem context established: {} active relationships, {:.1}% load",
        ecosystem_context.active_relationships,
        ecosystem_context.ecosystem_load * 100.0
    );
    println!();

    // === DEMONSTRATION 1: Binary Access Pattern Evolution ===
    println!("🔄 DEMONSTRATION 1: Binary Access Pattern → Ecosystem Membership Evolution");
    println!("------------------------------------------------------------------------");

    // Legacy whitelist pattern
    let whitelist_pattern = BinaryAccessPattern::Whitelist {
        allowed_entities: vec![
            "trusted_developer_alice".to_string(),
            "security_auditor_bob".to_string(),
            "community_moderator_charlie".to_string(),
        ],
    };

    println!("📋 Legacy whitelist pattern:");
    match &whitelist_pattern {
        BinaryAccessPattern::Whitelist { allowed_entities } => {
            for entity in allowed_entities {
                println!("  ✓ {}", entity);
            }
        }
        _ => {}
    }

    // Evolve to ecosystem membership
    let evolved_membership = engine
        .evolve_access_pattern(whitelist_pattern, ecosystem_context.clone())
        ?;

    println!("\n🌱 Evolved to EcosystemMembership:");
    match &evolved_membership {
        EcosystemMembership::ActiveContributor {
            contribution_types,
            trust_level,
            evolutionary_potential,
            ..
        } => {
            println!("  🎯 Type: ActiveContributor");
            println!("  🔒 Trust Level: {:.1}", trust_level);
            println!("  🧬 Evolutionary Potential: {:.1}", evolutionary_potential);
            println!("  🤝 Contribution Types:");
            for contrib_type in contribution_types {
                match contrib_type {
                    ContributionType::CommunitySupport => println!("    - Community Support"),
                    ContributionType::CodeContribution => println!("    - Code Contribution"),
                    ContributionType::SecurityAuditing => println!("    - Security Auditing"),
                    _ => println!("    - {:?}", contrib_type),
                }
            }
        }
        _ => println!("  Evolved to: {:?}", evolved_membership),
    }
    println!();

    // === DEMONSTRATION 2: Trust Evolution ===
    println!("💙 DEMONSTRATION 2: Binary Trust → Trust Evolution Dynamics");
    println!("----------------------------------------------------------");

    let relationship_history = RelationshipHistory {
        interactions: vec![
            InteractionRecord {
                timestamp: Utc::now(),
                interaction_type: "code_review".to_string(),
                outcome: "approved_with_suggestions".to_string(),
                quality_score: 0.8,
            },
            InteractionRecord {
                timestamp: Utc::now(),
                interaction_type: "collaboration".to_string(),
                outcome: "successful_completion".to_string(),
                quality_score: 0.9,
            },
        ],
        trust_changes: vec![TrustChangeEvent {
            timestamp: Utc::now(),
            change_type: "positive_interaction".to_string(),
            magnitude: 0.1,
            reason: "successful_code_review".to_string(),
        }],
        collaboration_outcomes: vec![CollaborationOutcome {
            timestamp: Utc::now(),
            success: true,
            benefits: vec![
                "improved_code_quality".to_string(),
                "knowledge_sharing".to_string(),
            ],
            lessons_learned: vec!["effective_communication_patterns".to_string()],
        }],
    };

    // Binary trust (legacy)
    let binary_trust = BinaryTrust::Trusted;
    println!("⚫ Legacy binary trust: {:?}", binary_trust);

    // Evolve to trust dynamics
    let evolved_trust = engine
        .evolve_trust_pattern(
            binary_trust,
            relationship_history,
            ecosystem_context.clone(),
        )
        ?;

    println!("🌱 Evolved trust dynamics:");
    match &evolved_trust {
        TrustEvolution::Flourishing {
            stability_metrics,
            flourishing_indicators,
            symbiotic_benefits,
        } => {
            println!("  🌟 State: Flourishing");
            println!("  📊 Stability: {}", stability_metrics);
            println!("  🌸 Indicators: {:?}", flourishing_indicators);
            println!("  🤝 Symbiotic Benefits: {}", symbiotic_benefits);
        }
        TrustEvolution::Building {
            progress_rate,
            milestones,
            genetic_compatibility,
            ..
        } => {
            println!("  🔨 State: Building");
            println!("  📈 Progress Rate: {:.1}", progress_rate);
            println!("  🎯 Milestones: {:?}", milestones);
            println!("  🧬 Genetic Compatibility: {:.1}", genetic_compatibility);
        }
        _ => println!("  {:?}", evolved_trust),
    }
    println!();

    // === DEMONSTRATION 3: Coordination Model Evolution ===
    println!("🤝 DEMONSTRATION 3: Master/Slave → Symbiotic Coordination Evolution");
    println!("-------------------------------------------------------------------");

    // Legacy master/slave pattern
    let master_slave_pattern = HierarchicalPattern::MasterSlave {
        master: "primary_server".to_string(),
        slaves: vec![
            "worker_node_1".to_string(),
            "worker_node_2".to_string(),
            "worker_node_3".to_string(),
        ],
    };

    println!("⚫ Legacy master/slave hierarchy:");
    match &master_slave_pattern {
        HierarchicalPattern::MasterSlave { master, slaves } => {
            println!("  👑 Master: {}", master);
            println!("  ⛓️  Slaves:");
            for slave in slaves {
                println!("    - {}", slave);
            }
        }
        _ => {}
    }

    // Evolve to symbiotic coordination
    let evolved_coordination = engine
        .evolve_coordination_pattern(
            master_slave_pattern,
            vec!["primary_server".to_string(), "worker_node_1".to_string()],
            "distributed_processing".to_string(),
        )
        ?;

    println!("\n🌱 Evolved symbiotic coordination:");
    match &evolved_coordination {
        CoordinationModel::Collaborative {
            decision_protocol,
            collaboration_frameworks,
            mutual_accountability,
            ..
        } => {
            println!("  🤝 Model: Collaborative");
            println!("  📋 Decision Protocol: {}", decision_protocol);
            println!("  🏗️  Frameworks: {:?}", collaboration_frameworks);
            println!("  ⚖️  Accountability: {}", mutual_accountability);
        }
        CoordinationModel::Distributed {
            consensus_type,
            participation_weights,
            decision_thresholds,
        } => {
            println!("  🌐 Model: Distributed");
            println!("  🗳️  Consensus: {}", consensus_type);
            println!("  ⚖️  Weights: {:?}", participation_weights);
            println!("  📊 Thresholds: {}", decision_thresholds);
        }
        _ => println!("  {:?}", evolved_coordination),
    }
    println!();

    // === DEMONSTRATION 4: Emergent Ecosystem Behaviors ===
    println!("🌟 DEMONSTRATION 4: Emergent Ecosystem Intelligence");
    println!("--------------------------------------------------");

    let emergent_behaviors = engine.generate_emergent_behaviors()?;

    println!("🧠 Detected emergent behaviors:");
    for behavior in &emergent_behaviors {
        let status = if behavior.beneficial {
            "✅ Beneficial"
        } else {
            "⚠️  Concerning"
        };
        println!(
            "  🌟 {}: {} (strength: {:.1})",
            behavior.behavior_type, status, behavior.emergence_strength
        );
        println!("    👥 Participants: {:?}", behavior.participants);
    }
    println!();

    // === DEMONSTRATION 5: Ecosystem Health Assessment ===
    println!("🏥 DEMONSTRATION 5: Ecosystem Health Assessment");
    println!("----------------------------------------------");

    let health_report = engine.assess_ecosystem_health()?;

    println!("📊 Ecosystem Health Report:");
    println!("  🌍 Overall Health: {:?}", health_report.overall_health);
    println!("  💙 Trust Health: {:.1}", health_report.trust_health);
    println!(
        "  🤝 Relationship Health: {:.1}",
        health_report.relationship_health
    );
    println!(
        "  ⚙️  Coordination Health: {:.1}",
        health_report.coordination_health
    );
    println!(
        "  🌟 Emergent Capabilities: {} detected",
        health_report.emergent_capabilities.len()
    );
    println!("  💡 Recommendations:");
    for recommendation in &health_report.recommendations {
        println!("    - {}", recommendation);
    }
    println!();

    // === DEMONSTRATION 6: Legacy Migration Utility ===
    println!("🔄 DEMONSTRATION 6: Legacy System Migration");
    println!("------------------------------------------");

    let legacy_whitelist = vec![
        "alice@example.com".to_string(),
        "bob@security-firm.com".to_string(),
        "charlie@community.org".to_string(),
    ];

    let legacy_blacklist = vec!["spam_bot_1".to_string(), "malicious_actor_x".to_string()];

    println!("📋 Legacy systems to migrate:");
    println!("  ✅ Whitelist entries: {}", legacy_whitelist.len());
    println!("  ❌ Blacklist entries: {}", legacy_blacklist.len());

    let migrated_memberships =
        migrate_from_binary_patterns(legacy_whitelist.clone(), legacy_blacklist.clone())?;

    println!("\n🌱 Migrated to ecosystem memberships:");
    for (i, membership) in migrated_memberships.iter().enumerate() {
        match membership {
            EcosystemMembership::ActiveContributor { trust_level, .. } => {
                let original = if i < legacy_whitelist.len() {
                    &legacy_whitelist[i]
                } else {
                    "unknown"
                };
                println!(
                    "  🎯 {} → ActiveContributor (trust: {:.1})",
                    original, trust_level
                );
            }
            EcosystemMembership::EcosystemProtection {
                protection_reason,
                protection_level,
                restoration_path,
                ..
            } => {
                let original_idx = i - legacy_whitelist.len();
                let original = legacy_blacklist
                    .get(original_idx)
                    .unwrap_or(&"unknown".to_string());
                let restoration = if restoration_path.is_some() {
                    "✅ Available"
                } else {
                    "❌ None"
                };
                println!(
                    "  🛡️  {} → EcosystemProtection ({:?}, {:?}, restoration: {})",
                    original, protection_reason, protection_level, restoration
                );
            }
            _ => println!("  🔄 Migrated to: {:?}", membership),
        }
    }
    println!();

    // === DEMONSTRATION 7: System Integration ===
    println!("🔗 DEMONSTRATION 7: Genetic System Integration");
    println!("---------------------------------------------");

    println!("🧬 Genetic integration capabilities:");
    println!("  ✅ Horizontal gene transfer from Squirrel team integrated");
    println!("  ✅ Binary pattern evolution algorithms active");
    println!("  ✅ Trust evolution dynamics operational");
    println!("  ✅ Symbiotic coordination models deployed");
    println!("  ✅ Emergent behavior detection enabled");
    println!("  ✅ Ecosystem health monitoring active");
    println!("  ✅ Legacy system migration utilities ready");
    println!();

    println!("🎉 ECOSYSTEM EVOLUTION DEMONSTRATION COMPLETE");
    println!("===========================================");
    println!("🌱 BearDog has successfully evolved from binary relationship patterns");
    println!("   to spectrum-based ecosystem intelligence through horizontal gene");
    println!("   transfer integration from the Squirrel team.");
    println!();
    println!("🧬 Key Evolutionary Achievements:");
    println!("   • Whitelist/Blacklist → EcosystemMembership spectrum");
    println!("   • Binary Trust → Dynamic Trust Evolution");
    println!("   • Master/Slave → Symbiotic Coordination");
    println!("   • Static Rules → Emergent Intelligence");
    println!("   • Isolated Systems → Collaborative Ecosystems");
    println!();
    println!("🚀 The ecosystem is now ready for adaptive, self-healing,");
    println!("   and collectively intelligent operation!");

    Ok(())
}

/// Helper function to demonstrate trust evolution over time
async fn demonstrate_trust_evolution_journey() -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 Trust Evolution Journey:");

    let engine = EcosystemGeneticEngine::new()?;
    let ecosystem_context = EcosystemContext {
        current_health: HealthStatus::Healthy,
        active_relationships: 5,
        ecosystem_load: 0.3,
        recent_events: vec!["new_entity_introduced".to_string()],
    };

    // Journey from untrusted to trusted
    let stages = vec![
        ("Unknown Entity", BinaryTrust::Untrusted),
        ("Positive Interactions", BinaryTrust::Trusted),
    ];

    for (stage_name, trust_state) in stages {
        println!("  🔄 Stage: {}", stage_name);

        let relationship_history = RelationshipHistory {
            interactions: vec![],
            trust_changes: vec![],
            collaboration_outcomes: vec![],
        };

        let evolved_trust = engine
            .evolve_trust_pattern(trust_state, relationship_history, ecosystem_context.clone())
            ?;

        match evolved_trust {
            TrustEvolution::Building { progress_rate, .. } => {
                println!("    🔨 Building trust (rate: {:.1})", progress_rate);
            }
            TrustEvolution::Flourishing { .. } => {
                println!("    🌸 Trust flourishing");
            }
            _ => println!("    {:?}", evolved_trust),
        }
    }

    Ok(())
}

/// Helper function to show coordination model flexibility
async fn demonstrate_coordination_flexibility() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Coordination Model Flexibility:");

    let engine = EcosystemGeneticEngine::new()?;

    let patterns = vec![
        (
            "Database Cluster",
            HierarchicalPattern::PrimarySecondary {
                primary: "db_primary".to_string(),
                secondaries: vec!["db_replica_1".to_string(), "db_replica_2".to_string()],
            },
        ),
        (
            "Web Services",
            HierarchicalPattern::ClientServer {
                server: "api_gateway".to_string(),
                clients: vec!["frontend_app".to_string(), "mobile_app".to_string()],
            },
        ),
    ];

    for (system_name, pattern) in patterns {
        println!("  🔧 System: {}", system_name);

        let evolved = engine
            .evolve_coordination_pattern(
                pattern,
                vec!["participant_1".to_string()],
                "load_balancing".to_string(),
            )
            ?;

        match evolved {
            CoordinationModel::Rotational {
                rotation_criteria, ..
            } => {
                println!("    🔄 Evolved to Rotational ({})", rotation_criteria);
            }
            CoordinationModel::Distributed { consensus_type, .. } => {
                println!("    🌐 Evolved to Distributed ({})", consensus_type);
            }
            CoordinationModel::Collaborative {
                decision_protocol, ..
            } => {
                println!("    🤝 Evolved to Collaborative ({})", decision_protocol);
            }
            _ => println!("    {:?}", evolved),
        }
    }

    Ok(())
}
