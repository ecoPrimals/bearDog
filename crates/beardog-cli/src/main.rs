//! BearDog CLI - AI-First Command Line Interface
//!
//! Pure Rust, machine-readable, automation-friendly CLI for BearDog operations.
//! - JSON output by default
//! - Batch operations support
//! - No interactive prompts
//! - Comprehensive error codes

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

use beardog_config::BearDogConfig;
use beardog_core::BearDogCore;
use beardog_errors::BearDogResult;

mod commands;
use commands::{execute_ai_command, AiCommand};

/// BearDog CLI - AI-First Security Operations
#[derive(Parser)]
#[command(name = "beardog")]
#[command(version = beardog_config::constants::version::VERSION)]
#[command(about = "AI-First Security Manager - Pure Rust, Machine-Readable")]
#[command(
    long_about = "BearDog provides enterprise-grade security operations through AI-optimized APIs and CLI tools. All operations are machine-readable with JSON output, batch processing, and automation-friendly interfaces."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path
    #[arg(long, short)]
    config: Option<PathBuf>,

    /// Enable verbose output
    #[arg(long, short)]
    verbose: bool,

    /// Output format (json, yaml, toml)
    #[arg(long, default_value = "json")]
    format: String,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// AI-optimized operations
    #[command(subcommand)]
    Ai(AiCommand),

    /// System health check
    Health {
        /// Include detailed metrics
        #[arg(long)]
        detailed: bool,
    },

    /// System information
    Info,

    /// Validate configuration
    Validate {
        /// Configuration file to validate
        #[arg(long)]
        config: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .with_ansi(!cli.no_color)
        .init();

    info!("🤖 BearDog AI-First CLI - Pure Rust Security Operations");

    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        BearDogConfig::from_file(config_path).map_err(|e| {
            beardog_errors::BearDogError::config(format!("Failed to load config: {e}"))
        })?
    } else {
        BearDogConfig::from_env().map_err(|e| {
            beardog_errors::BearDogError::config(format!("Failed to load config from env: {e}"))
        })?
    };

    // Initialize BearDog core for commands that need it
    let core = match &cli.command {
        Commands::Ai(_) => {
            let core = BearDogCore::new(config).await?;
            core.start().await?;
            Some(core)
        }
        Commands::Health { .. } => {
            let core = BearDogCore::new(config).await?;
            core.start().await?;
            Some(core)
        }
        _ => None,
    };

    // Execute command
    match cli.command {
        Commands::Ai(ai_command) => {
            execute_ai_command(ai_command, core.as_ref()).await?;
        }
        Commands::Health { detailed } => {
            if let Some(core) = &core {
                let health = core.health_check().await?;
                let response = serde_json::json!({
                    "success": true,
                    "data": {
                        "status": format!("{:?}", health.status),
                        "uptime_seconds": health.uptime.map(|d| d.num_seconds()).unwrap_or(0),
                        "components": health.components,
                        "metrics": if detailed { Some(health.metrics) } else { None },
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                });
                println!("{}", serde_json::to_string_pretty(&response)?);
            }
        }
        Commands::Info => {
            let info = serde_json::json!({
                "success": true,
                "data": {
                    "name": "BearDog Security Manager",
                    "version": beardog_config::constants::version::VERSION,
                    "description": beardog_config::constants::version::DESCRIPTION,
                    "mission": beardog_config::constants::version::MISSION,
                    "architecture": "AI-First, Pure Rust",
                    "interfaces": ["REST API", "CLI", "Batch Processing"],
                    "capabilities": [
                        "Multi-tier HSM management",
                        "Genetic node spawning",
                        "Distributed security operations",
                        "Batch processing",
                        "Machine-readable output",
                        "Automation-friendly"
                    ],
                    "hsm_support": {
                        "mobile_hsm": "Android StrongBox, iOS Secure Enclave",
                        "software_hsm": "Rust-based with secure key storage",
                        "hardware_hsm": "PKCS#11 compatible devices"
                    }
                },
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        Commands::Validate {
            config: config_path,
        } => {
            let config_to_validate = if let Some(path) = config_path {
                path
            } else if let Some(path) = &cli.config {
                path.clone()
            } else {
                return Err(beardog_errors::BearDogError::Configuration {
                    message: "No configuration file specified for validation".to_string(),
                });
            };

            let validation_result = match BearDogConfig::from_file(&config_to_validate) {
                Ok(_) => serde_json::json!({
                    "success": true,
                    "data": {
                        "valid": true,
                        "config_file": config_to_validate,
                        "message": "Configuration is valid",
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }),
                Err(e) => serde_json::json!({
                    "success": false,
                    "error": {
                        "code": "INVALID_CONFIG",
                        "message": e.to_string(),
                        "config_file": config_to_validate,
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }),
            };

            println!("{}", serde_json::to_string_pretty(&validation_result)?);

            if validation_result["success"] == false {
                std::process::exit(1);
            }
        }
    }

    // Graceful shutdown
    if let Some(core) = core {
        core.stop().await?;
    }

    Ok(())
}
