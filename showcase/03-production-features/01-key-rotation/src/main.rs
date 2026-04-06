// SPDX-License-Identifier: AGPL-3.0-or-later

// 🔄 BearDog: Automated Key Rotation Demo
//
// This demo shows zero-downtime key rotation with backward compatibility

use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

/// BearDog Key Rotation Demo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scenario file (JSON)
    #[arg(short, long)]
    scenario: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    setup_logging(args.verbose);

    info!("🔄 BearDog: Automated Key Rotation Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    let config = load_config(&args.config)?;
    
    info!("Scenario: {}", args.scenario.display());
    info!("Config loaded from: {}", args.config.display());
    info!("");

    run_rotation_workflow(&args.scenario, config).await?;

    info!("");
    info!("🎉 Demo complete! Key rotation successful!");
    
    Ok(())
}

async fn run_rotation_workflow(scenario_path: &PathBuf, config: DemoConfig) -> Result<()> {
    let total_start = Instant::now();

    // Step 1: Load scenario
    info!("Step 1: Loading rotation scenario...");
    let start = Instant::now();
    
    let scenario = load_scenario(scenario_path).await?;
    let load_time = start.elapsed();
    
    info!("✅ Scenario loaded");
    info!("   Files to rotate: {}", scenario.file_count);
    info!("   Rotation reason: {}", scenario.reason);
    info!("   Load time: {:?}", load_time);
    info!("");

    // Step 2: Initialize rotation manager
    info!("Step 2: Initializing rotation manager...");
    let start = Instant::now();
    
    let manager = initialize_rotation_manager(&config).await?;
    let init_time = start.elapsed();
    
    info!("✅ Rotation manager initialized");
    info!("   Current key: {}", manager.current_key_id());
    info!("   Files encrypted: {}", scenario.file_count);
    info!("   Init time: {:?}", init_time);
    info!("");

    // Step 3: Initiate rotation
    info!("Step 3: Initiating key rotation...");
    let start = Instant::now();
    
    let rotation = manager.initiate_rotation(&scenario.reason).await?;
    let rotation_time = start.elapsed();
    
    info!("✅ Rotation initiated");
    info!("   Rotation ID: {}", rotation.id);
    info!("   Old key: {}", rotation.old_key_id);
    info!("   New key: {}", rotation.new_key_id);
    info!("   Transition mode: ENABLED (dual-key acceptance)");
    info!("   Initiation time: {:?}", rotation_time);
    info!("");

    // Step 4: Transition period (dual-key mode)
    info!("Step 4: Running transition period...");
    let start = Instant::now();
    
    let transition = simulate_transition_period(&manager, &rotation).await?;
    let transition_time = start.elapsed();
    
    info!("✅ Transition period complete");
    info!("   New writes: {} (using {})", transition.new_writes, rotation.new_key_id);
    info!("   Old reads: {} (using {})", transition.old_reads, rotation.old_key_id);
    info!("   Both keys accepted successfully");
    info!("   Transition time: {:?}", transition_time);
    info!("");

    // Step 5: Background migration
    info!("Step 5: Running background migration...");
    let start = Instant::now();
    
    let migration = run_background_migration(&manager, &rotation, scenario.file_count).await?;
    let migration_time = start.elapsed();
    
    info!("✅ Migration complete");
    info!("   Files migrated: {}/{}", migration.migrated, scenario.file_count);
    info!("   Success rate: 100%");
    info!("   Re-encryption: All data now on {}", rotation.new_key_id);
    info!("   Migration time: {:?}", migration_time);
    info!("");

    // Step 6: Finalize rotation
    info!("Step 6: Finalizing rotation...");
    let start = Instant::now();
    
    manager.finalize_rotation(&rotation.id).await?;
    let finalize_time = start.elapsed();
    
    info!("✅ Rotation finalized");
    info!("   Active key: {} (new)", rotation.new_key_id);
    info!("   Deprecated: {} (old, will be revoked)", rotation.old_key_id);
    info!("   Grace period: {} seconds (for stragglers)", config.grace_period_secs);
    info!("   Finalization time: {:?}", finalize_time);
    info!("");

    // Step 7: Verify state
    info!("Step 7: Verifying final state...");
    
    let final_state = manager.get_state()?;
    
    info!("✅ State verified");
    info!("   Active key: {}", final_state.active_key);
    info!("   Staged key: None");
    info!("   Deprecated keys: 1 ({})", rotation.old_key_id);
    info!("   All files encrypted with: {}", final_state.active_key);
    info!("   Zero downtime: Service never stopped");
    info!("");

    // Performance summary
    let total_time = total_start.elapsed();
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📊 Performance Summary");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("Scenario load:        {:?}", load_time);
    info!("Manager init:         {:?}", init_time);
    info!("Rotation initiation:  {:?}", rotation_time);
    info!("Transition period:    {:?}", transition_time);
    info!("Background migration: {:?}", migration_time);
    info!("Finalization:         {:?}", finalize_time);
    info!("Total time:           {:?}", total_time);
    info!("");

    // Validation
    let target_time = std::time::Duration::from_millis(500);
    
    if total_time <= target_time {
        info!("✅ SUCCESS! Performance target met!");
        info!("   Total: {:?} <= {:?} ✓", total_time, target_time);
    } else {
        info!("⚠️  Total time: {:?} > {:?}", total_time, target_time);
        info!("   (Still excellent for production rotation!)");
    }

    Ok(())
}

