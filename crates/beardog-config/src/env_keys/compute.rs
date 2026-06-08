// SPDX-License-Identifier: AGPL-3.0-or-later

//! Testing, AI training, provider performance, and compute client environment variable keys.

// ── Testing / benchmarks ───────────────────────────────────────────────

/// General test timeout (seconds).
pub const ENV_TEST_TIMEOUT_SECS: &str = "BEARDOG_TEST_TIMEOUT_SECS";
/// Property test iterations.
pub const ENV_TEST_PROPERTY_ITERATIONS: &str = "BEARDOG_TEST_PROPERTY_ITERATIONS";
/// Benchmark iterations.
pub const ENV_BENCHMARK_ITERATIONS: &str = "BEARDOG_BENCHMARK_ITERATIONS";
/// Benchmark warmup iterations.
pub const ENV_BENCHMARK_WARMUP_ITERATIONS: &str = "BEARDOG_BENCHMARK_WARMUP_ITERATIONS";
/// Benchmark measurement duration (seconds).
pub const ENV_BENCHMARK_MEASUREMENT_DURATION_SECS: &str =
    "BEARDOG_BENCHMARK_MEASUREMENT_DURATION_SECS";
/// Fast test timeout (seconds).
pub const ENV_TEST_FAST_TIMEOUT_SECS: &str = "BEARDOG_TEST_FAST_TIMEOUT_SECS";
/// Fast property test iterations.
pub const ENV_TEST_FAST_PROPERTY_ITERATIONS: &str = "BEARDOG_TEST_FAST_PROPERTY_ITERATIONS";
/// Thorough property test iterations.
pub const ENV_TEST_THOROUGH_PROPERTY_ITERATIONS: &str = "BEARDOG_TEST_THOROUGH_PROPERTY_ITERATIONS";
/// Thorough test timeout (seconds).
pub const ENV_TEST_THOROUGH_TIMEOUT_SECS: &str = "BEARDOG_TEST_THOROUGH_TIMEOUT_SECS";
/// End-to-end test timeout (seconds).
pub const ENV_E2E_TIMEOUT_SECS: &str = "BEARDOG_E2E_TIMEOUT_SECS";
/// Benchmark confidence level (0.0–1.0).
pub const ENV_BENCHMARK_CONFIDENCE_LEVEL: &str = "BEARDOG_BENCHMARK_CONFIDENCE_LEVEL";
/// Quick benchmark iterations.
pub const ENV_BENCHMARK_QUICK_ITERATIONS: &str = "BEARDOG_BENCHMARK_QUICK_ITERATIONS";
/// Quick benchmark warmup iterations.
pub const ENV_BENCHMARK_QUICK_WARMUP_ITERATIONS: &str = "BEARDOG_BENCHMARK_QUICK_WARMUP_ITERATIONS";
/// Quick benchmark duration (seconds).
pub const ENV_BENCHMARK_QUICK_DURATION_SECS: &str = "BEARDOG_BENCHMARK_QUICK_DURATION_SECS";
/// Thorough benchmark iterations.
pub const ENV_BENCHMARK_THOROUGH_ITERATIONS: &str = "BEARDOG_BENCHMARK_THOROUGH_ITERATIONS";
/// Thorough benchmark warmup iterations.
pub const ENV_BENCHMARK_THOROUGH_WARMUP: &str = "BEARDOG_BENCHMARK_THOROUGH_WARMUP";
/// API integration test timeout (seconds).
pub const ENV_API_TEST_TIMEOUT_SECS: &str = "BEARDOG_API_TEST_TIMEOUT_SECS";
/// API integration test max concurrent requests.
pub const ENV_API_TEST_MAX_CONCURRENT: &str = "BEARDOG_API_TEST_MAX_CONCURRENT";
/// API integration test max retries.
pub const ENV_API_TEST_MAX_RETRIES: &str = "BEARDOG_API_TEST_MAX_RETRIES";
/// API integration test retry delay (milliseconds).
pub const ENV_API_TEST_RETRY_DELAY_MS: &str = "BEARDOG_API_TEST_RETRY_DELAY_MS";
/// Production test health check timeout (seconds).
pub const ENV_PROD_TEST_HEALTH_TIMEOUT_SECS: &str = "BEARDOG_PROD_TEST_HEALTH_TIMEOUT_SECS";
/// Canary deployment traffic percentage.
pub const ENV_CANARY_PERCENTAGE: &str = "BEARDOG_CANARY_PERCENTAGE";
/// Test-only env var for source resolution tests.
pub const ENV_TEST_VAR_UNIQUE_XYZ_NOT_SET: &str = "BEARDOG_TEST_VAR_UNIQUE_XYZ_NOT_SET";

