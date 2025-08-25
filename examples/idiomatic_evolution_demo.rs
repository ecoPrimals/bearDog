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


//! BearDog Idiomatic Error Evolution Demo
//!
//! **WEEK 1 FOUNDATION WORK COMPLETE** - Demonstration of our enhanced error system
//!
//! This demo shows how our idiomatic evolution preserves and enhances our rich
//! canonical error system while providing better type safety and developer experience.

use beardog_errors::{
    // Legacy system (still available)
    BearDogError, BearDogResult,
    
    // NEW: Idiomatic domain-specific types
    SecurityError, SecurityResult, SecurityMetadata, RemediationStep, ThreatLevel,
    GeneticsError, GeneticsResult, LineageMetadata, DiversityImprovement,
    NetworkError, NetworkResult,
    WorkflowError, WorkflowResult,
    
    // Migration helpers
    migrate_security_result, migrate_genetics_result,
    
    // Rich context types
    OperationContext,
};
use chrono::Utc;
use std::collections::HashMap;

/// **BEFORE**: Generic error handling (still works!)
fn authenticate_user_legacy(user_id: &str) -> BearDogResult<String> {
    if user_id.is_empty() {
        return Err(BearDogError::authentication("User ID cannot be empty"));
    }
    
    if user_id == "invalid_user" {
        return Err(BearDogError::authentication(format!("Authentication failed for user: {}", user_id)));
    }
    
    Ok(format!("session_token_for_{}", user_id))
}

/// **AFTER**: Rich, domain-specific error handling
fn authenticate_user_idiomatic(user_id: &str) -> SecurityResult<String> {
    if user_id.is_empty() {
        return Err(SecurityError::AuthenticationFailed {
            reason: "User ID cannot be empty".to_string(),
            user_id: user_id.to_string(),
            context: OperationContext {
                operation_id: "auth_001".to_string(),
                started_at: Utc::now(),
                completed_at: Utc::now(),
                component: "beardog-security".to_string(),
                initiator: "demo".to_string(),
                request_id: Some("req_123".to_string()),
                metadata: HashMap::new(),
            },
            metadata: SecurityMetadata {
                security_level: beardog_errors::SecurityLevel::High,
                compliance_context: beardog_errors::ComplianceContext {
                    standards: vec!["SOC2".to_string(), "GDPR".to_string()],
                    compliance_level: "strict".to_string(),
                },
                audit_trail: vec![],
                failed_attempt_count: 1,
                lockout_remaining: None,
                additional_context: HashMap::new(),
            },
            remediation: vec![
                RemediationStep::PasswordReset { user_id: user_id.to_string() },
                RemediationStep::EnableMfa,
            ],
            threat_assessment: beardog_errors::ThreatAssessment {
                threat_level: ThreatLevel::Medium,
                suspicious_patterns: vec!["empty_user_id".to_string()],
                recommended_actions: vec!["validate_input".to_string()],
            },
            metrics: beardog_errors::AuthenticationMetrics {
                duration: std::time::Duration::from_millis(50),
                attempts_analyzed: 1,
                security_checks_performed: 3,
                hsm_operations: 0,
            },
        });
    }
    
    Ok(format!("secure_session_token_for_{}", user_id))
}

/// **GENETICS**: Domain-specific error handling
fn perform_genetic_spawning(parent_ids: &[String]) -> GeneticsResult<String> {
    if parent_ids.len() < 2 {
        return Err(GeneticsError::SpawningFailed {
            parent_ids: parent_ids.to_vec(),
            diversity_score: 0.1,
            context: OperationContext {
                operation_id: "spawn_001".to_string(),
                started_at: Utc::now(),
                completed_at: Utc::now(),
                component: "beardog-genetics".to_string(),
                initiator: "demo".to_string(),
                request_id: None,
                metadata: HashMap::new(),
            },
            lineage_metadata: LineageMetadata {
                diversity_metrics: beardog_errors::DiversityMetrics {
                    current_score: 0.1,
                    target_score: 0.8,
                },
                genetic_health: beardog_errors::GeneticHealthScore {
                    overall_health: 0.3,
                    genetic_stability: 0.5,
                },
                evolutionary_context: beardog_errors::EvolutionaryContext {
                    generation: 1,
                    mutation_rate: 0.05,
                },
                spawning_recommendations: vec![],
            },
            performance_impact: beardog_errors::SpawningMetrics {
                spawning_duration: std::time::Duration::from_millis(200),
                genetic_operations: 5,
                diversity_calculations: 10,
                lineage_updates: 2,
            },
            remediation: vec![
                DiversityImprovement::IntroduceNewGenetics { source: "external_pool".to_string() },
                DiversityImprovement::ExpandPopulation { target_size: 100 },
            ],
        });
    }
    
    Ok("new_genetic_variant_id".to_string())
}

