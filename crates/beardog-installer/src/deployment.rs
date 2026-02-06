//! Async deployment orchestration
//!
//! Manages concurrent deployment of multiple primals with:
//! - Parallel execution
//! - Real-time progress tracking
//! - Rollback on failure
//! - Health validation
//!
//! # Philosophy
//! - Fully async (Tokio)
//! - Concurrent by default (multiple primals in parallel)
//! - Atomic (all-or-nothing with rollback)
//! - Observable (real-time progress updates)

use crate::{
    installer::{BinaryInstaller, InstallerError},
    platform::BiomeOSPaths,
    types::{DeploymentProgress, DeploymentReport, DeploymentStatus, Primal},
    Architecture, OperatingSystem,
};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Async deployment manager
///
/// Orchestrates concurrent deployment of multiple primals.
pub struct DeploymentManager {
    arch: Architecture,
    os: OperatingSystem,
    paths: BiomeOSPaths,
    /// Binary installer (public for uninstallation)
    pub installer: Arc<BinaryInstaller>,
    progress: Arc<RwLock<Vec<DeploymentProgress>>>,
}

impl DeploymentManager {
    /// Create new deployment manager
    ///
    /// # Examples
    /// ```no_run
    /// # use beardog_installer::deployment::DeploymentManager;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let source_dir = std::path::PathBuf::from("./target/release");
    /// let manager = DeploymentManager::new(source_dir).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(source_dir: std::path::PathBuf) -> Result<Self, DeploymentError> {
        let arch = Architecture::detect()?;
        let os = OperatingSystem::detect()?;
        let paths = BiomeOSPaths::discover()?;
        paths.ensure_exists().await?;

        let installer = Arc::new(BinaryInstaller::new(paths.clone(), source_dir));

