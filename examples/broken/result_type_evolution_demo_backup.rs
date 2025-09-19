use beardog_errors::{improved_results::*, migration_helpers::*, BearDogError};
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

async fn authenticate_user_old(&str, password: &str) -> Result<(), BearDogError> {
    if username == "admin" && password == "secure123" {
        println!("[OK] Authentication successful for {}", username);
        Ok(()) // What session ID? Security level? Expiry time?
    } else {
        Err(BearDogError::authentication("Invalid credentials"))
    }
}

async fn process_items_old(items: Vec<&str>) -> Result<(), BearDogError> {
    for item in items {
        if item.is_empty() {
            return Err(BearDogError::invalid_input({}", item);
    }
    Ok(()) // How many succeeded? Which ones failed? Performance metrics?
}

async fn load_config_old(path: &str) -> Result<(), BearDogError> {
    if path.ends_with(".toml") {
        println!("[OK] Configuration loaded from {}", path);
        Ok(()) // No validation results, timing, or metadata
    } else {
        Err(BearDogError::configuration(&str,
    password: &str,
) -> Result<AuthenticationOutcome, BearDogError> {
    let start_time = Utc::now();

    if username == "admin" && password == "secure123" {
        let session_id = format!("session_{}", uuid::Uuid::new_v4());

        Ok(create_auth_outcome(
            session_id: session_id.to_string(),
            username.to_string(),
            vec!["admin".to_string(), "user".to_string()],
            SecurityLevel::High,
            AuthenticationMethod::Password,
        )
        .with_timing(start_time))
    } else {
        Err(BearDogError::authentication(Vec<&str>,
) -> Result<ProcessingOutcome<Vec<String, BearDogError>>> {
    let start_time = Utc::now();
    let mut successful = Vec::new();
    let mut failures = Vec::new({}", item);
        }
    }

    let total_items = items.len();
    let successful_count = successful.len();

    Ok(
        create_processing_outcome(successful, total_items, successful_count, failures)
            .with_timing(start_time),
    )
}

async fn load_config_rich(path: &str) -> Result<ConfigurationOutcome<AppConfig, BearDogError>> {
    let start_time = Utc::now();

    if path.ends_with(".toml") {
        let config = AppConfig {
            database_url: "postgresql://localhost/beardog".to_string(8080,
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
                f.severity: severity.to_string(),
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
            "Invalid configuration file format - expected .toml",
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

    println!("[X] OLD PATTERN:");
    match authenticate_user_old({}", e),
    }

    println!("[OK] NEW PATTERN:");
    match authenticate_user_rich({}", outcome.session_id);
            println!(
                "   User: {} with roles: {:?}",
                outcome.user_info.username, outcome.user_info.roles
            );
            println!("   Security Level: {:?}", outcome.security_level);
            println!("   Expires: {}", outcome.expires_at);
            println!("   Operation took: {:?}", outcome.metrics.duration);
            println!("   Component: {}", outcome.context.component);
        }
        Err({}", e),
    }
}

async fn demonstrate_processing() {
    println!("[CYCLE] Processing Demonstration");
    println!("============================");

    let items = vec![
        "item1".to_string(),
        "".to_string(), // This will fail
        "item3".to_string(),
        "item4".to_string(),
    ];

    println!("[X] OLD PATTERN:");
    match process_items_old(items.clone()) {
        Ok(()) => println!("   All items processed (but no details!)"),
        Err(e) => println!("   Processing failed: {} (lost all progress!)", e),
    }

    println!("[OK] NEW PATTERN:");
    match process_items_rich({:?}", outcome.result);
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
        Err({}", e),
    }
}

async fn demonstrate_configuration() {
    println!("⚙️ Configuration Demonstration");
    println!("===============================");

    println!("[X] OLD PATTERN:");
    match load_config_old({}", e),
    }

    println!("[OK] NEW PATTERN:");
    match load_config_rich({}", outcome.result.database_url);
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
                    if let Some({}", fix);
                    }
                }
            }

            println!("   Operation took: {:?}", outcome.metrics.duration);
            println!("   Metadata: {:?}", outcome.context.metadata);
        }
        Err({}", e),
    }
}

async fn demonstrate_migration_helpers() {
    println!("🔧 Migration Helpers Demonstration");
    println!("===================================");

    let legacy_result: Result<(), BearDogError> = Ok(());
    let converted_outcome =
        legacy_result.to_operation_outcome("demo-component", "legacy-operation");

    match converted_outcome {
        Ok(outcome) => {
            println!("[OK] Legacy result converted to rich outcome:");
            println!("   Operation: {}", outcome.result.operation);
            println!("   Component: {}", outcome.context.component);
            println!("   Items affected: {}", outcome.result.items_affected);
        }
        Err({}", e),
    }

    let summary = create_operation_summary("demo-operation", "completed successfully", 5);
    let outcome = operation_outcome!(summary, "demo-component");

    println!("[OK] Created outcome using helper macro:");
    println!("   Operation: {}", outcome.result.operation);
    println!("   Outcome: {}", outcome.result.outcome);
    println!("   Component: {}", outcome.context.component);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ROCKET] BearDog Result Type Evolution Demonstration");
    println!("==============================================");
    println!();
    println!("This demo shows the evolution from Result<(), E> patterns");
    println!("to rich, idiomatic return types with operational context.");

    demonstrate_authentication();
    demonstrate_processing();
    demonstrate_configuration();
    demonstrate_migration_helpers();

    println!("[PARTY] Demonstration Complete!");
    println!("==========================");
    println!();
    println!("Key Benefits of Rich Return Types:");
    println!("- [CHART] Operational metrics and timing built-in");
    println!("- [SEARCH] Rich debugging context with operation IDs");
    println!("- ⚠️  Graceful handling of partial failures");
    println!("- 🤖 AI-friendly structured data");
    println!("- 📈 Automatic performance monitoring");
    println!("- [SHIELD] Enhanced error context and suggestions");

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