// ── AI / training ────────────────────────────────────────────────────

/// AI training batch size.
pub const ENV_AI_TRAINING_BATCH_SIZE: &str = "BEARDOG_AI_TRAINING_BATCH_SIZE";
/// AI learning rate.
pub const ENV_AI_LEARNING_RATE: &str = "BEARDOG_AI_LEARNING_RATE";
/// AI training epochs.
pub const ENV_AI_EPOCHS: &str = "BEARDOG_AI_EPOCHS";
/// AI training epochs (training-config profile).
pub const ENV_AI_TRAINING_EPOCHS: &str = "BEARDOG_AI_TRAINING_EPOCHS";
/// AI validation split ratio.
pub const ENV_AI_VALIDATION_SPLIT: &str = "BEARDOG_AI_VALIDATION_SPLIT";
/// AI early stopping patience (epochs).
pub const ENV_AI_EARLY_STOPPING_PATIENCE: &str = "BEARDOG_AI_EARLY_STOPPING_PATIENCE";
/// AI checkpoint save frequency (epochs).
pub const ENV_AI_CHECKPOINT_FREQUENCY: &str = "BEARDOG_AI_CHECKPOINT_FREQUENCY";
/// Training params epochs override.
pub const ENV_TRAINING_PARAMS_EPOCHS: &str = "BEARDOG_TRAINING_PARAMS_EPOCHS";
/// Training params batch size override.
pub const ENV_TRAINING_PARAMS_BATCH_SIZE: &str = "BEARDOG_TRAINING_PARAMS_BATCH_SIZE";
/// Training params validation split override.
pub const ENV_TRAINING_PARAMS_VALIDATION_SPLIT: &str = "BEARDOG_TRAINING_PARAMS_VALIDATION_SPLIT";
/// AI early stopping patience in epochs.
pub const ENV_AI_EARLY_STOPPING_PATIENCE_EPOCHS: &str = "BEARDOG_AI_EARLY_STOPPING_PATIENCE_EPOCHS";
/// AI early stopping minimum delta.
pub const ENV_AI_EARLY_STOPPING_MIN_DELTA: &str = "BEARDOG_AI_EARLY_STOPPING_MIN_DELTA";
/// AI checkpoint frequency in epochs.
pub const ENV_AI_CHECKPOINT_FREQUENCY_EPOCHS: &str = "BEARDOG_AI_CHECKPOINT_FREQUENCY_EPOCHS";
/// AI human oversight level.
pub const ENV_AI_HUMAN_OVERSIGHT_LEVEL: &str = "BEARDOG_AI_HUMAN_OVERSIGHT_LEVEL";
/// AI auto-decision confidence threshold.
pub const ENV_AI_AUTO_DECISION_THRESHOLD: &str = "BEARDOG_AI_AUTO_DECISION_THRESHOLD";
/// AI inference/training batch size.
pub const ENV_AI_BATCH_SIZE: &str = "BEARDOG_AI_BATCH_SIZE";
/// AI maximum acceptable latency (milliseconds).
pub const ENV_AI_MAX_LATENCY_MS: &str = "BEARDOG_AI_MAX_LATENCY_MS";
/// AI inference cache size (MB).
pub const ENV_AI_CACHE_SIZE_MB: &str = "BEARDOG_AI_CACHE_SIZE_MB";
/// AI inference cache size (bytes).
pub const ENV_AI_CACHE_SIZE_BYTES: &str = "BEARDOG_AI_CACHE_SIZE_BYTES";
/// AI decision confidence threshold.
pub const ENV_AI_CONFIDENCE_THRESHOLD: &str = "BEARDOG_AI_CONFIDENCE_THRESHOLD";
/// AI worker thread / CPU count.
pub const ENV_AI_CPU_THREADS: &str = "BEARDOG_AI_CPU_THREADS";
/// AI memory limit (MB).
pub const ENV_AI_MEMORY_LIMIT_MB: &str = "BEARDOG_AI_MEMORY_LIMIT_MB";
/// AI mini-batch size for training.
pub const ENV_AI_MINI_BATCH_SIZE: &str = "BEARDOG_AI_MINI_BATCH_SIZE";
/// AI fine-tuning initial learning rate.
pub const ENV_AI_FINETUNING_INITIAL_LR: &str = "BEARDOG_AI_FINETUNING_INITIAL_LR";
/// AI fine-tuning epoch count.
pub const ENV_AI_FINETUNING_EPOCHS: &str = "BEARDOG_AI_FINETUNING_EPOCHS";
/// AI inner-loop optimization steps.
pub const ENV_AI_INNER_LOOP_STEPS: &str = "BEARDOG_AI_INNER_LOOP_STEPS";
/// AI inner-loop learning rate.
pub const ENV_AI_INNER_LOOP_LR: &str = "BEARDOG_AI_INNER_LOOP_LR";
/// AI outer-loop optimization steps.
pub const ENV_AI_OUTER_LOOP_STEPS: &str = "BEARDOG_AI_OUTER_LOOP_STEPS";
/// AI outer-loop learning rate.
pub const ENV_AI_OUTER_LOOP_LR: &str = "BEARDOG_AI_OUTER_LOOP_LR";
/// AI hyperparameter search max trials.
pub const ENV_AI_HYPERPARAMETER_MAX_TRIALS: &str = "BEARDOG_AI_HYPERPARAMETER_MAX_TRIALS";
/// AI human-input wait timeout (seconds).
pub const ENV_AI_HUMAN_INPUT_TIMEOUT_SECS: &str = "BEARDOG_AI_HUMAN_INPUT_TIMEOUT_SECS";
/// AI inference timeout (seconds).
pub const ENV_AI_INFERENCE_TIMEOUT_SECS: &str = "BEARDOG_AI_INFERENCE_TIMEOUT_SECS";
/// AI maximum inference batch size.
pub const ENV_AI_MAX_BATCH_SIZE: &str = "BEARDOG_AI_MAX_BATCH_SIZE";
/// AI neural network dropout rate.
pub const ENV_AI_NEURAL_DROPOUT_RATE: &str = "BEARDOG_AI_NEURAL_DROPOUT_RATE";
/// AI L1 regularization coefficient.
pub const ENV_AI_REGULARIZATION_L1: &str = "BEARDOG_AI_REGULARIZATION_L1";
/// AI L2 regularization coefficient.
pub const ENV_AI_REGULARIZATION_L2: &str = "BEARDOG_AI_REGULARIZATION_L2";
/// AI regularization dropout rate.
pub const ENV_AI_REGULARIZATION_DROPOUT: &str = "BEARDOG_AI_REGULARIZATION_DROPOUT";
/// AI optimizer learning rate.
pub const ENV_AI_OPTIMIZER_LEARNING_RATE: &str = "BEARDOG_AI_OPTIMIZER_LEARNING_RATE";
/// AI worker pool size.
pub const ENV_AI_NUM_WORKERS: &str = "BEARDOG_AI_NUM_WORKERS";

