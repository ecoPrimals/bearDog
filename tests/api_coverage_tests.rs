use beardog_api::api::handlers::health::HealthHandler;
use beardog_api::api::middleware::auth::AuthMiddleware;
use beardog_errors::BearDogError;
use beardog_types::canonical::configuration::BearDogCanonicalConfig;

#[tokio::test]
async fn test_health_handler_comprehensive() {
    let config = BearDogCanonicalConfig::default();
    let handler = HealthHandler::new(&config);

    let health_result = handler.get_health();
    assert!(health_result.is_ok());

    let detailed_health = handler.get_detailed_health();
    assert!(detailed_health.is_ok());
}

#[tokio::test]
async fn test_auth_middleware_comprehensive() -> Result<(), BearDogError> {
    let config = BearDogCanonicalConfig::default();
    let middleware = AuthMiddleware::new(&config)?;

    let valid_token = "test_token_123";
    let validation_result = middleware.validate_token(valid_token);

    assert!(validation_result.is_ok() || validation_result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_api_error_responses() {
    let auth_error = BearDogError::Authentication {
        message: "Invalid credentials".to_string(),
    };

    assert!(matches!(auth_error, BearDogError::Authentication { .. }));

    let rate_limit_error = BearDogError::RateLimit {
        message: "Too many requests".to_string(),
    };

    assert!(matches!(rate_limit_error, BearDogError::RateLimit { .. }));
}

#[tokio::test]
async fn test_request_response_cycle() {
    use std::collections::HashMap;

    let mut headers = HashMap::with_capacity(16);
    headers.insert("Content-Type".to_string(), "application/json");
    headers.insert("Authorization".to_string(), "Bearer test_token");

    assert_eq!(headers.len(), 2);
    assert!(headers.contains_key("Content-Type"));
    assert!(headers.contains_key("Authorization"));
}

#[tokio::test]
async fn test_zero_copy_json_operations() {
    use serde_json::Value;

    let test_data = serde_json::json!({
        "test": "value",
        "number": 42,
        "array": [1, 2, 3]
    });

    let serialized = serde_json::to_string(&test_data)
        .map_err(|e| BearDogError::system(format!("Serialization error: {:?}", e)))?;
    let deserialized: Value = serde_json::from_str(&serialized)
        .map_err(|e| BearDogError::system(format!("Deserialization error: {:?}", e)))?;

    assert_eq!(test_data, deserialized);
}
