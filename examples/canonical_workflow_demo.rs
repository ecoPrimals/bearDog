// Standalone demo of the new canonical workflow traits
// This example shows how the new system works without any legacy dependencies

use beardog_workflows::workflows::canonical_examples::{
    ExampleWorkflow, ExampleWorkflowStatus, ProcessingContext,
    InMemoryWorkflowRepository, ExampleWorkflowProcessor, LoggingWorkflowObserver,
    run_comprehensive_example
};
use beardog_workflows::workflows::canonical_traits::WorkflowService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 BearDog Canonical Workflow System Demo");
    println!("==========================================");
    
    // Run the comprehensive example
    if let Err(e) = run_comprehensive_example().await {
        eprintln!("❌ Example failed: {}", e);
        return Err(e.into());
    }
    
    println!("\n✅ Demo completed successfully!");
    println!("The new canonical workflow system is working perfectly!");
    
    Ok(())
} 