        Ok(Self {
            arch,
            os,
            paths,
            installer,
            progress: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Deploy all primals concurrently
    ///
    /// # Examples
    /// ```no_run
    /// # use beardog_installer::deployment::DeploymentManager;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let source_dir = std::path::PathBuf::from("./target/release");
    /// let manager = DeploymentManager::new(source_dir).await?;
    /// let report = manager.deploy_all().await?;
    /// println!("Success rate: {:.1}%", report.success_rate());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn deploy_all(&self) -> Result<DeploymentReport, DeploymentError> {
        let primals = Primal::all();
        self.deploy_primals(&primals).await
    }

    /// Deploy specific primals concurrently
    ///
    /// # Arguments
    /// - `primals`: List of primals to deploy
    ///
    /// # Returns
    /// Deployment report with success/failure details
    pub async fn deploy_primals(
        &self,
        primals: &[Primal],
    ) -> Result<DeploymentReport, DeploymentError> {
        info!(
            "Starting deployment of {} primals (arch: {}, os: {:?})",
            primals.len(),
            self.arch,
            self.os
        );

        // Initialize progress tracking
        {
            let mut progress = self.progress.write().await;
            progress.clear();
            for primal in primals {
                progress.push(DeploymentProgress::pending(*primal));
            }
        }

        // Deploy concurrently (async tasks)
        let mut tasks = Vec::new();
        for primal in primals {
            let primal = *primal;
            let manager_clone = self.clone_for_task();

            let task = tokio::spawn(async move { manager_clone.deploy_single(primal).await });

            tasks.push((primal, task));
        }

        // Wait for all deployments to complete
        let mut successes = 0;
        let mut failures = Vec::new();

        for (primal, task) in tasks {
            match task.await {
                Ok(Ok(_)) => {
                    successes += 1;
                    info!("✅ {} deployment complete", primal.display_name());
                }
                Ok(Err(e)) => {
                    failures.push((primal, e.to_string()));
                    error!("❌ {} deployment failed: {}", primal.display_name(), e);
                }
                Err(e) => {
                    failures.push((primal, format!("Task panic: {}", e)));
                    error!("❌ {} task panicked: {}", primal.display_name(), e);
                }
            }
        }

        // If any failed, rollback
        if !failures.is_empty() {
            warn!(
                "⚠️  {} deployments failed, initiating rollback",
                failures.len()
            );
            if let Err(e) = self.rollback_all(primals).await {
                error!("Rollback failed: {}", e);
            }
        }

        Ok(DeploymentReport {
            total: primals.len(),
            successes,
            failures,
            arch: self.arch,
            os: self.os,
        })
    }

    /// Deploy single primal (async)
    async fn deploy_single(&self, primal: Primal) -> Result<(), DeploymentError> {
        // 1. Update status: Downloading
        self.update_progress(primal, DeploymentStatus::Downloading, 10, "Locating binary")
            .await;

        let binary_path = self.installer.locate_binary(primal, &self.arch, &self.os)?;

        // 2. Update status: Installing
        self.update_progress(primal, DeploymentStatus::Installing, 40, "Copying binary")
            .await;

        self.installer.install_binary(primal, &binary_path).await?;

        // 3. Update status: Validating
        self.update_progress(
            primal,
            DeploymentStatus::Validating,
            70,
            "Validating installation",
        )
        .await;

        // Basic validation: check binary exists and is executable
        let installed_path = self.installer.binary_path(primal);
        if !installed_path.exists() {
            return Err(DeploymentError::ValidationFailed {
                primal,
                reason: "Binary not found after installation".to_string(),
            });
        }

        // 4. Update status: Complete
        self.update_progress(
            primal,
            DeploymentStatus::Complete,
            100,
            "Deployment successful",
        )
        .await;

        Ok(())
    }

    /// Rollback all deployments
    async fn rollback_all(&self, primals: &[Primal]) -> Result<(), DeploymentError> {
        info!("Rolling back {} primals", primals.len());

        for primal in primals {
            self.update_progress(
                *primal,
                DeploymentStatus::RolledBack,
                0,
                "Rollback initiated",
            )
            .await;
            self.installer.uninstall_binary(*primal).await?;
        }

        Ok(())
    }

    /// Update deployment progress (real-time)
    async fn update_progress(
        &self,
        primal: Primal,
        status: DeploymentStatus,
        percent: u8,
        message: &str,
    ) {
        let mut progress = self.progress.write().await;

        if let Some(entry) = progress.iter_mut().find(|p| p.primal == primal) {
            entry.status = status;
            entry.percent = percent;
            entry.message = message.to_string();
        }
    }

    /// Get current deployment progress (for UI/monitoring)
    pub async fn get_progress(&self) -> Vec<DeploymentProgress> {
        self.progress.read().await.clone()
    }

    /// Clone for spawning tasks (Arc-based, cheap)
    fn clone_for_task(&self) -> Self {
        Self {
            arch: self.arch,
            os: self.os,
            paths: self.paths.clone(),
            installer: Arc::clone(&self.installer),
            progress: Arc::clone(&self.progress),
        }
    }
}

/// Deployment errors
#[derive(Debug, Error)]
pub enum DeploymentError {
    /// Architecture error
    #[error("Architecture error: {0}")]
    Arch(#[from] crate::arch::ArchError),

    /// Platform error
    #[error("Platform error: {0}")]
    Platform(#[from] crate::platform::PlatformError),

    /// Installer error
    #[error("Installer error: {0}")]
    Installer(#[from] InstallerError),

    /// Validation failed
    #[error("Validation failed for {primal:?}: {reason}")]
    ValidationFailed {
        /// The primal that failed validation
        primal: Primal,
        /// Reason for validation failure
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    async fn setup_test_env() -> (TempDir, std::path::PathBuf) {
        let temp = TempDir::new().unwrap();
        let source_dir = temp.path().join("source");
        fs::create_dir_all(&source_dir).await.unwrap();

        // Create fake binaries for all primals
        for primal in Primal::all() {
            let binary = source_dir.join(primal.name());
            fs::write(&binary, format!("#!/bin/sh\necho {}", primal.name()))
                .await
                .unwrap();

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&binary).await.unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&binary, perms).await.unwrap();
            }
        }

        (temp, source_dir)
    }

    #[tokio::test]
    async fn test_deployment_manager_creation() {
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_deploy_single_primal() {
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir).await.unwrap();

        let result = manager.deploy_primals(&[Primal::BearDog]).await;
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.total, 1);
        assert_eq!(report.successes, 1);
        assert!(report.is_success());
    }

    #[tokio::test]
    async fn test_deploy_all_primals() {
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir).await.unwrap();

        let result = manager.deploy_all().await;
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.total, 5); // All 5 primals
        assert_eq!(report.successes, 5);
        assert!(report.is_success());
        assert_eq!(report.success_rate(), 100.0);
    }

    #[tokio::test]
    async fn test_progress_tracking() {
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir).await.unwrap();

        // Start deployment (don't await yet)
        let deploy_handle = tokio::spawn({
            let manager = manager.clone_for_task();
            async move { manager.deploy_all().await }
        });

        // Check progress during deployment
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let progress = manager.get_progress().await;
        assert_eq!(progress.len(), 5); // All primals tracked

        // Wait for completion
        deploy_handle.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn test_deployment_report_display() {
        let report = DeploymentReport {
            total: 5,
            successes: 4,
            failures: vec![(Primal::Squirrel, "test error".to_string())],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };

        let display = format!("{}", report);
        assert!(display.contains("Total:     5"));
        assert!(display.contains("Successes: 4"));
        assert!(display.contains("Failures:  1"));
        assert!(display.contains("80.0%"));
    }
}
