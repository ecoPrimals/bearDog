// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for TimeoutConfigBuilder
//!
//! Added December 8, 2025 to increase coverage from 42.57% to 90%+

#[cfg(test)]
mod tests {
    use crate::domains::timeouts_new::TimeoutConfigBuilder;
    use crate::domains::timeouts_new::defaults::default_timeouts;

    #[test]
    fn test_builder_new_creates_default() {
        let builder = TimeoutConfigBuilder::new();
        let config = builder.build();

        // Should use all defaults
        assert_eq!(
            config.health_check_secs,
            default_timeouts::HEALTH_CHECK_SECS
        );
        assert_eq!(
            config.hsm_operation_secs,
            default_timeouts::HSM_OPERATION_SECS
        );
        assert_eq!(config.hsm_probe_millis, default_timeouts::HSM_PROBE_MILLIS);
    }

    #[test]
    fn test_builder_health_check_secs() {
        let config = TimeoutConfigBuilder::new().health_check_secs(10).build();

        assert_eq!(config.health_check_secs, 10);
    }

    #[test]
    fn test_builder_hsm_operation_secs() {
        let config = TimeoutConfigBuilder::new().hsm_operation_secs(20).build();

        assert_eq!(config.hsm_operation_secs, 20);
    }

    #[test]
    fn test_builder_hsm_probe_millis() {
        let config = TimeoutConfigBuilder::new().hsm_probe_millis(500).build();

        assert_eq!(config.hsm_probe_millis, 500);
    }

    #[test]
    fn test_builder_discovery_operation_secs() {
        let config = TimeoutConfigBuilder::new()
            .discovery_operation_secs(15)
            .build();

        assert_eq!(config.discovery_operation_secs, 15);
    }

    #[test]
    fn test_builder_ai_decision_secs() {
        let config = TimeoutConfigBuilder::new().ai_decision_secs(30).build();

        assert_eq!(config.ai_decision_secs, 30);
    }

    #[test]
    fn test_builder_ai_request_secs() {
        let config = TimeoutConfigBuilder::new().ai_request_secs(25).build();

        assert_eq!(config.ai_request_secs, 25);
    }

    #[test]
    fn test_builder_ai_batch_timeout_millis() {
        let config = TimeoutConfigBuilder::new()
            .ai_batch_timeout_millis(2000)
            .build();

        assert_eq!(config.ai_batch_timeout_millis, 2000);
    }

    #[test]
    fn test_builder_pool_idle_secs() {
        let config = TimeoutConfigBuilder::new().pool_idle_secs(300).build();

        assert_eq!(config.pool_idle_secs, 300);
    }

    #[test]
    fn test_builder_max_connection_age_secs() {
        let config = TimeoutConfigBuilder::new()
            .max_connection_age_secs(600)
            .build();

        assert_eq!(config.max_connection_age_secs, 600);
    }

    #[test]
    fn test_builder_network_operation_secs() {
        let config = TimeoutConfigBuilder::new()
            .network_operation_secs(45)
            .build();

        assert_eq!(config.network_operation_secs, 45);
    }

    #[test]
    fn test_builder_dns_resolution_timeout_secs() {
        let config = TimeoutConfigBuilder::new()
            .dns_resolution_timeout_secs(5)
            .build();

        assert_eq!(config.dns_resolution_timeout_secs, 5);
    }

    #[test]
    fn test_builder_connection_timeout_secs() {
        let config = TimeoutConfigBuilder::new()
            .connection_timeout_secs(10)
            .build();

        assert_eq!(config.connection_timeout_secs, 10);
    }

    #[test]
    fn test_builder_request_timeout_secs() {
        let config = TimeoutConfigBuilder::new().request_timeout_secs(60).build();

        assert_eq!(config.request_timeout_secs, 60);
    }

    #[test]
    fn test_builder_chaining_all_methods() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(1)
            .hsm_operation_secs(2)
            .hsm_probe_millis(100)
            .discovery_operation_secs(3)
            .ai_decision_secs(4)
            .ai_request_secs(5)
            .ai_batch_timeout_millis(1000)
            .pool_idle_secs(600)
            .max_connection_age_secs(1200)
            .network_operation_secs(6)
            .dns_resolution_timeout_secs(7)
            .connection_timeout_secs(8)
            .request_timeout_secs(9)
            .build();

