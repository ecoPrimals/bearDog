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


/// # Load Testing Configuration - Canonical
///
/// **UNIFIED LOAD TESTING CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Unified Load Testing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedLoadTestConfig {
    pub scenario: LoadTestScenario,
    pub defaults: LoadTestDefaults,
    pub monitoring: LoadTestMonitoringConfig,
    pub reporting: LoadTestReportingConfig,
}


/// **CANONICAL** Load Test Scenario Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestScenario {
    pub name: String,
    pub duration: Duration,
    pub concurrent_users: u32,
    pub ramp_up_time: Duration,
    pub test_data_file: Option<String>,
}

impl Default for LoadTestScenario {
    fn default() -> Self {
        Self {
            name: "default_scenario".to_string(),
            duration: Duration::from_secs(300), // 5 minutes
            concurrent_users: 10,
            ramp_up_time: Duration::from_secs(30),
            test_data_file: None,
        }
    }
}

/// **CANONICAL** Load Test Defaults Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestDefaults {
    pub timeout: Duration,
    pub retry_attempts: u32,
    pub think_time: Duration,
}

impl Default for LoadTestDefaults {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            think_time: Duration::from_millis(100),
        }
    }
}

/// **CANONICAL** Load Test Monitoring Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestMonitoringConfig {
    pub enabled: bool,
    pub metrics_collection_interval: Duration,
    pub resource_monitoring: bool,
    pub response_time_percentiles: Vec<f64>,
}

impl Default for LoadTestMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_collection_interval: Duration::from_secs(5),
            resource_monitoring: true,
            response_time_percentiles: vec![50.0, 90.0, 95.0, 99.0],
        }
    }
}

/// **CANONICAL** Load Test Reporting Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestReportingConfig {
    pub enabled: bool,
    pub output_format: String,
    pub output_directory: String,
    pub include_detailed_logs: bool,
}

impl Default for LoadTestReportingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            output_format: "html".to_string(),
            output_directory: "./load_test_reports".to_string(),
            include_detailed_logs: false,
        }
    }
}
