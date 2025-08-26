

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricValue {

    Integer(i64),

    Float(f64),

    String(String),

    Boolean(bool),

    Counter(u64),

    Gauge(f64),

    Histogram {
        buckets: Vec<f64>,
        counts: Vec<u64>,
        sum: f64,
        count: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {

    pub timestamp: DateTime<Utc>,

    pub cpu_usage: f64,

    pub memory_usage: f64,

    pub disk_usage: f64,

    pub network_in_bytes: u64,

    pub network_out_bytes: u64,

    pub response_time: f64,

    pub error_rate: f64,

    pub throughput: f64,

    pub active_connections: u32,

    pub request_count: u64,

    pub error_count: u64,

    pub custom_metrics: HashMap<String, MetricValue>,

pub struct SystemMetrics {

    pub cpu: CpuMetrics,

    pub memory: MemoryMetrics,

    pub disk: DiskMetrics,

    pub network: NetworkMetrics,

    pub process: ProcessMetrics,

    pub load_average: LoadAverage,

    pub uptime_seconds: u64,

pub struct ResourceMetrics {

    pub cpu_utilization: f64,

    pub memory_utilization: u64,

    pub disk_utilization: u64,

    pub network_utilization: u64,

    pub limits: ResourceLimits,

    pub requests: ResourceRequests,

pub struct CpuMetrics {

    pub usage_percent: f64,

    pub per_core_usage: Vec<f64>,

    pub frequency_mhz: f64,

    pub temperature_celsius: Option<f64>,

pub struct MemoryMetrics {

    pub total_bytes: u64,

    pub used_bytes: u64,

    pub available_bytes: u64,

    pub swap_total_bytes: u64,

    pub swap_used_bytes: u64,

pub struct DiskMetrics {

    pub read_bytes_per_sec: u64,

    pub write_bytes_per_sec: u64,

    pub iops: u64,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetrics {

    pub rx_bytes_per_sec: u64,

    pub tx_bytes_per_sec: u64,

    pub rx_packets_per_sec: u64,

    pub tx_packets_per_sec: u64,

    pub errors_per_sec: u64,

pub struct ProcessMetrics {

    pub pid: u32,

    pub cpu_percent: f64,

    pub memory_bytes: u64,

    pub thread_count: u32,

    pub fd_count: u32,

pub struct LoadAverage {

    pub one_minute: f64,

    pub five_minute: f64,

    pub fifteen_minute: f64,

pub struct ResourceLimits {

    pub max_cpu: f64,

    pub max_memory_bytes: u64,

    pub max_disk_bytes: u64,

    pub max_network_bytes_per_sec: u64,

pub struct ResourceRequests {

    pub cpu: f64,

    pub disk_bytes: u64,

impl Default for PerformanceMetrics {}

    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_in_bytes: 0,
            network_out_bytes: 0,
            response_time: 0.0,
            error_rate: 0.0,
            throughput: 0.0,
            active_connections: 0,
            request_count: 0,
            error_count: 0,
            custom_metrics: HashMap::with_capacity(16),
        }
    }
impl Default for SystemMetrics {
            cpu: CpuMetrics::default(),
            memory: MemoryMetrics::default(),
            disk: DiskMetrics::default(),
            network: NetworkMetrics::default(),
            process: ProcessMetrics::default(),
            load_average: LoadAverage::default(),
            uptime_seconds: 0,}

impl Default for ResourceMetrics {
            cpu_utilization: 0.0,
            memory_utilization: 0,
            disk_utilization: 0,
            network_utilization: 0,
            limits: ResourceLimits::default(),
            requests: ResourceRequests::default(),
impl Default for CpuMetrics {
            usage_percent: 0.0,
            per_core_usage: Vec::new(),
            frequency_mhz: 0.0,
            temperature_celsius: None,}

impl Default for MemoryMetrics {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
impl Default for DiskMetrics {
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            iops: 0,}

impl Default for ProcessMetrics {
            pid: 0,
            cpu_percent: 0.0,
            memory_bytes: 0,
            thread_count: 0,
            fd_count: 0,
impl Default for LoadAverage {
            one_minute: 0.0,
            five_minute: 0.0,
            fifteen_minute: 0.0,}

impl Default for ResourceLimits {
            max_cpu: 1.0,
            max_memory_bytes: 1_073_741_824,      // 1GB
            max_disk_bytes: 10_737_418_240,       // 10GB
            max_network_bytes_per_sec: 1_048_576, // 1MB/s
impl Default for ResourceRequests {
            cpu: 0.1,
            memory_bytes: 134_217_728, // 128MB
            disk_bytes: 1_073_741_824, // 1GB
