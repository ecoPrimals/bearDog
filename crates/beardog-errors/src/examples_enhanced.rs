// SPDX-License-Identifier: AGPL-3.0-only

// Enhanced Error Constructor Examples
// Demonstrates real-world usage of enhanced error constructors with remediation hints

use crate::{
    authentication_error_with_hint, authorization_error_with_hint, configuration_error_with_docs,
    crypto_error_with_details, network_error_with_context, validation_error_with_suggestion,
    BearDogError,
};

/// Example: Authentication with helpful hints
///
/// This shows how to use enhanced authentication errors that provide
/// actionable guidance to users when authentication fails.
///
/// # Errors
/// Returns an error demonstrating JWT authentication failure with remediation hints.
pub fn example_authentication_with_hint() -> Result<(), BearDogError> {
    // Simulate JWT verification failure
    Err(authentication_error_with_hint(
        "JWT token signature verification failed",
        "Verify the token was signed with the correct key. Check configuration: beardog.auth.jwt_secret"
    ))
}

/// Example: Authorization with role hints
///
/// This shows how to use enhanced authorization errors that tell users
/// exactly what permissions or roles they need.
///
/// # Errors
/// Returns an error demonstrating authorization failure with required permissions.
pub fn example_authorization_with_hint() -> Result<(), BearDogError> {
    // Simulate permission denied
    Err(authorization_error_with_hint(
        "/api/admin/users",
        "DELETE",
        "Requires 'admin' role or 'users:delete' permission. Contact your administrator to request access."
    ))
}

/// Example: Validation with suggestions
///
/// This shows how to use enhanced validation errors that provide
/// helpful suggestions for fixing invalid input.
///
/// # Errors
/// Returns an error demonstrating validation failure with correction suggestions.
pub fn example_validation_with_suggestion() -> Result<(), BearDogError> {
    // Simulate email validation failure
    Err(validation_error_with_suggestion(
        "email",
        "Must be a valid email address",
        "Use format: user@domain.com or check for typos in the domain name",
    ))
}

/// Example: Network error with full context
///
/// This shows how to use enhanced network errors that provide
/// complete context for debugging connection issues.
///
/// # Errors
/// Returns an error demonstrating network timeout with connection details.
pub fn example_network_with_context() -> Result<(), BearDogError> {
    // Simulate network timeout
    Err(network_error_with_context(
        "Connection timeout after 30s",
        "tcp://consul.service.local:8500",
        "service_discovery_init",
    ))
}

/// Example: Configuration error with documentation link
///
/// This shows how to use enhanced configuration errors that point
/// users to relevant documentation for troubleshooting.
///
/// # Errors
/// Returns an error demonstrating configuration issue with documentation link.
pub fn example_configuration_with_docs() -> Result<(), BearDogError> {
    // Simulate HSM configuration error
    Err(configuration_error_with_docs(
        "HSM provider 'yubico' not found in configuration",
        "hsm",
        "https://docs.beardog.dev/hsm/providers#supported-providers",
    ))
}

