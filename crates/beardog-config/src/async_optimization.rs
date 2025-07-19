use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Async optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AsyncOptimizationConfig {
    /// Parallel processing configuration
    pub parallel_processing: ParallelProcessingConfig,
    /// Batching configuration
    pub batching: BatchingConfig,
    /// Concurrency limits
    pub concurrency_limits: ConcurrencyLimitsConfig,
    /// Async runtime configuration
    pub runtime: AsyncRuntimeConfig,
    /// Task scheduling configuration
    pub task_scheduling: TaskSchedulingConfig,
}

/// Parallel processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelProcessingConfig {
    /// Enable parallel processing
    pub enabled: bool,
    /// Number of worker threads
    pub worker_threads: u32,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: u32,
    /// Task splitting strategy
    pub task_splitting: TaskSplittingConfig,
    /// Work-stealing configuration
    pub work_stealing: WorkStealingConfig,
    /// Parallel operation specific settings
    pub operation_configs: HashMap<String, OperationParallelConfig>,
}

/// Task splitting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSplittingConfig {
    /// Enable automatic task splitting
    pub enabled: bool,
    /// Minimum task size for splitting
    pub min_split_size: u32,
    /// Maximum task size before forced splitting
    pub max_task_size: u32,
    /// Split factor (how many subtasks to create)
    pub split_factor: u32,
    /// Dynamic splitting based on load
    pub dynamic_splitting: bool,
}

/// Work-stealing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStealingConfig {
    /// Enable work-stealing
    pub enabled: bool,
    /// Work-stealing strategy
    pub strategy: WorkStealingStrategy,
    /// Stealing attempts before yielding
    pub steal_attempts: u32,
    /// Steal backoff delay
    pub steal_backoff: Duration,
}

/// Work-stealing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkStealingStrategy {
    /// Random stealing
    Random,
    /// Round-robin stealing
    RoundRobin,
    /// Least loaded worker stealing
    LeastLoaded,
    /// NUMA-aware stealing
    NumaAware,
}

/// Operation-specific parallel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationParallelConfig {
    /// Enable parallel processing for this operation
    pub enabled: bool,
    /// Maximum parallelism level
    pub max_parallelism: u32,
    /// Batch size for parallel processing
    pub batch_size: u32,
    /// Chunk size for data processing
    pub chunk_size: u32,
    /// Enable SIMD optimization
    pub enable_simd: bool,
}

/// Batching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchingConfig {
    /// Enable batching
    pub enabled: bool,
    /// Batch configurations for different operations
    pub batch_configs: HashMap<String, BatchConfig>,
    /// Global batching settings
    pub global_settings: GlobalBatchingSettings,
}

/// Configuration for individual batch operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// Target batch size
    pub target_size: u32,
    /// Maximum batch size
    pub max_size: u32,
    /// Minimum batch size
    pub min_size: u32,
    /// Batch timeout (max time to wait for batch to fill)
    pub timeout: Duration,
    /// Batch flushing strategy
    pub flush_strategy: BatchFlushStrategy,
    /// Enable batch compression
    pub enable_compression: bool,
}

/// Batch flushing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchFlushStrategy {
    /// Flush when batch is full
    SizeBased,
    /// Flush based on time intervals
    TimeBased,
    /// Flush based on both size and time
    Hybrid,
    /// Adaptive flushing based on load
    Adaptive,
}

/// Global batching settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalBatchingSettings {
    /// Global batch timeout
    pub global_timeout: Duration,
    /// Enable batch priority ordering
    pub enable_priority_ordering: bool,
    /// Batch queue size limit
    pub queue_size_limit: u32,
    /// Enable batch metrics collection
    pub enable_metrics: bool,
}

