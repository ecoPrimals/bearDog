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


//! Genetics Migration Demonstration
//!
//! This example demonstrates the migration of genetics operations from
//! `Result<(), E>` patterns to rich, idiomatic return types.

use beardog_errors::{improved_results::*, migration_helpers::*, BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;

// Mock genetics types for demonstration
#[derive(Debug, Clone)]
pub struct BearDogGenetics {
    pub id: String,
    pub capabilities: Vec<String>,
    pub security_clearance: SecurityClearance,
    pub generation: u32,
    pub fitness_score: f64,
}

#[derive(Debug, Clone)]
pub enum SecurityClearance {
    Basic,
    Medium,
    High,
    Maximum,
}

#[derive(Debug, Clone)]
pub struct MockGeneticsRegistry {
    pub genetics: HashMap<String, BearDogGenetics>,
}

impl MockGeneticsRegistry {
    pub fn new() -> Self {
        Self {
            genetics: HashMap::new(),
        }
    }
}

// ============================================================================
// BEFORE: Non-idiomatic patterns
// ============================================================================

/// ❌ OLD: Non-idiomatic genetics registration - provides no useful information
impl MockGeneticsRegistry {
    pub fn register_genetics_old(&mut self, genetics: BearDogGenetics) -> BearDogResult<()> {
        // Simulate genetics registration
        self.genetics.insert(genetics.id.clone(), genetics);
        Ok(()) // What validation occurred? Performance metrics? Registry size?
    }

    pub fn terminate_spawn_old(&mut self, spawn_id: &str) -> BearDogResult<()> {
        // Simulate spawn termination
        if spawn_id == "valid_spawn" {
            Ok(()) // No cleanup info, performance metrics, or termination details
        } else {
            Err(BearDogError::authorization(format!("Spawn not found: {)", spawn_id),
            })
        }
    }
}

// ============================================================================
// AFTER: Rich, idiomatic patterns
// ============================================================================

/// ✅ NEW: Rich genetics registration with comprehensive validation and context
impl MockGeneticsRegistry {
    pub fn register_genetics_rich(
        &mut self,
        genetics: BearDogGenetics,
    ) -> BearDogResult<GeneticsRegistrationOutcome> {
        let start_time = Utc::now();

        // Extract genetics information for the outcome
        let genetics_id = genetics.id.clone();
        let capabilities = genetics.capabilities.clone();
        let security_clearance = format!("{:?}", genetics.security_clearance);
        let generation = genetics.generation;
        let fitness_score = genetics.fitness_score;

        // Perform the registration
        self.genetics.insert(genetics.id.clone(), genetics);

        // Create rich outcome with comprehensive context
        let mut outcome = create_genetics_outcome(
            genetics_id,
            capabilities,
            security_clearance,
            generation,
            fitness_score,
        );

        // Update timing and metrics
        outcome.context.started_at = start_time;
        outcome.context.completed_at = Utc::now();
        outcome.metrics.duration = (Utc::now() - start_time).to_std().unwrap_or_default();
        outcome.metrics.items_processed = 1;
        outcome.metrics.success_rate = 100.0;

        // Add metadata about the registry state
        outcome.context.metadata.insert(
            "registry_size".to_string(),
            serde_json::json!(self.genetics.len()),
        );
        outcome.context.metadata.insert(
            "generation_level".to_string(),
            serde_json::json!(generation),
        );

        Ok(outcome)
    }

    pub fn terminate_spawn_rich(
        &mut self,
        spawn_id: &str,
    ) -> BearDogResult<SpawnTerminationOutcome> {
        let start_time = Utc::now();

        if spawn_id == "valid_spawn" {
            // Calculate runtime (simulate 5 minutes)
            let total_runtime = std::time::Duration::from_secs(300);

            // Create rich termination outcome
            let mut outcome = create_termination_outcome(
                spawn_id.to_string(),
                "Manual termination requested".to_string(),
                total_runtime,
            );

            // Update timing and metrics
            outcome.context.started_at = start_time;
            outcome.context.completed_at = Utc::now();
            outcome.metrics.duration = (Utc::now() - start_time).to_std().unwrap_or_default();
            outcome.metrics.items_processed = 1;
            outcome.metrics.success_rate = 100.0;

            // Add metadata about the terminated spawn
            outcome.context.metadata.insert(
                "spawn_purpose".to_string(),
                serde_json::json!("demonstration"),
            );
            outcome
                .context
                .metadata
                .insert("cleanup_method".to_string(), serde_json::json!("graceful"));

            Ok(outcome)
        } else {
            Err(BearDogError::authorization(format!("Spawn not found: {)", spawn_id),
            })
        }
    }
}

// ============================================================================
// DEMONSTRATION FUNCTIONS
// ============================================================================

fn demonstrate_genetics_registration() {
    println!("🧬 Genetics Registration Demonstration");
    println!("======================================");

    let mut registry = MockGeneticsRegistry::new();

    let test_genetics = BearDogGenetics {
        id: "genetics_001".to_string(),
        capabilities: vec![
            "encrypt".to_string(),
            "sign".to_string(),
            "verify".to_string(),
        ],
        security_clearance: SecurityClearance::High,
        generation: 2,
        fitness_score: 0.87,
    };

    // Old pattern
    println!("\n❌ OLD PATTERN:");
    match registry.register_genetics_old(test_genetics.clone()) {
        Ok(()) => println!("   Genetics registered (but no context!)"),
        Err(e) => println!("   Registration failed: {}", e),
    }

    // New pattern
    println!("\n✅ NEW PATTERN:");
    match registry.register_genetics_rich(test_genetics) {
        Ok(outcome) => {
            println!("   Genetics registration succeeded!");
            println!("   Genetics ID: {}", outcome.genetics_id);
            println!(
                "   Capabilities: {:?}",
                outcome.capabilities_summary.capabilities
            );
            println!(
                "   Security Clearance: {}",
                outcome.capabilities_summary.security_clearance
            );
            println!("   Generation: {}", outcome.capabilities_summary.generation);
            println!(
                "   Fitness Score: {:.2}",
                outcome.capabilities_summary.fitness_score
            );
            println!(
                "   Security Score: {:.1}",
                outcome
                    .validation_results
                    .security_assessment
                    .security_score
            );
            println!(
                "   Risk Level: {}",
                outcome.validation_results.security_assessment.risk_level
            );
            println!("   Operation took: {:?}", outcome.metrics.duration);
            if let Some(registry_size) = outcome.context.metadata.get("registry_size") {
                println!("   Registry size: {}", registry_size);
            }
            println!("   Component: {}", outcome.context.component);
        }
        Err(e) => println!("   Registration failed: {}", e),
    }
}

fn demonstrate_spawn_termination() {
    println!("\n🔚 Spawn Termination Demonstration");
    println!("===================================");

    let mut registry = MockGeneticsRegistry::new();

    // Old pattern
    println!("\n❌ OLD PATTERN:");
    match registry.terminate_spawn_old("valid_spawn") {
        Ok(()) => println!("   Spawn terminated (but no cleanup details!)"),
        Err(e) => println!("   Termination failed: {}", e),
    }

    // New pattern
    println!("\n✅ NEW PATTERN:");
    match registry.terminate_spawn_rich("valid_spawn") {
        Ok(outcome) => {
            println!("   Spawn termination succeeded!");
            println!("   Spawn ID: {}", outcome.spawn_id);
            println!("   Termination Reason: {}", outcome.termination_reason);
            println!("   Runtime: {:?}", outcome.final_metrics.total_runtime);
            println!(
                "   Average CPU: {:.1}%",
                outcome.final_metrics.avg_cpu_usage
            );
            println!(
                "   Peak Memory: {} MB",
                outcome.final_metrics.peak_memory_usage / (1024 * 1024)
            );
            println!(
                "   Operations Completed: {}",
                outcome.final_metrics.operations_completed
            );
            println!(
                "   Success Rate: {:.1}%",
                outcome.final_metrics.success_rate
            );
            println!(
                "   Memory Cleaned: {}",
                outcome.cleanup_results.memory_cleaned
            );
            println!(
                "   Connections Closed: {}",
                outcome.cleanup_results.network_connections_closed
            );
            println!(
                "   Files Cleaned: {}",
                outcome.cleanup_results.files_cleaned
            );
            println!("   Operation took: {:?}", outcome.metrics.duration);
            println!("   Component: {}", outcome.context.component);
        }
        Err(e) => println!("   Termination failed: {}", e),
    }
}

fn demonstrate_migration_helpers() {
    println!("\n🔧 Migration Helpers Demonstration");
    println!("===================================");

    // Using the UnitResultExt trait to convert existing Result<(), E>
    let legacy_result: Result<(), BearDogError> = Ok(());
    let converted_outcome =
        legacy_result.to_operation_outcome("genetics-component", "legacy-genetics-operation");

    match converted_outcome {
        Ok(outcome) => {
            println!("✅ Legacy genetics result converted to rich outcome:");
            println!("   Operation: {}", outcome.result.operation);
            println!("   Component: {}", outcome.context.component);
            println!("   Items affected: {}", outcome.result.items_affected);
        }
        Err(e) => println!("❌ Conversion failed: {}", e),
    }

    // Creating a spawning outcome directly
    let spawn_outcome = create_spawning_outcome(
        "spawn_demo_001".to_string(),
        vec!["parent_001".to_string(), "parent_002".to_string()],
        "demonstration_spawn".to_string(),
        Some(Utc::now() + chrono::Duration::hours(24)),
    );

    println!("\n✅ Created spawning outcome using helper:");
    println!("   Spawn ID: {}", spawn_outcome.spawned_beardog.spawn_id);
    println!(
        "   Parent Count: {}",
        spawn_outcome.spawned_beardog.parent_ids.len()
    );
    println!(
        "   Purpose: {}",
        spawn_outcome.spawned_beardog.spawn_purpose
    );
    println!(
        "   Inheritance Method: {}",
        spawn_outcome.genetic_inheritance.inheritance_method
    );
    println!(
        "   Combined Fitness: {:.2}",
        spawn_outcome
            .genetic_inheritance
            .combined_genetics
            .fitness_score
    );
    println!("   Resource Validation: All systems valid");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 BearDog Genetics Migration Demonstration");
    println!("============================================");
    println!();
    println!("This demo shows the evolution from Result<(), E> patterns");
    println!("to rich, idiomatic return types for genetics operations.");

    demonstrate_genetics_registration();
    demonstrate_spawn_termination();
    demonstrate_migration_helpers();

    println!("\n🎉 Genetics Migration Demonstration Complete!");
    println!("==============================================");
    println!();
    println!("Key Benefits of Rich Genetics Operations:");
    println!("• 🧬 Comprehensive genetics validation and security assessment");
    println!("• 📊 Built-in performance metrics for spawning operations");
    println!("• 🔍 Rich debugging context with operation IDs and timing");
    println!("• 🛡️ Detailed security clearance and risk assessment");
    println!("• 📈 Automatic fitness score tracking and inheritance metrics");
    println!("• 🧹 Complete resource cleanup reporting for terminated spawns");
    println!("• 🤖 AI-friendly structured data for genetic analysis");

    Ok(())
}