/// Example: Cryptographic error with details
///
/// This shows how to use enhanced crypto errors that provide
/// detailed information about what went wrong and how to fix it.
///
/// # Errors
/// Returns an error demonstrating cryptographic key size mismatch with resolution guidance.
pub fn example_crypto_with_details() -> Result<(), BearDogError> {
    // Simulate key size mismatch
    Err(crypto_error_with_details(
        "AES-256-GCM encryption",
        "Key size is 128 bits, expected 256 bits",
        "Ensure you're using generate_key_256() or check key derivation function",
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// REAL-WORLD USAGE PATTERNS
// ═══════════════════════════════════════════════════════════════════════════

/// Example: API endpoint with enhanced errors
///
/// This shows how an API endpoint might use enhanced errors to provide
/// better user experience when authentication or authorization fails.
pub fn api_endpoint_example(token: &str, resource_id: &str) -> Result<String, BearDogError> {
    // 1. Authenticate user
    if token.is_empty() {
        return Err(authentication_error_with_hint(
            "Missing authentication token",
            "Include 'Authorization: Bearer <token>' header or use API key authentication",
        ));
    }

    // 2. Verify token format
    if !token.starts_with("Bearer ") {
        return Err(authentication_error_with_hint(
            "Invalid token format",
            "Token must be in format: 'Bearer <jwt_token>'. Example: 'Bearer eyJhbGc...'",
        ));
    }

    // 3. Check authorization
    if resource_id.starts_with("admin_") {
        return Err(authorization_error_with_hint(
            resource_id,
            "READ",
            "This resource requires 'admin' role. Your current role is 'user'. Contact support@beardog.dev to request elevated access."
        ));
    }

    Ok("Success".to_string())
}

/// Example: Configuration loading with enhanced errors
///
/// This shows how configuration loading might use enhanced errors to guide
/// users to documentation when configuration is invalid.
pub fn load_hsm_config(config_path: &str) -> Result<String, BearDogError> {
    // Simulate configuration parsing
    if config_path.contains("invalid") {
        return Err(configuration_error_with_docs(
            "Missing required field 'provider_type' in HSM configuration",
            "hsm",
            "https://docs.beardog.dev/configuration/hsm#required-fields",
        ));
    }

    Ok("Config loaded".to_string())
}

/// Example: Network operation with enhanced errors
///
/// This shows how network operations might use enhanced errors to provide
/// full context for debugging connection issues.
pub fn discover_service(service_name: &str, consul_addr: &str) -> Result<String, BearDogError> {
    // Simulate service discovery
    if consul_addr.contains("unreachable") {
        return Err(network_error_with_context(
            "Connection refused",
            consul_addr,
            &format!("discover_service({})", service_name),
        ));
    }

    Ok("Service discovered".to_string())
}

/// Example: Cryptographic operation with enhanced errors
///
/// This shows how crypto operations might use enhanced errors to provide
/// detailed information about what went wrong.
pub fn encrypt_data(data: &[u8], key_size: usize) -> Result<Vec<u8>, BearDogError> {
    // Simulate key size validation
    if key_size != 256 {
        return Err(crypto_error_with_details(
            "AES-256-GCM encryption",
            &format!("Key size is {key_size} bits, expected 256 bits"),
            "Use HsmProvider::generate_key_256() or KeyDerivation::derive_256bit_key() to create a properly sized key"
        ));
    }

    Ok(data.to_vec())
}

/// Example: Input validation with enhanced errors
///
/// This shows how input validation might use enhanced errors to provide
/// helpful suggestions for fixing invalid input.
pub fn validate_email(email: &str) -> Result<(), BearDogError> {
    // Simulate email validation
    if !email.contains('@') {
        return Err(validation_error_with_suggestion(
            "email",
            "Must contain '@' symbol",
            "Correct format is: username@domain.com (e.g., user@example.com)",
        ));
    }

    if !email.contains('.') {
        return Err(validation_error_with_suggestion(
            "email",
            "Must contain domain extension (e.g., .com, .org)",
            "Add a domain extension like '@example.com' or '@company.org'",
        ));
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // unwrap_err() is acceptable in test code
mod tests {
    use super::*;

    #[test]
    fn test_authentication_hint_example() {
        let result = example_authentication_with_hint();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("JWT token"));
        assert!(msg.contains("💡 Hint:"));
        assert!(msg.contains("beardog.auth.jwt_secret"));
    }

    #[test]
    fn test_authorization_hint_example() {
        let result = example_authorization_with_hint();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("/api/admin/users"));
        assert!(msg.contains("DELETE"));
        assert!(msg.contains("💡 Hint:"));
        assert!(msg.contains("admin"));
    }

    #[test]
    fn test_validation_suggestion_example() {
        let result = example_validation_with_suggestion();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("email"));
        assert!(msg.contains("💡 Suggestion:"));
        assert!(msg.contains("user@domain.com"));
    }

    #[test]
    fn test_network_context_example() {
        let result = example_network_with_context();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("service_discovery_init"));
        assert!(msg.contains("🌐 Endpoint:"));
        assert!(msg.contains("consul.service.local"));
        assert!(msg.contains("💡"));
    }

    #[test]
    fn test_configuration_docs_example() {
        let result = example_configuration_with_docs();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("yubico"));
        assert!(msg.contains("📚 Documentation:"));
        assert!(msg.contains("docs.beardog.dev"));
    }

    #[test]
    fn test_crypto_details_example() {
        let result = example_crypto_with_details();
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("AES-256-GCM"));
        assert!(msg.contains("128 bits"));
        assert!(msg.contains("256 bits"));
        assert!(msg.contains("💡 Hint:"));
    }

    #[test]
    fn test_api_endpoint_missing_token() {
        let result = api_endpoint_example("", "resource_123");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Missing authentication token"));
        assert!(msg.contains("Authorization: Bearer"));
    }

    #[test]
    fn test_api_endpoint_invalid_format() {
        let result = api_endpoint_example("InvalidToken", "resource_123");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid token format"));
        assert!(msg.contains("Bearer <jwt_token>"));
    }

    #[test]
    fn test_api_endpoint_unauthorized() {
        let result = api_endpoint_example("Bearer valid_token", "admin_secret");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("admin_secret"));
        assert!(msg.contains("admin"));
        assert!(msg.contains("support@beardog.dev"));
    }

    #[test]
    fn test_load_hsm_config_invalid() {
        let result = load_hsm_config("/path/to/invalid_config.toml");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("provider_type"));
        assert!(msg.contains("docs.beardog.dev"));
    }

    #[test]
    fn test_discover_service_unreachable() {
        let result = discover_service("my-service", "tcp://unreachable:8500");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Connection refused"));
        assert!(msg.contains("unreachable:8500"));
        assert!(msg.contains("discover_service"));
    }

    #[test]
    fn test_encrypt_data_wrong_key_size() {
        let result = encrypt_data(b"test data", 128);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("128 bits"));
        assert!(msg.contains("256 bits"));
        assert!(msg.contains("generate_key_256"));
    }

    #[test]
    fn test_validate_email_missing_at() {
        let result = validate_email("invalid.email.com");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("@"));
        assert!(msg.contains("username@domain.com"));
    }

    #[test]
    fn test_validate_email_missing_extension() {
        let result = validate_email("user@domain");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("domain extension"));
        assert!(msg.contains(".com"));
    }

    #[test]
    fn test_validate_email_valid() {
        let result = validate_email("user@example.com");
        assert!(result.is_ok());
    }
}
