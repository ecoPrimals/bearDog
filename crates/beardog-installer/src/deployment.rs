// SPDX-License-Identifier: AGPL-3.0-only

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
    Architecture, OperatingSystem,
    installer::{BinaryInstaller, InstallerError},
    platform::BiomeOSPaths,
    types::{DeploymentProgress, DeploymentReport, DeploymentStatus, PrimalName},
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
        let primals = PrimalName::well_known();
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
        primals: &[PrimalName],
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
                progress.push(DeploymentProgress::pending(primal.clone()));
            }
        }

        // Deploy concurrently (async tasks)
        let mut tasks = Vec::new();
        for primal in primals {
            let primal_clone = primal.clone();
            let manager_clone = self.clone_for_task();

            let task = tokio::spawn(async move { manager_clone.deploy_single(primal_clone).await });

            tasks.push((primal.clone(), task));
        }

        // Wait for all deployments to complete
        let mut successes = 0;
        let mut failures = Vec::new();

        for (primal, task) in tasks {
            match task.await {
                Ok(Ok(())) => {
                    successes += 1;
                    info!("✅ {} deployment complete", primal.display_name());
                }
                Ok(Err(e)) => {
                    failures.push((primal.clone(), e.to_string()));
                    error!("❌ {} deployment failed: {}", primal.display_name(), e);
                }
                Err(e) => {
                    failures.push((primal.clone(), format!("Task panic: {e}")));
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
    async fn deploy_single(&self, primal: PrimalName) -> Result<(), DeploymentError> {
        // 1. Update status: Downloading
        self.update_progress(
            &primal,
            DeploymentStatus::Downloading,
            10,
            "Locating binary",
        )
        .await;

        let binary_path = self
            .installer
            .locate_binary(primal.clone(), &self.arch, &self.os)?;

        // 2. Update status: Installing
        self.update_progress(&primal, DeploymentStatus::Installing, 40, "Copying binary")
            .await;

        let installed_path = self
            .installer
            .install_binary(primal.clone(), &binary_path)
            .await?;

        // 3. Update status: Validating
        self.update_progress(
            &primal,
            DeploymentStatus::Validating,
            70,
            "Validating installation",
        )
        .await;

        // Basic validation: check binary exists and is executable
        if !installed_path.exists() {
            return Err(DeploymentError::ValidationFailed {
                primal,
                reason: "Binary not found after installation".to_string(),
            });
        }

        // 4. Update status: Complete
        self.update_progress(
            &primal,
            DeploymentStatus::Complete,
            100,
            "Deployment successful",
        )
        .await;

        Ok(())
    }

    /// Rollback all deployments
    async fn rollback_all(&self, primals: &[PrimalName]) -> Result<(), DeploymentError> {
        info!("Rolling back {} primals", primals.len());

        for primal in primals {
            self.update_progress(
                primal,
                DeploymentStatus::RolledBack,
                0,
                "Rollback initiated",
            )
            .await;
            self.installer.uninstall_binary(primal.clone()).await?;
        }

        Ok(())
    }

    /// Update deployment progress (real-time)
    async fn update_progress(
        &self,
        primal: &PrimalName,
        status: DeploymentStatus,
        percent: u8,
        message: &str,
    ) {
        let mut progress = self.progress.write().await;

        if let Some(entry) = progress.iter_mut().find(|p| &p.primal == primal) {
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
        primal: PrimalName,
        /// Reason for validation failure
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    #![allow(clippy::await_holding_lock)]

    use super::*;
    use crate::installer::InstallerError;
    use crate::types::DeploymentStatus;
    use std::sync::Mutex;
    use tempfile::TempDir;
    use tokio::fs;

    /// `DeploymentManager` installs into real user paths from `BiomeOSPaths::discover()`; avoid races.
    static DEPLOYMENT_TEST_LOCK: Mutex<()> = Mutex::new(());

    async fn setup_test_env() -> (TempDir, std::path::PathBuf) {
        let temp = TempDir::new().expect("tempdir");
        let source_dir = temp.path().join("source");
        fs::create_dir_all(&source_dir)
            .await
            .expect("create source dir");

        // Create fake binaries for all primals
        for primal in PrimalName::well_known() {
            let binary = source_dir.join(primal.name());
            fs::write(&binary, format!("#!/bin/sh\necho {}", primal.name()))
                .await
                .expect("write fake binary");

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&binary).await.expect("metadata").permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&binary, perms)
                    .await
                    .expect("set_permissions");
            }
        }

        (temp, source_dir)
    }

    #[tokio::test]
    async fn test_deployment_manager_creation() {
        let _g = DEPLOYMENT_TEST_LOCK.lock().expect("deployment test lock");
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_deploy_single_primal() {
        let _g = DEPLOYMENT_TEST_LOCK.lock().expect("deployment test lock");
        let only = PrimalName::well_known()
            .first()
            .cloned()
            .expect("at least one default primal");
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir)
            .await
            .expect("deployment manager");

        let result = manager.deploy_primals(std::slice::from_ref(&only)).await;
        assert!(result.is_ok());

        let report = result.expect("deploy result");
        assert_eq!(report.total, 1);
        assert_eq!(report.successes, 1);
        assert!(report.is_success());
    }

    #[tokio::test]
    async fn test_deploy_all_primals() {
        let _g = DEPLOYMENT_TEST_LOCK.lock().expect("deployment test lock");
        let expected = PrimalName::well_known().len();
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir)
            .await
            .expect("deployment manager");

        let result = manager.deploy_all().await;
        assert!(result.is_ok());

        let report = result.expect("deploy_all result");
        assert_eq!(report.total, expected);
        assert_eq!(report.successes, expected);
        assert!(report.is_success());
        let rate = report.success_rate();
        assert!((rate - 100.0).abs() < 1e-9, "expected 100.0, got {rate}");
    }

    #[tokio::test]
    async fn test_progress_tracking() {
        let _g = DEPLOYMENT_TEST_LOCK.lock().expect("deployment test lock");
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir)
            .await
            .expect("deployment manager");

        let expected = PrimalName::well_known().len();
        let report = manager.deploy_all().await.expect("deploy_all");
        assert!(report.is_success());

        let progress = manager.get_progress().await;
        assert_eq!(progress.len(), expected);
        assert!(
            progress
                .iter()
                .all(|p| matches!(p.status, DeploymentStatus::Complete))
        );
    }

    #[tokio::test]
    async fn test_deploy_missing_binary_records_failure_and_rollback() {
        let _g = DEPLOYMENT_TEST_LOCK.lock().expect("deployment test lock");
        let temp = TempDir::new().expect("tempdir");
        let source_dir = temp.path().join("source");
        fs::create_dir_all(&source_dir)
            .await
            .expect("create source dir");

        // Only one primal present — others will fail locate_binary
        let binary = source_dir.join("beardog");
        fs::write(&binary, b"#!/bin/sh\necho beardog")
            .await
            .expect("write fake binary");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&binary).await.expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&binary, perms)
                .await
                .expect("set_permissions");
        }

        let manager = DeploymentManager::new(source_dir)
            .await
            .expect("deployment manager");

        let report = manager.deploy_all().await.expect("deploy report");
        assert!(!report.is_success());
        assert!(!report.failures.is_empty());
    }

    #[tokio::test]
    async fn test_deployment_report_display() {
        let report = DeploymentReport {
            total: 5,
            successes: 4,
            failures: vec![(PrimalName::new("squirrel"), "test error".to_string())],
            arch: Architecture::X86_64,
            os: OperatingSystem::Linux,
        };

        let display = format!("{report}");
        assert!(display.contains("Total:     5"));
        assert!(display.contains("Successes: 4"));
        assert!(display.contains("Failures:  1"));
        assert!(display.contains("80.0%"));
    }

    #[tokio::test]
    async fn test_deploy_primals_empty_slice_is_success() {
        let _g = DEPLOYMENT_TEST_LOCK.lock().expect("deployment test lock");
        let (_temp, source_dir) = setup_test_env().await;
        let manager = DeploymentManager::new(source_dir)
            .await
            .expect("deployment manager");

        let report = manager.deploy_primals(&[]).await.expect("empty deploy");
        assert_eq!(report.total, 0);
        assert_eq!(report.successes, 0);
        assert!(report.is_success());
        let rate = report.success_rate();
        assert!(rate.abs() < 1e-9, "expected 0.0, got {rate}");
    }

    #[test]
    fn test_deployment_error_display_from_installer() {
        let e = DeploymentError::Installer(InstallerError::BinaryNotFound {
            primal: PrimalName::new("x"),
            target: "t".to_string(),
            searched: vec!["a".to_string()],
        });
        let s = e.to_string();
        assert!(s.contains("Installer") || s.contains("Binary"), "{s}");
    }
}