// ── Infrastructure / capabilities ──────────────────────────────────────

/// Maximum network bandwidth (Mbps).
pub const ENV_MAX_BANDWIDTH_MBPS: &str = "BEARDOG_MAX_BANDWIDTH_MBPS";
/// Storage max capacity (bytes).
pub const ENV_STORAGE_MAX_CAPACITY: &str = "BEARDOG_STORAGE_MAX_CAPACITY";
/// Default CPU core count.
pub const ENV_DEFAULT_CORE_COUNT: &str = "BEARDOG_DEFAULT_CORE_COUNT";
/// Default memory (GB).
pub const ENV_DEFAULT_MEMORY_GB: &str = "BEARDOG_DEFAULT_MEMORY_GB";
/// Performance max operations per second.
pub const ENV_PERF_MAX_OPS_PER_SECOND: &str = "BEARDOG_PERF_MAX_OPS_PER_SECOND";
/// Performance average response time (milliseconds).
pub const ENV_PERF_AVG_RESPONSE_TIME_MS: &str = "BEARDOG_PERF_AVG_RESPONSE_TIME_MS";
/// Power consumption (watts).
pub const ENV_POWER_CONSUMPTION_WATTS: &str = "BEARDOG_POWER_CONSUMPTION_WATTS";
/// Minimum interaction time (milliseconds).
pub const ENV_MIN_INTERACTION_TIME_MS: &str = "BEARDOG_MIN_INTERACTION_TIME_MS";
/// Maximum interaction time (milliseconds).
pub const ENV_MAX_INTERACTION_TIME_MS: &str = "BEARDOG_MAX_INTERACTION_TIME_MS";

