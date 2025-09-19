use beardog_workflows::workflows::canonical_examples::{
    run_comprehensive_example, ExampleWorkflow, ExampleWorkflowProcessor, ExampleWorkflowStatus,
    InMemoryWorkflowRepository, LoggingWorkflowObserver, ProcessingContext,
};
use beardog_workflows::workflows::canonical_traits::WorkflowService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    if let Err(e) = run_comprehensive_example() {
        eprintln!("Error: {}", e);
        return Err(e.into());
    }

    println!("[OK] Demo completed successfully!");
    println!("The new canonical workflow system is working perfectly!");

    Ok(())
}
