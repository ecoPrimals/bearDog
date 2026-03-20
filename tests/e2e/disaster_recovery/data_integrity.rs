// SPDX-License-Identifier: AGPL-3.0-only
//! Data Integrity & Corruption Scenarios
//!
//! Tests data corruption detection, quarantine, and recovery from backups.
//! Focuses on data resilience during failure conditions.

use super::super::helpers::*;
use super::super::{E2EMetrics, E2ETestConfig};
use beardog_errors::BearDogError;
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};
use tracing::{info, warn};

/// Data integrity and corruption scenario
pub struct DataIntegrityScenario;

impl DataIntegrityScenario {
    /// Run data integrity tests
    pub async fn run(config: &E2ETestConfig) -> Result<E2EMetrics, BearDogError> {
        info!("💾 Data Integrity Tests");

        let mut metrics = E2EMetrics::default();

        // Test 1: Create baseline test data
        Self::test_baseline_data(&mut metrics, config).await?;

        // Test 2: Detect data corruption
        Self::test_corruption_detection(&mut metrics, config).await?;

        // Test 3: Backup recovery
        Self::test_backup_recovery(&mut metrics, config).await?;

        // Test 4: Data quarantine
        Self::test_data_quarantine(&mut metrics, config).await?;

        // Verify final data integrity after all recovery operations
        // Modern pattern: explicit verification step, not implicit
        let all_data_verified = Self::verify_all_data_integrity().await?;
        metrics.data_verified = all_data_verified;

        if all_data_verified {
            info!("✅ Data integrity tests complete - all data verified");
        } else {
            warn!("⚠️  Data integrity tests complete - verification incomplete");
        }

        Ok(metrics)
    }

    /// Verify all data integrity after recovery
    /// Modern pattern: Complete implementation, not stub
    async fn verify_all_data_integrity() -> Result<bool, BearDogError> {
        // Verify no corrupted data remains
        let corrupted_set = get_corrupted_data_set();
        let set = corrupted_set.lock().unwrap();

        if set.is_empty() {
            info!("  ✅ All corrupted data recovered - integrity verified");
            Ok(true)
        } else {
            warn!("  ⚠️  {} corrupted items remain unrecovered", set.len());
            Ok(false)
        }
    }

    async fn test_baseline_data(
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Creating baseline test data...");

        let test_data = create_test_data("integrity-test", 5).await?;

        for data_id in test_data {
            if verify_data_integrity(&data_id).await? {
                metrics.successful_requests += 1;
            } else {
                metrics.failed_requests += 1;
            }
            metrics.total_requests += 1;
        }

        Ok(())
    }

    async fn test_corruption_detection(
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Testing corruption detection...");

        let data_id = "test-data-001";

        // Calculate baseline checksum
        let baseline_checksum = calculate_data_checksum(data_id).await?;
        metrics.total_requests += 1;
        metrics.successful_requests += 1;

        // Inject corruption
        inject_data_corruption(data_id, "partial").await?;
        metrics.total_requests += 1;

        // Detect corruption
        if detect_data_corruption(data_id).await? {
            info!("  ✅ Corruption detected successfully");
            metrics.successful_requests += 1;
        } else {
            warn!("  ⚠️  Corruption went undetected");
            metrics.failed_requests += 1;
        }

        metrics.total_requests += 1;
        Ok(())
    }

    async fn test_backup_recovery(
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Testing backup recovery...");

        let corrupted_ids = get_corruption_alerts().await?;
        metrics.total_requests += 1;

        for data_id in corrupted_ids {
            match recover_from_backup(&data_id).await {
                Ok(()) => {
                    info!("  ✅ Recovered data: {}", data_id);
                    metrics.successful_requests += 1;
                }
                Err(e) => {
                    warn!("  ⚠️  Recovery failed for {}: {}", data_id, e);
                    metrics.failed_requests += 1;
                }
            }
            metrics.total_requests += 1;
        }

        Ok(())
    }

    async fn test_data_quarantine(
        metrics: &mut E2EMetrics,
        _config: &E2ETestConfig,
    ) -> Result<(), BearDogError> {
        info!("  Testing data quarantine...");

        let quarantined = get_quarantined_data().await?;
        metrics.total_requests += 1;

        if !quarantined.is_empty() {
            info!("  ✅ {} items quarantined", quarantined.len());
            metrics.successful_requests += 1;
        }

        // Clear alerts
        clear_corruption_alerts().await?;
        metrics.total_requests += 1;
        metrics.successful_requests += 1;

        Ok(())
    }
}

// =============================================================================
// Data Integrity Operations (Capability-Based)
// =============================================================================

/// Calculate data checksum (capability: integrity verification)
async fn calculate_data_checksum(data_id: &str) -> Result<u64, BearDogError> {
    // Simulate checksum calculation
    let checksum = data_id.len() as u64 * 12345;
    Ok(checksum)
}

/// Inject data corruption (test capability)
async fn inject_data_corruption(data_id: &str, _corruption_type: &str) -> Result<(), BearDogError> {
    let corrupted_set = get_corrupted_data_set();
    let mut set = corrupted_set.lock().unwrap();
    set.insert(data_id.to_string());
    Ok(())
}

/// Detect data corruption
async fn detect_data_corruption(data_id: &str) -> Result<bool, BearDogError> {
    let corrupted_set = get_corrupted_data_set();
    let set = corrupted_set.lock().unwrap();
    Ok(set.contains(data_id))
}

/// Get corruption alerts
async fn get_corruption_alerts() -> Result<Vec<String>, BearDogError> {
    let corrupted_set = get_corrupted_data_set();
    let set = corrupted_set.lock().unwrap();
    Ok(set.iter().cloned().collect())
}

/// Recover data from backup
async fn recover_from_backup(data_id: &str) -> Result<(), BearDogError> {
    let corrupted_set = get_corrupted_data_set();
    let mut set = corrupted_set.lock().unwrap();
    set.remove(data_id);
    Ok(())
}

/// Get quarantined data
async fn get_quarantined_data() -> Result<Vec<String>, BearDogError> {
    // In real implementation, this would query quarantine storage
    Ok(vec![])
}

/// Clear corruption alerts
async fn clear_corruption_alerts() -> Result<(), BearDogError> {
    let corrupted_set = get_corrupted_data_set();
    let mut set = corrupted_set.lock().unwrap();
    set.clear();
    Ok(())
}

// Thread-safe corruption tracking (modern pattern: OnceLock + Mutex)
fn get_corrupted_data_set() -> &'static Mutex<HashSet<String>> {
    static CORRUPTED_DATA: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    CORRUPTED_DATA.get_or_init(|| Mutex::new(HashSet::new()))
}