/// Concurrency limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyLimitsConfig {
    /// Global concurrency limit
    pub global_limit: u32,
    /// Per-operation concurrency limits
    pub operation_limits: HashMap<String, u32>,
    /// Per-user concurrency limits
    pub user_limits: HashMap<String, u32>,
    /// Semaphore configuration
    pub semaphore: SemaphoreConfig,
    /// Rate limiting for concurrent operations
    pub rate_limiting: ConcurrentRateLimitConfig,
}

/// Semaphore configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemaphoreConfig {
    /// Enable semaphore-based concurrency control
    pub enabled: bool,
    /// Permit acquisition timeout
    pub acquire_timeout: Duration,
    /// Enable fair semaphore (FIFO ordering)
    pub fair: bool,
    /// Semaphore monitoring
    pub enable_monitoring: bool,
}

/// Concurrent rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrentRateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Rate limit window
    pub window: Duration,
    /// Maximum operations per window
    pub max_operations: u32,
    /// Burst allowance
    pub burst_allowance: u32,
    /// Rate limit enforcement strategy
    pub enforcement_strategy: RateLimitEnforcementStrategy,
}

/// Rate limit enforcement strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitEnforcementStrategy {
    /// Drop excess requests
    Drop,
    /// Queue excess requests
    Queue,
    /// Delay excess requests
    Delay,
    /// Adaptive rate limiting
    Adaptive,
}

/// Async runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncRuntimeConfig {
    /// Runtime type
    pub runtime_type: AsyncRuntimeType,
    /// Core threads
    pub core_threads: u32,
    /// Maximum blocking threads
    pub max_blocking_threads: u32,
    /// Thread keep-alive time
    pub thread_keep_alive: Duration,
    /// Thread stack size
    pub thread_stack_size: u32,
    /// Enable thread parking
    pub enable_thread_parking: bool,
    /// Runtime monitoring
    pub monitoring: RuntimeMonitoringConfig,
}

/// Async runtime type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsyncRuntimeType {
    /// Tokio current-thread runtime
    CurrentThread,
    /// Tokio multi-thread runtime
    MultiThread,
    /// Custom runtime
    Custom,
}

/// Runtime monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMonitoringConfig {
    /// Enable runtime monitoring
    pub enabled: bool,
    /// Task monitoring
    pub task_monitoring: bool,
    /// Thread monitoring
    pub thread_monitoring: bool,
    /// Resource monitoring
    pub resource_monitoring: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
}

/// Task scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSchedulingConfig {
    /// Task scheduler type
    pub scheduler_type: TaskSchedulerType,
    /// Task priority levels
    pub priority_levels: u32,
    /// Task queue configurations
    pub queue_configs: HashMap<String, TaskQueueConfig>,
    /// Scheduling algorithm
    pub scheduling_algorithm: SchedulingAlgorithm,
    /// Task preemption
    pub preemption: TaskPreemptionConfig,
}

/// Task scheduler type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskSchedulerType {
    /// First-Come, First-Served
    Fcfs,
    /// Round-Robin
    RoundRobin,
    /// Priority-based
    Priority,
    /// Completely Fair Scheduler
    Cfs,
    /// Work-conserving scheduler
    WorkConserving,
}

/// Task queue configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQueueConfig {
    /// Queue type
    pub queue_type: TaskQueueType,
    /// Maximum queue size
    pub max_size: u32,
    /// Queue overflow strategy
    pub overflow_strategy: QueueOverflowStrategy,
    /// Enable queue monitoring
    pub enable_monitoring: bool,
}

/// Task queue type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskQueueType {
    /// FIFO queue
    Fifo,
    /// LIFO queue
    Lifo,
    /// Priority queue
    Priority,
    /// Work-stealing queue
    WorkStealing,
}

/// Queue overflow strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueOverflowStrategy {
    /// Drop oldest tasks
    DropOldest,
    /// Drop newest tasks
    DropNewest,
    /// Block until space is available
    Block,
    /// Reject new tasks
    Reject,
}

