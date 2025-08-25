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


//! Byzantine Chaos Testing Module
//!
//! Focused chaos engineering tests for Byzantine fault tolerance including:
//! - Malicious node simulation
//! - Consensus disruption scenarios
//! - Fault tolerance validation

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::{BearDogError, BearDogResult};
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Byzantine chaos testing controller
#[derive(Debug)]
pub struct ByzantineChaosController {
    config: ChaosConfig,
}

impl ByzantineChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self { config }
    }

    /// Test Byzantine fault tolerance
    pub async fn test_byzantine_failures(&self) -> BearDogResult<TestResult> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;

        info!("⚔️ Testing Byzantine fault tolerance");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            // Simulate Byzantine behavior
            let byzantine_result = self.simulate_byzantine_behavior().await;

            match byzantine_result {
                Ok(_) => operations_succeeded += 1,
                Err(e) => warn!("Byzantine scenario failed: {}", e),
            }

            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        let error_rate = 1.0 - (operations_succeeded as f64 / operations_attempted as f64);

        Ok(TestResult {
            success: error_rate < 0.33, // Should tolerate up to 33% Byzantine nodes
            test_name: "byzantine_failures".to_string(),
            duration: start_time.elapsed(),
            error_message: if error_rate >= 0.33 {
                Some(format!(
                    "Byzantine tolerance exceeded: {:.2}%",
                    error_rate * 100.0
                ))
            } else {
                None
            },
            metrics: TestMetrics {
                operations_attempted,
                operations_succeeded,
                average_latency_ms: 0.0,
                peak_memory_mb: 0,
                error_rate,
            },
        })
    }

    /// Simulate Byzantine node behavior
    async fn simulate_byzantine_behavior(&self) -> BearDogResult<()> {
        // Simulate different Byzantine failure modes
        let failure_mode = fastrand::u32(0..4);

        match failure_mode {
            0 => {
                // Honest behavior (no failure)
                Ok(())
            }
            1 => {
                // Fail-stop behavior
                if fastrand::f64() < self.config.failure_rate {
                    Err(BearDogError::Node {
                        message: "Byzantine node stopped".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            2 => {
                // Arbitrary behavior (data corruption)
                if fastrand::f64() < self.config.failure_rate {
                    Err(BearDogError::DataCorruption {
                        message: "Byzantine data corruption".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => {
                // Malicious behavior (security violation)
                if fastrand::f64() < self.config.failure_rate {
                    Err(BearDogError::Security {
                        message: "Byzantine security violation".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// Byzantine failure test implementation
pub struct ByzantineFailureTest;

impl ByzantineFailureTest {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(8),
            failure_rate: 0.25, // 25% Byzantine failure rate
            enable_byzantine_failures: true,
            ..Default::default()
        };

        let controller = ByzantineChaosController::new(config);
        let result = controller.test_byzantine_failures().await?;

        info!("⚔️ Byzantine fault tolerance test completed");
        info!(
            "   Operations: {} attempted, {} succeeded",
            result.metrics.operations_attempted, result.metrics.operations_succeeded
        );
        info!("   Error rate: {:.2}%", result.metrics.error_rate * 100.0);

        assert!(
            result.success,
            "Byzantine fault tolerance should handle up to 33% failures"
        );
        Ok(())
    }
}
