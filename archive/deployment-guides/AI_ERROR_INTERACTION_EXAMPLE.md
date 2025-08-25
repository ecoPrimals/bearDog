# AI Error Interaction Example: World-Class Error System

## 🤖 **How AI Agents Understand BearDog Errors**

This demonstrates how an AI agent acting on behalf of a human can **understand, analyze, and respond** to BearDog errors intelligently.

## 📋 **Example Scenario: Network Connection Failure**

### **1. Error Occurs in Production**
```rust
// In production code - network request fails
let result = client.get("https://api.example.com/data").send().await;

match result {
    Ok(response) => { /* handle success */ }
    Err(network_error) => {
        // Create AI-friendly error
        let error = BearDogError::enhanced("NET_001")
            .severity(ErrorSeverity::Medium)
            .category(ErrorCategory::Network)
            .message("Failed to connect to external API")
            .component("http_client")
            .operation("fetch_external_data")
            .user_id("user_12345")
            .request_id("req_98765")
            .add_metadata("endpoint", "https://api.example.com/data")
            .add_metadata("timeout_ms", 5000)
            .add_metadata("retry_count", 2)
            .add_remediation(RemediationAction {
                action_type: "retry_with_backoff".to_string(),
                description: "Retry request with exponential backoff".to_string(),
                parameters: [
                    ("max_retries".to_string(), json!(3)),
                    ("base_delay_ms".to_string(), json!(1000)),
                    ("max_delay_ms".to_string(), json!(10000)),
                ].into(),
                estimated_time_seconds: Some(15),
                automatable: true,
                prerequisites: vec![],
            })
            .add_remediation(RemediationAction {
                action_type: "switch_to_backup_endpoint".to_string(),
                description: "Use backup API endpoint".to_string(),
                parameters: [
                    ("backup_endpoint".to_string(), json!("https://backup-api.example.com/data")),
                ].into(),
                estimated_time_seconds: Some(2),
                automatable: true,
                prerequisites: vec!["backup_endpoint_available".to_string()],
            })
            .add_remediation(RemediationAction {
                action_type: "notify_human_operator".to_string(),
                description: "Alert human operator about persistent network issues".to_string(),
                parameters: [
                    ("urgency".to_string(), json!("medium")),
                    ("escalation_path".to_string(), json!("network_team")),
                ].into(),
                estimated_time_seconds: Some(300), // 5 minutes for human response
                automatable: false,
                prerequisites: vec!["human_available".to_string()],
            })
            .retryable(true)
            .retry_after(5)
            .related_error("NET_002") // DNS resolution failures
            .related_error("NET_003") // Certificate validation errors
            .build();
        
        return Err(error);
    }
}
```

### **2. AI Agent Receives Error**
```rust
// AI Agent error handling
async fn handle_beardog_error(error: &BearDogError) -> AIDecision {
    // Convert to AI-readable report
    let ai_report = error.to_ai_report();
    
    println!("🤖 AI Agent analyzing error:");
    println!("{}", serde_json::to_string_pretty(&ai_report).unwrap());
    
    // AI decision-making process
    match error.category() {
        ErrorCategory::Network => handle_network_error(error).await,
        ErrorCategory::Security => handle_security_error(error).await,
        ErrorCategory::Resource => handle_resource_error(error).await,
        _ => handle_generic_error(error).await,
    }
}

async fn handle_network_error(error: &BearDogError) -> AIDecision {
    // AI analyzes the error context
    if let Some(context) = error.context() {
        println!("🔍 AI Context Analysis:");
        println!("  Component: {}", context.component);
        println!("  Operation: {}", context.operation);
        println!("  User Impact: {}", context.user_id.as_ref().unwrap_or(&"unknown".to_string()));
    }
    
    // Check severity for urgency
    match error.severity() {
        ErrorSeverity::Critical => {
            println!("🚨 CRITICAL: Immediate action required!");
            return AIDecision::EscalateToHuman;
        }
        ErrorSeverity::High => {
            println!("⚠️  HIGH: Attempting automated resolution");
        }
        _ => {
            println!("ℹ️  Routine issue - proceeding with standard remediation");
        }
    }
    
    // AI examines automated remediation options
    let automated_actions = error.automated_remediation_actions();
    println!("🔧 AI found {} automated remediation options:", automated_actions.len());
    
    for (i, action) in automated_actions.iter().enumerate() {
        println!("  {}. {} ({}s estimated)", 
            i + 1, 
            action.description, 
            action.estimated_time_seconds.unwrap_or(0)
        );
    }
    
    // AI chooses the fastest automated action
    if let Some(best_action) = automated_actions.iter()
        .min_by_key(|a| a.estimated_time_seconds.unwrap_or(u64::MAX)) {
        
        println!("🎯 AI selecting: {}", best_action.description);
        return AIDecision::ExecuteAutomatedAction {
            action: best_action.clone(),
            estimated_time: best_action.estimated_time_seconds.unwrap_or(0),
        };
    }
    
    // If no automated actions available, check manual ones
    let manual_actions = error.manual_remediation_actions();
    if !manual_actions.is_empty() {
        println!("👤 Requires human intervention:");
        for action in manual_actions {
            println!("  - {}", action.description);
        }
        return AIDecision::RequestHumanAssistance {
            actions: manual_actions.into_iter().cloned().collect(),
            urgency: error.severity(),
        };
    }
    
    AIDecision::MonitorAndRetry
}
```

