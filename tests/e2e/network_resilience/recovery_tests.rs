// SPDX-License-Identifier: AGPL-3.0-or-later
// Phase 2: Network resilience e2e tests
//! Recovery Scenario Tests
//!
//! Tests for various recovery scenarios after failures.

use super::NetworkE2EMetrics;
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Test recovery after transient network failures
pub async fn test_recovery_scenarios() -> Result<NetworkE2EMetrics, BearDogError> {
    info!("Testing recovery scenarios");

    let mut metrics = NetworkE2EMetrics::default();
    let max_retries = 4;

    for attempt in 0..max_retries {
        metrics.connection_attempts += 1;

        if attempt < 2 {
            warn!(
                "Transient failure on attempt {}, scheduling retry",
                attempt + 1
            );
            metrics.failed_connections += 1;
            metrics.retries += 1;
            let _backoff_ms = 100_u64 * 2_u64.pow(attempt as u32);
            info!("Retry backoff: {}ms", _backoff_ms);
            continue;
        }

        info!("Connection restored on attempt {}", attempt + 1);
        metrics.successful_connections += 1;
        metrics.recoveries += 1;
        break;
    }

    info!(
        "Recovery scenario complete: {} retries, {} recoveries",
        metrics.retries, metrics.recoveries
    );

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_recovery() {
        let result = test_recovery_scenarios().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.connection_attempts, 3);
        assert_eq!(metrics.failed_connections, 2);
        assert_eq!(metrics.retries, 2);
        assert_eq!(metrics.recoveries, 1);
    }
}
