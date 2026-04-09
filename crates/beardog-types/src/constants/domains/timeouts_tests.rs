// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for timeout constants
//!
//! This module provides exhaustive testing for all timeout duration constants,
//! ensuring they maintain logical consistency and operational safety.

use super::timeouts::*;
use std::time::Duration;

#[cfg(test)]
mod timeout_tests {
    use super::*;

    // ============================================================================
    // NETWORK TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_network_timeout_values() {
        assert_eq!(NETWORK_CONNECTION_TIMEOUT.as_secs(), 10);
        assert_eq!(NETWORK_READ_TIMEOUT.as_secs(), 30);
        assert_eq!(NETWORK_WRITE_TIMEOUT.as_secs(), 30);
        assert_eq!(DEFAULT_OPERATION_TIMEOUT.as_secs(), 60);
        assert_eq!(IDLE_CONNECTION_TIMEOUT.as_secs(), 300);
    }

    #[test]
    fn test_network_timeout_relationships() {
        // Connection timeout should be shorter than read/write
        assert!(
            NETWORK_CONNECTION_TIMEOUT < NETWORK_READ_TIMEOUT,
            "Connection should timeout faster than reads"
        );

        // Read and write timeouts should be equal (symmetry)
        assert_eq!(
            NETWORK_READ_TIMEOUT, NETWORK_WRITE_TIMEOUT,
            "Read and write timeouts should match"
        );

        // Idle timeout should be longer than operation timeout
        assert!(
            IDLE_CONNECTION_TIMEOUT > DEFAULT_OPERATION_TIMEOUT,
            "Idle connections can wait longer than active operations"
        );
    }

    // ============================================================================
    // HTTP/GRPC TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_http_grpc_timeout_values() {
        assert_eq!(HTTP_REQUEST_TIMEOUT.as_secs(), 30);
        assert_eq!(GRPC_REQUEST_TIMEOUT.as_secs(), 60);
        assert_eq!(STREAMING_TIMEOUT.as_secs(), 300);
    }

    #[test]
    fn test_http_grpc_timeout_relationships() {
        // gRPC can take longer than HTTP (more complex operations)
        assert!(
            GRPC_REQUEST_TIMEOUT >= HTTP_REQUEST_TIMEOUT,
            "gRPC timeout should be >= HTTP timeout"
        );

        // Streaming should have much longer timeout
        assert!(
            STREAMING_TIMEOUT > GRPC_REQUEST_TIMEOUT,
            "Streaming operations need longer timeouts"
        );
    }

    // ============================================================================
    // HEALTH CHECK TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_health_check_timeout_values() {
        assert_eq!(HEALTH_CHECK_TIMEOUT.as_secs(), 5);
        assert_eq!(HEALTH_CHECK_INTERVAL.as_secs(), 30);
        assert_eq!(READINESS_CHECK_TIMEOUT.as_secs(), 10);
        assert_eq!(METRICS_COLLECTION_TIMEOUT.as_secs(), 15);
    }

    #[test]
    fn test_health_check_relationships() {
        // Health checks should be fast
        assert!(
            HEALTH_CHECK_TIMEOUT < READINESS_CHECK_TIMEOUT,
            "Health checks should be faster than readiness checks"
        );

        // Interval should be longer than timeout
        assert!(
            HEALTH_CHECK_INTERVAL > HEALTH_CHECK_TIMEOUT,
            "Check interval should be longer than check timeout"
        );
    }

    // ============================================================================
    // DISCOVERY TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_discovery_timeout_values() {
        assert_eq!(SERVICE_DISCOVERY_TIMEOUT.as_secs(), 15);
        assert_eq!(PROVIDER_DISCOVERY_TIMEOUT.as_secs(), 10);
        assert_eq!(DNS_RESOLUTION_TIMEOUT.as_secs(), 5);
        assert_eq!(REGISTRATION_TIMEOUT.as_secs(), 20);
    }

    #[test]
    fn test_discovery_timeout_relationships() {
        // DNS should be fastest
        assert!(
            DNS_RESOLUTION_TIMEOUT < PROVIDER_DISCOVERY_TIMEOUT,
            "DNS resolution should be fastest"
        );

        // Provider discovery < service discovery (provider is subset)
        assert!(
            PROVIDER_DISCOVERY_TIMEOUT <= SERVICE_DISCOVERY_TIMEOUT,
            "Provider discovery should be <= service discovery"
        );

        // Registration can take longest (validation overhead)
        assert!(
            REGISTRATION_TIMEOUT >= SERVICE_DISCOVERY_TIMEOUT,
            "Registration can take longer than discovery"
        );
    }

    // ============================================================================
    // HSM TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_hsm_timeout_values() {
        assert_eq!(HSM_OPERATION_TIMEOUT.as_secs(), 30);
        assert_eq!(HSM_PROBE_TIMEOUT.as_secs(), 5);
        assert_eq!(CRYPTO_OPERATION_TIMEOUT.as_secs(), 60);
        assert_eq!(KEY_ROTATION_TIMEOUT.as_secs(), 120);
    }

    #[test]
    fn test_hsm_timeout_relationships() {
        // Probe should be fastest (just checking availability)
        assert!(
            HSM_PROBE_TIMEOUT < HSM_OPERATION_TIMEOUT,
            "HSM probe should be faster than operations"
        );

        // Crypto operations can take longer than basic HSM ops
        assert!(
            CRYPTO_OPERATION_TIMEOUT >= HSM_OPERATION_TIMEOUT,
            "Crypto operations can take longer"
        );

        // Key rotation is most complex
        assert!(
            KEY_ROTATION_TIMEOUT > CRYPTO_OPERATION_TIMEOUT,
            "Key rotation is more complex than single operations"
        );
    }

    // ============================================================================
    // AI/ML TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_ai_timeout_values() {
        assert_eq!(AI_DECISION_TIMEOUT.as_secs(), 10);
        assert_eq!(AI_REQUEST_TIMEOUT.as_secs(), 30);
        assert_eq!(AI_BATCH_TIMEOUT.as_secs(), 300);
    }

    #[test]
    fn test_ai_timeout_progression() {
        // Decision < Request < Batch
        assert!(
            AI_DECISION_TIMEOUT < AI_REQUEST_TIMEOUT,
            "AI decisions should be quick for UX"
        );
        assert!(
            AI_REQUEST_TIMEOUT < AI_BATCH_TIMEOUT,
            "Batch processing can take longer"
        );
    }

    // ============================================================================
    // DATABASE TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_database_timeout_values() {
        assert_eq!(DATABASE_QUERY_TIMEOUT.as_secs(), 30);
        assert_eq!(DATABASE_CONNECTION_TIMEOUT.as_secs(), 10);
        assert_eq!(TRANSACTION_TIMEOUT.as_secs(), 60);
        assert_eq!(FILE_OPERATION_TIMEOUT.as_secs(), 30);
    }

    #[test]
    fn test_database_timeout_relationships() {
        // Connection should be faster than queries
        assert!(
            DATABASE_CONNECTION_TIMEOUT < DATABASE_QUERY_TIMEOUT,
            "Connection should be faster than queries"
        );

        // Transactions can span multiple queries
        assert!(
            TRANSACTION_TIMEOUT >= DATABASE_QUERY_TIMEOUT,
            "Transactions can take longer than single queries"
        );
    }

    // ============================================================================
    // CACHE TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_cache_timeout_values() {
        assert_eq!(CACHE_OPERATION_TIMEOUT.as_secs(), 5);
        assert_eq!(CACHE_WARMUP_TIMEOUT.as_secs(), 120);
    }

    #[test]
    fn test_cache_timeout_relationships() {
        // Cache ops should be very fast
        assert!(
            CACHE_OPERATION_TIMEOUT < NETWORK_CONNECTION_TIMEOUT,
            "Cache should be faster than network"
        );

        // Warmup can take much longer
        assert!(
            CACHE_WARMUP_TIMEOUT > CACHE_OPERATION_TIMEOUT * 10,
            "Warmup involves many operations"
        );
    }

    // ============================================================================
    // WORKFLOW TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_workflow_timeout_values() {
        assert_eq!(WORKFLOW_STEP_TIMEOUT.as_secs(), 300);
        assert_eq!(WORKFLOW_TOTAL_TIMEOUT.as_secs(), 1800);
        assert_eq!(BACKGROUND_JOB_TIMEOUT.as_secs(), 600);
    }

    #[test]
    fn test_workflow_timeout_relationships() {
        // Total workflow should be longer than single step
        assert!(
            WORKFLOW_TOTAL_TIMEOUT > WORKFLOW_STEP_TIMEOUT,
            "Total workflow timeout should exceed single step"
        );

        // Background jobs can run long
        assert!(
            BACKGROUND_JOB_TIMEOUT >= WORKFLOW_STEP_TIMEOUT,
            "Background jobs can be complex"
        );
    }

    // ============================================================================
    // RETRY TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_retry_timeout_values() {
        assert_eq!(INITIAL_RETRY_DELAY.as_secs(), 1);
        assert_eq!(MAX_RETRY_DELAY.as_secs(), 60);
        assert_eq!(RETRY_TIMEOUT.as_secs(), 300);
    }

    #[test]
    fn test_retry_timeout_relationships() {
        // Initial < Max
        assert!(
            INITIAL_RETRY_DELAY < MAX_RETRY_DELAY,
            "Initial retry delay should be less than max"
        );

        // Total retry timeout should allow multiple max delays
        assert!(
            RETRY_TIMEOUT > MAX_RETRY_DELAY * 3,
            "Retry timeout should allow multiple attempts at max delay"
        );
    }

    // ============================================================================
    // SHUTDOWN TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_shutdown_timeout_values() {
        assert_eq!(GRACEFUL_SHUTDOWN_TIMEOUT.as_secs(), 30);
        assert_eq!(FORCE_SHUTDOWN_TIMEOUT.as_secs(), 10);
    }

    #[test]
    fn test_shutdown_timeout_relationships() {
        // Graceful should be longer than force
        assert!(
            GRACEFUL_SHUTDOWN_TIMEOUT > FORCE_SHUTDOWN_TIMEOUT,
            "Graceful shutdown should have more time than force"
        );
    }

    // ============================================================================
    // BACKWARD COMPATIBILITY TESTS
    // ============================================================================

    #[test]
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    fn test_deprecated_aliases_match() {
        assert_eq!(DEFAULT_TIMEOUT, NETWORK_CONNECTION_TIMEOUT);
        assert_eq!(DEFAULT_REQUEST_TIMEOUT, DEFAULT_OPERATION_TIMEOUT);
        assert_eq!(REQUEST_TIMEOUT, HTTP_REQUEST_TIMEOUT);
        assert_eq!(CONNECTION_TIMEOUT, NETWORK_CONNECTION_TIMEOUT);
        assert_eq!(OPERATION_TIMEOUT, DEFAULT_OPERATION_TIMEOUT);
    }

    // ============================================================================
    // PROPERTY TESTS (Invariants)
    // ============================================================================

    #[test]
    fn test_all_timeouts_positive() {
        // All timeouts must be > 0
        let timeouts = [
            NETWORK_CONNECTION_TIMEOUT,
            NETWORK_READ_TIMEOUT,
            NETWORK_WRITE_TIMEOUT,
            DEFAULT_OPERATION_TIMEOUT,
            HEALTH_CHECK_TIMEOUT,
            SERVICE_DISCOVERY_TIMEOUT,
            HSM_OPERATION_TIMEOUT,
            AI_DECISION_TIMEOUT,
            DATABASE_QUERY_TIMEOUT,
            CACHE_OPERATION_TIMEOUT,
        ];

        for timeout in &timeouts {
            assert!(timeout.as_secs() > 0, "Timeout {timeout:?} must be > 0");
        }
    }

    #[test]
    fn test_timeouts_reasonable() {
        // No timeout should be longer than 1 hour (except workflow total)
        let timeouts = [
            NETWORK_CONNECTION_TIMEOUT,
            HTTP_REQUEST_TIMEOUT,
            HEALTH_CHECK_TIMEOUT,
            SERVICE_DISCOVERY_TIMEOUT,
            HSM_OPERATION_TIMEOUT,
            AI_REQUEST_TIMEOUT,
            DATABASE_QUERY_TIMEOUT,
        ];

        const ONE_HOUR: Duration = Duration::from_secs(3600);
        for timeout in &timeouts {
            assert!(
                timeout < &ONE_HOUR,
                "Timeout {timeout:?} should be < 1 hour"
            );
        }
    }

    #[test]
    fn test_fast_timeouts_under_10_seconds() {
        // Health checks and probes should be fast
        let fast_timeouts = [
            HEALTH_CHECK_TIMEOUT,
            HSM_PROBE_TIMEOUT,
            CACHE_OPERATION_TIMEOUT,
            DNS_RESOLUTION_TIMEOUT,
        ];

        const TEN_SECONDS: Duration = Duration::from_secs(10);
        for timeout in &fast_timeouts {
            assert!(
                timeout <= &TEN_SECONDS,
                "Fast timeout {timeout:?} should be <= 10 seconds"
            );
        }
    }

    #[test]
    fn test_timeout_consistency_with_intervals() {
        // Intervals should be longer than timeouts
        assert!(
            HEALTH_CHECK_INTERVAL > HEALTH_CHECK_TIMEOUT * 2,
            "Check interval should allow timeout + buffer"
        );
    }

    // ============================================================================
    // SPECIALIZED TIMEOUT TESTS
    // ============================================================================

    #[test]
    fn test_streaming_timeout_appropriate() {
        // Streaming should be at least 5x longer than regular requests
        assert!(
            STREAMING_TIMEOUT >= HTTP_REQUEST_TIMEOUT * 5,
            "Streaming needs much longer timeout than regular requests"
        );
    }

    #[test]
    fn test_idle_timeout_appropriate() {
        // Idle should be at least 5x longer than operation timeout
        assert!(
            IDLE_CONNECTION_TIMEOUT >= DEFAULT_OPERATION_TIMEOUT * 5,
            "Idle connections should wait much longer"
        );
    }

    #[test]
    fn test_health_check_fast() {
        // Health checks must be fast for quick failure detection
        assert!(
            HEALTH_CHECK_TIMEOUT.as_secs() <= 5,
            "Health checks should complete within 5 seconds"
        );
    }

    #[test]
    fn test_dns_timeout_fast() {
        // DNS should be very fast
        assert!(
            DNS_RESOLUTION_TIMEOUT.as_secs() <= 5,
            "DNS resolution should complete within 5 seconds"
        );
    }
}
