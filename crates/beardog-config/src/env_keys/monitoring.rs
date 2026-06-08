// SPDX-License-Identifier: AGPL-3.0-or-later

//! Monitoring, health checks, metrics endpoints, and system logging environment variable keys.

// ── Monitoring ───────────────────────────────────────────────────────

/// Log level (e.g. `debug`, `info`, `warn`).
pub const ENV_LOG_LEVEL: &str = "BEARDOG_LOG_LEVEL";
/// Log format (e.g. `json`, `pretty`).
pub const ENV_LOG_FORMAT: &str = "BEARDOG_LOG_FORMAT";
/// Tracing sample rate (0.0–1.0).
pub const ENV_TRACING_SAMPLE_RATE: &str = "BEARDOG_TRACING_SAMPLE_RATE";

// ── Health checks ────────────────────────────────────────────────────

/// Global health check timeout (seconds).
pub const ENV_HEALTH_GLOBAL_TIMEOUT_SECS: &str = "BEARDOG_HEALTH_GLOBAL_TIMEOUT_SECS";
/// Health check interval (seconds).
pub const ENV_HEALTH_CHECK_INTERVAL_SECS: &str = "BEARDOG_HEALTH_CHECK_INTERVAL_SECS";
/// Consecutive failures before unhealthy.
pub const ENV_HEALTH_FAILURE_THRESHOLD: &str = "BEARDOG_HEALTH_FAILURE_THRESHOLD";
/// Consecutive successes before healthy.
pub const ENV_HEALTH_SUCCESS_THRESHOLD: &str = "BEARDOG_HEALTH_SUCCESS_THRESHOLD";
/// HTTP health check timeout (seconds).
pub const ENV_HTTP_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_HTTP_HEALTH_CHECK_TIMEOUT_SECS";
/// TCP health check timeout (seconds).
pub const ENV_TCP_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_TCP_HEALTH_CHECK_TIMEOUT_SECS";
/// Database health check timeout (seconds).
pub const ENV_DB_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_DB_HEALTH_CHECK_TIMEOUT_SECS";
/// Service discovery interval (seconds).
pub const ENV_SERVICE_DISCOVERY_INTERVAL_SECS: &str = "BEARDOG_SERVICE_DISCOVERY_INTERVAL_SECS";
/// Auto-recovery max attempts.
pub const ENV_AUTO_RECOVERY_ATTEMPTS: &str = "BEARDOG_AUTO_RECOVERY_ATTEMPTS";
/// Recovery delay (seconds).
pub const ENV_HEALTH_RECOVERY_DELAY_SECS: &str = "BEARDOG_HEALTH_RECOVERY_DELAY_SECS";
/// Recovery backoff multiplier.
pub const ENV_HEALTH_RECOVERY_BACKOFF: &str = "BEARDOG_HEALTH_RECOVERY_BACKOFF";
/// Maximum recovery delay (seconds).
pub const ENV_HEALTH_MAX_RECOVERY_DELAY_SECS: &str = "BEARDOG_HEALTH_MAX_RECOVERY_DELAY_SECS";
/// Health check timeout (seconds) — used by timeout builder.
pub const ENV_HEALTH_CHECK_TIMEOUT_SECS: &str = "BEARDOG_HEALTH_CHECK_TIMEOUT_SECS";

// ── Monitoring (extended) ────────────────────────────────────────────

