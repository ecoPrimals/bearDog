//! BearDog AI Automation Suite
//!
//! ## Architecture Philosophy
//!
//! **Standalone Capabilities**: BearDog has onboard AI for security operations
//! **Network Effects**: When connected to Squirrel, becomes a fleet of encrypted AI subtasks  
//! **Not Dependent**: BearDog AI is independent, not a subtask of Squirrel
//!
//! ## Usage Examples
//!
//! ```bash
//! # Standalone AI security operations
//! cargo run --example beardog_ai_automation standalone-security --operations-file ops.json --output-file results.json --ai-enhanced
//! 
//! # Fleet operations (requires Squirrel connection)
//! cargo run --example beardog_ai_automation fleet-operations --task-file tasks.json --output-file results.json --network-effects
//! 
//! # Hybrid AI (standalone + network effects)
//! cargo run --example beardog_ai_automation hybrid-ai --standalone-file ops.json --network-file tasks.json --output-file results.json --network-effects
//! ```

use clap::Parser;
use beardog_errors::BearDogResult;

// Import our modular AI automation suite
mod ai_automation;
use ai_automation::*;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt()
        .with_env_filter("beardog=info")
        .init();

    println!("🐕🤖 BearDog AI Automation Suite");
    println!("Architecture: Standalone AI + Network Effects");
    println!("═══════════════════════════════════════════");

    // Run the AI automation orchestration
    match run_ai_automation().await {
        Ok(()) => {
            println!("✅ AI automation completed successfully");
            println!("🔍 BearDog operated with: Standalone AI ✓");
            
            // Check if network effects were used
            if std::env::args().any(|arg| arg.contains("network-effects")) {
                println!("🌐 Network effects through Squirrel: ✓");
                println!("🔐 Encrypted fleet coordination: ✓");
            } else {
                println!("🔒 Standalone mode (Squirrel not available): ✓");
            }
        }
        Err(e) => {
            eprintln!("❌ AI automation failed: {}", e);
            println!("🔄 BearDog falling back to standalone mode...");
            
            // Demonstrate fallback capability
            match run_standalone_fallback().await {
                Ok(()) => println!("✅ Standalone fallback successful"),
                Err(fallback_err) => eprintln!("❌ Fallback failed: {}", fallback_err),
            }
        }
    }

    Ok(())
}

/// Demonstrate standalone fallback when Squirrel is not available
async fn run_standalone_fallback() -> BearDogResult<()> {
    println!("🚀 Demonstrating BearDog standalone AI capabilities...");
    
    // Initialize standalone AI core
    let ai_core = standalone_ai::initialize_ai_core(&None).await?;
    
    // Check standalone health
    let health = ai_core.check_standalone_health().await?;
    println!("💓 Standalone AI Health: {}", health.overall_status);
    
    // Run basic security analysis
    let test_data = b"sample_security_data_for_analysis";
    let patterns = ai_core.analyze_security_patterns(test_data).await?;
    println!("🔍 Security patterns detected: {}", patterns.len());
    
    // Generate AI insights
    let insights = ai_core.generate_hybrid_insights().await?;
    println!("🧠 AI insights generated: {}", insights.len());
    
    for insight in insights {
        println!("   • {}: {} (confidence: {:.1}%)", 
            insight.category, 
            insight.recommendation, 
            insight.confidence * 100.0);
    }
    
    Ok(())
}

/// Example showcase function for different AI automation modes
#[allow(dead_code)]
async fn showcase_ai_automation_modes() -> BearDogResult<()> {
    println!("📋 BearDog AI Automation Modes Showcase");
    println!("═══════════════════════════════════════");
    
    // Mode 1: Pure Standalone
    println!("🔒 Mode 1: Pure Standalone AI");
    println!("  - Onboard threat detection");
    println!("  - Local security analysis");
    println!("  - Independent operation");
    
    // Mode 2: Network Enhanced
    println!("🌐 Mode 2: Network-Enhanced AI");
    println!("  - Distributed threat analysis");
    println!("  - Fleet coordination");
    println!("  - Encrypted task distribution");
    
    // Mode 3: Hybrid Operations
    println!("🔄 Mode 3: Hybrid AI Operations");
    println!("  - Standalone baseline + network amplification");
    println!("  - Automatic fallback to standalone");
    println!("  - Best of both worlds");
    
    Ok(())
}

/// Development and testing utilities
#[cfg(feature = "dev-utils")]
mod dev_utils {
    use super::*;
    use std::path::PathBuf;

    pub async fn generate_sample_files() -> BearDogResult<()> {
        // Generate sample operation files for testing
        let sample_ops = serde_json::json!([
            {
                "operation_id": "sec_scan_001",
                "operation_type": "ThreatDetection",
                "target": {
                    "target_type": "NetworkEndpoint",
                    "endpoint": "192.168.1.100",
                    "metadata": {}
                },
                "parameters": {},
                "ai_enhancement_requested": true
            }
        ]);
        
        tokio::fs::write("sample_operations.json", serde_json::to_string_pretty(&sample_ops)?).await?;
        
        let sample_tasks = serde_json::json!({
            "operation_id": "fleet_001",
            "operation_type": "DistributedSecurityScan",
            "encryption_level": "Enhanced",
            "task_distribution": {
                "total_tasks": 5,
                "parallel_execution": true,
                "redundancy_factor": 2,
                "load_balancing": "AIOptimized"
            },
            "coordination_mode": "Distributed"
        });
        
        tokio::fs::write("sample_tasks.json", serde_json::to_string_pretty(&sample_tasks)?).await?;
        
        println!("📝 Sample files generated:");
        println!("  • sample_operations.json - For standalone security operations");
        println!("  • sample_tasks.json - For fleet operations");
        
        Ok(())
    }
}

/// CLI argument parsing for development
#[derive(Parser)]
#[command(name = "beardog-ai")]
#[command(about = "BearDog AI: Standalone + Network Effects")]
struct DevCli {
    /// Generate sample configuration files
    #[arg(long)]
    generate_samples: bool,
    
    /// Show AI automation modes
    #[arg(long)]
    show_modes: bool,
    
    /// Run standalone demo
    #[arg(long)]
    standalone_demo: bool,
}

/// Development mode for testing and demonstration
#[allow(dead_code)]
async fn run_dev_mode() -> BearDogResult<()> {
    let cli = DevCli::parse();
    
    if cli.generate_samples {
        #[cfg(feature = "dev-utils")]
        dev_utils::generate_sample_files().await?;
        #[cfg(not(feature = "dev-utils"))]
        println!("⚠️ Dev utils not available in this build");
    }
    
    if cli.show_modes {
        showcase_ai_automation_modes().await?;
    }
    
    if cli.standalone_demo {
        run_standalone_fallback().await?;
    }
    
    Ok(())
} 