// ── Provider performance ─────────────────────────────────────────────

/// Provider max concurrent requests.
pub const ENV_PROVIDER_MAX_CONCURRENT_REQUESTS: &str = "BEARDOG_PROVIDER_MAX_CONCURRENT_REQUESTS";
/// Provider cache max entries.
pub const ENV_PROVIDER_CACHE_MAX_ENTRIES: &str = "BEARDOG_PROVIDER_CACHE_MAX_ENTRIES";
/// Provider cache TTL (seconds).
pub const ENV_PROVIDER_CACHE_TTL_SECS: &str = "BEARDOG_PROVIDER_CACHE_TTL_SECS";
/// Compression level (0–9).
pub const ENV_COMPRESSION_LEVEL: &str = "BEARDOG_COMPRESSION_LEVEL";
/// Minimum payload size for compression (bytes).
pub const ENV_COMPRESSION_MIN_SIZE: &str = "BEARDOG_COMPRESSION_MIN_SIZE";
/// Read buffer size (bytes).
pub const ENV_READ_BUFFER_SIZE: &str = "BEARDOG_READ_BUFFER_SIZE";
/// Write buffer size (bytes).
pub const ENV_WRITE_BUFFER_SIZE: &str = "BEARDOG_WRITE_BUFFER_SIZE";
/// Provider metrics collection interval (seconds).
pub const ENV_PROVIDER_METRICS_INTERVAL_SECS: &str = "BEARDOG_PROVIDER_METRICS_INTERVAL_SECS";
/// Provider max response time SLO (milliseconds).
pub const ENV_PROVIDER_MAX_RESPONSE_TIME_MS: &str = "BEARDOG_PROVIDER_MAX_RESPONSE_TIME_MS";
/// Provider max error rate SLO (percent).
pub const ENV_PROVIDER_MAX_ERROR_RATE: &str = "BEARDOG_PROVIDER_MAX_ERROR_RATE";
/// Provider max CPU usage SLO (percent).
pub const ENV_PROVIDER_MAX_CPU_USAGE: &str = "BEARDOG_PROVIDER_MAX_CPU_USAGE";
/// Provider max memory usage SLO (bytes).
pub const ENV_PROVIDER_MAX_MEMORY_USAGE: &str = "BEARDOG_PROVIDER_MAX_MEMORY_USAGE";
/// Provider max concurrent connections SLO.
pub const ENV_PROVIDER_MAX_CONCURRENT_CONNECTIONS: &str =
    "BEARDOG_PROVIDER_MAX_CONCURRENT_CONNECTIONS";
