// SPDX-License-Identifier: AGPL-3.0-or-later
//! Timeout Handling Tests
//!
//! Tests for network timeout detection and handling.

use super::NetworkE2EMetrics;
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Test network timeout handling
pub async fn test_network_timeout_handling() -> Result<NetworkE2EMetrics, BearDogError> {
    info!("⏱️ Testing network timeout handling");

    let mut metrics = NetworkE2EMetrics::default();

    // Simulate operations with various timeout scenarios
    for i in 0..5 {
        metrics.connection_attempts += 1;

        // Operation delays: first 2 fast (50ms), last 3 slow (150ms+)
        let operation_delay = if i < 2 {
            tokio::time::Duration::from_millis(50)
        } else {
            tokio::time::Duration::from_millis(150)
        };

        let result = tokio::time::timeout(
            tokio::time::Duration::from_millis(100), // 100ms timeout
            tokio::time::sleep(operation_delay),
        )
        .await;

        if result == Ok(()) {
            info!("Operation {} completed within timeout", i);
            metrics.successful_connections += 1;
        } else {
            warn!("Operation {} timed out", i);
            metrics.timeout_errors += 1;
        }
    }

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_detection() {
        let result = test_network_timeout_handling().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.connection_attempts, 5);
        assert_eq!(metrics.successful_connections, 2); // First 2 fast operations
        assert_eq!(metrics.timeout_errors, 3); // Last 3 slow operations
    }
}
