// BearDog Idiomatic Evolution Demo
// 
// Demonstrates the evolution from legacy error handling to rich, idiomatic error types.

use beardog_errors::{BearDogError, SecurityError, GeneticsError, SecurityResult};
use std::collections::HashMap;

/// Mock token for demonstration
const DEMO_TOKEN: &str = "demo_token_123";

/// Mock variant for genetics demo
const DEMO_VARIANT: &str = "variant_alpha";

/// Legacy authentication function (still supported)
fn authenticate_user_legacy(token: &str) -> Result<String, BearDogError> {
    if token == "valid_token" {
        Ok("user_authenticated".to_string())
    } else {
        Err(BearDogError::security("Authentication failed ".to_string()))
    }
}

/// Modern idiomatic authentication with rich error context
fn authenticate_user_idiomatic(token: &str) -> SecurityResult<String> {
    if token == "valid_token" {
        Ok("user_authenticated".to_string())
    } else {
        Err(SecurityError::AuthenticationFailed {
            reason: "Invalid token provided".to_string(),
            user_id: Some("unknown_user".to_string()),
            context: HashMap::new(),
            metadata: Default::default(),
            remediation: vec!["Check token validity".to_string(), "Refresh authentication".to_string()],
            threat_assessment: None,
        })
    }
}

/// Migration helper for backward compatibility
fn migrate_security_result(legacy_result: Result<String, BearDogError>) -> SecurityResult<String> {
    match legacy_result {
        Ok(value) => Ok(value),
        Err(e) => Err(SecurityError::AuthenticationFailed {
            reason: e.to_string(),
            user_id: None,
            context: HashMap::new(),
            metadata: Default::default(),
            remediation: vec!["Migrate to modern authentication".to_string()],
            threat_assessment: None,
        }),
    }
}

/// Mock genetics spawning function
fn perform_genetic_spawning(variant: &str) -> Result<String, GeneticsError> {
    if variant == "valid_variant" {
        Ok("spawning_successful".to_string())
    } else {
        Err(GeneticsError::SpawningFailed {
            diversity_score: 0.25,
            lineage_metadata: Default::default(),
            remediation: vec!["Increase genetic diversity".to_string()],
        })
    }
}

/// Demonstration function
fn demonstrate_migration() {
    println!("[ROCKET] BearDog Idiomatic Error Evolution Demo");
    println!("==========================================");

    println!("[CHART] LEGACY SYSTEM (still supported):");
    match authenticate_user_legacy(DEMO_TOKEN) {
        Err(e) => {
            println!("[X] Legacy error: {}", e);
        }
        Ok(_) => println!("[OK] Authentication successful (legacy)"),
    }

    println!("");
    println!("[TARGET] NEW IDIOMATIC SYSTEM (enhanced):");
    match authenticate_user_idiomatic(DEMO_TOKEN) {
        Err(SecurityError::AuthenticationFailed {
            reason,
            user_id,
            remediation,
            ..
        }) => {
            println!("[SEARCH] Rich error context available:");
            println!("  - Reason: {}", reason);
            if let Some(uid) = user_id {
                println!("  - User ID: {}", uid);
            }
            println!("  - Remediation steps: {} available", remediation.len());
            for (i, step) in remediation.iter().enumerate() {
                println!("    {}. {}", i + 1, step);
            }
        }
        _ => println!("[OK] Authentication successful"),
    }

    println!("");
    println!("[CYCLE] MIGRATION HELPER (backward compatibility):");
    let legacy_result: Result<String, BearDogError> = authenticate_user_legacy("invalid_user");
    let migrated_result: SecurityResult<String> = migrate_security_result(legacy_result);
    match migrated_result {
        Err(e) => {
            println!("[CYCLE] Migrated error: {:?}", e);
        }
        Ok(_) => println!("[OK] Migration successful"),
    }

    println!("");
    println!("[DNA] GENETICS DOMAIN ERRORS:");
    match perform_genetic_spawning(DEMO_VARIANT) {
        Err(GeneticsError::SpawningFailed {
            diversity_score,
            remediation,
            ..
        }) => {
            println!("[DNA] Genetics-specific error context:");
            println!("  - Diversity Score: {:.2}", diversity_score);
            println!("  - Improvement Strategies: {} available", remediation.len());
            for (i, strategy) in remediation.iter().enumerate() {
                println!("    {}. {}", i + 1, strategy);
            }
        }
        _ => println!("[OK] Genetic spawning successful"),
    }

    println!("");
    println!("[PARTY] IDIOMATIC EVOLUTION COMPLETE!");
    println!("[OK] Rich canonical error system preserved and enhanced");
    println!("[OK] Domain-specific error types provide precise handling");
    println!("[OK] AI-driven remediation suggestions available");
    println!("[OK] Backward compatibility maintained");
    println!("[OK] Type safety and compile-time optimization gained");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    demonstrate_migration();
    Ok(())
}