/// Provider alert cooldown (seconds).
pub const ENV_PROVIDER_ALERT_COOLDOWN_SECS: &str = "BEARDOG_PROVIDER_ALERT_COOLDOWN_SECS";
/// Provider alert escalation threshold (consecutive breaches).
pub const ENV_PROVIDER_ALERT_ESCALATION_THRESHOLD: &str =
    "BEARDOG_PROVIDER_ALERT_ESCALATION_THRESHOLD";
/// Provider session timeout (seconds).
pub const ENV_PROVIDER_SESSION_TIMEOUT_SECS: &str = "BEARDOG_PROVIDER_SESSION_TIMEOUT_SECS";
/// Provider key rotation interval (seconds).
pub const ENV_PROVIDER_KEY_ROTATION_INTERVAL_SECS: &str =
    "BEARDOG_PROVIDER_KEY_ROTATION_INTERVAL_SECS";
/// Key derivation iteration count.
pub const ENV_KEY_DERIVATION_ITERATIONS: &str = "BEARDOG_KEY_DERIVATION_ITERATIONS";
/// Key derivation salt length (bytes).
pub const ENV_KEY_DERIVATION_SALT_LENGTH: &str = "BEARDOG_KEY_DERIVATION_SALT_LENGTH";
/// Connection pool minimum size.
pub const ENV_CONNECTION_POOL_MIN_SIZE: &str = "BEARDOG_CONNECTION_POOL_MIN_SIZE";
/// Connection pool acquire timeout (seconds).
pub const ENV_POOL_CONNECTION_TIMEOUT: &str = "BEARDOG_POOL_CONNECTION_TIMEOUT";
/// Connection pool idle timeout (seconds, provider domain).
pub const ENV_POOL_IDLE_TIMEOUT: &str = "BEARDOG_POOL_IDLE_TIMEOUT";
/// Cache TTL (seconds, provider domain).
pub const ENV_CACHE_TTL_SECS: &str = "BEARDOG_CACHE_TTL_SECS";
/// Request timeout (seconds, provider domain).
pub const ENV_REQUEST_TIMEOUT_SECS: &str = "BEARDOG_REQUEST_TIMEOUT_SECS";
/// Maximum retry delay (milliseconds).
pub const ENV_MAX_RETRY_DELAY_MS: &str = "BEARDOG_MAX_RETRY_DELAY_MS";
/// Performance keep-alive interval (seconds).
pub const ENV_PERFORMANCE_KEEP_ALIVE_SECS: &str = "BEARDOG_PERFORMANCE_KEEP_ALIVE_SECS";
/// Performance request timeout (seconds).
pub const ENV_PERFORMANCE_REQUEST_TIMEOUT_SECS: &str = "BEARDOG_PERFORMANCE_REQUEST_TIMEOUT_SECS";
/// Load balancing algorithm selector.
pub const ENV_LOAD_BALANCING_ALGORITHM: &str = "BEARDOG_LOAD_BALANCING_ALGORITHM";

// ── Provider registry ──────────────────────────────────────────────────

/// Provider registry maximum entries.
pub const ENV_PROVIDER_REGISTRY_MAX_PROVIDERS: &str = "BEARDOG_PROVIDER_REGISTRY_MAX_PROVIDERS";
/// Provider health check interval (seconds).
pub const ENV_PROVIDER_HEALTH_CHECK_INTERVAL_SECS: &str =
    "BEARDOG_PROVIDER_HEALTH_CHECK_INTERVAL_SECS";
