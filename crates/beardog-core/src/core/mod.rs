// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// BearDog Core Module
///
/// Central coordination for all BearDog core functionality

pub mod components;
pub mod lifecycle;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::HealthStatus;
use beardog_types::config::unified::BearDogConfig; // Use canonical unified config
use beardog_errors::idiomatic::SystemResult;
// Workflow imports removed - workflows crate disabled for canonical modernization
// use beardog_workflows::workflows::InMemoryApprovalStore;
// use beardog_workflows::workflows::InMemoryWorkflowStore;
// use beardog_workflows::workflows::MultiPartyWorkflowEngine;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
/// Core BearDog system coordinator
pub struct BearDogCore {
    pub state: Arc<RwLock<CoreState>>,
    pub config: BearDogConfig,
    pub security: Arc<BearDogSecurityProvider>,
    pub monitor: Arc<SystemMonitor>,
    pub genetic_optimizer: Arc<GeneticOptimizer>,
}
impl BearDogCore {
    /// Create new BearDog core instance
    pub async fn new(config: BearDogConfig) -> BearDogResult<Self> {
        let state = Arc::new(RwLock::new(CoreState::default()));
        let security = Arc::new(BearDogSecurityProvider::new()?);
        let monitor = Arc::new(SystemMonitor::new()?);
        let genetic_optimizer = Arc::new(GeneticOptimizer::new()?);
        Ok(Self {
            state,
            config,
            security,
            monitor,
            genetic_optimizer,
        })
    }
    /// Initialize all core components
    pub async fn initialize(&self) -> Result<(), SystemError> {
        info!("🚀 Initializing BearDog Core");
        // Update state to show initialization
        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Starting);
        }
        // Initialize components
        self.security.initialize().await?;
        self.monitor.start().await?;
        self.genetic_optimizer.initialize().await?;
        // Update state to show running
                .insert("core".to_string(), ComponentStatus::Running);
            state.overall_health = HealthStatus::healthy();
        info!("✅ BearDog Core initialized successfully");
        Ok(())
// Re-export core types
