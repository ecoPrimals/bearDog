// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for timeout validation
//!
//! Added December 8, 2025 to increase coverage from 57.89% to 90%+

#[cfg(test)]
mod tests {
    use crate::domains::timeouts_new::TimeoutConfigBuilder;
    use crate::domains::timeouts_new::validation::validate_config;

    #[test]
    fn test_validate_config_all_valid() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .discovery_operation_secs(60)
            .ai_decision_secs(60)
            .ai_request_secs(60)
            .ai_batch_timeout_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        assert!(validate_config(&config).is_ok());
    }

    // Health check validation tests
    #[test]
    fn test_validate_health_check_zero() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(0)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("health_check_secs")
        );
    }

    #[test]
    fn test_validate_health_check_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(61)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("health_check_secs")
        );
    }

    #[test]
    fn test_validate_health_check_min_valid() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(1)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_health_check_max_valid() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(60)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        assert!(validate_config(&config).is_ok());
    }

    // HSM operation validation tests
    #[test]
    fn test_validate_hsm_operation_zero() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(0)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("hsm_operation_secs")
        );
    }

    #[test]
    fn test_validate_hsm_operation_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(11)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("hsm_operation_secs")
        );
    }

    #[test]
    fn test_validate_hsm_operation_boundaries() {
        let config_min = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(1)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();
        assert!(validate_config(&config_min).is_ok());

        let config_max = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(10)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();
        assert!(validate_config(&config_max).is_ok());
    }

    // HSM probe validation tests
    #[test]
    fn test_validate_hsm_probe_too_low() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(99)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hsm_probe_millis"));
    }

    #[test]
    fn test_validate_hsm_probe_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(5001)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hsm_probe_millis"));
    }

    #[test]
    fn test_validate_hsm_probe_boundaries() {
        let config_min = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(100)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();
        assert!(validate_config(&config_min).is_ok());

        let config_max = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(5000)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();
        assert!(validate_config(&config_max).is_ok());
    }

    // Discovery operation validation tests
    #[test]
    fn test_validate_discovery_zero() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .discovery_operation_secs(0)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("discovery_operation_secs")
        );
    }

    #[test]
    fn test_validate_discovery_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .discovery_operation_secs(301)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("discovery_operation_secs")
        );
    }

    // AI decision validation tests
    #[test]
    fn test_validate_ai_decision_zero() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .ai_decision_secs(0)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ai_decision_secs"));
    }

    #[test]
    fn test_validate_ai_decision_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .ai_decision_secs(301)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ai_decision_secs"));
    }

    // AI request validation tests
    #[test]
    fn test_validate_ai_request_zero() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .ai_request_secs(0)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ai_request_secs"));
    }

    #[test]
    fn test_validate_ai_request_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .ai_request_secs(301)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ai_request_secs"));
    }

    // AI batch validation tests
    #[test]
    fn test_validate_ai_batch_zero() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .ai_batch_timeout_millis(0)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("ai_batch_timeout_millis")
        );
    }

    #[test]
    fn test_validate_ai_batch_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .ai_batch_timeout_millis(1001)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("ai_batch_timeout_millis")
        );
    }

    // Pool idle validation tests
    #[test]
    fn test_validate_pool_idle_too_low() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(59)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("pool_idle_secs"));
    }

    #[test]
    fn test_validate_pool_idle_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(7201)
            .max_connection_age_secs(3600)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("pool_idle_secs"));
    }

    #[test]
    fn test_validate_pool_idle_boundaries() {
        let config_min = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(60)
            .max_connection_age_secs(3600)
            .build();
        assert!(validate_config(&config_min).is_ok());

        let config_max = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(7200)
            .max_connection_age_secs(3600)
            .build();
        assert!(validate_config(&config_max).is_ok());
    }

    // Max connection age validation tests
    #[test]
    fn test_validate_max_connection_age_too_low() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(299)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("max_connection_age_secs")
        );
    }

    #[test]
    fn test_validate_max_connection_age_too_high() {
        let config = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(86401)
            .build();

        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("max_connection_age_secs")
        );
    }

    #[test]
    fn test_validate_max_connection_age_boundaries() {
        let config_min = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(300)
            .build();
        assert!(validate_config(&config_min).is_ok());

        let config_max = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(86400)
            .build();
        assert!(validate_config(&config_max).is_ok());
    }

    #[test]
    fn test_validate_error_messages_specific() {
        let config1 = TimeoutConfigBuilder::new()
            .health_check_secs(0)
            .hsm_operation_secs(5)
            .hsm_probe_millis(500)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();
        let err1 = validate_config(&config1).unwrap_err();
        assert!(err1.to_string().contains("got 0"));

        let config2 = TimeoutConfigBuilder::new()
            .health_check_secs(30)
            .hsm_operation_secs(5)
            .hsm_probe_millis(50)
            .pool_idle_secs(300)
            .max_connection_age_secs(3600)
            .build();
        let err2 = validate_config(&config2).unwrap_err();
        assert!(err2.to_string().contains("got 50"));
    }
}
