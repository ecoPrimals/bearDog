

use beardog_errors::{BearDogError, BearDogResult, ErrorSeverity, ErrorCategory, RemediationAction};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("🤖 AI Error System Demo");
    println!("========================");
    
    demonstrate_ai_understanding().await?;
    
    Ok(())
}

async fn demonstrate_ai_understanding() -> Result<(), BearDogError> {
    println!("\n🔐 Creating a crypto error that an AI can understand...\n");

    let crypto_error = BearDogError::enhanced("CRYPTO_001")
        .severity(ErrorSeverity::High)
        .category(ErrorCategory::Security)
        .message("AES-256-GCM encryption failed: Invalid key material")
        .component("hsm_manager")
        .operation("encrypt_user_data")
        .user_id("user_12345")
        .request_id("req_crypto_789")
        .add_metadata("key_id", "hsm_key_001")
        .add_metadata("algorithm", "AES-256-GCM")
        .technical_details("HSM reported key material corruption")
        .add_remediation(RemediationAction {
            action_type: "regenerate_key".to_string(),
            description: "Generate new encryption key from HSM".to_string(),
            parameters: HashMap::with_capacity(16),
            estimated_time_seconds: Some(10),
            automatable: true,
            prerequisites: vec!["hsm_available".to_string()],
        })
        .add_remediation(RemediationAction {
            action_type: "notify_security_team".to_string(),
            description: "Alert security team about HSM issue".to_string(),
            parameters: HashMap::with_capacity(16),
            estimated_time_seconds: Some(180),
            automatable: false,
            prerequisites: vec!["security_team_available".to_string()],
        })
        .retryable(true)
        .retry_after(10)
        .build();

    show_ai_understanding(&crypto_error).await;
    
    Ok(())
}

async fn show_ai_understanding(error: &BearDogError) {
    println!("🤖 AI Agent analyzing error...");

    match error.severity() {
        ErrorSeverity::Critical => println!("🚨 AI: CRITICAL - Immediate action required!"),
        ErrorSeverity::High => println!("⚠️  AI: HIGH PRIORITY - Attempting resolution"),
        ErrorSeverity::Medium => println!("ℹ️  AI: MEDIUM - Standard workflow"),
        _ => println!("✅ AI: LOW PRIORITY - Monitoring"),
    }

    if error.is_retryable() {
        if let Some(retry_after) = error.retry_after_seconds() {
            println!("🔄 AI: Error is retryable after {} seconds", retry_after);
        }
    }

    let automated_actions = error.automated_remediation_actions();
    let manual_actions = error.manual_remediation_actions();
    
    println!("\n🔧 AI Action Analysis:");
    println!("   Automated actions: {}", automated_actions.len());
    println!("   Manual actions: {}", manual_actions.len());

    if let Some(best_action) = automated_actions.iter()
        .min_by_key(|action| action.estimated_time_seconds.unwrap_or(u64::MAX)) {
        
        println!("\n🎯 AI Selected Action:");
        println!("   Action: {}", best_action.description);
        println!("   Estimated Time: {}s", best_action.estimated_time_seconds.unwrap_or(0));
        println!("   Can Automate: {}", best_action.automatable);
    }

    let ai_report = error.to_ai_report();
    println!("\n📋 AI Generated Analysis:");
    println!("{}", serde_json::to_string_pretty(&ai_report).unwrap_or_default());

    println!("\n🤖 AI Decision:");
    if error.is_critical() {
        println!("   Escalating to human immediately due to critical severity");
    } else if !automated_actions.is_empty() {
        println!("   Executing automated remediation");
        simulate_ai_action(automated_actions[0]).await;
    } else if !manual_actions.is_empty() {
        println!("   Requesting human assistance for manual actions");
    } else {
        println!("   Monitoring and logging for pattern analysis");
    }
}

async fn simulate_ai_action(action: &RemediationAction) {
    println!("\n⚡ AI Executing: {}", action.action_type);

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    match action.action_type.as_str() {
        "regenerate_key" => {
            println!("   🔑 Generating new HSM key...");
            println!("   ✅ New key generated successfully");
        }
        _ => {
            println!("   🔄 Executing action...");
            println!("   ✅ Action completed");
        }
    }
    
    println!("   ⏱️  Completed in {}s", action.estimated_time_seconds.unwrap_or(0));
} 