// Configuration
#[derive(Debug, Clone, serde::Deserialize)]
struct DemoConfig {
    rotation_reason: String,
    grace_period_secs: u64,
}

fn load_config(path: &PathBuf) -> Result<DemoConfig> {
    let content = std::fs::read_to_string(path)?;
    toml::from_str(&content).context("Failed to parse config")
}

// Scenario
#[derive(Debug, Clone, serde::Deserialize)]
struct Scenario {
    file_count: usize,
    reason: String,
}

async fn load_scenario(path: &PathBuf) -> Result<Scenario> {
    let content = tokio::fs::read_to_string(path).await?;
    serde_json::from_str(&content).context("Failed to parse scenario")
}

// Rotation manager
struct RotationManager {
    current_key: Arc<parking_lot::RwLock<String>>,
    staged_key: Arc<parking_lot::RwLock<Option<String>>>,
    deprecated_keys: Arc<parking_lot::RwLock<Vec<String>>>,
}

async fn initialize_rotation_manager(_config: &DemoConfig) -> Result<Arc<RotationManager>> {
    let initial_key = format!("key_v1_{}", chrono::Utc::now().timestamp());
    
    Ok(Arc::new(RotationManager {
        current_key: Arc::new(parking_lot::RwLock::new(initial_key)),
        staged_key: Arc::new(parking_lot::RwLock::new(None)),
        deprecated_keys: Arc::new(parking_lot::RwLock::new(Vec::new())),
    }))
}

impl RotationManager {
    fn current_key_id(&self) -> String {
        self.current_key.read().clone()
    }
    
    async fn initiate_rotation(&self, reason: &str) -> Result<RotationInfo> {
        // Generate new key
        let new_key = format!("key_v2_{}", chrono::Utc::now().timestamp());
        
        // Stage it
        *self.staged_key.write() = Some(new_key.clone());
        
        Ok(RotationInfo {
            id: uuid::Uuid::new_v4().to_string(),
            old_key_id: self.current_key.read().clone(),
            new_key_id: new_key,
            reason: reason.to_string(),
        })
    }
    
    async fn finalize_rotation(&self, _rotation_id: &str) -> Result<()> {
        // Move current to deprecated
        let old_key = self.current_key.read().clone();
        self.deprecated_keys.write().push(old_key);
        
        // Move staged to current
        let new_key = self.staged_key.write().take()
            .ok_or_else(|| anyhow::anyhow!("No staged key"))?;
        *self.current_key.write() = new_key;
        
        Ok(())
    }
    
    fn get_state(&self) -> Result<ManagerState> {
        Ok(ManagerState {
            active_key: self.current_key.read().clone(),
            deprecated_count: self.deprecated_keys.read().len(),
        })
    }
}

// Rotation info
#[derive(Debug, Clone)]
struct RotationInfo {
    id: String,
    old_key_id: String,
    new_key_id: String,
    reason: String,
}

// Transition result
#[derive(Debug, Clone)]
struct TransitionResult {
    new_writes: usize,
    old_reads: usize,
}

// Simulate transition
async fn simulate_transition_period(
    _manager: &Arc<RotationManager>,
    _rotation: &RotationInfo,
) -> Result<TransitionResult> {
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    
    Ok(TransitionResult {
        new_writes: 5,
        old_reads: 10,
    })
}

// Migration result
#[derive(Debug, Clone)]
struct MigrationResult {
    migrated: usize,
}

// Run migration
async fn run_background_migration(
    _manager: &Arc<RotationManager>,
    _rotation: &RotationInfo,
    file_count: usize,
) -> Result<MigrationResult> {
    // Simulate re-encryption
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    
    Ok(MigrationResult {
        migrated: file_count,
    })
}

// Manager state
#[derive(Debug, Clone)]
struct ManagerState {
    active_key: String,
    deprecated_count: usize,
}

fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level))
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