/// **MIGRATION**: Smooth transition from legacy to idiomatic
fn demonstrate_migration() {
    println!("🚀 BearDog Idiomatic Error Evolution Demo");
    println!("==========================================\n");
    
    // 1. Legacy system still works
    println!("📊 LEGACY SYSTEM (still supported):");
    match authenticate_user_legacy("test_user") {
        Ok(token) => println!("✅ Legacy auth success: {}", token),
        Err(e) => println!("❌ Legacy auth error: {}", e),
    }
    
    // 2. New idiomatic system provides rich context
    println!("\n🎯 NEW IDIOMATIC SYSTEM (enhanced):");
    match authenticate_user_idiomatic("") {
        Ok(token) => println!("✅ Idiomatic auth success: {}", token),
        Err(SecurityError::AuthenticationFailed { 
            reason, 
            user_id, 
            context, 
            metadata, 
            remediation, 
            threat_assessment, 
            .. 
        }) => {
            println!("🔍 Rich error context available:");
            println!("  • Reason: {}", reason);
            println!("  • User ID: {}", user_id);
            println!("  • Operation ID: {}", context.operation_id);
            println!("  • Security Level: {:?}", metadata.security_level);
            println!("  • Threat Level: {:?}", threat_assessment.threat_level);
            println!("  • Remediation Steps: {} available", remediation.len());
            for (i, step) in remediation.iter().enumerate() {
                println!("    {}. {:?}", i + 1, step);
            }
        }
        _ => unreachable!(),
    }
    
    // 3. Migration helper for smooth transition
    println!("\n🔄 MIGRATION HELPER (backward compatibility):");
    let legacy_result: BearDogResult<String> = authenticate_user_legacy("invalid_user");
    let migrated_result: SecurityResult<String> = migrate_security_result(legacy_result);
    
    match migrated_result {
        Ok(token) => println!("✅ Migrated auth success: {}", token),
        Err(e) => println!("🔄 Migrated error with rich context: {:?}", e),
    }
    
    // 4. Domain-specific genetics errors
    println!("\n🧬 GENETICS DOMAIN ERRORS:");
    match perform_genetic_spawning(&["parent1".to_string()]) {
        Ok(variant) => println!("✅ Genetic spawning success: {}", variant),
        Err(GeneticsError::SpawningFailed { 
            diversity_score, 
            lineage_metadata, 
            remediation, 
            .. 
        }) => {
            println!("🧬 Genetics-specific error context:");
            println!("  • Diversity Score: {:.2}", diversity_score);
            println!("  • Target Score: {:.2}", lineage_metadata.diversity_metrics.target_score);
            println!("  • Genetic Health: {:.2}", lineage_metadata.genetic_health.overall_health);
            println!("  • Generation: {}", lineage_metadata.evolutionary_context.generation);
            println!("  • Improvement Strategies: {} available", remediation.len());
            for (i, strategy) in remediation.iter().enumerate() {
                println!("    {}. {:?}", i + 1, strategy);
            }
        }
        _ => unreachable!(),
    }
    
    println!("\n🎉 IDIOMATIC EVOLUTION COMPLETE!");
    println!("✅ Rich canonical error system preserved and enhanced");
    println!("✅ Domain-specific error types provide precise handling");
    println!("✅ AI-driven remediation suggestions available");
    println!("✅ Backward compatibility maintained");
    println!("✅ Type safety and compile-time optimization gained");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    demonstrate_migration();
    Ok(())
} 