

use beardog_errors::{BearDogError, BearDogResult, ErrorSeverity, ErrorCategory, RemediationAction};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🤖 AI Error Interaction Demo");
    println!("============================");

    demonstrate_crypto_error_ai_response().await?;
    demonstrate_network_error_ai_response().await?;
    demonstrate_resource_exhaustion_ai_response().await?;
    
    Ok(())
}

async fn demonstrate_crypto_error_ai_response() -> BearDogResult<()> {
    println!("\n🔐 SCENARIO 1: Cryptographic Operation Failure");
    println!("==============================================");

    let crypto_error = BearDogError::enhanced("CRYPTO_001")
        .severity(ErrorSeverity::High)
        .category(ErrorCategory::Security)
        .message("AES-256-GCM encryption failed: Invalid key material")
        .component("hsm_manager")
        .operation("encrypt_user_data")
        .user_id("user_12345")
        .request_id("req_crypto_789")
        .add_metadata("key_id", "hsm_key_001")
        .add_metadata("data_size_bytes", 1024)
        .add_metadata("algorithm", "AES-256-GCM")
        .technical_details("HSM reported key material corruption - possible hardware failure")
        .add_remediation(RemediationAction {
            action_type: "regenerate_key".to_string(),
            description: "Generate new encryption key from HSM".to_string(),
            parameters: [
                ("key_type".to_string(), json!("AES-256")),
                ("hsm_slot".to_string(), json!(1)),
            ].into(),
            estimated_time_seconds: Some(10),
            automatable: true,
            prerequisites: vec!["hsm_available".to_string()],
        })
        .add_remediation(RemediationAction {
            action_type: "use_software_fallback".to_string(),
            description: "Fall back to software encryption temporarily".to_string(),
            parameters: HashMap::with_capacity(16),
            estimated_time_seconds: Some(2),
            automatable: true,
            prerequisites: vec![],
        })
        .add_remediation(RemediationAction {
            action_type: "notify_security_team".to_string(),
            description: "Alert security team about potential HSM hardware issue".to_string(),
            parameters: [
                ("urgency".to_string(), json!("high")),
                ("component".to_string(), json!("hsm_hardware")),
            ].into(),
            estimated_time_seconds: Some(180), // 3 minutes for human response
            automatable: false,
            prerequisites: vec!["security_team_available".to_string()],
        })
        .retryable(true)
        .retry_after(10)
        .related_error("CRYPTO_002") // Key generation failures
        .related_error("HSM_001")    // HSM hardware issues
        .build();

    ai_error_analysis("Crypto AI Agent", &crypto_error).await;
    
    Ok(())
}

async fn demonstrate_network_error_ai_response() -> BearDogResult<()> {
    println!("\n🌐 SCENARIO 2: Network Connectivity Issue");
    println!("=========================================");
    
    let network_error = BearDogError::enhanced("NET_001")
        .severity(ErrorSeverity::Medium)
        .category(ErrorCategory::Network)
        .message("Failed to connect to external API: Connection timeout")
        .component("external_api_client")
        .operation("fetch_user_preferences")
        .user_id("user_67890")
        .request_id("req_net_456")
        .add_metadata("endpoint", "https://api.example.com/preferences")
        .add_metadata("timeout_ms", 5000)
        .add_metadata("retry_count", 2)
        .add_metadata("http_status", json!(null))
        .technical_details("TCP connection timeout after 5 seconds - possible network congestion")
        .add_remediation(RemediationAction {
            action_type: "retry_with_backoff".to_string(),
            description: "Retry with exponential backoff strategy".to_string(),
            parameters: [
                ("max_retries".to_string(), json!(3)),
                ("base_delay_ms".to_string(), json!(1000)),
                ("max_delay_ms".to_string(), json!(30000)),
            ].into(),
            estimated_time_seconds: Some(60),
            automatable: true,
            prerequisites: vec![],
        })
        .add_remediation(RemediationAction {
            action_type: "use_cached_data".to_string(),
            description: "Serve from cache while network is unavailable".to_string(),
            parameters: [
                ("cache_ttl_seconds".to_string(), json!(300)),
                ("cache_key".to_string(), json!("user_prefs_67890")),
            ].into(),
            estimated_time_seconds: Some(1),
            automatable: true,
            prerequisites: vec!["cache_available".to_string()],
        })
        .retryable(true)
        .retry_after(5)
        .build();
    
    ai_error_analysis("Network AI Agent", &network_error).await;
    
    Ok(())
}

async fn demonstrate_resource_exhaustion_ai_response() -> BearDogResult<()> {
    println!("\n💾 SCENARIO 3: Resource Exhaustion");
    println!("==================================");
    
    let resource_error = BearDogError::enhanced("RES_001")
        .severity(ErrorSeverity::Critical)
        .category(ErrorCategory::Resource)
        .message("Memory limit exceeded: 8.2GB used of 8.0GB limit")
        .component("genetics_engine")
        .operation("spawn_population")
        .add_metadata("memory_used_bytes", 8_796_093_440_u64) // 8.2GB
        .add_metadata("memory_limit_bytes", 8_589_934_592_u64) // 8.0GB
        .add_metadata("active_spawns", 150)
        .add_metadata("pending_spawns", 75)
        .technical_details("Memory allocation failed during genetic crossover operation")
        .add_remediation(RemediationAction {
            action_type: "cleanup_inactive_spawns".to_string(),
            description: "Clean up completed spawn processes to free memory".to_string(),
            parameters: [
                ("max_age_minutes".to_string(), json!(30)),
                ("force_cleanup".to_string(), json!(true)),
            ].into(),
            estimated_time_seconds: Some(15),
            automatable: true,
            prerequisites: vec![],
        })
        .add_remediation(RemediationAction {
            action_type: "increase_memory_limit".to_string(),
            description: "Increase memory limit to 16GB".to_string(),
            parameters: [
                ("new_limit_gb".to_string(), json!(16)),
                ("requires_restart".to_string(), json!(false)),
            ].into(),
            estimated_time_seconds: Some(5),
            automatable: true,
            prerequisites: vec!["admin_privileges".to_string(), "memory_available".to_string()],
        })
        .add_remediation(RemediationAction {
            action_type: "emergency_spawn_pause".to_string(),
            description: "Temporarily pause new spawn creation".to_string(),
            parameters: [
                ("pause_duration_minutes".to_string(), json!(15)),
                ("notify_users".to_string(), json!(true)),
            ].into(),
            estimated_time_seconds: Some(1),
            automatable: true,
            prerequisites: vec![],
        })
        .retryable(false) // Resource exhaustion usually isn't automatically retryable
        .build();
    
    ai_error_analysis("Resource Management AI", &resource_error).await;
    
    Ok(())
}

