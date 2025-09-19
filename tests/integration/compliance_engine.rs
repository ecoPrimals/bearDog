use beardog_compliance::ComplianceHandler;
use beardog_errors::BearDogError;
use beardog_types::canonical::configuration::ComplianceConfig;

#[cfg(test)]
mod compliance_engine_integration_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_compliance_engine_initialization() -> Result<(), BearDogError> {
        let config = ComplianceConfig::default();
        let handler = ComplianceHandler::new(config)?;

        assert!(handler.is_initialized());
        Ok(())
    }

    #[tokio::test]
    async fn test_compliance_validation() -> Result<(), BearDogError> {
        let config = ComplianceConfig::default();
        let handler = ComplianceHandler::new(config)?;

        // Test basic compliance validation
        let result = handler.validate_system_compliance()?;
        assert!(result.is_compliant());

        Ok(())
    }

    #[tokio::test]
    async fn test_audit_trail_creation() -> Result<(), BearDogError> {
        let config = ComplianceConfig::default();
        let handler = ComplianceHandler::new(config)?;

        // Create audit entry
        handler
            .create_audit_entry("test_operation", "test_resource")
            ?;

        let audit_trail = handler.get_audit_trail()?;
        assert!(!audit_trail.is_empty());

        Ok(())
    }
}
