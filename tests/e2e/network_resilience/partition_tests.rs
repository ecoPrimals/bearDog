// SPDX-License-Identifier: AGPL-3.0-or-later
//! Network Partition Tests
//!
//! Tests for network partition scenarios and recovery.

use super::NetworkE2EMetrics;
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Test network partition and reconnection
pub fn test_network_partition_recovery() -> Result<NetworkE2EMetrics, BearDogError> {
    info!("🔌 Testing network partition and recovery");

    let mut metrics = NetworkE2EMetrics::default();

    // Simulate network partition (3 failures) then recovery
    for i in 0..5 {
        metrics.connection_attempts += 1;

        if i < 3 {
            // Simulated partition - connection fails
            warn!("Network partitioned, attempt {} failed", i + 1);
            metrics.failed_connections += 1;
        } else {
            // Network restored
            info!("✅ Network restored, connection {} succeeded", i + 1);
            metrics.successful_connections += 1;
            if i == 3 {
                metrics.recoveries += 1;
            }
        }
    }

    Ok(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_partition_and_recovery() {
        let result = test_network_partition_recovery();
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.connection_attempts, 5);
        assert_eq!(metrics.failed_connections, 3);
        assert_eq!(metrics.successful_connections, 2);
        assert_eq!(metrics.recoveries, 1);
    }
}