async fn ai_error_analysis(agent_name: &str, error: &BearDogError) {
    println!("\n🤖 {} analyzing error...", agent_name);

    let ai_report = error.to_ai_report();

    match error.severity() {
        ErrorSeverity::Critical => {
            println!("🚨 CRITICAL ERROR DETECTED");
            println!("   AI Decision: Immediate automated response + human escalation");
        }
        ErrorSeverity::High => {
            println!("⚠️  HIGH PRIORITY ERROR");
            println!("   AI Decision: Attempt automated resolution");
        }
        ErrorSeverity::Medium => {
            println!("ℹ️  MEDIUM PRIORITY ERROR");
            println!("   AI Decision: Standard remediation workflow");
        }
        _ => {
            println!("✅ LOW PRIORITY ERROR");
            println!("   AI Decision: Log and monitor");
        }
    }

    if let Some(context) = error.context() {
        println!("\n📊 AI Context Analysis:");
        println!("   Error ID: {}", context.error_id);
        println!("   Component: {}", context.component);
        println!("   Operation: {}", context.operation);
        println!("   Timestamp: {}", context.timestamp);
        
        if let Some(user_id) = &context.user_id {
            println!("   User Impact: {} (specific user affected)", user_id);
        } else {
            println!("   User Impact: System-wide issue");
        }
    }

    let automated_actions = error.automated_remediation_actions();
    let manual_actions = error.manual_remediation_actions();
    
    println!("\n🔧 AI Action Analysis:");
    println!("   Automated actions available: {}", automated_actions.len());
    println!("   Manual actions required: {}", manual_actions.len());

    if !automated_actions.is_empty() {
        let best_action = automated_actions.iter()
            .min_by_key(|action| action.estimated_time_seconds.unwrap_or(u64::MAX))
            .unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
});
        
        println!("\n🎯 AI Selected Action:");
        println!("   Action: {}", best_action.description);
        println!("   Type: {}", best_action.action_type);
        println!("   Estimated Time: {}s", best_action.estimated_time_seconds.unwrap_or(0));
        println!("   Prerequisites: {:?}", best_action.prerequisites);

        simulate_ai_action_execution(best_action).await;
    }

    if !manual_actions.is_empty() || error.is_critical() {
        println!("\n👤 AI Human Assistance Request:");
        println!("   Reason: {}", if error.is_critical() {
            "Critical severity requires human oversight"
        } else {
            "Manual actions required"
        });
        
        for action in manual_actions {
            println!("   Manual Action: {}", action.description);
        }
    }

    println!("\n📋 AI Generated Report (JSON):");
    println!("{}", serde_json::to_string_pretty(&ai_report).unwrap_or_default());
    
    println!("\n" + "=".repeat(60).as_str());
}

async fn simulate_ai_action_execution(action: &RemediationAction) {
    println!("\n⚡ AI Executing Action: {}", action.action_type);
    
    match action.action_type.as_str() {
        "retry_with_backoff" => {
            println!("   🔄 AI implementing exponential backoff retry strategy");
            
            if let Some(max_retries) = action.parameters.get("max_retries").and_then(|v| v.as_u64()) {
                println!("   📊 Max retries: {}", max_retries);
            }

            for i in 1..=3 {
                println!("   ⏳ Retry attempt {} in progress...", i);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                
                if i == 2 {
                    println!("   ✅ AI: Retry successful on attempt {}", i);
                    break;
                }
                println!("   ❌ AI: Attempt {} failed, continuing...", i);
            }
        }
        
        "use_cached_data" => {
            println!("   💾 AI switching to cached data source");
            
            if let Some(cache_key) = action.parameters.get("cache_key").and_then(|v| v.as_str()) {
                println!("   🔑 Cache key: {}", cache_key);
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            println!("   ✅ AI: Successfully serving from cache");
        }
        
        "cleanup_inactive_spawns" => {
            println!("   🧹 AI cleaning up inactive spawn processes");

            for i in 1..=5 {
                println!("   🔄 Cleaning spawn batch {}...", i);
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
            
            println!("   ✅ AI: Freed 2.1GB of memory by cleaning 47 inactive spawns");
        }
        
        "increase_memory_limit" => {
            println!("   📈 AI increasing system memory limits");
            
            if let Some(new_limit) = action.parameters.get("new_limit_gb").and_then(|v| v.as_u64()) {
                println!("   💾 New limit: {}GB", new_limit);
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            println!("   ✅ AI: Memory limit increased successfully");
        }
        
        _ => {
            println!("   ❓ AI: Unknown action type - logging for human review");
        }
    }
    
    println!("   ⏱️  Action completed in {}s", action.estimated_time_seconds.unwrap_or(0));
} 