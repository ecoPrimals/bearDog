// SPDX-License-Identifier: AGPL-3.0-only
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! `deploy-pixel8` — CLI entrypoint for BearDog Android deployment (check, build, deploy, run, logs).

use beardog_errors::BearDogError;
mod android;
mod builder;
mod command_runner;
mod device;
mod error;

use crate::{android::AndroidDeployment, builder::RustBuilder, device::DeviceManager};
use clap::{Parser, Subcommand};
use console::{Term, style};
use std::path::PathBuf;
use tracing::Level;

#[derive(Parser)]
#[command(name = "deploy-pixel8")]
#[command(about = "Pure Rust deployment tool for BearDog on Android devices")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    project_root: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(long)]
        device_only: bool,
    },

    Build {
        #[arg(short, long)]
        release: bool,

        #[arg(short, long, default_value = "aarch64-linux-android")]
        target: String,
    },

    Deploy {
        #[arg(short, long)]
        release: bool,

        #[arg(long)]
        skip_build: bool,
    },

    Run {
        args: Vec<String>,
    },

    Logs {
        #[arg(short, long, default_value = "beardog ")]
        package: String,

        #[arg(short, long)]
        follow: bool,
    },

    Full {
        #[arg(short, long)]
        release: bool,
    },
}

#[derive(Debug)]
#[expect(dead_code, reason = "DeployConfig shape kept for future CLI wiring")]
struct DeployConfig {
    release: bool,
    skip_build: bool,
}

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();
    let term = Term::stdout();

    term.write_line(
        &style("🐻 BearDog Pure Rust Android Deployment Tool")
            .blue()
            .bold()
            .to_string(),
    )?;
    term.write_line("")?;

    let project_root = cli.project_root.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("Failed to get current directory: {e}");
            std::process::exit(1);
        })
    });

    let android_deployment = AndroidDeployment::new(None, 33); // Default API level 33
    let device_manager = DeviceManager::new();
    let builder = RustBuilder::new(&project_root);

    match cli.command {
        Commands::Check { device_only } => {
            check_command(&android_deployment, &device_manager, device_only)?;
        }
        Commands::Build { release, target } => {
            build_command(&builder, &android_deployment, release, &target).await?;
        }
        Commands::Deploy {
            release,
            skip_build,
        } => {
            deploy_command(
                &builder,
                &android_deployment,
                &device_manager,
                release,
                skip_build,
            )
            .await?;
        }
        Commands::Run { args } => {
            run_command(&device_manager, &args)?;
        }
        Commands::Logs { package, follow } => {
            logs_command(&device_manager, &package, follow)?;
        }
        Commands::Full { release } => {
            full_command(&builder, &android_deployment, &device_manager, release).await?;
        }
    }

    Ok(())
}

fn check_command(
    android_deployment: &AndroidDeployment,
    device_manager: &DeviceManager,
    device_only: bool,
) -> Result<(), BearDogError> {
    println!("🔍 Checking deployment prerequisites...");

    if !device_only {
        android_deployment.verify_environment()?;
    }

    device_manager.check_devices()?;
    println!("✅ All checks passed!");
    Ok(())
}

/// Builds command
async fn build_command(
    builder: &RustBuilder,
    android_deployment: &AndroidDeployment,
    release: bool,
    target: &str,
) -> Result<(), BearDogError> {
    android_deployment.verify_environment()?;
    builder.build_android_app(release, target).await?;
    Ok(())
}

async fn deploy_command(
    builder: &RustBuilder,
    android_deployment: &AndroidDeployment,
    device_manager: &DeviceManager,
    release: bool,
    skip_build: bool,
) -> Result<(), BearDogError> {
    if !skip_build {
        build_command(
            builder,
            android_deployment,
            release,
            "aarch64-linux-android",
        )
        .await?;
    }

    device_manager.deploy_app(release)?;
    Ok(())
}

/// Runs command
fn run_command(device_manager: &DeviceManager, args: &[String]) -> Result<(), BearDogError> {
    device_manager.run_app(args)?;
    Ok(())
}

fn logs_command(
    device_manager: &DeviceManager,
    package: &str,
    follow: bool,
) -> Result<(), BearDogError> {
    device_manager.show_logs(package, follow)?;
    Ok(())
}

async fn full_command(
    builder: &RustBuilder,
    android_deployment: &AndroidDeployment,
    device_manager: &DeviceManager,
    release: bool,
) -> Result<(), BearDogError> {
    println!("🚀 Running full deployment pipeline...");

    check_command(android_deployment, device_manager, false)?;
    build_command(
        builder,
        android_deployment,
        release,
        "aarch64-linux-android",
    )
    .await?;
    deploy_command(builder, android_deployment, device_manager, release, true).await?;

    println!("✅ Full deployment completed successfully!");
    Ok(())
}

#[cfg(test)]
mod cli_entry_tests {
    use super::*;
    use crate::command_runner::mock::MockAdbCommandRunner;

    #[test]
    fn check_command_device_only_skips_toolchain_verify() {
        let android = AndroidDeployment::new(None, 33);
        let dm = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        check_command(&android, &dm, true).expect("adb mock should satisfy device check");
    }

    #[test]
    fn run_command_delegates_to_device_manager() {
        let dm = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        run_command(&dm, &["extra".to_string()]).expect("mock launch");
    }

    #[test]
    fn logs_command_snapshot_mode() {
        let dm = DeviceManager::with_command_runner(Box::new(MockAdbCommandRunner::new()));
        logs_command(&dm, "com.beardog.test", false).expect("logcat snapshot");
    }
}
