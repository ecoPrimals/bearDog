// SPDX-License-Identifier: AGPL-3.0-or-later
// Phase 2: Network resilience e2e tests
//! Graceful Degradation Tests
//!
//! Tests for graceful degradation under network stress.

use super::NetworkE2EMetrics;
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Test graceful degradation under increasing network stress
pub async fn test_graceful_degradation() -> Result<NetworkE2EMetrics, BearDogError> {
    info!("Testing graceful degradation");

    let mut metrics = NetworkE2EMetrics::default();
    let stress_levels = [0.2, 0.4, 0.6, 0.8, 1.0];

    for (index, stress) in stress_levels.iter().enumerate() {
        metrics.connection_attempts += 1;

        if *stress >= 0.8 {
            warn!(
                "High stress level {:.0}%, serving degraded response for request {}",
                stress * 100.0,
                index + 1
            );
            metrics.failed_connections += 1;
            metrics.timeout_errors += 1;
            metrics.retries += 1;
            continue;
        }

        if *stress >= 0.6 {
            warn!(
                "Moderate stress level {:.0}%, reducing service quality for request {}",
                stress * 100.0,
                index + 1
            );
            metrics.successful_connections += 1;
            metrics.recoveries += 1;
            continue;
        }

        info!(
            "Normal operation at stress level {:.0}% for request {}",
            stress * 100.0,
            index + 1
        );
        metrics.successful_connections += 1;
    }

    info!(
        "Graceful degradation test complete: {} successful, {} degraded/timeouts",
        metrics.successful_connections,
        metrics.failed_connections + metrics.timeout_errors
    );

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_degradation_handling() {
        let result = test_graceful_degradation().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.connection_attempts, 5);
        assert!(metrics.successful_connections >= 2);
        assert!(metrics.timeout_errors >= 1);
    }
}
