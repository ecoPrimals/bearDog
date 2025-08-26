

use std::time::Duration;

pub mod processing {
    use super::Duration;

    pub const MAX_CONCURRENT_WORKFLOWS: usize = 100;

    pub const DEFAULT_WORKFLOW_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour

    pub const EXTENDED_WORKFLOW_TIMEOUT: Duration = Duration::from_secs(14400); // 4 hours

    pub const WORKFLOW_STEP_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

    pub const MAX_WORKFLOW_RETRIES: u32 = 3;

    pub const WORKFLOW_QUEUE_SIZE: usize = 10000;

    pub const WORKFLOW_BATCH_SIZE: usize = 50;

    pub const CHECKPOINT_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

    pub const WORKFLOW_HISTORY_RETENTION: Duration = Duration::from_secs(86400 * 30); // 30 days

    pub const PRIORITY_LOW: u8 = 1;
    pub const PRIORITY_NORMAL: u8 = 5;
    pub const PRIORITY_HIGH: u8 = 8;
    pub const PRIORITY_CRITICAL: u8 = 10;
}

pub mod genetic {

    pub const DEFAULT_POPULATION_SIZE: usize = 100;

    pub const SMALL_POPULATION_SIZE: usize = 50;

    pub const LARGE_POPULATION_SIZE: usize = 500;

    pub const MAX_GENERATIONS: usize = 1000;

    pub const CONVERGENCE_THRESHOLD: f64 = 0.001;

    pub const DEFAULT_MUTATION_RATE: f64 = 0.01;

    pub const DEFAULT_CROSSOVER_RATE: f64 = 0.8;

    pub const ELITE_SELECTION_PERCENT: f64 = 0.1;

    pub const TOURNAMENT_SELECTION_SIZE: usize = 5;

    pub const FITNESS_EVALUATION_TIMEOUT_MS: u64 = 1000;

    pub const MAX_CHROMOSOME_LENGTH: usize = 1000;

    pub const MIN_DIVERSITY_THRESHOLD: f64 = 0.1;
    pub const RESTART_DIVERSITY_THRESHOLD: f64 = 0.05;

    pub const MAX_STAGNANT_GENERATIONS: usize = 50;
    pub const TARGET_FITNESS_THRESHOLD: f64 = 0.99;

pub mod states {

    pub const STATE_PENDING: &str = "pending";
    pub const STATE_RUNNING: &str = "running";
    pub const STATE_PAUSED: &str = "paused";
    pub const STATE_COMPLETED: &str = "completed";
    pub const STATE_FAILED: &str = "failed";
    pub const STATE_CANCELLED: &str = "cancelled";
    pub const STATE_TIMEOUT: &str = "timeout";

    pub const STEP_STATE_WAITING: &str = "waiting";
    pub const STEP_STATE_EXECUTING: &str = "executing";
    pub const STEP_STATE_SUCCESS: &str = "success";
    pub const STEP_STATE_ERROR: &str = "error";
    pub const STEP_STATE_SKIPPED: &str = "skipped";

    pub const TRIGGER_MANUAL: &str = "manual";
    pub const TRIGGER_SCHEDULED: &str = "scheduled";
    pub const TRIGGER_EVENT: &str = "event";
    pub const TRIGGER_WEBHOOK: &str = "webhook";
    pub const TRIGGER_API: &str = "api";

    pub const MODE_SEQUENTIAL: &str = "sequential";
    pub const MODE_PARALLEL: &str = "parallel";
    pub const MODE_MIXED: &str = "mixed";
    pub const MODE_CONDITIONAL: &str = "conditional";
