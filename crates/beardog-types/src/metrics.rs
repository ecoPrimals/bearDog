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


/// **CANONICAL METRICS TYPES** - Unified metrics system for `BearDog`
///
/// This module provides the canonical metrics types used across all `BearDog` components
/// for performance monitoring, system health tracking, and operational visibility.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// **CANONICAL** Metric Value - Universal metric data type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricValue {
    /// Integer metric value
    Integer(i64),
    /// Floating point metric value
    Float(f64),
    /// String metric value
    String(String),
    /// Boolean metric value
    Boolean(bool),
    /// Counter metric (monotonically increasing)
    Counter(u64),
    /// Gauge metric (can go up or down)
    Gauge(f64),
    /// Histogram metric with buckets
    Histogram {
        buckets: Vec<f64>,
        counts: Vec<u64>,
        sum: f64,
        count: u64,
    },
}
/// **CANONICAL** Performance Metrics - System performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: DateTime<Utc>,
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage: f64,
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// Disk usage percentage (0.0 to 100.0)
    pub disk_usage: f64,
    /// Network input bytes per second
    pub network_in_bytes: u64,
    /// Network output bytes per second
    pub network_out_bytes: u64,
    /// Average response time in milliseconds
    pub response_time: f64,
    /// Error rate percentage (0.0 to 100.0)
    pub error_rate: f64,
    /// Requests per second throughput
    pub throughput: f64,
    /// Active connections count
    pub active_connections: u32,
    /// Total request count
    pub request_count: u64,
    /// Total error count
    pub error_count: u64,
    /// Custom metrics for extensibility
    pub custom_metrics: HashMap<String, MetricValue>,
/// **CANONICAL** System Metrics - Comprehensive system resource data
pub struct SystemMetrics {
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// Process metrics
    pub process: ProcessMetrics,
    /// System load average
    pub load_average: LoadAverage,
    /// System uptime in seconds
    pub uptime_seconds: u64,
/// **CANONICAL** Resource Metrics - Resource utilization tracking
pub struct ResourceMetrics {
    /// CPU resource utilization
    pub cpu_utilization: f64,
    /// Memory resource utilization in bytes
    pub memory_utilization: u64,
    /// Disk space utilization in bytes
    pub disk_utilization: u64,
    /// Network bandwidth utilization in bytes/sec
    pub network_utilization: u64,
    /// Resource allocation limits
    pub limits: ResourceLimits,
    /// Resource requests
    pub requests: ResourceRequests,
/// CPU-specific metrics
pub struct CpuMetrics {
    /// Overall CPU usage percentage
    pub usage_percent: f64,
    /// Per-core CPU usage percentages
    pub per_core_usage: Vec<f64>,
    /// CPU frequency in MHz
    pub frequency_mhz: f64,
    /// CPU temperature in Celsius (if available)
    pub temperature_celsius: Option<f64>,
/// Memory-specific metrics
pub struct MemoryMetrics {
    /// Total memory in bytes
    pub total_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Available memory in bytes
    pub available_bytes: u64,
    /// Memory usage percentage
    /// Swap total in bytes
    pub swap_total_bytes: u64,
    /// Swap used in bytes
    pub swap_used_bytes: u64,
/// Disk-specific metrics
pub struct DiskMetrics {
    /// Total disk space in bytes
    /// Used disk space in bytes
    /// Available disk space in bytes
    /// Disk usage percentage
    /// Disk read bytes per second
    pub read_bytes_per_sec: u64,
    /// Disk write bytes per second
    pub write_bytes_per_sec: u64,
    /// Disk I/O operations per second
    pub iops: u64,
/// Network-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetrics {
    /// Bytes received per second
    pub rx_bytes_per_sec: u64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: u64,
    /// Packets received per second
    pub rx_packets_per_sec: u64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: u64,
    /// Network errors per second
    pub errors_per_sec: u64,
/// Process-specific metrics
pub struct ProcessMetrics {
    /// Process ID
    pub pid: u32,
    /// CPU usage percentage for this process
    pub cpu_percent: f64,
    /// Memory usage in bytes for this process
    pub memory_bytes: u64,
    /// Number of threads
    pub thread_count: u32,
    /// Number of file descriptors
    pub fd_count: u32,
/// System load average metrics
pub struct LoadAverage {
    /// 1-minute load average
    pub one_minute: f64,
    /// 5-minute load average
    pub five_minute: f64,
    /// 15-minute load average
    pub fifteen_minute: f64,
/// Resource limits configuration
pub struct ResourceLimits {
    /// Maximum CPU usage (cores)
    pub max_cpu: f64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
    /// Maximum disk usage in bytes
    pub max_disk_bytes: u64,
    /// Maximum network bandwidth in bytes/sec
    pub max_network_bytes_per_sec: u64,
/// Resource requests configuration
pub struct ResourceRequests {
    /// Requested CPU (cores)
    pub cpu: f64,
    /// Requested memory in bytes
    /// Requested disk space in bytes
    pub disk_bytes: u64,
// Default implementations}


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
            custom_metrics: HashMap::new(),
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
