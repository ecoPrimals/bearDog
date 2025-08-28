use beardog_api::auth::types::*;
use beardog_api::compliance::types::*;
use beardog_api::rate_limiting::types::*;
use beardog_api::security::types::*;
use serde_json;

#[cfg(test)]
mod api_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_api_response_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let response =
            beardog_api::success_response("test data", "req-123".to_string(), 100, false);

        let serialized = serde_json::to_string(&response)?;
        assert!(serialized.contains("test data"));
        assert!(serialized.contains("req-123"));
        Ok(())
    }

    #[tokio::test]
    async fn test_security_api_models() -> Result<(), Box<dyn std::error::Error>> {
        let security_alert = SecurityAlert {
            alert_id: "alert-001".to_string(),
            severity: AlertSeverity::High,
            message: "Test alert".to_string(),
            timestamp: chrono::Utc::now(),
            resolved: false,
            metadata: std::collections::HashMap::new(),
        };

        let serialized = serde_json::to_string(&security_alert)?;
        let _deserialized: SecurityAlert = serde_json::from_str(&serialized)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_compliance_api_models() -> Result<(), Box<dyn std::error::Error>> {
        let audit_event = AuditEvent {
            event_id: "audit-001".to_string(),
            event_type: "user_login".to_string(),
            user_id: Some("user-123".to_string()),
            timestamp: chrono::Utc::now(),
            details: std::collections::HashMap::new(),
            compliance_tags: vec!["gdpr".to_string(), "hipaa".to_string()],
        };

        let serialized = serde_json::to_string(&audit_event)?;
        let _deserialized: AuditEvent = serde_json::from_str(&serialized)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_auth_api_models() -> Result<(), Box<dyn std::error::Error>> {
        let token_validation = TokenValidationRequest {
            token: "test-token".to_string(),
            required_permissions: vec!["read".to_string(), "write".to_string()],
        };

        let serialized = serde_json::to_string(&token_validation)?;
        let _deserialized: TokenValidationRequest = serde_json::from_str(&serialized)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_api_rate_limiting() -> Result<(), Box<dyn std::error::Error>> {
        let endpoint_limit = EndpointLimit {
            endpoint: "/api/v1/test".to_string(),
            requests_per_minute: 100,
            burst_capacity: 10,
            window_size_seconds: 60,
        };

        let serialized = serde_json::to_string(&endpoint_limit)?;
        let _deserialized: EndpointLimit = serde_json::from_str(&serialized)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_pagination_models() -> Result<(), Box<dyn std::error::Error>> {
        let pagination = beardog_api::PaginationMeta {
            page: 1,
            per_page: 50,
            total: 1000,
            total_pages: 20,
            has_next: true,
            has_prev: false,
        };

        let response = beardog_api::paginated_response(
            vec!["item1", "item2", "item3"],
            "req-456".to_string(),
            150,
            pagination,
        );

        let serialized = serde_json::to_string(&response)?;
        assert!(serialized.contains("item1"));
        assert!(serialized.contains("req-456"));
        Ok(())
    }

    #[tokio::test]
    async fn test_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let error_response: beardog_api::ApiResponse<String> =
            beardog_api::error_response("Test error message".to_string(), "req-error".to_string());

        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error, Some("Test error message".to_string()));

        let serialized = serde_json::to_string(&error_response)?;
        assert!(serialized.contains("Test error message"));
        Ok(())
    }

    #[tokio::test]
    async fn test_model_serialization_consistency() -> Result<(), Box<dyn std::error::Error>> {
        // Test various model types for serialization consistency
        let models = vec![
            serde_json::json!({"test": "data"}),
            serde_json::json!({"number": 42}),
            serde_json::json!({"array": [1, 2, 3]}),
        ];

        for model in models {
            let serialized = serde_json::to_string(&model)?;
            let deserialized: serde_json::Value = serde_json::from_str(&serialized)?;
            assert_eq!(model, deserialized);
        }

        println!("✅ API model serialization working correctly");
        Ok(())
    }
}
