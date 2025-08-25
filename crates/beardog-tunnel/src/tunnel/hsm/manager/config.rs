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


/// # HSM Manager Configuration
///
/// This module contains all configuration structures for the HSM manager,
/// including health monitoring, failover, performance, and overall manager config.

use super::super::types::HsmConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Simple HSM tier enum for internal tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleHsmTier {
    /// Smartphone-based HSM tier
    Smartphone,
    /// Software-based HSM tier
    Software,
    /// Hardware-based HSM tier
    Hardware,
    /// Hybrid HSM tier combining multiple approaches
    Hybrid,
}
impl std::fmt::Display for SimpleHsmTier {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleHsmTier::Smartphone => write!(f, "Smartphone"),
            SimpleHsmTier::Software => write!(f, "Software"),
            SimpleHsmTier::Hardware => write!(f, "Hardware"),
            SimpleHsmTier::Hybrid => write!(f, "Hybrid"),
        }
    }
/// HSM Manager configuration
#[derive(Debug, Clone, Default)]
pub struct HsmManagerConfig {
    /// List of HSM configurations to manage
    pub hsm_configs: Vec<HsmConfig>,
    /// Health monitoring configuration
    pub health_config: HealthConfig,
    /// Failover configuration
    pub failover_config: FailoverConfig,
    /// Performance configuration
    pub performance_config: PerformanceConfig,
/// Health monitoring configuration}


#[derive(Debug, Clone)]
pub struct HealthConfig {
    /// Interval between health checks
    pub check_interval: Duration,
    /// Number of failures before marking HSM as unhealthy
    pub failure_threshold: u32,
    /// Number of successful checks before marking HSM as healthy again
    pub recovery_threshold: u32,
    /// Timeout for individual health checks
    pub timeout: Duration,
/// Failover configuration
pub struct FailoverConfig {
    /// Whether failover is enabled
    pub enabled: bool,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Delay between retry attempts
    pub retry_delay: Duration,
    /// Threshold for circuit breaker activation
    pub circuit_breaker_threshold: u32,
    /// Timeout for circuit breaker reset
    pub circuit_breaker_timeout: Duration,
/// Performance configuration
pub struct PerformanceConfig {
    /// Whether load balancing is enabled
    pub enable_load_balancing: bool,
    /// Whether caching is enabled
    pub enable_caching: bool,
    /// Maximum number of concurrent operations
    pub max_concurrent_operations: usize,
    /// Timeout for individual operations
    pub operation_timeout: Duration,}


impl Default for HealthConfig {}


    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 5,
            timeout: Duration::from_secs(5),
impl Default for FailoverConfig {
            enabled: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(30),}


impl Default for PerformanceConfig {
            enable_load_balancing: true,
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(30),
