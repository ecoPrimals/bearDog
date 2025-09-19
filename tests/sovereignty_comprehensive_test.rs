use beardog_errors::sovereignty::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod sovereignty_tests {
    use super::*;

    #[tokio::test]
    async fn test_sovereignty_initialization() {
        let sovereignty = SovereigntyManager::new();
        assert!(sovereignty.is_ok());
    }

    #[tokio::test]
    async fn test_human_dignity_protections() {
        let sovereignty = SovereigntyManager::new()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))))?;

        let surveillance_request = create_mock_surveillance_request();
        let result = sovereignty.validate_request(&surveillance_request);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BearDogError::SovereigntyViolation(_)
        ));
    }

    #[tokio::test]
    async fn test_consent_based_interactions() {
        let sovereignty = SovereigntyManager::new()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))))?;

        let request_with_consent = create_mock_consented_request();
        let result = sovereignty.validate_request(&request_with_consent);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_primal_sovereignty_model() {
        let sovereignty = SovereigntyManager::new()
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e))))?;

        let hardcoded_request = create_mock_hardcoded_primal_request();
        let result = sovereignty.validate_request(&hardcoded_request);

        assert!(result.is_err());
    }

    #[test]
    fn test_privacy_by_design_principles() {
        let default_config = SovereigntyConfig::default();

        assert_eq!(default_config.data_collection_enabled, false);
        assert_eq!(default_config.surveillance_protection, true);
        assert_eq!(default_config.consent_required, true);
    }

    fn create_mock_surveillance_request() -> MockRequest {
        MockRequest {
            request_type: "surveillance".to_string(),
        }
    }

    fn create_mock_consented_request() -> MockRequest {
        MockRequest {
            request_type: "normal".to_string(),
        }
    }

    fn create_mock_hardcoded_primal_request() -> MockRequest {
        MockRequest {
            request_type: "primal_access".to_string(),
            hardcoded_endpoint: Some(true,
        }
    }
}

#[derive(Debug, Clone)]
    data_collection: bool,
    consent_provided: bool,
    hardcoded_endpoint: Option<String>,
}
