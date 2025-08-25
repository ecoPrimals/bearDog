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


/// # Canonical Metrics - System Performance Metrics
///
/// **UNIFIED METRICS SYSTEM** for the BearDog ecosystem

use serde::{Deserialize, Serialize};

/// **CANONICAL** System Metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetrics {
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics  
    pub memory: MemoryMetrics,
    /// Disk metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// Load average
    pub load_average: LoadAverage,
}

/// **CANONICAL** CPU Metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuMetrics {
    /// CPU usage percentage
    pub usage_percent: f64,
    /// Number of CPU cores
    pub cores: u32,
    /// CPU temperature (if available)
    pub temperature: Option<f64>,
}

/// **CANONICAL** Memory Metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryMetrics {
    /// Total memory in bytes
    pub total_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Available memory in bytes
    pub available_bytes: u64,
    /// Memory usage percentage
    pub usage_percent: f64,
}

/// **CANONICAL** Disk Metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskMetrics {
    /// Total disk space in bytes
    pub total_bytes: u64,
    /// Used disk space in bytes
    pub used_bytes: u64,
    /// Available disk space in bytes
    pub available_bytes: u64,
    /// Disk usage percentage
    pub usage_percent: f64,
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
}

/// **CANONICAL** Network Metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetrics {
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Packets received
    pub packets_received: u64,
    /// Packets sent
    pub packets_sent: u64,
    /// Network errors
    pub errors: u64,
}

/// **CANONICAL** Load Average
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadAverage {
    /// 1-minute load average
    pub one_minute: f64,
    /// 5-minute load average
    pub five_minute: f64,
    /// 15-minute load average
    pub fifteen_minute: f64,
}
