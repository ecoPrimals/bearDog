//! # `BearDog` Pure Rust Android Deployment Tool
//!
//! A comprehensive deployment tool for `BearDog` HSM system on Android devices,
//! written entirely in Rust without any shell dependencies.

mod android;
mod builder;
mod device;
mod error;

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::{style, Term};
use std::path::PathBuf;
use tracing::Level;

use crate::{android::AndroidManager, builder::RustBuilder, device::DeviceManager};

#[derive(Parser)]
#[command(name = "deploy-pixel8")]
#[command(about = "Pure Rust deployment tool for BearDog on Pixel 8 devices")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Project root directory
    #[arg(short, long)]
    project_root: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check prerequisites and device connectivity
    Check {
        /// Check device only (skip build prerequisites)
        #[arg(short, long)]
        device_only: bool,
    },
    /// Build the Rust Android application
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
        /// Target architecture
        #[arg(short, long, default_value = "aarch64-linux-android")]
        target: String,
    },
    /// Deploy to connected device
    Deploy {
        /// Build in release mode before deploying
        #[arg(short, long)]
        release: bool,
        /// Skip build and deploy existing binary
        #[arg(short, long)]
        skip_build: bool,
    },
    /// Run the deployed application
    Run {
        /// Application arguments
        #[arg(short, long)]
        args: Vec<String>,
    },
    /// Monitor application logs
    Logs {
        /// Filter pattern for logs
        #[arg(short, long, default_value = "BearDog")]
        filter: String,
        /// Follow logs continuously
        #[arg(short, long)]
        follow: bool,
    },
    /// Complete deployment workflow
    Full {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    let term = Term::stdout();

    // Print banner
    term.write_line(
        &style("🐻 BearDog Pure Rust Android Deployment Tool")
            .blue()
            .bold()
            .to_string(),
    )?;
    term.write_line(
        &style("==============================================")
            .blue()
            .to_string(),
    )?;
    term.write_line("")?;

    // Determine project root
    let project_root = cli.project_root.unwrap_or_else(|| {
        let default_dir = std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("Failed to get current directory: {e}");
            std::process::exit(1);
        });
        default_dir
    });

    // Initialize managers
    let android_manager = AndroidManager::new(project_root.clone());
    let device_manager = DeviceManager::new();
    let builder = RustBuilder::new(project_root.clone());

    match cli.command {
        Commands::Check { device_only } => {
            check_command(&android_manager, &device_manager, device_only).await?;
        }
        Commands::Build { release, target } => {
            build_command(&builder, &android_manager, release, &target).await?;
        }
        Commands::Deploy {
            release,
            skip_build,
        } => {
            deploy_command(
                &builder,
                &android_manager,
                &device_manager,
                release,
                skip_build,
            )
            .await?;
        }
        Commands::Run { args } => {
            run_command(&device_manager, args).await?;
        }
        Commands::Logs { filter, follow } => {
            logs_command(&device_manager, &filter, follow).await?;
        }
        Commands::Full { release } => {
            full_command(&builder, &android_manager, &device_manager, release).await?;
        }
    }

    Ok(())
}

async fn check_command(
    android_manager: &AndroidManager,
    device_manager: &DeviceManager,
    device_only: bool,
) -> Result<()> {
    let term = Term::stdout();

    term.write_line(
        &style("🔍 Checking system prerequisites...")
            .yellow()
            .to_string(),
    )?;

    if !device_only {
        // Check build environment
        android_manager.check_prerequisites().await?;
        term.write_line(&style("✅ Build prerequisites OK").green().to_string())?;
    }

    // Check device
    let device_info = device_manager.check_device().await?;
    term.write_line(&format!(
        "✅ Device: {} {} (Android {})",
        device_info.manufacturer, device_info.model, device_info.android_version
    ))?;

    if device_info.is_pixel {
        term.write_line(
            &style("✅ Pixel device detected - optimal for BearDog")
                .green()
                .to_string(),
        )?;
    }

    if device_info.has_grapheneos {
        term.write_line(
            &style("✅ GrapheneOS detected - enhanced security")
                .green()
                .to_string(),
        )?;
    }

    term.write_line(&style("🎉 All checks passed!").green().bold().to_string())?;
    Ok(())
}