### **3. AI Agent Takes Action**
```rust
#[derive(Debug, Clone)]
enum AIDecision {
    ExecuteAutomatedAction {
        action: RemediationAction,
        estimated_time: u64,
    },
    RequestHumanAssistance {
        actions: Vec<RemediationAction>,
        urgency: ErrorSeverity,
    },
    EscalateToHuman,
    MonitorAndRetry,
}

async fn execute_ai_decision(decision: AIDecision, original_error: &BearDogError) -> BearDogResult<()> {
    match decision {
        AIDecision::ExecuteAutomatedAction { action, estimated_time } => {
            println!("🤖 AI executing: {}", action.description);
            
            match action.action_type.as_str() {
                "retry_with_backoff" => {
                    let max_retries = action.parameters.get("max_retries")
                        .and_then(|v| v.as_u64()).unwrap_or(3);
                    let base_delay = action.parameters.get("base_delay_ms")
                        .and_then(|v| v.as_u64()).unwrap_or(1000);
                    
                    println!("🔄 Retrying with backoff: {} retries, {}ms base delay", 
                        max_retries, base_delay);
                    
                    // AI implements exponential backoff
                    for attempt in 1..=max_retries {
                        let delay = base_delay * 2_u64.pow(attempt as u32 - 1);
                        println!("  Attempt {}: waiting {}ms", attempt, delay);
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                        
                        // Retry the original operation
                        match retry_original_operation().await {
                            Ok(_) => {
                                println!("✅ AI successfully resolved issue on attempt {}", attempt);
                                return Ok(());
                            }
                            Err(_) if attempt < max_retries => {
                                println!("❌ Attempt {} failed, continuing...", attempt);
                                continue;
                            }
                            Err(e) => {
                                println!("❌ All retries exhausted, escalating to human");
                                return Err(BearDogError::enhanced("AI_001")
                                    .severity(ErrorSeverity::High)
                                    .category(ErrorCategory::External)
                                    .message("AI automated retry failed - human intervention required")
                                    .component("ai_agent")
                                    .operation("automated_remediation")
                                    .add_metadata("original_error", original_error.to_json().unwrap_or_default())
                                    .add_metadata("failed_action", action.action_type)
                                    .add_metadata("attempts", max_retries)
                                    .build());
                            }
                        }
                    }
                }
                
                "switch_to_backup_endpoint" => {
                    let backup_endpoint = action.parameters.get("backup_endpoint")
                        .and_then(|v| v.as_str()).unwrap_or("unknown");
                    
                    println!("🔄 AI switching to backup endpoint: {}", backup_endpoint);
                    
                    // AI updates configuration and retries
                    match use_backup_endpoint(backup_endpoint).await {
                        Ok(_) => {
                            println!("✅ AI successfully switched to backup endpoint");
                            return Ok(());
                        }
                        Err(e) => {
                            println!("❌ Backup endpoint also failed: {}", e);
                            return Err(e);
                        }
                    }
                }
                
                _ => {
                    println!("❓ AI doesn't know how to execute: {}", action.action_type);
                    return Err(BearDogError::enhanced("AI_002")
                        .severity(ErrorSeverity::Medium)
                        .category(ErrorCategory::Internal)
                        .message(format!("Unknown automated action: {}", action.action_type))
                        .component("ai_agent")
                        .operation("action_execution")
                        .build());
                }
            }
        }
        
        AIDecision::RequestHumanAssistance { actions, urgency } => {
            println!("👤 AI requesting human assistance (urgency: {:?})", urgency);
            
            // AI creates structured request for human
            let human_request = serde_json::json!({
                "type": "assistance_request",
                "urgency": urgency,
                "context": original_error.to_ai_report(),
                "required_actions": actions,
                "ai_attempted": "Automated options exhausted",
                "human_needed_for": actions.iter()
                    .filter(|a| !a.automatable)
                    .map(|a| &a.description)
                    .collect::<Vec<_>>()
            });
            
            println!("📋 Human assistance request:");
            println!("{}", serde_json::to_string_pretty(&human_request).unwrap());
            
            // In real implementation, this would send to human operator interface
            send_to_human_operator(human_request).await?;
        }
        
        AIDecision::EscalateToHuman => {
            println!("🚨 AI escalating critical issue to human immediately");
            
            let escalation = serde_json::json!({
                "type": "critical_escalation",
                "error": original_error.to_ai_report(),
                "reason": "Critical severity requires immediate human intervention",
                "timestamp": chrono::Utc::now(),
                "ai_assessment": "Beyond automated capabilities"
            });
            
            emergency_escalate_to_human(escalation).await?;
        }
        
        AIDecision::MonitorAndRetry => {
            println!("👀 AI monitoring situation for changes");
            
            // AI sets up monitoring with intelligent retry
            if original_error.is_retryable() {
                if let Some(retry_after) = original_error.retry_after_seconds() {
                    println!("⏰ AI will retry after {} seconds", retry_after);
                    tokio::time::sleep(tokio::time::Duration::from_secs(retry_after)).await;
                    // Retry logic here
                }
            }
        }
    }
    
    Ok(())
}

// Mock functions for demonstration
async fn retry_original_operation() -> BearDogResult<String> {
    // Simulate network request
    use rand::Rng;
    if rand::thread_rng().gen::<f32>() > 0.7 {
        Ok("Success!".to_string())
    } else {
        Err(BearDogError::config("Still failing"))
    }
}

async fn use_backup_endpoint(endpoint: &str) -> BearDogResult<String> {
    println!("🔗 Connecting to backup: {}", endpoint);
    Ok("Backup connection successful".to_string())
}

async fn send_to_human_operator(request: serde_json::Value) -> BearDogResult<()> {
    println!("📨 Sent to human operator interface");
    Ok(())
}

async fn emergency_escalate_to_human(escalation: serde_json::Value) -> BearDogResult<()> {
    println!("🚨 EMERGENCY ESCALATION TRIGGERED");
    Ok(())
}
```

