// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, info};

#[derive(Debug, Clone)]
    /// The zero cost metrics value
    pub zero_cost_metrics: ZeroCostMetrics,

    /// The memory metrics value
    pub memory_metrics: MemoryAllocationMetrics,

    /// The hot path metrics value
    pub hot_path_metrics: HotPathMetrics,

    /// The system metrics value
    pub system_metrics: SystemPerformanceMetrics,
}

pub struct SimdOperationMetrics {

    /// Number of total_operations
    pub total_operations: u64,

    /// Number of avg_latency_ns
    pub avg_latency_ns: u64,

    /// The throughput ops per sec value
    pub throughput_ops_per_sec: f64,

    /// The chunk efficiency percent value
    pub chunk_efficiency_percent: f64,

    /// The vectorization ratio value
    pub vectorization_ratio: f64,

pub struct ZeroCostMetrics {

    /// Number of zero_cost_operations
    pub zero_cost_operations: u64,

    /// Number of abstraction_overhead_ns
    pub abstraction_overhead_ns: u64,

    /// The inlining effectiveness value
    pub inlining_effectiveness: f64,

    /// The optimization success rate value
    pub optimization_success_rate: f64,

pub struct MemoryAllocationMetrics {


    pub allocations_avoided: u64,

    /// Number of memory_saved_bytes
    pub memory_saved_bytes: u64,

    /// The pool hit rate value
    pub pool_hit_rate: f64,

    /// Number of avg_allocation_size
    pub avg_allocation_size: usize,

    /// Number of peak_memory_usage
    pub peak_memory_usage: usize,

pub struct HotPathMetrics {

    /// Number of encryption_latency_us
    pub encryption_latency_us: u64,

    /// Number of decryption_latency_us
    pub decryption_latency_us: u64,

    /// Number of key_derivation_latency_us
    pub key_derivation_latency_us: u64,

    /// Number of hsm_operation_latency_us
    pub hsm_operation_latency_us: u64,

    /// The hot cache hit rate value
    pub hot_cache_hit_rate: f64,

pub struct SystemPerformanceMetrics {

    /// The cpu utilization value
    pub cpu_utilization: f64,

    /// The memory utilization value
    pub memory_utilization: f64,

    /// The io ops per sec value
    pub io_ops_per_sec: f64,

    /// The network throughput value
    pub network_throughput: f64,

    /// The load average value
    pub load_average: f64,

pub struct AdvancedPerformanceMonitor {

    metrics: Arc<RwLock<AdvancedPerformanceMetrics>>,

    simd_counters: Arc<SimdCounters>,

    zero_cost_counters: Arc<ZeroCostCounters>,

    memory_counters: Arc<MemoryCounters>,

    hot_path_timers: Arc<RwLock<HotPathTimers>>,

