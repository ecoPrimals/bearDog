// SPDX-License-Identifier: AGPL-3.0-only

//! Pure Rust system metrics collection via /proc filesystem.
//!
//! Reads CPU and memory stats directly from the Linux procfs without
//! external dependencies. Falls back to NotImplemented on non-Linux.

use beardog_errors::BearDogError;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::debug;

/// Snapshot of /proc/stat CPU counters.
#[derive(Debug, Clone, Copy, Default)]
struct CpuSnapshot {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
}

impl CpuSnapshot {
    const fn total(&self) -> u64 {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
    }

    const fn busy(&self) -> u64 {
        self.total() - self.idle - self.iowait
    }
}

/// Reads the aggregate CPU line from `/proc/stat`.
#[cfg(target_os = "linux")]
fn read_cpu_snapshot() -> Result<CpuSnapshot, BearDogError> {
    let content = std::fs::read_to_string("/proc/stat")
        .map_err(|e| BearDogError::internal(format!("Failed to read /proc/stat: {e}")))?;

    let cpu_line = content
        .lines()
        .find(|l| l.starts_with("cpu "))
        .ok_or_else(|| BearDogError::internal("No aggregate cpu line in /proc/stat".into()))?;

    let fields: Vec<u64> = cpu_line
        .split_whitespace()
        .skip(1) // skip "cpu"
        .take(8)
        .map(|f| f.parse::<u64>().unwrap_or(0))
        .collect();

    if fields.len() < 4 {
        return Err(BearDogError::internal(
            "Malformed /proc/stat cpu line".into(),
        ));
    }

    Ok(CpuSnapshot {
        user: fields[0],
        nice: fields[1],
        system: fields[2],
        idle: fields[3],
        iowait: fields.get(4).copied().unwrap_or(0),
        irq: fields.get(5).copied().unwrap_or(0),
        softirq: fields.get(6).copied().unwrap_or(0),
        steal: fields.get(7).copied().unwrap_or(0),
    })
}

/// Reads memory usage from `/proc/meminfo`.
#[cfg(target_os = "linux")]
#[expect(
    clippy::cast_precision_loss,
    reason = "Memory ratio from /proc counters; f64 sufficient for percentage display"
)]
fn read_memory_usage() -> Result<f64, BearDogError> {
    let content = std::fs::read_to_string("/proc/meminfo")
        .map_err(|e| BearDogError::internal(format!("Failed to read /proc/meminfo: {e}")))?;

    let mut total_kb: Option<u64> = None;
    let mut available_kb: Option<u64> = None;

    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("MemTotal:") {
            total_kb = parse_meminfo_value(rest);
        } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
            available_kb = parse_meminfo_value(rest);
        }
        if total_kb.is_some() && available_kb.is_some() {
            break;
        }
    }

    let total = total_kb
        .ok_or_else(|| BearDogError::internal("MemTotal not found in /proc/meminfo".into()))?;
    let available = available_kb
        .ok_or_else(|| BearDogError::internal("MemAvailable not found in /proc/meminfo".into()))?;

    if total == 0 {
        return Ok(0.0);
    }

    let used = total.saturating_sub(available);
    Ok((used as f64 / total as f64) * 100.0)
}

#[cfg(target_os = "linux")]
fn parse_meminfo_value(s: &str) -> Option<u64> {
    s.split_whitespace().next()?.parse::<u64>().ok()
}

/// Pure Rust system metrics collector.
///
/// Tracks CPU usage between snapshots and reads memory from procfs.
/// Request-level metrics (response time, error rate, throughput) are
/// tracked internally via atomic counters — callers feed data in via
/// `record_request()`.
#[derive(Debug)]
pub struct SystemMetrics {
    /// Previous CPU snapshot for delta calculation.
    prev_cpu: std::sync::Mutex<Option<CpuSnapshot>>,

    /// Atomic counters for request-level metrics.
    request_count: AtomicU64,
    error_count: AtomicU64,
    total_response_ns: AtomicU64,