/// Provider operation timeout (seconds).
pub const ENV_PROVIDER_TIMEOUT_SECS: &str = "BEARDOG_PROVIDER_TIMEOUT_SECS";
/// Provider migration timeout (seconds).
pub const ENV_PROVIDER_MIGRATION_TIMEOUT_SECS: &str = "BEARDOG_PROVIDER_MIGRATION_TIMEOUT_SECS";
/// Adapter cache duration (seconds).
pub const ENV_ADAPTER_CACHE_DURATION_SECS: &str = "BEARDOG_ADAPTER_CACHE_DURATION_SECS";
/// Adapter instance identifier.
pub const ENV_ADAPTER_ID: &str = "BEARDOG_ADAPTER_ID";
/// Adapter maximum concurrent connections.
pub const ENV_ADAPTER_MAX_CONNECTIONS: &str = "BEARDOG_ADAPTER_MAX_CONNECTIONS";
/// Adapter connection timeout (seconds).
pub const ENV_ADAPTER_CONNECTION_TIMEOUT_SECS: &str = "BEARDOG_ADAPTER_CONNECTION_TIMEOUT_SECS";
/// Adapter global operation timeout (seconds).
pub const ENV_ADAPTER_TIMEOUT_SECS: &str = "BEARDOG_ADAPTER_TIMEOUT_SECS";
/// Comma-separated adapter discovery endpoints.
pub const ENV_ADAPTER_DISCOVERY_ENDPOINTS: &str = "BEARDOG_ADAPTER_DISCOVERY_ENDPOINTS";
/// Enable adapter performance optimization.
pub const ENV_ADAPTER_OPTIMIZATION_ENABLED: &str = "BEARDOG_ADAPTER_OPTIMIZATION_ENABLED";
/// Adapter optimization level (0–9).
pub const ENV_ADAPTER_OPTIMIZATION_LEVEL: &str = "BEARDOG_ADAPTER_OPTIMIZATION_LEVEL";
/// Enable SIMD for adapter processing.
pub const ENV_ADAPTER_SIMD_ENABLED: &str = "BEARDOG_ADAPTER_SIMD_ENABLED";
/// Adapter optimization buffer size (bytes).
pub const ENV_OPTIMIZATION_BUFFER_SIZE: &str = "BEARDOG_OPTIMIZATION_BUFFER_SIZE";
/// Adapter chain maximum length.
pub const ENV_ADAPTER_MAX_CHAIN_LENGTH: &str = "BEARDOG_ADAPTER_MAX_CHAIN_LENGTH";
/// Adapter chain processing timeout (seconds).
pub const ENV_ADAPTER_PROCESSING_TIMEOUT_SECS: &str = "BEARDOG_ADAPTER_PROCESSING_TIMEOUT_SECS";
/// Adapter chain maximum worker count.
pub const ENV_ADAPTER_MAX_WORKERS: &str = "BEARDOG_ADAPTER_MAX_WORKERS";
/// Adapter chain step timeout (seconds).
pub const ENV_ADAPTER_STEP_TIMEOUT_SECS: &str = "BEARDOG_ADAPTER_STEP_TIMEOUT_SECS";
/// Adapter chain step max retry attempts.
pub const ENV_ADAPTER_STEP_MAX_ATTEMPTS: &str = "BEARDOG_ADAPTER_STEP_MAX_ATTEMPTS";
/// Enable adapter chain step validation.
pub const ENV_ADAPTER_STEP_VALIDATION_ENABLED: &str = "BEARDOG_ADAPTER_STEP_VALIDATION_ENABLED";
/// Adapter retry max attempts.
pub const ENV_ADAPTER_RETRY_MAX_ATTEMPTS: &str = "BEARDOG_ADAPTER_RETRY_MAX_ATTEMPTS";
/// Adapter retry initial delay (milliseconds).
pub const ENV_ADAPTER_RETRY_INITIAL_DELAY_MS: &str = "BEARDOG_ADAPTER_RETRY_INITIAL_DELAY_MS";
/// Adapter retry maximum delay (seconds).
pub const ENV_ADAPTER_RETRY_MAX_DELAY_SECS: &str = "BEARDOG_ADAPTER_RETRY_MAX_DELAY_SECS";
/// Adapter retry backoff multiplier.
pub const ENV_ADAPTER_RETRY_BACKOFF_MULTIPLIER: &str = "BEARDOG_ADAPTER_RETRY_BACKOFF_MULTIPLIER";
/// Adapter metrics collection interval (seconds).
pub const ENV_ADAPTER_METRICS_INTERVAL_SECS: &str = "BEARDOG_ADAPTER_METRICS_INTERVAL_SECS";
/// Adapter metrics retention period (seconds).
pub const ENV_ADAPTER_RETENTION_PERIOD_SECS: &str = "BEARDOG_ADAPTER_RETENTION_PERIOD_SECS";
/// Adapter circuit breaker failure threshold.
pub const ENV_ADAPTER_CIRCUIT_BREAKER_THRESHOLD: &str = "BEARDOG_ADAPTER_CIRCUIT_BREAKER_THRESHOLD";
/// Service mesh protocol (`http`, `grpc`, etc.).
pub const ENV_MESH_PROTOCOL: &str = "BEARDOG_MESH_PROTOCOL";
/// Service mesh discovery port.
pub const ENV_MESH_DISCOVERY_PORT: &str = "BEARDOG_MESH_DISCOVERY_PORT";
/// Service mesh discovery timeout (seconds).
pub const ENV_MESH_DISCOVERY_TIMEOUT_SECS: &str = "BEARDOG_MESH_DISCOVERY_TIMEOUT_SECS";
/// Handoff retry max attempts.
pub const ENV_HANDOFF_RETRY_MAX_ATTEMPTS: &str = "BEARDOG_HANDOFF_RETRY_MAX_ATTEMPTS";
/// Handoff retry timeout (seconds).
pub const ENV_HANDOFF_RETRY_TIMEOUT_SECS: &str = "BEARDOG_HANDOFF_RETRY_TIMEOUT_SECS";
/// Cache cleanup interval (seconds).
pub const ENV_CACHE_CLEANUP_INTERVAL_SECS: &str = "BEARDOG_CACHE_CLEANUP_INTERVAL_SECS";
/// Universal adapter cache TTL (seconds, unprefixed).
pub const ENV_UNIVERSAL_ADAPTER_CACHE_TTL_SECS: &str = "UNIVERSAL_ADAPTER_CACHE_TTL_SECS";