/// Scheduling algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulingAlgorithm {
    /// First-Come, First-Served
    Fcfs,
    /// Shortest Job First
    Sjf,
    /// Round-Robin
    RoundRobin,
    /// Priority scheduling
    Priority,
    /// Multi-level queue
    MultiLevel,
    /// Completely Fair Scheduler
    Cfs,
}

/// Task preemption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPreemptionConfig {
    /// Enable task preemption
    pub enabled: bool,
    /// Preemption strategy
    pub strategy: PreemptionStrategy,
    /// Time slice for preemption
    pub time_slice: Duration,
    /// Preemption priority threshold
    pub priority_threshold: u32,
}

/// Preemption strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreemptionStrategy {
    /// Time-based preemption
    TimeBased,
    /// Priority-based preemption
    PriorityBased,
    /// Resource-based preemption
    ResourceBased,
    /// Adaptive preemption
    Adaptive,
}

impl Default for ParallelProcessingConfig {
    fn default() -> Self {
        let mut operation_configs = HashMap::new();

        // Configure parallel processing for key operations
        operation_configs.insert(
            "encryption".to_string(),
            OperationParallelConfig {
                enabled: true,
                max_parallelism: 8,
                batch_size: 100,
                chunk_size: 4096,
                enable_simd: true,
            },
        );

        operation_configs.insert(
            "genetic_spawning".to_string(),
            OperationParallelConfig {
                enabled: true,
                max_parallelism: 4,
                batch_size: 50,
                chunk_size: 1024,
                enable_simd: false,
            },
        );

        operation_configs.insert(
            "database_operations".to_string(),
            OperationParallelConfig {
                enabled: true,
                max_parallelism: 16,
                batch_size: 1000,
                chunk_size: 8192,
                enable_simd: false,
            },
        );

        Self {
            enabled: true,
            worker_threads: num_cpus::get() as u32,
            max_concurrent_tasks: 1000,
            task_splitting: TaskSplittingConfig::default(),
            work_stealing: WorkStealingConfig::default(),
            operation_configs,
        }
    }
}

impl Default for TaskSplittingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_split_size: 10,
            max_task_size: 1000,
            split_factor: 4,
            dynamic_splitting: true,
        }
    }
}

impl Default for WorkStealingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: WorkStealingStrategy::LeastLoaded,
            steal_attempts: 3,
            steal_backoff: Duration::from_millis(1),
        }
    }
}

impl Default for BatchingConfig {
    fn default() -> Self {
        let mut batch_configs = HashMap::new();

        // Configure batching for different operations
        batch_configs.insert(
            "database_writes".to_string(),
            BatchConfig {
                target_size: 100,
                max_size: 1000,
                min_size: 10,
                timeout: Duration::from_millis(100),
                flush_strategy: BatchFlushStrategy::Hybrid,
                enable_compression: true,
            },
        );

        batch_configs.insert(
            "crypto_operations".to_string(),
            BatchConfig {
                target_size: 50,
                max_size: 500,
                min_size: 5,
                timeout: Duration::from_millis(50),
                flush_strategy: BatchFlushStrategy::SizeBased,
                enable_compression: false,
            },
        );

        batch_configs.insert(
            "genetic_operations".to_string(),
            BatchConfig {
                target_size: 25,
                max_size: 100,
                min_size: 5,
                timeout: Duration::from_millis(200),
                flush_strategy: BatchFlushStrategy::Adaptive,
                enable_compression: true,
            },
        );

        Self {
            enabled: true,
            batch_configs,
            global_settings: GlobalBatchingSettings::default(),
        }
    }
}

impl Default for GlobalBatchingSettings {
    fn default() -> Self {
        Self {
            global_timeout: Duration::from_secs(1),
            enable_priority_ordering: true,
            queue_size_limit: 10000,
            enable_metrics: true,
        }
    }
}