        assert_eq!(config.health_check_secs, 1);
        assert_eq!(config.hsm_operation_secs, 2);
        assert_eq!(config.hsm_probe_millis, 100);
        assert_eq!(config.discovery_operation_secs, 3);
        assert_eq!(config.ai_decision_secs, 4);
        assert_eq!(config.ai_request_secs, 5);
        assert_eq!(config.ai_batch_timeout_millis, 1000);
        assert_eq!(config.pool_idle_secs, 600);
        assert_eq!(config.max_connection_age_secs, 1200);
        assert_eq!(config.network_operation_secs, 6);
        assert_eq!(config.dns_resolution_timeout_secs, 7);
        assert_eq!(config.connection_timeout_secs, 8);
        assert_eq!(config.request_timeout_secs, 9);
    }

    #[test]
    fn test_builder_partial_configuration() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(10)
            .ai_decision_secs(20)
            .build();

        // Set values should be custom
        assert_eq!(config.health_check_secs, 10);
        assert_eq!(config.ai_decision_secs, 20);

        // Unset values should be defaults
        assert_eq!(
            config.hsm_operation_secs,
            default_timeouts::HSM_OPERATION_SECS
        );
        assert_eq!(config.pool_idle_secs, default_timeouts::POOL_IDLE_SECS);
    }

    #[test]
    fn test_builder_zero_values() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(0)
            .hsm_probe_millis(0)
            .build();

        assert_eq!(config.health_check_secs, 0);
        assert_eq!(config.hsm_probe_millis, 0);
    }

    #[test]
    fn test_builder_max_values() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(u64::MAX)
            .hsm_operation_secs(u64::MAX)
            .build();

        assert_eq!(config.health_check_secs, u64::MAX);
        assert_eq!(config.hsm_operation_secs, u64::MAX);
    }

    #[test]
    fn test_builder_from_env_leaves_defaults_when_no_env() {
        let config = TimeoutConfigBuilder::new().from_env().build();
        assert_eq!(
            config.health_check_secs,
            default_timeouts::HEALTH_CHECK_SECS
        );
        assert_eq!(
            config.hsm_operation_secs,
            default_timeouts::HSM_OPERATION_SECS
        );
    }

    #[test]
    fn test_programmatic_override_same_as_explicit_builder_chain() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(15)
            .hsm_operation_secs(25)
            .hsm_probe_millis(750)
            .build();
        assert_eq!(config.health_check_secs, 15);
        assert_eq!(config.hsm_operation_secs, 25);
        assert_eq!(config.hsm_probe_millis, 750);
    }

    #[test]
    fn test_builder_explicit_after_from_env() {
        let config = TimeoutConfigBuilder::new()
            .from_env()
            .health_check_secs(10)
            .build();
        assert_eq!(config.health_check_secs, 10);
    }

    #[test]
    fn test_builder_from_env_then_explicit_chain() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(10)
            .from_env()
            .build();
        assert_eq!(config.health_check_secs, 10);
    }

    #[test]
    fn test_builder_clone() {
        let builder1 = TimeoutConfigBuilder::new()
            .health_check_secs(10)
            .hsm_operation_secs(20);

        let builder2 = builder1.clone();

        let config1 = builder1.build();
        let config2 = builder2.build();

        assert_eq!(config1.health_check_secs, config2.health_check_secs);
        assert_eq!(config1.hsm_operation_secs, config2.hsm_operation_secs);
    }

    #[test]
    fn test_builder_debug_format() {
        let builder = TimeoutConfigBuilder::new().health_check_secs(10);

        let debug_str = format!("{builder:?}");
        assert!(debug_str.contains("TimeoutConfigBuilder"));
    }

    #[test]
    fn test_builder_default_trait() {
        let builder1: TimeoutConfigBuilder = Default::default();
        let builder2 = TimeoutConfigBuilder::new();

        let config1 = builder1.build();
        let config2 = builder2.build();

        assert_eq!(config1.health_check_secs, config2.health_check_secs);
    }
}