// ── AI / hybrid intelligence (runtime) ───────────────────────────────────

/// AI model registry endpoint URL.
pub const ENV_AI_REGISTRY_ENDPOINT: &str = "BEARDOG_AI_REGISTRY_ENDPOINT";
/// AI deployment CPU allocation.
pub const ENV_AI_RESOURCE_CPU: &str = "BEARDOG_AI_RESOURCE_CPU";
/// AI deployment memory allocation (MB).
pub const ENV_AI_RESOURCE_MEMORY_MB: &str = "BEARDOG_AI_RESOURCE_MEMORY_MB";
/// AI deployment GPU allocation.
pub const ENV_AI_RESOURCE_GPU: &str = "BEARDOG_AI_RESOURCE_GPU";
/// AI deployment storage allocation (GB).
pub const ENV_AI_RESOURCE_STORAGE_GB: &str = "BEARDOG_AI_RESOURCE_STORAGE_GB";
/// Enable online learning mode.
pub const ENV_AI_ONLINE_LEARNING_ENABLED: &str = "BEARDOG_AI_ONLINE_LEARNING_ENABLED";
/// Online learning rate.
pub const ENV_AI_ONLINE_LEARNING_RATE: &str = "BEARDOG_AI_ONLINE_LEARNING_RATE";
/// Online learning batch size.
pub const ENV_AI_ONLINE_BATCH_SIZE: &str = "BEARDOG_AI_ONLINE_BATCH_SIZE";
/// Online learning update frequency (samples between updates).
pub const ENV_AI_ONLINE_UPDATE_FREQUENCY: &str = "BEARDOG_AI_ONLINE_UPDATE_FREQUENCY";
/// Adaptive learning rate.
pub const ENV_AI_ADAPTIVE_LEARNING_RATE: &str = "BEARDOG_AI_ADAPTIVE_LEARNING_RATE";
/// Adaptive learning batch size.
pub const ENV_AI_ADAPTIVE_BATCH_SIZE: &str = "BEARDOG_AI_ADAPTIVE_BATCH_SIZE";
/// AI memory buffer size (samples).
pub const ENV_AI_MEMORY_BUFFER_SIZE: &str = "BEARDOG_AI_MEMORY_BUFFER_SIZE";
/// Learning model update frequency (seconds).
pub const ENV_LEARNING_UPDATE_FREQUENCY_SECS: &str = "BEARDOG_LEARNING_UPDATE_FREQUENCY_SECS";
/// AI metrics collection interval (seconds).
pub const ENV_AI_METRICS_INTERVAL_SECS: &str = "BEARDOG_AI_METRICS_INTERVAL_SECS";
/// AI serving max concurrent requests.
pub const ENV_AI_SERVING_MAX_CONCURRENT_REQUESTS: &str =
    "BEARDOG_AI_SERVING_MAX_CONCURRENT_REQUESTS";
