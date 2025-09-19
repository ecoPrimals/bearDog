use beardog_workflows::WorkflowEngine;
use beardog_errors::BearDogError;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("🔄 BearDog Workflow Engine Demo");
    println!("===============================");

    let workflow_engine = WorkflowEngine::new()?;
    
    // Create a simple workflow
    let workflow_config = r#"
    {
        "workflow_id": "demo_workflow",
        "name": "User Authentication Workflow",
        "description": "Demonstrates user authentication and security validation",
        "steps": [
            {
                "step_id": "authenticate_user",
                "name": "Authenticate User",
                "type": "authentication"
            },
            {
                "step_id": "validate_security",
                "name": "Security Validation",
                "type": "security_check"
            }
        ]
    }
    "#;

    println!("📋 Creating workflow from configuration...");
    let workflow = workflow_engine.create_workflow_from_json(workflow_config)?;
    
    println!("✅ Workflow created successfully: {}", workflow.name);
    println!("🔢 Number of steps: {}", workflow.steps.len());

    // Execute the workflow
    println!("🚀 Executing workflow...");
    let execution_result = workflow_engine.execute_workflow(&workflow.id)?;
    
    if execution_result.success {
        println!("✅ Workflow executed successfully!");
        println!("⏱️ Execution time: {}ms", execution_result.execution_time_ms);
    } else {
        println!("❌ Workflow execution failed: {}", execution_result.error_message.unwrap_or_default());
    }

    // Test different user types
    let user_types = vec!["standard_user", "power_user", "admin"];
    
    for user_type in user_types {
        println!("👤 Testing workflow for user type: {}", user_type);
        
        let user_result = workflow_engine.execute_workflow_for_user(&workflow.id, user_type)?;
        
        match user_result.status.as_str() {
            "success" => println!("  ✅ User type '{}' workflow succeeded", user_type),
            "failed " => println!("  ❌ User type '{}' workflow failed ", user_type),
            _ => println!("  ⚠️ User type '{}' workflow status: {}", user_type, user_result.status),
        }
    }

    println!("🏁 Workflow demonstration completed!");
    Ok(())
}
