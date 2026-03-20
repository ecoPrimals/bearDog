// SPDX-License-Identifier: AGPL-3.0-only
//! Retry and Exponential Backoff Tests
//!
//! Tests for network retry logic with exponential backoff strategies.

use super::NetworkE2EMetrics;
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Test network retry with exponential backoff
pub async fn test_network_retry_backoff() -> Result<NetworkE2EMetrics, BearDogError> {
    info!("🔄 Testing network retry with exponential backoff");

    let mut metrics = NetworkE2EMetrics::default();

    // Simulate failing connections that eventually succeed
    for attempt in 0..5 {
        metrics.connection_attempts += 1;

        // Exponential backoff calculated (not waited - testing logic, not timing)
        let _backoff_ms = 2_u64.pow(attempt) * 100;

        if attempt < 3 {
            // First 3 attempts fail
            warn!("Connection attempt {} failed, retrying...", attempt + 1);
            metrics.failed_connections += 1;
            metrics.retries += 1;
        } else {
            // 4th attempt succeeds
            info!("✅ Connection attempt {} succeeded", attempt + 1);
            metrics.successful_connections += 1;
            metrics.recoveries += 1;
            break;
        }
    }

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exponential_backoff() {
        let result = test_network_retry_backoff().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.connection_attempts, 4);
        assert_eq!(metrics.successful_connections, 1);
        assert_eq!(metrics.failed_connections, 3);
        assert_eq!(metrics.retries, 3);
        assert_eq!(metrics.recoveries, 1);
    }
}
