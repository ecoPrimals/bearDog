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


/// # Testing Types - Canonical
///
/// **UNIFIED TESTING TYPES** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Test Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_retries: u32,
    pub parallel_execution: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(300),
            max_retries: 3,
            parallel_execution: true,
        }
    }
}

/// **CANONICAL** Test Metrics Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetricsConfig {
    pub collect_metrics: bool,
    pub metrics_interval: Duration,
    pub store_results: bool,
}

impl Default for TestMetricsConfig {
    fn default() -> Self {
        Self {
            collect_metrics: true,
            metrics_interval: Duration::from_secs(1),
            store_results: true,
        }
    }
}

/// **CANONICAL** Integration Test Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationTestConfig {
    pub enabled: bool,
    pub test_environment: String,
    pub cleanup_after_test: bool,
}

impl Default for IntegrationTestConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            test_environment: "test".to_string(),
            cleanup_after_test: true,
        }
    }
}

/// **CANONICAL** Test Harness Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TestHarnessConfig {
    pub test_config: TestConfig,
    pub metrics: TestMetricsConfig,
    pub integration: IntegrationTestConfig,
}