/// Monitoring sampling rate (0.0–1.0).
pub const ENV_MONITORING_SAMPLING_RATE: &str = "BEARDOG_MONITORING_SAMPLING_RATE";
/// Monitoring event buffer size.
pub const ENV_MONITORING_BUFFER_SIZE: &str = "BEARDOG_MONITORING_BUFFER_SIZE";
/// Monitoring flush interval (seconds).
pub const ENV_MONITORING_FLUSH_INTERVAL_SECS: &str = "BEARDOG_MONITORING_FLUSH_INTERVAL_SECS";
/// Data retention max age (seconds).
pub const ENV_RETENTION_MAX_AGE_SECS: &str = "BEARDOG_RETENTION_MAX_AGE_SECS";
/// Data retention max size (bytes).
pub const ENV_RETENTION_MAX_SIZE_BYTES: &str = "BEARDOG_RETENTION_MAX_SIZE_BYTES";
/// Data retention max entry count.
pub const ENV_RETENTION_MAX_COUNT: &str = "BEARDOG_RETENTION_MAX_COUNT";
/// Retention cleanup interval (seconds).
pub const ENV_RETENTION_CLEANUP_INTERVAL_SECS: &str = "BEARDOG_RETENTION_CLEANUP_INTERVAL_SECS";
/// Batch processing size.
pub const ENV_BATCH_SIZE: &str = "BEARDOG_BATCH_SIZE";
/// Batch flush interval (seconds).
pub const ENV_BATCH_FLUSH_INTERVAL_SECS: &str = "BEARDOG_BATCH_FLUSH_INTERVAL_SECS";
/// Batch max wait time (seconds).
pub const ENV_BATCH_MAX_WAIT_TIME_SECS: &str = "BEARDOG_BATCH_MAX_WAIT_TIME_SECS";
/// Retry policy max retries (monitoring domain).
pub const ENV_RETRY_POLICY_MAX_RETRIES: &str = "BEARDOG_RETRY_POLICY_MAX_RETRIES";
/// Retry initial delay (milliseconds).
pub const ENV_RETRY_INITIAL_DELAY_MS: &str = "BEARDOG_RETRY_INITIAL_DELAY_MS";
/// Retry max delay (seconds).
pub const ENV_RETRY_MAX_DELAY_SECS: &str = "BEARDOG_RETRY_MAX_DELAY_SECS";
/// Monitoring backoff multiplier.
pub const ENV_MONITORING_BACKOFF_MULTIPLIER: &str = "BEARDOG_MONITORING_BACKOFF_MULTIPLIER";
/// Enable monitoring subsystem.
pub const ENV_MONITORING_ENABLED: &str = "BEARDOG_MONITORING_ENABLED";
/// Monitoring collection interval (seconds).
pub const ENV_MONITORING_INTERVAL: &str = "BEARDOG_MONITORING_INTERVAL";
/// Metrics smoothing factor (0.0–1.0).
pub const ENV_METRICS_SMOOTHING_FACTOR: &str = "BEARDOG_METRICS_SMOOTHING_FACTOR";
/// Histogram maximum bucket count.
pub const ENV_HISTOGRAM_MAX_BUCKETS: &str = "BEARDOG_HISTOGRAM_MAX_BUCKETS";
/// Metrics collection buffer size.
pub const ENV_METRICS_BUFFER_SIZE: &str = "BEARDOG_METRICS_BUFFER_SIZE";
/// Metrics collection thread count.
pub const ENV_METRICS_COLLECTION_THREADS: &str = "BEARDOG_METRICS_COLLECTION_THREADS";
/// Maximum consecutive metrics collection errors before backoff.
pub const ENV_METRICS_MAX_COLLECTION_ERRORS: &str = "BEARDOG_METRICS_MAX_COLLECTION_ERRORS";
/// Metrics statistical confidence interval.
pub const ENV_METRICS_CONFIDENCE_INTERVAL: &str = "BEARDOG_METRICS_CONFIDENCE_INTERVAL";
/// Anomaly detection sensitivity (0.0–1.0).
pub const ENV_ANOMALY_DETECTION_SENSITIVITY: &str = "BEARDOG_ANOMALY_DETECTION_SENSITIVITY";
/// Minimum data points required for anomaly detection.
pub const ENV_ANOMALY_MIN_DATA_POINTS: &str = "BEARDOG_ANOMALY_MIN_DATA_POINTS";
/// Metrics trend detection threshold.
pub const ENV_METRICS_TREND_THRESHOLD: &str = "BEARDOG_METRICS_TREND_THRESHOLD";
/// Metric export batch size.
pub const ENV_METRIC_EXPORT_BATCH_SIZE: &str = "BEARDOG_METRIC_EXPORT_BATCH_SIZE";
/// Alert rule evaluation interval (seconds).
pub const ENV_ALERT_EVALUATION_INTERVAL_SECS: &str = "BEARDOG_ALERT_EVALUATION_INTERVAL_SECS";
/// Alert notification delivery timeout (seconds).
pub const ENV_ALERT_NOTIFICATION_TIMEOUT_SECS: &str = "BEARDOG_ALERT_NOTIFICATION_TIMEOUT_SECS";
/// Maximum alerts emitted per minute.
pub const ENV_MAX_ALERTS_PER_MINUTE: &str = "BEARDOG_MAX_ALERTS_PER_MINUTE";
/// Alert firing threshold.
pub const ENV_ALERT_THRESHOLD: &str = "BEARDOG_ALERT_THRESHOLD";
/// Trend analysis window (seconds).
pub const ENV_TREND_ANALYSIS_PERIOD_SECS: &str = "BEARDOG_TREND_ANALYSIS_PERIOD_SECS";
/// Prediction horizon (seconds).
pub const ENV_PREDICTION_HORIZON_SECS: &str = "BEARDOG_PREDICTION_HORIZON_SECS";

