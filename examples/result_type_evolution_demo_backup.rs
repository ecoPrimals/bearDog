

use beardog_errors::{improved_results::*, migration_helpers::*, BearDogError, BearDogResult};
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

async fn authenticate_user_old(username: &str, password: &str) -> BearDogResult<()> {

    if username == "admin" && password == "secure123" {
        println!("✅ Authentication successful for {}", username);
        Ok(()) // What session ID? Security level? Expiry time?
    } else {
        Err(BearDogError::authentication(
            "Invalid credentials".to_string(),
        ))
    }
}

async fn process_items_old(items: Vec<&str>) -> BearDogResult<()> {
    for item in items {
        if item.is_empty() {
            return Err(BearDogError::invalid_input("Empty item found".to_string()));
        }

        println!("Processing: {}", item);
    }
    Ok(()) // How many succeeded? Which ones failed? Performance metrics?
}

async fn load_config_old(path: &str) -> BearDogResult<()> {

    if path.ends_with(".toml") {
        println!("✅ Configuration loaded from {}", path);
        Ok(()) // No validation results, timing, or metadata
    } else {
        Err(BearDogError::configuration(
            "Invalid configuration file format".to_string(),
        ))
    }
}

async fn authenticate_user_rich(
    username: &str,
    password: &str,
) -> BearDogResult<AuthenticationOutcome> {
    let start_time = Utc::now();

    if username == "admin" && password == "secure123" {
        let session_id = format_args!("session_{}", uuid::Uuid::new_v4().to_string());

        Ok(create_auth_outcome(
            session_id,
            username.to_string(),
            vec!["admin".to_string(), "user".to_string()],
            SecurityLevel::High,
            AuthenticationMethod::Password,
        )
        .with_timing(start_time))
    } else {
        Err(BearDogError::authentication(
            "Invalid credentials".to_string(),
        ))
    }
}

async fn process_items_rich(items: Vec<&str>) -> BearDogResult<ProcessingOutcome<Vec<String>>> {
    let start_time = Utc::now();
    let mut successful = Vec::new();
    let mut failures = Vec::new();

    for (index, item) in items.iter().enumerate() {
        if item.is_empty() {
            failures.push(create_processing_failure(
                index,
                json!(item),
                "Empty item not allowed",
            ));
        } else {

            successful.push(format_args!("processed_{}", item).to_string());
            println!("✅ Processed: {}", item);
        }
    }

    let total_items = items.len();
    let successful_count = successful.len();

    Ok(
        create_processing_outcome(successful, total_items, successful_count, failures)
            .with_timing(start_time),
    )
}