/// AI routing request timeout (seconds).
pub const ENV_AI_ROUTING_REQUEST_TIMEOUT_SECS: &str = "BEARDOG_AI_ROUTING_REQUEST_TIMEOUT_SECS";
/// AI inference batch size.
pub const ENV_AI_INFERENCE_BATCH_SIZE: &str = "BEARDOG_AI_INFERENCE_BATCH_SIZE";
/// AI max inference time (milliseconds).
pub const ENV_AI_MAX_INFERENCE_TIME_MS: &str = "BEARDOG_AI_MAX_INFERENCE_TIME_MS";
/// AI integration endpoint URL.
pub const ENV_AI_INTEGRATION_ENDPOINT: &str = "BEARDOG_AI_INTEGRATION_ENDPOINT";

// ── Universal compute client ─────────────────────────────────────────────

/// Compute client request timeout (milliseconds).
pub const ENV_COMPUTE_REQUEST_TIMEOUT_MS: &str = "BEARDOG_COMPUTE_REQUEST_TIMEOUT_MS";
/// Compute service discovery timeout (milliseconds).
pub const ENV_COMPUTE_DISCOVERY_TIMEOUT_MS: &str = "BEARDOG_COMPUTE_DISCOVERY_TIMEOUT_MS";
/// Compute endpoint cache duration (milliseconds).
pub const ENV_COMPUTE_CACHE_DURATION_MS: &str = "BEARDOG_COMPUTE_CACHE_DURATION_MS";

// ── Performance optimizer ────────────────────────────────────────────────

/// Optimizer max concurrent connections.
pub const ENV_OPTIMIZER_MAX_CONNECTIONS: &str = "BEARDOG_OPTIMIZER_MAX_CONNECTIONS";
/// Optimizer connection timeout (seconds).
pub const ENV_OPTIMIZER_CONNECTION_TIMEOUT_SECS: &str = "BEARDOG_OPTIMIZER_CONNECTION_TIMEOUT_SECS";
/// Optimizer cache TTL (seconds).
pub const ENV_OPTIMIZER_CACHE_TTL_SECS: &str = "BEARDOG_OPTIMIZER_CACHE_TTL_SECS";
/// Optimizer rate limit (requests per second).
pub const ENV_OPTIMIZER_RATE_LIMIT: &str = "BEARDOG_OPTIMIZER_RATE_LIMIT";
/// Optimizer health check interval (seconds).
pub const ENV_OPTIMIZER_HEALTH_CHECK_INTERVAL_SECS: &str =
    "BEARDOG_OPTIMIZER_HEALTH_CHECK_INTERVAL_SECS";