// ── Monitoring endpoints ─────────────────────────────────────────────────

/// Prometheus scrape endpoint host/address.
pub const ENV_PROMETHEUS_ENDPOINT: &str = "BEARDOG_PROMETHEUS_ENDPOINT";
/// Prometheus scrape port.
pub const ENV_PROMETHEUS_PORT: &str = "BEARDOG_PROMETHEUS_PORT";
/// Grafana dashboard URL.
pub const ENV_GRAFANA_URL: &str = "BEARDOG_GRAFANA_URL";
/// Grafana dashboard URL (unprefixed alias).
pub const ENV_GRAFANA_URL_UNPREFIXED: &str = "GRAFANA_URL";
/// Jaeger collector endpoint URL.
pub const ENV_JAEGER_ENDPOINT: &str = "BEARDOG_JAEGER_ENDPOINT";
/// Jaeger collector endpoint URL (unprefixed alias).
pub const ENV_JAEGER_ENDPOINT_UNPREFIXED: &str = "JAEGER_ENDPOINT";
/// Local environment metrics collection interval (seconds).
pub const ENV_LOCAL_METRICS_INTERVAL_SECS: &str = "BEARDOG_LOCAL_METRICS_INTERVAL_SECS";
/// Development metrics collection interval (seconds).
pub const ENV_DEV_METRICS_INTERVAL_SECS: &str = "BEARDOG_DEV_METRICS_INTERVAL_SECS";
/// Testing metrics collection interval (seconds).
pub const ENV_TEST_METRICS_INTERVAL_SECS: &str = "BEARDOG_TEST_METRICS_INTERVAL_SECS";
/// Staging metrics collection interval (seconds).
pub const ENV_STAGING_METRICS_INTERVAL_SECS: &str = "BEARDOG_STAGING_METRICS_INTERVAL_SECS";
/// Production metrics collection interval (seconds).
pub const ENV_PRODUCTION_METRICS_INTERVAL_SECS: &str = "BEARDOG_PRODUCTION_METRICS_INTERVAL_SECS";

// ── Monitoring (health / metrics) ────────────────────────────────────────

/// Cache service host for health checks.
pub const ENV_CACHE_HOST: &str = "BEARDOG_CACHE_HOST";
/// External API URL for health checks.
pub const ENV_EXTERNAL_API_URL: &str = "BEARDOG_EXTERNAL_API_URL";
/// HSM provider name for health checks.
pub const ENV_HSM_PROVIDER: &str = "BEARDOG_HSM_PROVIDER";
/// Metrics collection interval (seconds).
pub const ENV_METRICS_COLLECTION_INTERVAL_SECS: &str = "BEARDOG_METRICS_COLLECTION_INTERVAL_SECS";
/// Metrics history buffer size.
pub const ENV_METRICS_HISTORY_SIZE: &str = "BEARDOG_METRICS_HISTORY_SIZE";
/// Metrics analysis window (seconds).
pub const ENV_ANALYSIS_WINDOW_SECS: &str = "BEARDOG_ANALYSIS_WINDOW_SECS";

// ── System logging ───────────────────────────────────────────────────────

/// Maximum log file size in megabytes.
pub const ENV_SYSTEM_LOG_MAX_SIZE_MB: &str = "BEARDOG_SYSTEM_LOG_MAX_SIZE_MB";
/// Maximum number of rotated log files.
pub const ENV_SYSTEM_LOG_MAX_FILES: &str = "BEARDOG_SYSTEM_LOG_MAX_FILES";
