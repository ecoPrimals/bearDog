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


/// Performance thresholds configuration and validation

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
/// Performance monitoring thresholds with comprehensive validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_cpu_percent: f64,
    pub max_memory_mb: f64,
    pub max_latency_ms: u64,
    pub min_success_rate: f64,
    pub max_error_rate: f64,
}
impl Default for PerformanceThresholds {}


    fn default() -> Self {
        Self {
            max_cpu_percent: 80.0,
            max_memory_mb: 4096.0,
            max_latency_ms: 1000,
            min_success_rate: 95.0,
            max_error_rate: 5.0,
        }
    }
impl PerformanceThresholds {
    /// Comprehensive threshold validation - Deep safety architecture}


    pub fn validate(&self) -> BearDogResult<()> {
        if self.max_cpu_percent < 0.0 || self.max_cpu_percent > 100.0 {
            return Err(BearDogError::validation(format!(
                "Invalid CPU threshold: {}% (must be 0-100)",
                self.max_cpu_percent
            )));
        if self.max_memory_mb < 0.0 {
                "Invalid memory threshold: {}MB (must be positive)",
                self.max_memory_mb
        if self.max_latency_ms == 0 {
            return Err(BearDogError::validation(
                "Invalid latency threshold: 0ms (must be positive)".to_string(),
            ));
        if self.min_success_rate < 0.0 || self.min_success_rate > 100.0 {
                "Invalid success rate threshold: {}% (must be 0-100)",
                self.min_success_rate
        if self.max_error_rate < 0.0 || self.max_error_rate > 100.0 {
                "Invalid error rate threshold: {}% (must be 0-100)",
                self.max_error_rate
        Ok(())
    /// Create validated thresholds
    pub fn new(
        max_cpu_percent: f64,
        max_memory_mb: f64,
        max_latency_ms: u64,
        min_success_rate: f64,
        max_error_rate: f64,
    ) -> BearDogResult<Self> {
        let thresholds = Self {
            max_cpu_percent,
            max_memory_mb,
            max_latency_ms,
            min_success_rate,
            max_error_rate,
        };
        thresholds.validate()?;
        Ok(thresholds)
