

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedLoadTestConfig {
    pub scenario: LoadTestScenario,
    pub defaults: LoadTestDefaults,
    pub monitoring: LoadTestMonitoringConfig,
    pub reporting: LoadTestReportingConfig,
}

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