impl Default for ConcurrencyLimitsConfig {
    fn default() -> Self {
        let mut operation_limits = HashMap::new();
        operation_limits.insert("encryption".to_string(), 100);
        operation_limits.insert("database".to_string(), 50);
        operation_limits.insert("genetic_spawning".to_string(), 20);
        operation_limits.insert("file_operations".to_string(), 25);

        let mut user_limits = HashMap::new();
        user_limits.insert("default".to_string(), 10);
        user_limits.insert("admin".to_string(), 50);
        user_limits.insert("system".to_string(), 100);

        Self {
            global_limit: 1000,
            operation_limits,
            user_limits,
            semaphore: SemaphoreConfig::default(),
            rate_limiting: ConcurrentRateLimitConfig::default(),
        }
    }
}

impl Default for SemaphoreConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            acquire_timeout: Duration::from_secs(30),
            fair: true,
            enable_monitoring: true,
        }
    }
}

impl Default for ConcurrentRateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            window: Duration::from_secs(60),
            max_operations: 1000,
            burst_allowance: 100,
            enforcement_strategy: RateLimitEnforcementStrategy::Queue,
        }
    }
}

impl Default for AsyncRuntimeConfig {
    fn default() -> Self {
        Self {
            runtime_type: AsyncRuntimeType::MultiThread,
            core_threads: num_cpus::get() as u32,
            max_blocking_threads: 512,
            thread_keep_alive: Duration::from_secs(60),
            thread_stack_size: 2 * 1024 * 1024, // 2MB
            enable_thread_parking: true,
            monitoring: RuntimeMonitoringConfig::default(),
        }
    }
}

impl Default for RuntimeMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            task_monitoring: true,
            thread_monitoring: true,
            resource_monitoring: true,
            metrics_interval: Duration::from_secs(10),
        }
    }
}

impl Default for TaskSchedulingConfig {
    fn default() -> Self {
        let mut queue_configs = HashMap::new();

        queue_configs.insert(
            "high_priority".to_string(),
            TaskQueueConfig {
                queue_type: TaskQueueType::Priority,
                max_size: 1000,
                overflow_strategy: QueueOverflowStrategy::Block,
                enable_monitoring: true,
            },
        );

        queue_configs.insert(
            "normal_priority".to_string(),
            TaskQueueConfig {
                queue_type: TaskQueueType::Fifo,
                max_size: 5000,
                overflow_strategy: QueueOverflowStrategy::DropOldest,
                enable_monitoring: true,
            },
        );

        queue_configs.insert(
            "low_priority".to_string(),
            TaskQueueConfig {
                queue_type: TaskQueueType::WorkStealing,
                max_size: 10000,
                overflow_strategy: QueueOverflowStrategy::DropNewest,
                enable_monitoring: false,
            },
        );

        Self {
            scheduler_type: TaskSchedulerType::Priority,
            priority_levels: 5,
            queue_configs,
            scheduling_algorithm: SchedulingAlgorithm::Priority,
            preemption: TaskPreemptionConfig::default(),
        }
    }
}

impl Default for TaskPreemptionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: PreemptionStrategy::PriorityBased,
            time_slice: Duration::from_millis(10),
            priority_threshold: 3,
        }
    }
}

impl AsyncOptimizationConfig {
    /// Create production-optimized async configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Aggressive parallel processing
        config.parallel_processing.enabled = true;
        config.parallel_processing.worker_threads = (num_cpus::get() * 2) as u32;
        config.parallel_processing.max_concurrent_tasks = 2000;

        // Larger batch sizes for better throughput
        for batch_config in config.batching.batch_configs.values_mut() {
            batch_config.target_size *= 2;
            batch_config.max_size *= 2;
            batch_config.timeout = Duration::from_millis(50);
        }

        // Higher concurrency limits
        config.concurrency_limits.global_limit = 2000;
        for limit in config.concurrency_limits.operation_limits.values_mut() {
            *limit *= 2;
        }

        // Optimized runtime
        config.runtime.core_threads = (num_cpus::get() * 2) as u32;
        config.runtime.max_blocking_threads = 1024;
        config.runtime.monitoring.enabled = true;