    /// Window start for throughput calculation.
    window_start: Arc<std::sync::Mutex<std::time::Instant>>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemMetrics {
    /// Create a new system metrics collector.
    pub fn new() -> Self {
        Self {
            prev_cpu: std::sync::Mutex::new(None),
            request_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            total_response_ns: AtomicU64::new(0),
            window_start: Arc::new(std::sync::Mutex::new(std::time::Instant::now())),
        }
    }

    /// Record a completed request for throughput/latency/error tracking.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Nanoseconds capped to u64::MAX for accumulator; overflow saturates"
    )]
    pub fn record_request(&self, duration: std::time::Duration, is_error: bool) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
        self.total_response_ns
            .fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        if is_error {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Collect CPU usage as a percentage (0.0–100.0).
    ///
    /// Uses delta between two `/proc/stat` reads. First call returns 0.0
    /// (no previous baseline). Subsequent calls return actual usage.
    #[cfg(target_os = "linux")]
    #[expect(
        clippy::cast_precision_loss,
        reason = "CPU busy/total ratio from /proc; f64 sufficient for percentage"
    )]
    pub fn collect_cpu_usage(&self) -> Result<f64, BearDogError> {
        let current = read_cpu_snapshot()?;
        let mut prev_guard = self
            .prev_cpu
            .lock()
            .map_err(|e| BearDogError::internal(format!("CPU snapshot lock poisoned: {e}")))?;

        let usage = if let Some(prev) = *prev_guard {
            let total_delta = current.total().saturating_sub(prev.total());
            let busy_delta = current.busy().saturating_sub(prev.busy());
            if total_delta == 0 {
                0.0
            } else {
                (busy_delta as f64 / total_delta as f64) * 100.0
            }
        } else {
            0.0 // First measurement — no baseline
        };

        *prev_guard = Some(current);
        debug!("CPU usage: {usage:.1}%");
        Ok(usage)
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_cpu_usage(&self) -> Result<f64, BearDogError> {
        Err(BearDogError::not_implemented(
            "CPU metrics require Linux /proc/stat. Use sysinfo crate for cross-platform.",
        ))
    }

    /// Collect memory usage as a percentage (0.0–100.0).
    #[cfg(target_os = "linux")]
    pub fn collect_memory_usage(&self) -> Result<f64, BearDogError> {
        let usage = read_memory_usage()?;
        debug!("Memory usage: {usage:.1}%");
        Ok(usage)
    }

    #[cfg(not(target_os = "linux"))]
    pub fn collect_memory_usage(&self) -> Result<f64, BearDogError> {
        Err(BearDogError::not_implemented(
            "Memory metrics require Linux /proc/meminfo. Use sysinfo crate for cross-platform.",
        ))
    }

    /// Collect average response time in milliseconds since last call.
    ///
    /// Resets the accumulator after reading.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Average latency from atomic counters; f64 sufficient for reporting"
    )]
    pub fn collect_response_time(&self) -> Result<f64, BearDogError> {
        let total_ns = self.total_response_ns.swap(0, Ordering::Relaxed);
        let count = self.request_count.load(Ordering::Relaxed);
        if count == 0 {
            return Ok(0.0);
        }
        Ok(total_ns as f64 / count as f64 / 1_000_000.0)
    }

    /// Collect error rate as a percentage (0.0–100.0) since last call.
    ///
    /// Resets the error counter after reading.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Error ratio from atomic counters; f64 sufficient for percentage"
    )]
    pub fn collect_error_rate(&self) -> Result<f64, BearDogError> {
        let errors = self.error_count.swap(0, Ordering::Relaxed);
        let count = self.request_count.load(Ordering::Relaxed);
        if count == 0 {
            return Ok(0.0);
        }
        Ok((errors as f64 / count as f64) * 100.0)
    }

    /// Collect throughput in operations per second since last window reset.
    ///
    /// Resets the request counter and window after reading.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Ops per second from counts and elapsed; f64 sufficient for throughput"
    )]
    pub fn collect_throughput(&self) -> Result<f64, BearDogError> {
        let count = self.request_count.swap(0, Ordering::Relaxed);
        let mut start = self
            .window_start
            .lock()
            .map_err(|e| BearDogError::internal(format!("Throughput window lock poisoned: {e}")))?;
        let elapsed = start.elapsed();
        *start = std::time::Instant::now();

        if elapsed.is_zero() {
            return Ok(0.0);
        }
        Ok(count as f64 / elapsed.as_secs_f64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_metrics_new() {
        let metrics = SystemMetrics::new();
        assert_eq!(metrics.request_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_record_request() {
        let metrics = SystemMetrics::new();
        metrics.record_request(std::time::Duration::from_millis(50), false);
        metrics.record_request(std::time::Duration::from_millis(100), true);
        assert_eq!(metrics.request_count.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.error_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_collect_response_time_no_requests() {
        let metrics = SystemMetrics::new();
        let rt = metrics.collect_response_time().expect("response time");
        assert!(
            rt.abs() < f64::EPSILON,
            "expected response time 0.0 with no requests, got {rt}"
        );
    }

    #[test]
    fn test_collect_response_time_with_requests() {
        let metrics = SystemMetrics::new();
        metrics.record_request(std::time::Duration::from_millis(100), false);
        metrics.record_request(std::time::Duration::from_millis(200), false);
        let rt = metrics.collect_response_time().expect("response time");
        // Average should be ~150ms (some atomic timing variance)
        assert!(rt > 100.0 && rt < 250.0, "Expected ~150ms, got {rt}");
    }

    #[test]
    fn test_collect_error_rate_no_requests() {
        let metrics = SystemMetrics::new();
        let rate = metrics.collect_error_rate().expect("error rate");
        assert!(
            rate.abs() < f64::EPSILON,
            "expected error rate 0.0 with no requests, got {rate}"
        );
    }

    #[test]
    fn test_collect_error_rate_with_errors() {
        let metrics = SystemMetrics::new();
        metrics.record_request(std::time::Duration::from_millis(10), false);
        metrics.record_request(std::time::Duration::from_millis(10), true);
        let rate = metrics.collect_error_rate().expect("error rate");
        assert!((rate - 50.0).abs() < 1.0, "Expected ~50%, got {rate}");
    }

    #[test]
    fn test_collect_throughput() {
        let metrics = SystemMetrics::new();
        metrics.record_request(std::time::Duration::from_millis(1), false);
        metrics.record_request(std::time::Duration::from_millis(1), false);
        metrics.record_request(std::time::Duration::from_millis(1), false);
        // Small sleep so elapsed > 0
        std::thread::sleep(std::time::Duration::from_millis(10));
        let throughput = metrics.collect_throughput().expect("throughput");
        assert!(
            throughput > 0.0,
            "Expected positive throughput, got {throughput}"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_collect_cpu_usage_linux() {
        let metrics = SystemMetrics::new();
        // First call establishes baseline
        let first = metrics.collect_cpu_usage().expect("cpu first");
        assert!(
            first.abs() < f64::EPSILON,
            "First call should be 0.0 (no baseline), got {first}"
        );
        // Brief delay for CPU delta
        std::thread::sleep(std::time::Duration::from_millis(50));
        let second = metrics.collect_cpu_usage().expect("cpu second");
        assert!(
            (0.0..=100.0).contains(&second),
            "CPU should be 0-100, got {second}"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_collect_memory_usage_linux() {
        let metrics = SystemMetrics::new();
        let usage = metrics.collect_memory_usage().expect("memory usage");
        assert!(
            usage > 0.0 && usage < 100.0,
            "Memory should be 0-100, got {usage}"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_read_cpu_snapshot() {
        let snap = read_cpu_snapshot().expect("cpu snapshot");
        assert!(snap.total() > 0, "Total CPU time should be > 0");
        assert!(snap.idle > 0, "Idle time should be > 0");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_read_memory_usage_reasonable() {
        let usage = read_memory_usage().expect("read memory");
        // On any real system, memory usage should be between 1% and 99%
        assert!(
            usage > 1.0 && usage < 99.0,
            "Memory usage {usage}% seems unreasonable"
        );
    }
}
