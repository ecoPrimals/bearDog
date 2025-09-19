use beardog_errors::{improved_results::*, migration_helpers::*, BearDogError};

fn authenticate_user_old(&str, password: &str) -> Result<(), BearDogError> {
    if username == "admin" && password == "secure123" {
        println!("[OK] Authentication successful for {}", username);
        Ok(()) // What session ID? Security level? Expiry time?
    } else {
        Err(BearDogError::authentication(&str,
    password: &str,
) -> Result<AuthenticationOutcome, BearDogError> {
    if username == "admin" && password == "secure123" {
        let session_id = format!("session_{}", uuid::Uuid::new_v4());

        Ok(create_auth_outcome(
            session_id: session_id.to_string(),
            username.to_string(),
            vec!["admin".to_string(), "user".to_string()],
            SecurityLevel::High,
            AuthenticationMethod::Password,
        ))
    } else {
        Err(BearDogError::authentication("Invalid credentials"))
    }
}

fn process_items_old(items: Vec<&str>) -> Result<(), BearDogError> {
    for item in items {
        if item.is_empty() {
            return Err(BearDogError::invalid_input({}", item);
    }
    Ok(()) // How many succeeded? Which ones failed? Performance metrics?
}

fn process_items_rich(items: Vec<&str>) -> Result<ProcessingOutcome<Vec<String, BearDogError>>> {
    let mut successful = Vec::new();
    let mut failures = Vec::new();

    for (index, item) in items.iter().enumerate() {
        if item.is_empty() {
            failures.push(create_processing_failure(
                index,
                serde_json::json!(item),
                "Empty item not allowed",
            ));
        } else {
            successful.push({}", item);
        }
    }

    let successful_count = successful.len();

    Ok(create_processing_outcome(
        successful,
        items.len(),
        successful_count,
        failures,
    ))
}

fn demonstrate_authentication() {
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

fn demonstrate_processing() {
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

fn demonstrate_migration_helpers() {
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
    let outcome = OperationOutcome::success(summary, "demo-component");

    println!("[OK] Created outcome using helper functions:");
    println!("   Operation: {}", outcome.result.operation);
    println!("   Outcome: {}", outcome.result.outcome);
    println!("   Component: {}", outcome.context.component);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ROCKET] BearDog Result Type Evolution Demonstration");
    println!("==============================================");
    println!();
    println!("This demo shows the evolution from Result<(), E> patterns");
    println!("to rich, idiomatic return types with operational context.");

    demonstrate_authentication();
    demonstrate_processing();
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