        // Aggressive task scheduling
        config.task_scheduling.scheduler_type = TaskSchedulerType::WorkConserving;
        config.task_scheduling.preemption.enabled = true;
        config.task_scheduling.preemption.time_slice = Duration::from_millis(5);

        config
    }

    /// Create development-optimized async configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Moderate parallel processing
        config.parallel_processing.worker_threads = num_cpus::get() as u32;
        config.parallel_processing.max_concurrent_tasks = 500;

        // Smaller batch sizes for faster feedback
        for batch_config in config.batching.batch_configs.values_mut() {
            batch_config.target_size /= 2;
            batch_config.timeout = Duration::from_millis(200);
        }

        // Conservative concurrency limits
        config.concurrency_limits.global_limit = 500;

        // Development runtime
        config.runtime.monitoring.enabled = true;
        config.runtime.monitoring.task_monitoring = true;
        config.runtime.monitoring.metrics_interval = Duration::from_secs(5);

        // Simple task scheduling
        config.task_scheduling.scheduler_type = TaskSchedulerType::RoundRobin;

        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.parallel_processing.worker_threads == 0 {
            return Err("worker_threads must be greater than 0".to_string());
        }

        if self.concurrency_limits.global_limit == 0 {
            return Err("global_limit must be greater than 0".to_string());
        }

        if self.runtime.core_threads == 0 {
            return Err("core_threads must be greater than 0".to_string());
        }

        for (name, batch_config) in &self.batching.batch_configs {
            if batch_config.min_size > batch_config.max_size {
                return Err(format!("Batch '{name}': min_size must be <= max_size"));
            }
            if batch_config.target_size > batch_config.max_size {
                return Err(format!("Batch '{name}': target_size must be <= max_size"));
            }
            if batch_config.max_size > 100_000 {
                return Err(format!("Batch '{name}': max_size must be <= 100,000"));
            }
        }

        Ok(())
    }
}

/// Async optimization utilities
pub mod utils {
    use super::*;

    /// Calculate optimal batch size based on system resources
    pub fn calculate_optimal_batch_size(
        operation_latency: Duration,
        system_load: f64,
        memory_available: u64,
    ) -> u32 {
        let base_size = 100;
        let latency_factor = 1.0 + (operation_latency.as_millis() as f64 / 1000.0);
        let load_factor = 1.0 + (1.0 - system_load);
        let memory_factor = 1.0 + (memory_available as f64 / 1024.0 / 1024.0 / 1024.0); // GB

        ((base_size as f64 * latency_factor * load_factor * memory_factor) as u32).clamp(10, 10000)
    }

    /// Calculate optimal concurrency level
    pub fn calculate_optimal_concurrency(cpu_cores: u32, io_ratio: f64, memory_gb: u64) -> u32 {
        let cpu_bound_concurrency = cpu_cores * 2;
        let io_bound_concurrency = cpu_cores * 4;
        let memory_bound_concurrency = (memory_gb / 2).max(1) as u32;

        let base_concurrency = if io_ratio > 0.5 {
            io_bound_concurrency
        } else {
            cpu_bound_concurrency
        };

        base_concurrency.min(memory_bound_concurrency)
    }

    /// Estimate task parallelism benefit
    pub fn estimate_parallelism_benefit(
        task_size: u32,
        overhead_ratio: f64,
        cpu_cores: u32,
    ) -> f64 {
        if task_size < 100 {
            return 0.0; // Too small to benefit from parallelism
        }

        let theoretical_speedup = cpu_cores as f64;
        let overhead_penalty = 1.0 - overhead_ratio;
        let actual_speedup = theoretical_speedup * overhead_penalty;

        // Amdahl's law approximation
        let serial_portion = 0.1; // Assume 10% serial portion
        let parallel_speedup = 1.0 / (serial_portion + (1.0 - serial_portion) / actual_speedup);

        parallel_speedup.min(cpu_cores as f64)
    }
}
