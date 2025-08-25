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


//! Benchmark utilities and helpers

use std::time::Duration;

/// Calculate benchmark statistics
pub fn calculate_stats(measurements: &[Duration]) -> BenchmarkStats {
    if measurements.is_empty() {
        return BenchmarkStats::default();
    }

    let total: u64 = measurements.iter().map(|d| d.as_nanos() as u64).sum();
    let mean = total as f64 / measurements.len() as f64;

    let variance: f64 = measurements
        .iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / measurements.len() as f64;

    let std_dev = variance.sqrt();

    BenchmarkStats {
        mean_ns: mean,
        std_dev_ns: std_dev,
        min_ns: measurements.iter().min().map(|d| d.as_nanos() as f64).unwrap_or(0.0),
        max_ns: measurements.iter().max().map(|d| d.as_nanos() as f64).unwrap_or(0.0),
        count: measurements.len(),
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkStats {
    pub mean_ns: f64,
    pub std_dev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub count: usize,
}

impl Default for BenchmarkStats {
    fn default() -> Self {
        Self {
            mean_ns: 0.0,
            std_dev_ns: 0.0,
            min_ns: 0.0,
            max_ns: 0.0,
            count: 0,
        }
    }
}