async fn build_command(
    builder: &RustBuilder,
    android_manager: &AndroidManager,
    release: bool,
    target: &str,
) -> Result<()> {
    let term = Term::stdout();

    term.write_line(
        &style("🔨 Building BearDog Android application...")
            .yellow()
            .to_string(),
    )?;

    // Setup build environment
    android_manager.setup_build_environment().await?;

    // Build the application
    builder.build_android_app(release, target).await?;

    term.write_line(
        &style("✅ Build completed successfully!")
            .green()
            .bold()
            .to_string(),
    )?;
    Ok(())
}

async fn deploy_command(
    builder: &RustBuilder,
    android_manager: &AndroidManager,
    device_manager: &DeviceManager,
    release: bool,
    skip_build: bool,
) -> Result<()> {
    let term = Term::stdout();

    if !skip_build {
        // Build first
        build_command(builder, android_manager, release, "aarch64-linux-android").await?;
        term.write_line("")?;
    }

    term.write_line(&style("📲 Deploying to device...").yellow().to_string())?;

    // Check device
    let device_info = device_manager.check_device().await?;
    term.write_line(&format!(
        "📱 Target: {} {}",
        device_info.manufacturer, device_info.model
    ))?;

    // Deploy
    let build_type = if release { "release" } else { "debug" };
    device_manager.deploy_app(build_type).await?;

    term.write_line(&style("✅ Deployment completed!").green().bold().to_string())?;
    Ok(())
}

async fn run_command(device_manager: &DeviceManager, args: Vec<String>) -> Result<()> {
    let term = Term::stdout();

    term.write_line(
        &style("🚀 Running BearDog on device...")
            .yellow()
            .to_string(),
    )?;

    device_manager.run_app(args).await?;
    Ok(())
}

async fn logs_command(device_manager: &DeviceManager, filter: &str, follow: bool) -> Result<()> {
    let term = Term::stdout();

    term.write_line(&format!("📊 Monitoring logs (filter: '{filter}')..."))?;
    term.write_line(&style("Press Ctrl+C to stop").blue().to_string())?;
    term.write_line("")?;

    device_manager.monitor_logs(filter, follow).await?;
    Ok(())
}

async fn full_command(
    builder: &RustBuilder,
    android_manager: &AndroidManager,
    device_manager: &DeviceManager,
    release: bool,
) -> Result<()> {
    let term = Term::stdout();

    term.write_line(
        &style("🚀 Starting full deployment workflow...")
            .blue()
            .bold()
            .to_string(),
    )?;
    term.write_line("")?;

    // Step 1: Check prerequisites
    term.write_line(&style("Step 1: System Check").bold().to_string())?;
    check_command(android_manager, device_manager, false).await?;
    term.write_line("")?;

    // Step 2: Build
    term.write_line(&style("Step 2: Build Application").bold().to_string())?;
    build_command(builder, android_manager, release, "aarch64-linux-android").await?;
    term.write_line("")?;

    // Step 3: Deploy
    term.write_line(&style("Step 3: Deploy to Device").bold().to_string())?;
    deploy_command(builder, android_manager, device_manager, release, true).await?;
    term.write_line("")?;

    // Success message
    term.write_line(
        &style("🎉 BearDog deployed successfully to Pixel 8!")
            .green()
            .bold()
            .to_string(),
    )?;
    term.write_line("")?;
    term.write_line(&style("Next steps:").bold().to_string())?;
    term.write_line("  • Run 'deploy-pixel8 run' to execute the application")?;
    term.write_line("  • Run 'deploy-pixel8 logs' to monitor output")?;
    term.write_line("  • Run 'deploy-pixel8 logs --follow' for live monitoring")?;

    Ok(())
}