async fn load_config_rich(path: &str) -> BearDogResult<ConfigurationOutcome<AppConfig>> {
    let start_time = Utc::now();

    if path.ends_with(".toml") {
        let config = AppConfig {
            database_url: "postgresql://localhost/beardog".to_string(),
            api_port: 8080,
            log_level: "info".to_string(),
        };

        let mut findings = Vec::new();
        if config.api_port == 8080 {
            findings.push(create_validation_finding(
                FindingSeverity::Warning,
                "Using default port 8080 - consider using a custom port for production",
                Some("api_port".to_string()),
                Some("Set api_port to a custom value".to_string()),
            ));
        }

        let validation_status = if findings.iter().any(|f| {
            matches!(
                f.severity,
                FindingSeverity::Error | FindingSeverity::Critical
            )
        }) {
            ValidationStatus::Failed
        } else if !findings.is_empty() {
            ValidationStatus::PassedWithWarnings
        } else {
            ValidationStatus::Passed
        };

        Ok(create_config_outcome(config, validation_status, findings)
            .with_timing(start_time)
            .with_metadata(hashmap! {
                "config_path" => json!(path),
                "file_size" => json!(1024),
                "validation_rules" => json!(3),
            }))
    } else {
        Err(BearDogError::configuration(
            "Invalid configuration file format - expected .toml".to_string(),
        ))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct AppConfig {
    database_url: String,
    api_port: u16,
    log_level: String,
}

async fn demonstrate_authentication() {
    println!("🔐 Authentication Demonstration");
    println!("================================");

    println!("\n❌ OLD PATTERN:");
    match authenticate_user_old("admin", "secure123").await {
        Ok(()) => println!("   Authentication succeeded (but no context!)"),
        Err(e) => println!("   Authentication failed: {}", e),
    }

    println!("\n✅ NEW PATTERN:");
    match authenticate_user_rich("admin", "secure123").await {
        Ok(outcome) => {
            println!("   Authentication succeeded!");
            println!("   Session ID: {}", outcome.session_id);
            println!(
                "   User: {} with roles: {:?}",
                outcome.user_info.username, outcome.user_info.roles
            );
            println!("   Security Level: {:?}", outcome.security_level);
            println!("   Expires: {}", outcome.expires_at);
            println!("   Operation took: {:?}", outcome.metrics.duration);
            println!("   Component: {}", outcome.context.component);
        }
        Err(e) => println!("   Authentication failed: {}", e),
    }
}

async fn demonstrate_processing() {
    println!("\n🔄 Processing Demonstration");
    println!("============================");

    let items = vec![
        "item1".to_string(),
        "".to_string(), // This will fail
        "item3".to_string(),
        "item4".to_string(),
    ];

    println!("\n❌ OLD PATTERN:");
    match process_items_old(items.clone()).await {
        Ok(()) => println!("   All items processed (but no details!)"),
        Err(e) => println!("   Processing failed: {} (lost all progress!)", e),
    }

    println!("\n✅ NEW PATTERN:");
    match process_items_rich(items).await {
        Ok(outcome) => {
            println!("   Processing completed with partial success!");
            println!("   Successful items: {:?}", outcome.result);
            println!("   Total items: {}", outcome.statistics.total_items);
            println!("   Successful: {}", outcome.statistics.successful_items);
            println!("   Failed: {}", outcome.statistics.failed_items);
            println!("   Success rate: {:.1}%", outcome.statistics.success_rate);

            if !outcome.failures.is_empty() {
                println!("   Failures:");
                for failure in &outcome.failures {
                    println!("     - Item {}: {}", failure.item_index, failure.error);
                }
            }

            println!("   Operation took: {:?}", outcome.metrics.duration);
        }
        Err(e) => println!("   Processing failed: {}", e),
    }
}

async fn demonstrate_configuration() {
    println!("\n⚙️ Configuration Demonstration");
    println!("===============================");

    println!("\n❌ OLD PATTERN:");
    match load_config_old("app.toml").await {
        Ok(()) => println!("   Config loaded (but no validation info!)"),
        Err(e) => println!("   Config loading failed: {}", e),
    }

    println!("\n✅ NEW PATTERN:");
    match load_config_rich("app.toml").await {
        Ok(outcome) => {
            println!("   Configuration loaded successfully!");
            println!("   Database URL: {}", outcome.result.database_url);
            println!("   API Port: {}", outcome.result.api_port);
            println!("   Log Level: {}", outcome.result.log_level);

            println!(
                "   Validation Status: {:?}",
                outcome.validation_results.status
            );
            if !outcome.validation_results.findings.is_empty() {
                println!("   Validation Findings:");
                for finding in &outcome.validation_results.findings {
                    println!("     - {:?}: {}", finding.severity, finding.message);
                    if let Some(fix) = &finding.suggested_fix {
                        println!("       Suggestion: {}", fix);
                    }
                }
            }

            println!("   Operation took: {:?}", outcome.metrics.duration);
            println!("   Metadata: {:?}", outcome.context.metadata);
        }
        Err(e) => println!("   Config loading failed: {}", e),
    }
}

async fn demonstrate_migration_helpers() {
    println!("\n🔧 Migration Helpers Demonstration");
    println!("===================================");

    let legacy_result: Result<(), BearDogError> = Ok(());
    let converted_outcome =
        legacy_result.to_operation_outcome("demo-component", "legacy-operation");

    match converted_outcome {
        Ok(outcome) => {
            println!("✅ Legacy result converted to rich outcome:");
            println!("   Operation: {}", outcome.result.operation);
            println!("   Component: {}", outcome.context.component);
            println!("   Items affected: {}", outcome.result.items_affected);
        }
        Err(e) => println!("❌ Conversion failed: {}", e),
    }

    let summary = create_operation_summary("demo-operation", "completed successfully", 5);
    let outcome = operation_outcome!(summary, "demo-component");

    println!("\n✅ Created outcome using helper macro:");
    println!("   Operation: {}", outcome.result.operation);
    println!("   Outcome: {}", outcome.result.outcome);
    println!("   Component: {}", outcome.context.component);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 BearDog Result Type Evolution Demonstration");
    println!("==============================================");
    println!();
    println!("This demo shows the evolution from Result<(), E> patterns");
    println!("to rich, idiomatic return types with operational context.");

    demonstrate_authentication().await;
    demonstrate_processing().await;
    demonstrate_configuration().await;
    demonstrate_migration_helpers().await;

    println!("\n🎉 Demonstration Complete!");
    println!("==========================");
    println!();
    println!("Key Benefits of Rich Return Types:");
    println!("• 📊 Operational metrics and timing built-in");
    println!("• 🔍 Rich debugging context with operation IDs");
    println!("• ⚠️  Graceful handling of partial failures");
    println!("• 🤖 AI-friendly structured data");
    println!("• 📈 Automatic performance monitoring");
    println!("• 🛡️ Enhanced error context and suggestions");

    Ok(())
}

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::with_capacity(16);
        $(map.insert($key.to_string(), $val);)*
        map
    }};
}
