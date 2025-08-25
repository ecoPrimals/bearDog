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


/// # Workflow Constants
///
/// **CANONICAL WORKFLOW-RELATED CONSTANTS**
/// Workflow processing, genetic algorithms, and workflow management settings.

use std::time::Duration;
/// **CANONICAL WORKFLOW CONSTANTS** - Workflow processing and management
pub mod processing {
    use super::Duration;
    /// Maximum concurrent workflows
    pub const MAX_CONCURRENT_WORKFLOWS: usize = 100;
    /// Default workflow timeout
    pub const DEFAULT_WORKFLOW_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    /// Extended workflow timeout for complex operations
    pub const EXTENDED_WORKFLOW_TIMEOUT: Duration = Duration::from_secs(14400); // 4 hours
    /// Workflow step timeout
    pub const WORKFLOW_STEP_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    /// Maximum workflow retries
    pub const MAX_WORKFLOW_RETRIES: u32 = 3;
    /// Workflow queue size
    pub const WORKFLOW_QUEUE_SIZE: usize = 10000;
    /// Workflow batch size
    pub const WORKFLOW_BATCH_SIZE: usize = 50;
    /// Workflow checkpoint interval
    pub const CHECKPOINT_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
    /// Workflow heartbeat interval
    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
    /// Maximum workflow history retention
    pub const WORKFLOW_HISTORY_RETENTION: Duration = Duration::from_secs(86400 * 30); // 30 days
    /// Workflow priority levels
    pub const PRIORITY_LOW: u8 = 1;
    pub const PRIORITY_NORMAL: u8 = 5;
    pub const PRIORITY_HIGH: u8 = 8;
    pub const PRIORITY_CRITICAL: u8 = 10;
}
/// **CANONICAL GENETIC ALGORITHM CONSTANTS** - Genetic optimization parameters
pub mod genetic {
    /// Population size for genetic algorithms
    pub const DEFAULT_POPULATION_SIZE: usize = 100;
    /// Small population size for quick evolution
    pub const SMALL_POPULATION_SIZE: usize = 50;
    /// Large population size for thorough exploration
    pub const LARGE_POPULATION_SIZE: usize = 500;
    /// Maximum generations
    pub const MAX_GENERATIONS: usize = 1000;
    /// Convergence threshold (fitness improvement)
    pub const CONVERGENCE_THRESHOLD: f64 = 0.001;
    /// Mutation rate (probability)
    pub const DEFAULT_MUTATION_RATE: f64 = 0.01;
    /// Crossover rate (probability)
    pub const DEFAULT_CROSSOVER_RATE: f64 = 0.8;
    /// Elite selection percentage
    pub const ELITE_SELECTION_PERCENT: f64 = 0.1;
    /// Tournament selection size
    pub const TOURNAMENT_SELECTION_SIZE: usize = 5;
    /// Fitness evaluation timeout per individual
    pub const FITNESS_EVALUATION_TIMEOUT_MS: u64 = 1000;
    /// Maximum chromosome length
    pub const MAX_CHROMOSOME_LENGTH: usize = 1000;
    /// Genetic diversity thresholds
    pub const MIN_DIVERSITY_THRESHOLD: f64 = 0.1;
    pub const RESTART_DIVERSITY_THRESHOLD: f64 = 0.05;
    /// Genetic algorithm termination criteria
    pub const MAX_STAGNANT_GENERATIONS: usize = 50;
    pub const TARGET_FITNESS_THRESHOLD: f64 = 0.99;
/// **CANONICAL WORKFLOW STATE CONSTANTS** - Workflow execution states
pub mod states {
    /// Workflow execution states
    pub const STATE_PENDING: &str = "pending";
    pub const STATE_RUNNING: &str = "running";
    pub const STATE_PAUSED: &str = "paused";
    pub const STATE_COMPLETED: &str = "completed";
    pub const STATE_FAILED: &str = "failed";
    pub const STATE_CANCELLED: &str = "cancelled";
    pub const STATE_TIMEOUT: &str = "timeout";
    /// Workflow step states
    pub const STEP_STATE_WAITING: &str = "waiting";
    pub const STEP_STATE_EXECUTING: &str = "executing";
    pub const STEP_STATE_SUCCESS: &str = "success";
    pub const STEP_STATE_ERROR: &str = "error";
    pub const STEP_STATE_SKIPPED: &str = "skipped";
    /// Workflow trigger types
    pub const TRIGGER_MANUAL: &str = "manual";
    pub const TRIGGER_SCHEDULED: &str = "scheduled";
    pub const TRIGGER_EVENT: &str = "event";
    pub const TRIGGER_WEBHOOK: &str = "webhook";
    pub const TRIGGER_API: &str = "api";
    /// Workflow execution modes
    pub const MODE_SEQUENTIAL: &str = "sequential";
    pub const MODE_PARALLEL: &str = "parallel";
    pub const MODE_MIXED: &str = "mixed";
    pub const MODE_CONDITIONAL: &str = "conditional";