    start_time: Instant,

struct SimdCounters {
    operations: AtomicU64,
    total_latency_ns: AtomicU64,
    chunks_processed: AtomicU64,
    vectorized_operations: AtomicU64,

struct ZeroCostCounters {
    zero_cost_ops: AtomicU64,
    overhead_ns: AtomicU64,
    inlined_calls: AtomicU64,
    total_calls: AtomicU64,

struct MemoryCounters {
    avoided_allocations: AtomicU64,
    saved_bytes: AtomicU64,
    pool_hits: AtomicU64,
    pool_misses: AtomicU64,
    peak_usage: AtomicUsize,

#[derive(Debug, Clone)]
    decryption_samples: Vec<Duration>,
    key_derivation_samples: Vec<Duration>,
    hsm_samples: Vec<Duration>,
    cache_hits: u64,
    cache_total: u64,}

impl AdvancedPerformanceMonitor {

/// New operation.
    #[must_use] pub fn new(SimdOperationMetrics {
                total_operations: 0,
                avg_latency_ns: 0,
                throughput_ops_per_sec: 0.0,
                chunk_efficiency_percent: 0.0,
                vectorization_ratio: 0.0,
            },
            zero_cost_metrics: ZeroCostMetrics {
                zero_cost_operations: 0,
                abstraction_overhead_ns: 0,
                inlining_effectiveness: 0.0,
                optimization_success_rate: 0.0,
            memory_metrics: MemoryAllocationMetrics {
                allocations_avoided: 0,
                memory_saved_bytes: 0,
                pool_hit_rate: 0.0,
                avg_allocation_size: 0,
                peak_memory_usage: 0,
            hot_path_metrics: HotPathMetrics {
                encryption_latency_us: 0,
                decryption_latency_us: 0,
                key_derivation_latency_us: 0,
                hsm_operation_latency_us: 0,
                hot_cache_hit_rate: 0.0,
            system_metrics: SystemPerformanceMetrics {
                cpu_utilization: 0.0,
                memory_utilization: 0.0,
                io_ops_per_sec: 0.0,
                network_throughput: 0.0,
                load_average: 0.0,
        };
        Self {
            metrics: Arc::new(RwLock::new(metrics)),
            simd_counters: Arc::new(SimdCounters {
                operations: AtomicU64::new(0),
                total_latency_ns: AtomicU64::new(0),
                chunks_processed: AtomicU64::new(0),
                vectorized_operations: AtomicU64::new(0),
            }),
            zero_cost_counters: Arc::new(ZeroCostCounters {
                zero_cost_ops: AtomicU64::new(0),
                overhead_ns: AtomicU64::new(0),
                inlined_calls: AtomicU64::new(0),
                total_calls: AtomicU64::new(0),
            memory_counters: Arc::new(MemoryCounters {
                avoided_allocations: AtomicU64::new(0),
                saved_bytes: AtomicU64::new(0),
                pool_hits: AtomicU64::new(0),
                pool_misses: AtomicU64::new(0),
                peak_usage: AtomicUsize::new(0),
            hot_path_timers: Arc::new(RwLock::new(HotPathTimers::default())),
            start_time: Instant::now(Duration,
        chunks_processed: u64,
        vectorized: bool,
    ) {
        let latency_ns = latency.as_nanos() as u64;
        self.simd_counters
            .operations
            .fetch_add(1, Ordering::Relaxed);
            .total_latency_ns
            .fetch_add(latency_ns, Ordering::Relaxed);
            .chunks_processed
            .fetch_add(chunks_processed, Ordering::Relaxed);
        if vectorized {
            self.simd_counters
                .vectorized_operations
                .fetch_add(1, Ordering::Relaxed);
        debug!(
            "📊 SIMD operation recorded: {}ns, {} chunks",
            latency_ns, chunks_processed
        );

/// Record Zero Cost Operation operation.
    pub fn record_zero_cost_operation(Duration, inlined: bool) {
        let overhead_ns = overhead.as_nanos() as u64;
        self.zero_cost_counters
            .zero_cost_ops
            .overhead_ns
            .fetch_add(overhead_ns, Ordering::Relaxed);
            .total_calls
        if inlined {
            self.zero_cost_counters
                .inlined_calls
            "📊 Zero-cost operation: {}ns overhead, inlined: {}",
            overhead_ns, inlined

/// Record Allocation Avoided operation.
    pub fn record_allocation_avoided(u64, pool_hit: bool) {
        self.memory_counters
            .avoided_allocations
            .saved_bytes
            .fetch_add(bytes_saved, Ordering::Relaxed);
        if pool_hit {
            self.memory_counters
                .pool_hits
        } else {
                .pool_misses
            "📊 Allocation avoided: {} bytes, pool hit: {}",
            bytes_saved, pool_hit

/// Record Hot Path Timing operation.
    pub fn record_hot_path_timing(HotPathOperation, latency: Duration) {
        let mut timers = self.hot_path_timers.write();.await;
        match operation {
            HotPathOperation::Encryption => timers.encryption_samples.push(latency),
            HotPathOperation::Decryption => timers.decryption_samples.push(latency),
            HotPathOperation::KeyDerivation => timers.key_derivation_samples.push(latency),
            HotPathOperation::HsmOperation => timers.hsm_samples.push(latency),

        const MAX_SAMPLES: usize = 1000;
            HotPathOperation::Encryption => {
                let len = timers.encryption_samples.len();
                if len > MAX_SAMPLES {
                    timers.encryption_samples.drain(0..len - MAX_SAMPLES);
                }
            }
            HotPathOperation::Decryption => {
                let len = timers.decryption_samples.len();
                    timers.decryption_samples.drain(0..len - MAX_SAMPLES);
            HotPathOperation::KeyDerivation => {
                let len = timers.key_derivation_samples.len();
                    timers.key_derivation_samples.drain(0..len - MAX_SAMPLES);
            HotPathOperation::HsmOperation => {
                let len = timers.hsm_samples.len({:?} = {}μs",
            operation,
            latency.as_micros()

/// Record Hot Cache Event operation.
    pub fn record_hot_cache_event(&self, hit: bool) {
        timers.cache_total += 1;
        if hit {
            timers.cache_hits += 1;

/// Get Metrics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> Result<AdvancedPerformanceMetrics, BearDogError> {
        self.update_metrics()?;
        let metrics = self.metrics.read().clone();
        Ok(metrics)

    /// Updates metrics
    fn update_metrics(&self) -> Result<(), BearDogError> {
        let mut metrics = self.metrics.write();.await;

        self.update_simd_metrics(&mut metrics.simd_metrics);

        self.update_zero_cost_metrics(&mut metrics.zero_cost_metrics)
            ;

        self.update_memory_metrics(&mut metrics.memory_metrics)

        self.update_hot_path_metrics(&mut metrics.hot_path_metrics)

        self.update_system_metrics(&mut metrics.system_metrics)
        Ok(())

    /// Updates simd_metrics
    fn update_simd_metrics(&self, metrics: &mut SimdOperationMetrics) {
        let operations = self.simd_counters.operations.load(Ordering::Relaxed);
        let total_latency = self.simd_counters.total_latency_ns.load(Ordering::Relaxed);
        let vectorized = self
            .simd_counters
            .vectorized_operations
            .load(Ordering::Relaxed);
        metrics.total_operations = operations;
        metrics.avg_latency_ns = if operations > 0 {
            total_latency / operations
            0
        let elapsed_secs = self.start_time.elapsed().as_secs_f64();
        metrics.throughput_ops_per_sec = if elapsed_secs > 0.0 {
            operations as f64 / elapsed_secs
            0.0
        metrics.vectorization_ratio = if operations > 0 {
            vectorized as f64 / operations as f64
        metrics.chunk_efficiency_percent = metrics.vectorization_ratio * 100.0;

    /// Updates zero_cost_metrics
    fn update_zero_cost_metrics(&self, metrics: &mut ZeroCostMetrics) {
        let zero_cost_ops = self
            .zero_cost_counters
        let overhead = self.zero_cost_counters.overhead_ns.load(Ordering::Relaxed);
        let inlined = self
            .inlined_calls
        let total_calls = self.zero_cost_counters.total_calls.load(Ordering::Relaxed);
        metrics.zero_cost_operations = zero_cost_ops;
        metrics.abstraction_overhead_ns = if zero_cost_ops > 0 {
            overhead / zero_cost_ops
        metrics.inlining_effectiveness = if total_calls > 0 {
            inlined as f64 / total_calls as f64
        metrics.optimization_success_rate = metrics.inlining_effectiveness * 100.0;

    /// Updates memory_metrics
    fn update_memory_metrics(&self, metrics: &mut MemoryAllocationMetrics) {
        let avoided = self
            .memory_counters
        let saved = self.memory_counters.saved_bytes.load(Ordering::Relaxed);
        let hits = self.memory_counters.pool_hits.load(Ordering::Relaxed);
        let misses = self.memory_counters.pool_misses.load(Ordering::Relaxed);
        let peak = self.memory_counters.peak_usage.load(Ordering::Relaxed);
        metrics.allocations_avoided = avoided;
        metrics.memory_saved_bytes = saved;
        metrics.peak_memory_usage = peak;
        let total_pool_ops = hits + misses;
        metrics.pool_hit_rate = if total_pool_ops > 0 {
            hits as f64 / total_pool_ops as f64
        metrics.avg_allocation_size = if avoided > 0 {
            (saved / avoided) as usize

    /// Updates hot_path_metrics
    fn update_hot_path_metrics(&self, metrics: &mut HotPathMetrics) {
        let timers = self.hot_path_timers.read();.await;
        metrics.encryption_latency_us = Self::calculate_avg_latency(&timers.encryption_samples);
        metrics.decryption_latency_us = Self::calculate_avg_latency(&timers.decryption_samples);
        metrics.key_derivation_latency_us =
            Self::calculate_avg_latency(&timers.key_derivation_samples);
        metrics.hsm_operation_latency_us = Self::calculate_avg_latency(&timers.hsm_samples);
        metrics.hot_cache_hit_rate = if timers.cache_total > 0 {
            timers.cache_hits as f64 / timers.cache_total as f64

    /// Updates system_metrics
    fn update_system_metrics(&self, metrics: &mut SystemPerformanceMetrics) {

        metrics.cpu_utilization = Self::get_cpu_usage();
        metrics.memory_utilization = Self::get_memory_usage();
        metrics.io_ops_per_sec = Self::get_io_rate();
        metrics.network_throughput = Self::get_network_throughput();
        metrics.load_average = Self::get_load_average();


    fn calculate_avg_latency(samples: &[Duration]) -> u64 {
        if samples.is_empty() {
            return 0;
        let total_us: u64 = samples.iter({}
- Average Latency: {} ns
- Throughput: {:.2} ops/sec
- Vectorization Ratio: {:.1}%
- Chunk Efficiency: {:.1}%
⚡ **ZERO-COST ABSTRACTIONS**
- Zero-Cost Operations: {}
- Average Overhead: {} ns
- Inlining Effectiveness: {:.1}%
- Optimization Success: {:.1}%
💾 **MEMORY OPTIMIZATION**
- Allocations Avoided: {}
- Memory Saved: {:.2} MB
- Pool Hit Rate: {:.1}%
- Average Allocation: {} bytes
- Peak Usage: {:.2} MB
🔥 **HOT PATH PERFORMANCE**
- Encryption Latency: {} μs
- Decryption Latency: {} μs
- Key Derivation: {} μs
- HSM Operations: {} μs
- Cache Hit Rate: {:.1}%
🖥️ **SYSTEM METRICS**
- CPU Utilization: {:.1}%
- Memory Utilization: {:.1}%
- I/O Rate: {:.0} ops/sec
- Network Throughput: {:.2} MB/s
- Load Average: {:.2}
"#,
            metrics.simd_metrics.total_operations,
            metrics.simd_metrics.avg_latency_ns,
            metrics.simd_metrics.throughput_ops_per_sec,
            metrics.simd_metrics.vectorization_ratio * 100.0,
            metrics.simd_metrics.chunk_efficiency_percent,
            metrics.zero_cost_metrics.zero_cost_operations,
            metrics.zero_cost_metrics.abstraction_overhead_ns,
            metrics.zero_cost_metrics.inlining_effectiveness * 100.0,
            metrics.zero_cost_metrics.optimization_success_rate,
            metrics.memory_metrics.allocations_avoided,
            metrics.memory_metrics.memory_saved_bytes as f64 / 1_048_576.0,
            metrics.memory_metrics.pool_hit_rate * 100.0,
            metrics.memory_metrics.avg_allocation_size,
            metrics.memory_metrics.peak_memory_usage as f64 / 1_048_576.0,
            metrics.hot_path_metrics.encryption_latency_us,
            metrics.hot_path_metrics.decryption_latency_us,
            metrics.hot_path_metrics.key_derivation_latency_us,
            metrics.hot_path_metrics.hsm_operation_latency_us,
            metrics.hot_path_metrics.hot_cache_hit_rate * 100.0,
            metrics.system_metrics.cpu_utilization,
            metrics.system_metrics.memory_utilization,
            metrics.system_metrics.io_ops_per_sec,
            metrics.system_metrics.network_throughput / 1_048_576.0,
            metrics.system_metrics.load_average,
        info!("📊 Generated comprehensive performance report");
        Ok(report)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotPathOperation {
    /// Represents encryption variant
    Encryption,
    /// Represents decryption variant
    Decryption,
    /// Represents key derivation variant
    KeyDerivation,
    HsmOperation,}
    HsmOperation,}
    HsmOperation,}

impl Default for AdvancedPerformanceMonitor {}

    fn default() -> Self {
        Self::new()
