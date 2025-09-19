use beardog_errors::{
    improved_results::*, migration_examples::*, operation_outcome, outcome_with_data, BearDogError,
    BearDogResult,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[CYCLE] BearDog Error Handling Migration Demo");
    println!("========================================");

    println!("📝 Demo 1: Authentication Pattern Evolution");
    println!("-------------------------------------------");

    println!("🔴 OLD Pattern: authenticate_user({}", e),
    }

    println!("🟢 NEW Pattern: authenticate_user() -> Result<AuthenticationOutcome, E>");
    let new_auth_result = authenticate_user_new("admin", "secure123");
    match new_auth_result {
        Ok(auth_outcome) => {
            println!("[OK] Authentication succeeded with rich context:");
            println!("   🔐 Security Level: {:?}", auth_outcome.security_level);
            if let Some({}", session.session_id);
                println!("   👤 Permissions: {:?}", session.permissions);
            }
            if let Some({}", expires_at.format("%Y-%m-%d %H:%M:%S"));
            }
            println!("   🔑 Method: {:?}", auth_outcome.method);
        }
        Err({}", e),
    }

    println!("{}", "=".repeat(60));

    println!("📝 Demo 2: Key Generation Pattern Evolution");
    println!("--------------------------------------------");

    println!("🔴 OLD Pattern: generate_key({}", e),
    }

    println!("🟢 NEW Pattern: generate_key() -> Result<KeyGenerationOutcome, E>");
    let new_key_result = generate_key_new("ed25519");
    match new_key_result {
        Ok(key_outcome) => {
            println!("[OK] Key generated with full metadata:");
            println!("   🆔 Key ID: {}", key_outcome.key_id);
            println!("   🔐 Key Type: {:?}", key_outcome.key_type);
            println!("   💪 Strength: {:?}", key_outcome.key_strength);
            println!("   🏭 HSM Provider: {:?}", key_outcome.hsm_provider);
            println!("   [TARGET] Usage: {:?}", key_outcome.usage_permissions);
            println!("   ⏰ Expires: {:?}", key_outcome.expires_at);
        }
        Err({}", e),
    }

    println!("{}", "=".repeat(60));

    println!("📝 Demo 3: Validation Pattern Evolution");
    println!("----------------------------------------");

    println!("🔴 OLD Pattern: validate_config({}", e),
    }

    println!("🟢 NEW Pattern: validate_config() -> Result<ValidationOutcome, E>");
    let new_validation_result = validate_config_new("host=localhost");
    match new_validation_result {
        Ok(validation) => {
            println!("[CHART] Validation completed with detailed analysis:");
            println!("   [OK] Valid: {}", validation.valid);
            println!("   📈 Score: {:.2}/1.0", validation.score);
            println!("   [SEARCH] Findings: {} issues found", validation.findings.len());

            for finding in &validation.findings {
                let icon = match finding.severity {
                    FindingSeverity::Critical => "🔴",
                    FindingSeverity::Error => "🟠",
                    FindingSeverity::Warning => "🟡",
                    FindingSeverity::Info => "🔵",
                };
                println!(
                    "     {} {}: {} ({})",
                    icon, finding.severity, finding.message, finding.code
                );
                if let Some({}", suggestion);
                }
            }

            println!("   📋 Criteria checked:");
            for criteria in &validation.criteria {
                let status = if criteria.passed { "[OK]" } else { "[X]" };
                println!(
                    "     {} {} (weight: {:.1})",
                    status, criteria.name, criteria.weight
                );
            }
        }
        Err({}", e),
    }

    println!("{}", "=".repeat(60));

    println!("📝 Demo 4: Processing Pattern Evolution");
    println!("---------------------------------------");

    let test_items = vec![
        "item1".to_string(),
        "".to_string(), // This will fail
        "item3".to_string(),
        "item4".to_string(),
    ];

    println!("🔴 OLD Pattern: process_items({} (fails fast, no partial results!)",
            e
        ),
    }

    println!("🟢 NEW Pattern: process_items() -> Result<ProcessingOutcome<T>, E>");
    let new_processing_result = process_items_new(test_items);
    match new_processing_result {
        Ok(processing) => {
            println!("[CHART] Processing completed with detailed statistics:");
            println!("   📈 Total Items: {}", processing.statistics.total_items);
            println!(
                "   [OK] Successful: {}",
                processing.statistics.successful_items
            );
            println!("   [X] Failed: {}", processing.statistics.failed_items);
            println!(
                "   [LIGHTNING] Rate: {:.2} items/second",
                processing.statistics.processing_rate_per_second
            );
            println!(
                "   ⏱️  Avg Time: {:?}",
                processing.statistics.average_item_processing_time
            );

            println!("   [TARGET] Successfully Processed Items:");
            for item in &processing.items {
                println!("     📦 {}", item);
            }

            if !processing.failed_items.is_empty() {
                println!("   ⚠️  Failed Items:");
                for failure in &processing.failed_items {
                    println!(
                        "     💥 Item {}: {} ({})",
                        failure.item_id, failure.error_message, failure.error_code
                    );
                }
            }

            println!("   ⚙️  Configuration Used:");
            println!(
                "     🔧 Batch Size: {}",
                processing.configuration.batch_size
            );
            println!(
                "     👥 Workers: {}",
                processing.configuration.parallel_workers
            );
            println!(
                "     ⏰ Timeout: {:?}",
                processing.configuration.timeout_per_item
            );
            println!(
                "     [CYCLE] Retries: {}",
                processing.configuration.retry_attempts
            );
        }
        Err({}", e),
    }

    println!("{}", "=".repeat(60));

    println!("📝 Demo 5: Simple Operations with OperationOutcome");
    println!("--------------------------------------------------");

    println!("🟢 NEW Pattern: Using OperationOutcome for simple operations");
    let init_result = initialize_system_new();
    match init_result {
        Ok(outcome) => {
            println!("[OK] Operation completed with context:");
            println!("   [TARGET] Operation: {}", outcome.result.operation);
            println!("   📝 Outcome: {}", outcome.result.outcome);
            println!("   [CHART] Items Affected: {}", outcome.result.items_affected);
            println!("   🆔 Operation ID: {}", outcome.context.operation_id);
            println!("   🏗️  Component: {}", outcome.context.component);
            println!("   ⏱️  Duration: {:?}", outcome.metrics.duration);

            if !outcome.warnings.is_empty() {
                println!("   ⚠️  Warnings:");
                for warning in &outcome.warnings {
                    println!("     🟡 {}: {}", warning.code, warning.message);
                }
            }
        }
        Err({}", e),
    }

    println!("{}", "=".repeat(60));

    println!("📝 Demo 6: Custom Data with OperationOutcome");
    println!("---------------------------------------------");

    println!("🟢 NEW Pattern: Using OperationOutcome with custom data types");
    let backup_result = backup_data_new(1024 * 1024 * 500); // 500MB
    match backup_result {
        Ok(outcome) => {
            println!("[OK] Backup completed with detailed results:");
            println!("   🆔 Backup ID: {}", outcome.result.backup_id);
            println!(
                "   📦 Data Size: {:.2} MB",
                outcome.result.data_size_bytes as f64 / 1024.0 / 1024.0
            );
            println!(
                "   🗜️  Compression: {:.1}%",
                outcome.result.compression_ratio * 100.0
            );
            println!("   📍 Location: {}", outcome.result.backup_location);
            println!("   🔐 Checksum: {}", outcome.result.checksum);

            println!("   [CHART] Operation Context:");
            println!("     🆔 Operation ID: {}", outcome.context.operation_id);
            println!("     🏗️  Component: {}", outcome.context.component);
            println!("     ⏱️  Duration: {:?}", outcome.metrics.duration);
            println!("     👤 Initiator: {}", outcome.context.initiator);
        }
        Err({}", e),
    }

    println!("[PARTY] Migration Demo Complete!");
    println!("============================");
    println!("Key Benefits of New Patterns:");
    println!("[OK] Rich context and metadata");
    println!("[OK] Meaningful return values");
    println!("[OK] Better error diagnostics");
    println!("[OK] Performance metrics included");
    println!("[OK] Warnings and partial success handling");
    println!("[OK] Idiomatic Rust patterns");
    println!("[OK] AI-friendly structured data");

    Ok(())
}