## 🎯 **Key AI-Friendly Features**

### **1. Structured Error Codes**
- `CRYPTO_001`, `NET_001`, `SYNC_001` - AI can categorize and pattern-match
- Related error codes help AI understand error families

### **2. Severity-Based Decision Making**
```rust
match error.severity() {
    ErrorSeverity::Critical => AIDecision::EscalateToHuman,
    ErrorSeverity::High => AIDecision::ExecuteAutomatedAction,
    ErrorSeverity::Medium => AIDecision::MonitorAndRetry,
    ErrorSeverity::Low => AIDecision::LogAndContinue,
}
```

### **3. Automated vs Manual Actions**
```rust
// AI knows what it can do vs what needs humans
let automated = error.automated_remediation_actions();
let manual = error.manual_remediation_actions();

if !automated.is_empty() {
    // AI takes action
} else if !manual.is_empty() {
    // AI requests human help
} else {
    // AI monitors and waits
}
```

### **4. Time-Based Planning**
```rust
// AI can estimate resolution time and plan accordingly
if let Some(time) = error.estimated_resolution_time_seconds() {
    if time < 60 {
        // Quick fix - do it now
    } else if time < 300 {
        // Medium fix - schedule appropriately
    } else {
        // Long fix - may need human approval
    }
}
```

### **5. Rich Context for Decisions**
```rust
// AI understands the full context
if let Some(context) = error.context() {
    // User impact assessment
    let user_impact = match context.user_id {
        Some(_) => "High - affects specific user",
        None => "Medium - system-wide issue"
    };
    
    // Component criticality
    let criticality = match context.component.as_str() {
        "auth_service" => "Critical - affects all users",
        "metrics_collector" => "Low - monitoring only",
        _ => "Medium - standard component"
    };
}
```

## 🏆 **Result: World-Class AI-Human Collaboration**

This error system enables:

✅ **AI Understanding**: Structured, machine-readable error information  
✅ **Intelligent Automation**: AI knows what it can fix vs what needs humans  
✅ **Smart Escalation**: Severity-based decision making  
✅ **Context Awareness**: Rich metadata for informed decisions  
✅ **Time Management**: Estimated resolution times for planning  
✅ **Pattern Recognition**: Error categorization and relationships  
✅ **Human Collaboration**: Clear requests for human assistance when needed  

**An AI agent can now act as an intelligent first responder, handling routine issues automatically while knowing exactly when and how to involve humans for complex problems.** 