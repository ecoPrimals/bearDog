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


//! BearDog Error Handling Migration Demo
//!
//! This example demonstrates the evolution from old `Result<(), E>` patterns
//! to idiomatic Rust error handling with rich context, following SongBird's lead.

use beardog_errors::{
    improved_results::*, migration_examples::*, operation_outcome, outcome_with_data, BearDogError,
    BearDogResult,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 BearDog Error Handling Migration Demo");
    println!("========================================\n");

    // Demo 1: Authentication Improvements
    println!("📝 Demo 1: Authentication Pattern Evolution");
    println!("-------------------------------------------");

    println!("🔴 OLD Pattern: authenticate_user() -> Result<(), E>");
    let old_auth_result = authenticate_user_old("admin", "secure123").await;
    match old_auth_result {
        Ok(()) => println!("✅ Authentication succeeded (but no details!)"),
        Err(e) => println!("❌ Authentication failed: {}", e),
    }

    println!("\n🟢 NEW Pattern: authenticate_user() -> Result<AuthenticationOutcome, E>");
    let new_auth_result = authenticate_user_new("admin", "secure123").await;
    match new_auth_result {
        Ok(auth_outcome) => {
            println!("✅ Authentication succeeded with rich context:");
            println!("   🔐 Security Level: {:?}", auth_outcome.security_level);
            if let Some(session) = &auth_outcome.session {
                println!("   🆔 Session ID: {}", session.session_id);
                println!("   👤 Permissions: {:?}", session.permissions);
            }
            if let Some(expires_at) = auth_outcome.expires_at {
                println!("   ⏰ Expires: {}", expires_at.format("%Y-%m-%d %H:%M:%S"));
            }
            println!("   🔑 Method: {:?}", auth_outcome.method);
        }
        Err(e) => println!("❌ Authentication failed: {}", e),
    }

    println!("\n{}", "=".repeat(60));

    // Demo 2: Key Generation Improvements
    println!("\n📝 Demo 2: Key Generation Pattern Evolution");
    println!("--------------------------------------------");

    println!("🔴 OLD Pattern: generate_key() -> Result<(), E>");
    let old_key_result = generate_key_old("ed25519").await;
    match old_key_result {
        Ok(()) => println!("✅ Key generated (but we know nothing about it!)"),
        Err(e) => println!("❌ Key generation failed: {}", e),
    }

    println!("\n🟢 NEW Pattern: generate_key() -> Result<KeyGenerationOutcome, E>");
    let new_key_result = generate_key_new("ed25519").await;
    match new_key_result {
        Ok(key_outcome) => {
            println!("✅ Key generated with full metadata:");
            println!("   🆔 Key ID: {}", key_outcome.key_id);
            println!("   🔐 Key Type: {:?}", key_outcome.key_type);
            println!("   💪 Strength: {:?}", key_outcome.key_strength);
            println!("   🏭 HSM Provider: {:?}", key_outcome.hsm_provider);
            println!("   🎯 Usage: {:?}", key_outcome.usage_permissions);
            println!("   ⏰ Expires: {:?}", key_outcome.expires_at);
        }
        Err(e) => println!("❌ Key generation failed: {}", e),
    }

    println!("\n{}", "=".repeat(60));

    // Demo 3: Validation Improvements
    println!("\n📝 Demo 3: Validation Pattern Evolution");
    println!("----------------------------------------");

    println!("🔴 OLD Pattern: validate_config() -> Result<(), E>");
    let old_validation_result = validate_config_old("host=localhost");
    match old_validation_result {
        Ok(()) => println!("✅ Config valid (but no validation details!)"),
        Err(e) => println!("❌ Config invalid: {}", e),
    }

    println!("\n🟢 NEW Pattern: validate_config() -> Result<ValidationOutcome, E>");
    let new_validation_result = validate_config_new("host=localhost");
    match new_validation_result {
        Ok(validation) => {
            println!("📊 Validation completed with detailed analysis:");
            println!("   ✅ Valid: {}", validation.valid);
            println!("   📈 Score: {:.2}/1.0", validation.score);
            println!("   🔍 Findings: {} issues found", validation.findings.len());

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
                if let Some(suggestion) = &finding.suggestion {
                    println!("       💡 Suggestion: {}", suggestion);
                }
            }

            println!("   📋 Criteria checked:");
            for criteria in &validation.criteria {
                let status = if criteria.passed { "✅" } else { "❌" };
                println!(
                    "     {} {} (weight: {:.1})",
                    status, criteria.name, criteria.weight
                );
            }
        }
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    println!("\n{}", "=".repeat(60));

    // Demo 4: Processing Improvements
    println!("\n📝 Demo 4: Processing Pattern Evolution");
    println!("---------------------------------------");

    let test_items = vec![
        "item1".to_string(),
        "".to_string(), // This will fail
        "item3".to_string(),
        "item4".to_string(),
    ];

    println!("🔴 OLD Pattern: process_items() -> Result<(), E>");
    let old_processing_result = process_items_old(test_items.clone()).await;
    match old_processing_result {
        Ok(()) => println!("✅ Processing succeeded (but no processing details!)"),
        Err(e) => println!(
            "❌ Processing failed: {} (fails fast, no partial results!)",
            e
        ),
    }

    println!("\n🟢 NEW Pattern: process_items() -> Result<ProcessingOutcome<T>, E>");
    let new_processing_result = process_items_new(test_items).await;
    match new_processing_result {
        Ok(processing) => {
            println!("📊 Processing completed with detailed statistics:");
            println!("   📈 Total Items: {}", processing.statistics.total_items);
            println!(
                "   ✅ Successful: {}",
                processing.statistics.successful_items
            );
            println!("   ❌ Failed: {}", processing.statistics.failed_items);
            println!(
                "   ⚡ Rate: {:.2} items/second",
                processing.statistics.processing_rate_per_second
            );
            println!(
                "   ⏱️  Avg Time: {:?}",
                processing.statistics.average_item_processing_time
            );

            println!("   🎯 Successfully Processed Items:");
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
                "     🔄 Retries: {}",
                processing.configuration.retry_attempts
            );
        }
        Err(e) => println!("❌ Processing failed: {}", e),
    }

    println!("\n{}", "=".repeat(60));

    // Demo 5: Simple Operations with Rich Context
    println!("\n📝 Demo 5: Simple Operations with OperationOutcome");
    println!("--------------------------------------------------");

    println!("🟢 NEW Pattern: Using OperationOutcome for simple operations");
    let init_result = initialize_system_new().await;
    match init_result {
        Ok(outcome) => {
            println!("✅ Operation completed with context:");
            println!("   🎯 Operation: {}", outcome.result.operation);
            println!("   📝 Outcome: {}", outcome.result.outcome);
            println!("   📊 Items Affected: {}", outcome.result.items_affected);
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
        Err(e) => println!("❌ Operation failed: {}", e),
    }

    println!("\n{}", "=".repeat(60));

    // Demo 6: Custom Data with OperationOutcome
    println!("\n📝 Demo 6: Custom Data with OperationOutcome");
    println!("---------------------------------------------");

    println!("🟢 NEW Pattern: Using OperationOutcome with custom data types");
    let backup_result = backup_data_new(1024 * 1024 * 500).await; // 500MB
    match backup_result {
        Ok(outcome) => {
            println!("✅ Backup completed with detailed results:");
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

            println!("   📊 Operation Context:");
            println!("     🆔 Operation ID: {}", outcome.context.operation_id);
            println!("     🏗️  Component: {}", outcome.context.component);
            println!("     ⏱️  Duration: {:?}", outcome.metrics.duration);
            println!("     👤 Initiator: {}", outcome.context.initiator);
        }
        Err(e) => println!("❌ Backup failed: {}", e),
    }

    println!("\n🎉 Migration Demo Complete!");
    println!("============================");
    println!("Key Benefits of New Patterns:");
    println!("✅ Rich context and metadata");
    println!("✅ Meaningful return values");
    println!("✅ Better error diagnostics");
    println!("✅ Performance metrics included");
    println!("✅ Warnings and partial success handling");
    println!("✅ Idiomatic Rust patterns");
    println!("✅ AI-friendly structured data");

    Ok(())
}
