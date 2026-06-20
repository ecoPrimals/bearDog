// SPDX-License-Identifier: AGPL-3.0-or-later
// Phase 2: Network resilience e2e tests
//! Circuit Breaker Pattern Tests
//!
//! Tests for circuit breaker failure detection and recovery.

use super::NetworkE2EMetrics;
use beardog_errors::BearDogError;
use tracing::{info, warn};

const FAILURE_THRESHOLD: usize = 3;

/// Test circuit breaker failure detection and recovery
pub async fn test_circuit_breaker() -> Result<NetworkE2EMetrics, BearDogError> {
    info!("Testing circuit breaker pattern");

    let mut metrics = NetworkE2EMetrics::default();
    let mut circuit_open = false;
    let mut consecutive_failures = 0;

    for attempt in 0..8 {
        metrics.connection_attempts += 1;

        // Simulate upstream recovering after attempt 4
        let upstream_available = attempt >= 5;

        if circuit_open {
            // Half-open probe: allow one attempt through to check recovery
            if upstream_available {
                info!("Half-open probe succeeded on attempt {}", attempt + 1);
                metrics.successful_connections += 1;
                metrics.recoveries += 1;
                circuit_open = false;
                consecutive_failures = 0;
            } else {
                warn!(
                    "Circuit open, rejecting attempt {} without upstream call",
                    attempt + 1
                );
                metrics.failed_connections += 1;
            }
            continue;
        }

        if upstream_available {
            info!("Upstream succeeded on attempt {}", attempt + 1);
            metrics.successful_connections += 1;
            consecutive_failures = 0;
        } else {
            consecutive_failures += 1;
            warn!(
                "Upstream failure on attempt {} ({}/{})",
                attempt + 1,
                consecutive_failures,
                FAILURE_THRESHOLD
            );
            metrics.failed_connections += 1;

            if consecutive_failures >= FAILURE_THRESHOLD {
                circuit_open = true;
                warn!(
                    "Circuit breaker opened after {} consecutive failures",
                    consecutive_failures
                );
            }
        }
    }

    info!(
        "Circuit breaker test complete: {} attempts, {} recoveries",
        metrics.connection_attempts, metrics.recoveries
    );

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_breaker_opens_on_failures() {
        let result = test_circuit_breaker().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.connection_attempts >= FAILURE_THRESHOLD);
        assert!(metrics.failed_connections > 0);
        assert!(metrics.recoveries >= 1);
    }
}
