//! BearDog AI Automation Suite
//!
//! ## Architecture Philosophy
//! 
//! **Standalone Capabilities**: BearDog has onboard minimal AI for security operations
//! **Network Effects**: When connected to Squirrel, becomes a fleet of encrypted AI subtasks
//! **Not Dependent**: BearDog AI is independent, not a subtask of Squirrel
//!
//! ## Modules
//!
//! - `standalone_ai` - BearDog's onboard AI security capabilities
//! - `network_effects` - Fleet coordination through Squirrel connection
//! - `encrypted_tasks` - Distributed AI task encryption and coordination
//! - `security_automation` - AI-driven security operations
//! - `genetic_automation` - AI-enhanced genetic spawning
//! - `performance_ai` - AI-driven performance optimization

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use beardog_errors::BearDogResult;

pub mod standalone_ai;
pub mod network_effects;
pub mod encrypted_tasks;
pub mod security_automation;
pub mod genetic_automation;
pub mod performance_ai;

/// AI Automation Suite CLI
#[derive(Parser)]
#[command(name = "beardog-ai-suite")]
#[command(about = "BearDog AI: Standalone + Network Effects Architecture")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file
    #[arg(long, short)]
    config: Option<PathBuf>,
    
    /// Output format
    #[arg(long, default_value = "json")]
    format: OutputFormat,
    
    /// Enable Squirrel network effects (if available)
    #[arg(long)]
    network_effects: bool,
    
    /// Verbose output
    #[arg(long, short)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// BearDog standalone AI security operations
    StandaloneSecurity {
        /// Operations file (JSON)
        #[arg(long)]
        operations_file: PathBuf,
        /// Results output file  
        #[arg(long)]
        output_file: PathBuf,
        /// Use onboard AI enhancement
        #[arg(long)]
        ai_enhanced: bool,
    },
    
    /// Fleet AI operations (requires Squirrel connection)
    FleetOperations {
        /// Task configuration file
        #[arg(long)]
        task_file: PathBuf,
        /// Fleet coordination mode
        #[arg(long, default_value = "encrypted")]
        mode: FleetMode,
        /// Results output file
        #[arg(long)]
        output_file: PathBuf,
    },
    
    /// Hybrid AI operations (standalone + network effects)
    HybridAI {
        /// Standalone operations file
        #[arg(long)]
        standalone_file: PathBuf,
        /// Network tasks file
        #[arg(long)]
        network_file: PathBuf,
        /// Output file
        #[arg(long)]
        output_file: PathBuf,
    },
    
    /// AI-enhanced genetic spawning
    GeneticAI {
        /// Spawn requests file
        #[arg(long)]
        requests_file: PathBuf,
        /// Use AI optimization
        #[arg(long)]
        ai_optimize: bool,
        /// Output file
        #[arg(long)]
        output_file: PathBuf,
    },
    
    /// Performance optimization AI
    PerformanceAI {
        /// Benchmark file
        #[arg(long)]
        benchmark_file: PathBuf,
        /// AI-driven optimization
        #[arg(long)]
        ai_optimize: bool,
        /// Output file
        #[arg(long)]
        output_file: PathBuf,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum OutputFormat {
    Json,
    Yaml,
    Csv,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum FleetMode {
    Encrypted,
    Coordinated,
    Distributed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAutomationResult<T> {
    pub success: bool,
    pub standalone_result: Option<T>,
    pub network_enhanced_result: Option<T>,
    pub fleet_coordination: Option<FleetCoordination>,
    pub ai_insights: Vec<AIInsight>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetCoordination {
    pub connected_nodes: u32,
    pub distributed_tasks: u32,
    pub encryption_level: String,
    pub network_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct AIInsight {
    pub category: String,
    pub confidence: f64,
    pub recommendation: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Main AI automation orchestration
pub async fn run_ai_automation() -> BearDogResult<()> {
    let cli = Cli::parse();
    
    // Initialize BearDog core with AI capabilities
    let ai_core = standalone_ai::initialize_ai_core(&cli.config).await?;
    
    // Check for Squirrel network effects availability
    let network_capability = if cli.network_effects {
        network_effects::check_squirrel_connectivity().await?
    } else {
        None
    };
    
    match cli.command {
        Commands::StandaloneSecurity { operations_file, output_file, ai_enhanced } => {
            security_automation::run_standalone_security(
                &ai_core,
                &operations_file,
                &output_file,
                ai_enhanced
            ).await
        }
        
        Commands::FleetOperations { task_file, mode, output_file } => {
            if let Some(network) = network_capability {
                encrypted_tasks::run_fleet_operations(
                    &ai_core,
                    &network,
                    &task_file,
                    mode,
                    &output_file
                ).await
            } else {
                Err(beardog_errors::BearDogError::network("Squirrel connection required for fleet operations"))
            }
        }
        
        Commands::HybridAI { standalone_file, network_file, output_file } => {
            run_hybrid_ai_operations(
                &ai_core,
                network_capability.as_ref(),
                &standalone_file,
                &network_file,
                &output_file
            ).await
        }
        
        Commands::GeneticAI { requests_file, ai_optimize, output_file } => {
            genetic_automation::run_ai_genetics(
                &ai_core,
                network_capability.as_ref(),
                &requests_file,
                ai_optimize,
                &output_file
            ).await
        }
        
        Commands::PerformanceAI { benchmark_file, ai_optimize, output_file } => {
            performance_ai::run_performance_optimization(
                &ai_core,
                network_capability.as_ref(),
                &benchmark_file,
                ai_optimize,
                &output_file
            ).await
        }
    }
}

async fn run_hybrid_ai_operations(
    ai_core: &standalone_ai::BearDogAICore,
    network: Option<&network_effects::SquirrelNetwork>,
    standalone_file: &PathBuf,
    network_file: &PathBuf,
    output_file: &PathBuf,
) -> BearDogResult<()> {
    // Run standalone operations
    let standalone_result = security_automation::run_standalone_security(
        ai_core,
        standalone_file,
        &PathBuf::from("/tmp/standalone_result.json"),
        true
    ).await?;
    
    // If network available, run enhanced operations
    let network_result = if let Some(net) = network {
        Some(encrypted_tasks::run_fleet_operations(
            ai_core,
            net,
            network_file,
            FleetMode::Encrypted,
            &PathBuf::from("/tmp/network_result.json")
        ).await?)
    } else {
        None
    };
    
    // Combine results with AI insights
    let hybrid_result = AIAutomationResult {
        success: true,
        standalone_result: Some(standalone_result),
        network_enhanced_result: network_result,
        fleet_coordination: network.map(|n| n.get_coordination_metrics()),
        ai_insights: ai_core.generate_hybrid_insights().await?,
        execution_time_ms: 0, // TODO: measure actual time
    };
    
    // Save combined results
    let output = serde_json::to_string_pretty(&hybrid_result)?;
    tokio::fs::write(output_file, output).await?;
    
    Ok(())
} 