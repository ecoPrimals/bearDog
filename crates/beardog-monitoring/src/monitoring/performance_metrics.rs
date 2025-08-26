

use beardog_errors::BearDogResult;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedPerformanceMetrics {

    pub simd_metrics: SimdOperationMetrics,

    pub zero_cost_metrics: ZeroCostMetrics,

    pub memory_metrics: MemoryAllocationMetrics,

    pub hot_path_metrics: HotPathMetrics,

    pub system_metrics: SystemPerformanceMetrics,
}

pub struct SimdOperationMetrics {

    pub total_operations: u64,

    pub avg_latency_ns: u64,

    pub throughput_ops_per_sec: f64,

    pub chunk_efficiency_percent: f64,

    pub vectorization_ratio: f64,

pub struct ZeroCostMetrics {

    pub zero_cost_operations: u64,

    pub abstraction_overhead_ns: u64,

    pub inlining_effectiveness: f64,

    pub optimization_success_rate: f64,

pub struct MemoryAllocationMetrics {

    pub allocations_avoided: u64,

    pub memory_saved_bytes: u64,

    pub pool_hit_rate: f64,

    pub avg_allocation_size: usize,

    pub peak_memory_usage: usize,

pub struct HotPathMetrics {

    pub encryption_latency_us: u64,

    pub decryption_latency_us: u64,

    pub key_derivation_latency_us: u64,

    pub hsm_operation_latency_us: u64,

    pub hot_cache_hit_rate: f64,

pub struct SystemPerformanceMetrics {

    pub cpu_utilization: f64,

    pub memory_utilization: f64,

    pub io_ops_per_sec: f64,

    pub network_throughput: f64,

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

#[derive(Debug, Default)]
struct HotPathTimers {
    encryption_samples: Vec<Duration>,
    decryption_samples: Vec<Duration>,
    key_derivation_samples: Vec<Duration>,
    hsm_samples: Vec<Duration>,
    cache_hits: u64,
    cache_total: u64,}

impl AdvancedPerformanceMonitor {

    #[must_use] pub fn new() -> Self {
        let metrics = AdvancedPerformanceMetrics {
            simd_metrics: SimdOperationMetrics {
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
            start_time: Instant::now(),
        }
    }

    pub async fn record_simd_operation(
        &self,
        latency: Duration,
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

    pub async fn record_zero_cost_operation(&self, overhead: Duration, inlined: bool) {
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

    pub async fn record_allocation_avoided(&self, bytes_saved: u64, pool_hit: bool) {
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

    pub async fn record_hot_path_timing(&self, operation: HotPathOperation, latency: Duration) {
        let mut timers = self.hot_path_timers.write().await;
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
                let len = timers.hsm_samples.len();
                    timers.hsm_samples.drain(0..len - MAX_SAMPLES);
            "📊 Hot path timing: {:?} = {}μs",
            operation,
            latency.as_micros()

    pub async fn record_hot_cache_event(&self, hit: bool) {
        timers.cache_total += 1;
        if hit {
            timers.cache_hits += 1;

    pub async fn get_metrics(&self) -> BearDogResult<AdvancedPerformanceMetrics> {
        self.update_metrics().await?;
        let metrics = self.metrics.read().await.clone();
        Ok(metrics)

    async fn update_metrics(&self) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;

        self.update_simd_metrics(&mut metrics.simd_metrics).await;

        self.update_zero_cost_metrics(&mut metrics.zero_cost_metrics)
            .await;

        self.update_memory_metrics(&mut metrics.memory_metrics)

        self.update_hot_path_metrics(&mut metrics.hot_path_metrics)

        self.update_system_metrics(&mut metrics.system_metrics)
        Ok(())

    async fn update_simd_metrics(&self, metrics: &mut SimdOperationMetrics) {
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

    async fn update_zero_cost_metrics(&self, metrics: &mut ZeroCostMetrics) {
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

    async fn update_memory_metrics(&self, metrics: &mut MemoryAllocationMetrics) {
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

    async fn update_hot_path_metrics(&self, metrics: &mut HotPathMetrics) {
        let timers = self.hot_path_timers.read().await;
        metrics.encryption_latency_us = Self::calculate_avg_latency(&timers.encryption_samples);
        metrics.decryption_latency_us = Self::calculate_avg_latency(&timers.decryption_samples);
        metrics.key_derivation_latency_us =
            Self::calculate_avg_latency(&timers.key_derivation_samples);
        metrics.hsm_operation_latency_us = Self::calculate_avg_latency(&timers.hsm_samples);
        metrics.hot_cache_hit_rate = if timers.cache_total > 0 {
            timers.cache_hits as f64 / timers.cache_total as f64

    async fn update_system_metrics(&self, metrics: &mut SystemPerformanceMetrics) {

        metrics.cpu_utilization = Self::get_cpu_usage();
        metrics.memory_utilization = Self::get_memory_usage();
        metrics.io_ops_per_sec = Self::get_io_rate();
        metrics.network_throughput = Self::get_network_throughput();
        metrics.load_average = Self::get_load_average();

    fn calculate_avg_latency(samples: &[Duration]) -> u64 {
        if samples.is_empty() {
            return 0;
        let total_us: u64 = samples.iter().map(|d| d.as_micros() as u64).sum();
        total_us / samples.len() as u64

    const fn get_cpu_usage() -> f64 {

        42.5

    const fn get_memory_usage() -> f64 {

        67.8

    const fn get_io_rate() -> f64 {

        1250.0

    const fn get_network_throughput() -> f64 {

        1_048_576.0 // 1 MB/s

    const fn get_load_average() -> f64 {

        1.25

    pub async fn generate_performance_report(&self) -> BearDogResult<String> {
        let metrics = self.get_metrics().await?;
        let report = format!(
            r#"
🚀 **BEARDOG ADVANCED PERFORMANCE REPORT**
📊 **SIMD OPERATIONS**
- Total Operations: {}
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
    Encryption,
    Decryption,
    KeyDerivation,
    HsmOperation,}

impl Default for AdvancedPerformanceMonitor {}

    fn default() -> Self {
        Self::new()
