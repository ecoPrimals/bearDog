// SPDX-License-Identifier: AGPL-3.0-only

//! # beardog-installer CLI
//!
//! Universal genomeBin installer command-line interface.
//!
//! ## Usage
//!
//! ```bash
//! # Install all primals
//! beardog-installer install
//!
//! # Install specific primals
//! beardog-installer install --primals beardog,songbird
//!
//! # Validate installation
//! beardog-installer validate
//!
//! # Show installation paths
//! beardog-installer paths
//!
//! # Uninstall
//! beardog-installer uninstall
//! ```

use anyhow::Result;
use beardog_installer::{
    BiomeOSPaths, PrimalName, cli, deployment::DeploymentManager, validator::BinaryValidator,
};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "beardog-installer")]
#[command(about = "Universal genomeBin Installer for biomeOS NUCLEUS", long_about = None)]
#[command(version)]
struct Cli {
    /// Source directory containing compiled binaries
    #[arg(short, long, default_value = "./target/release")]
    source: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install genomeBin primals
    Install {
        /// Specific primals to install (comma-separated)
        /// If not specified, installs all primals
        #[arg(short, long)]
        primals: Option<String>,

        /// Dry run (don't actually install)
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Validate installed binaries
    Validate {
        /// Specific primals to validate (comma-separated)
        /// If not specified, validates all installed primals
        #[arg(short, long)]
        primals: Option<String>,
    },

    /// Uninstall genomeBin primals
    Uninstall {
        /// Specific primals to uninstall (comma-separated)
        /// If not specified, uninstalls all primals
        #[arg(short, long)]
        primals: Option<String>,
    },

    /// Show installation paths
    Paths,

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Commands::Install { primals, dry_run } => {
            let primals = cli::parse_primals(primals)?;

            if dry_run {
                println!("🔍 Dry run mode - no actual installation");
                println!("Would install: {}", cli::primals_to_string(&primals));
                return Ok(());
            }

            install_primals(&cli.source, &primals).await?;
        }

        Commands::Validate { primals } => {
            let primals = cli::parse_primals(primals)?;
            validate_primals(&primals).await?;
        }

        Commands::Uninstall { primals } => {
            let primals = cli::parse_primals(primals)?;
            uninstall_primals(&cli.source, &primals).await?;
        }

        Commands::Paths => {
            show_paths().await?;
        }

        Commands::Version => {
            println!(
                "{} v{}",
                beardog_installer::NAME,
                beardog_installer::VERSION
            );
            println!("Universal genomeBin installer - Pure Rust, async, platform-agnostic");
        }
    }

    Ok(())
}

/// Install primals
async fn install_primals(source_dir: &Path, primals: &[PrimalName]) -> Result<()> {
    println!("🧬 Installing {} primals...", primals.len());
    println!("   Source: {}", source_dir.display());
    println!("   Primals: {}\n", cli::primals_to_string(primals));

    let manager = DeploymentManager::new(source_dir.to_path_buf()).await?;
    let report = manager.deploy_primals(primals).await?;

    println!("\n{report}");

    if report.is_success() {
        println!("✅ Installation complete!");
    } else {
        anyhow::bail!("Installation failed for {} primals", report.failures.len());
    }

    Ok(())
}

/// Validate primals
async fn validate_primals(primals: &[PrimalName]) -> Result<()> {
    println!("🔍 Validating {} primals...\n", primals.len());

    let paths = BiomeOSPaths::discover()?;
    let validator = BinaryValidator::new();

    let binaries: Vec<_> = primals
        .iter()
        .map(|p| (p.clone(), paths.bin_dir.join(p.name())))
        .collect();

    let reports = validator.validate_all(binaries).await;

    for report in &reports {
        println!("{report}");
    }

    let healthy_count = reports.iter().filter(|r| r.healthy).count();
    let total = reports.len();

    println!("\n📊 Summary: {healthy_count}/{total} primals healthy");

    if healthy_count == total {
        println!("✅ All primals validated successfully!");
    } else {
        anyhow::bail!("{} primals failed validation", total - healthy_count);
    }

    Ok(())
}

/// Uninstall primals
async fn uninstall_primals(source_dir: &Path, primals: &[PrimalName]) -> Result<()> {
    println!("🗑️  Uninstalling {} primals...", primals.len());
    println!("   Primals: {}\n", cli::primals_to_string(primals));

    let manager = DeploymentManager::new(source_dir.to_path_buf()).await?;

    for primal in primals {
        manager.installer.uninstall_binary(primal.clone()).await?;
    }

    println!("✅ Uninstallation complete!");
    Ok(())
}

/// Show installation paths
async fn show_paths() -> Result<()> {
    let paths = BiomeOSPaths::discover()?;

    println!("📂 Installation Paths:\n");
    println!("  Binaries:      {}", paths.bin_dir.display());
    println!("  Data:          {}", paths.data_dir.display());
    println!("  Configuration: {}", paths.config_dir.display());
    println!("  Runtime:       {}", paths.runtime_dir.display());
    println!("  Cache:         {}", paths.cache_dir.display());

    Ok(())
}
