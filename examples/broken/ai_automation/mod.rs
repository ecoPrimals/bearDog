

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use beardog_errors::BearDogError;

pub mod standalone_ai;
pub mod network_effects;
pub mod encrypted_tasks;
pub mod security_automation;
pub mod genetic_automation;
pub mod performance_ai;

#[derive(Parser)]
#[command(name = "beardog-ai-suite")]
#[command(about = "BearDog AI: Standalone + Network Effects Architecture")]
struct Cli {
    #[command(Commands,

    #[arg(Option<PathBuf>,

    #[arg(OutputFormat,

    #[arg(bool,

    #[arg(bool,
}

#[derive(PathBuf,

        #[arg(PathBuf,

        #[arg(bool,
    },

    FleetOperations {

        #[arg(PathBuf,

        #[arg(FleetMode,

        #[arg(PathBuf,
    },

    HybridAI {

        #[arg(PathBuf,

        #[arg(PathBuf,

        #[arg(PathBuf,
    },

    GeneticAI {

        #[arg(PathBuf,

        #[arg(bool,

        #[arg(PathBuf,
    },

    PerformanceAI {

        #[arg(PathBuf,

        #[arg(bool,

        #[arg(PathBuf,
    },
}

#[derive(bool,
    pub standalone_result: Option<T>,
    pub network_enhanced_result: Option<T>,
    pub fleet_coordination: Option<FleetCoordination>,
    pub ai_insights: Vec<AIInsight>,
    pub execution_time_ms: u64,
}

#[derive(u32,
    pub distributed_tasks: u32,
    pub encryption_level: String,
    pub network_efficiency: f64,
}

#[derive(String,
    pub confidence: f64,
    pub recommendation: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

pub async fn run_ai_automation() -> Result<(), BearDogError> {
    let cli = Cli::parse();

    let ai_core = standalone_ai::initialize_ai_core(&cli.config)?;

    let network_capability = if cli.network_effects {
        network_effects::check_squirrel_connectivity()?
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
            )
        }
        
        Commands::FleetOperations { task_file, mode, output_file } => {
            if let Some(network) = network_capability {
                encrypted_tasks::run_fleet_operations(
                    &ai_core,
                    &network,
                    &task_file,
                    mode,
                    &output_file
                )
            } else {
                Err(beardog_errors::BearDogError::network("AutomationService connection required for fleet operations"))
            }
        }
        
        Commands::HybridAI { standalone_file, network_file, output_file } => {
            run_hybrid_ai_operations(
                &ai_core,
                network_capability.as_ref(),
                &standalone_file,
                &network_file,
                &output_file
            )
        }
        
        Commands::GeneticAI { requests_file, ai_optimize, output_file } => {
            genetic_automation::run_ai_genetics(
                &ai_core,
                network_capability.as_ref(),
                &requests_file,
                ai_optimize,
                &output_file
            )
        }
        
        Commands::PerformanceAI { benchmark_file, ai_optimize, output_file } => {
            performance_ai::run_performance_optimization(&standalone_ai::BearDogAICore,
    network: Option<&network_effects::AutomationServiceNetwork>,
    standalone_file: &PathBuf,
    network_file: &PathBuf,
    output_file: &PathBuf,
) -> Result<(), BearDogError> {

    let standalone_result = security_automation::run_standalone_security(
        ai_core,
        standalone_file,
        &PathBuf::from("/tmp/standalone_result.json"),
        true
    )?;

    let network_result = if let Some(net) = network {
        Some(encrypted_tasks::run_fleet_operations(
            ai_core,
            net,
            network_file,
            FleetMode::Encrypted,
            &PathBuf::from(true,
        standalone_result: Some(network_result,
        fleet_coordination: network.map(|n| n.get_coordination_metrics()),
        ai_insights: ai_core.generate_hybrid_insights(0, // Execution time measurement not yet implemented
    };

    let output = serde_json::to_string_pretty(&hybrid_result)?;
    tokio::fs::write(output_file, output)?;
    
    Ok(())
} 