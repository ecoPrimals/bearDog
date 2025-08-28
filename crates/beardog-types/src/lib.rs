pub mod canonical;

pub mod aliases;
pub mod config;
pub mod constants;
pub mod hsm;
pub mod network;
pub mod providers;
pub mod testing;
pub mod zero_cost;

pub use canonical::*;

// Deprecated Result aliases removed - use Result<T, BearDogError> directly

pub fn validate_canonical_usage() -> Result<(), Vec<String>> {
    let errors = Vec::new();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[must_use]
pub fn canonical_type_info() -> Vec<(&'static str, &'static str)> {
    vec![
        ("HealthStatus", "Canonical health status for all systems"),
        ("SessionConfig", "Canonical session configuration"),
        (
            "SecurityContext",
            "Canonical security context for all operations",
        ),
        ("SecurityAuditEvent", "Canonical audit event structure"),
        ("PolicyDecision", "Canonical policy decision enum"),
        ("KeyStatus", "Canonical key status enum"),
        ("WorkflowStatus", "Canonical workflow status enum"),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_canonical_types_available() {
        let _health = HealthStatus::Healthy;
        let _security_context = SecurityContext::default();
        let _policy_decision = PolicyDecision::Allow;
    }
    #[test]
    fn test_canonical_types_direct_access() {
        let _health: HealthStatus = HealthStatus::Healthy;
        let _context: SecurityContext = SecurityContext::default();
    }
    #[test]
    fn test_canonical_validation() {
        assert!(canonical::CanonicalTypeRegistry::is_canonical_type(
            "HealthStatus"
        ));
        assert!(canonical::CanonicalTypeRegistry::is_canonical_type(
            "SessionConfig"
        ));
        assert!(canonical::CanonicalTypeRegistry::is_canonical_type(
            "SecurityContext"
        ));
        assert!(!canonical::CanonicalTypeRegistry::is_canonical_type(
            "NonExistentType"
        ));
    }
    #[test]
    fn test_configuration_helpers() {
        let api_host = canonical::constants::default_api_host();
        assert!(!api_host.is_empty());
        let api_port = canonical::constants::default_api_port();
        assert!(api_port > 0);
        let bind_address = canonical::constants::default_api_bind_address();
        assert!(bind_address.contains(':'));
        assert!(bind_address.len() > 3); // At least "x:y"
    }
    #[test]
    fn test_timeout_configurations() {
        let timeout = canonical::constants::default_timeout_ms();
        assert!(timeout <= 300_000); // Max 5 minutes is reasonable
        assert!(timeout > 0); // Must be positive
        let health_check = canonical::constants::default_health_check_interval_ms();
        assert!(health_check <= 600_000); // Max 10 minutes is reasonable
    }
    #[test]
    fn test_canonical_type_info() {
        let type_info = canonical_type_info();
        assert!(!type_info.is_empty());

        let type_names: Vec<&str> = type_info.iter().map(|(name, _)| *name).collect();
        assert!(type_names.contains(&"HealthStatus"));
        assert!(type_names.contains(&"SecurityContext"));
    }
    #[test]
    fn test_canonical_usage_validation() {
        let result = validate_canonical_usage();
        assert!(result.is_ok(), "Canonical usage validation should pass");
    }
}
