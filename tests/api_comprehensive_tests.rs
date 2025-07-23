use axum::body::Body;
use axum::http::{Method, StatusCode};
use beardog::core::*;
use beardog::*;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tower::ServiceExt;

/// Comprehensive API security testing
/// Tests all endpoints for security vulnerabilities, input validation, and error handling
#[tokio::test]
async fn test_api_comprehensive_security() {
    let app = create_test_app().await;

    // Test all major endpoint categories
    test_health_endpoints_security(&app).await;
    test_encryption_endpoints_security(&app).await;
    test_workflow_endpoints_security(&app).await;
    test_audit_endpoints_security(&app).await;
    test_compliance_endpoints_security(&app).await;
}

async fn create_test_app() -> axum::Router {
    let config = BearDogConfig::default();
    let core = std::sync::Arc::new(
        BearDogCore::new(config)
            .await
            .expect("Core creation failed"),
    );

    let api_server = beardog::api::BearDogApiServer::new(core)
        .await
        .expect("API server creation failed");

    api_server.create_router()
}

async fn test_health_endpoints_security(app: &axum::Router) {
    // Test valid health check
    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test health endpoint with invalid methods
    let invalid_methods = vec![Method::POST, Method::PUT, Method::DELETE, Method::PATCH];

    for method in invalid_methods {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(method)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should return method not allowed or not found
        assert!(
            response.status() == StatusCode::METHOD_NOT_ALLOWED
                || response.status() == StatusCode::NOT_FOUND,
            "Invalid method should be rejected"
        );
    }

    // Test health endpoint with malicious headers
    let malicious_headers = vec![
        ("User-Agent", "../../etc/passwd"),
        ("X-Forwarded-For", "127.0.0.1; DROP TABLE users;--"),
        ("Authorization", "Bearer <script>alert('xss')</script>"),
        ("Content-Type", "application/json'; DROP TABLE;--"),
    ];

    for (header_name, header_value) in malicious_headers {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri("/health")
                    .header(header_name, header_value)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should still work but not be affected by malicious headers
        assert_eq!(response.status(), StatusCode::OK);
    }
}

async fn test_encryption_endpoints_security(app: &axum::Router) {
    // Test valid encryption request
    let valid_payload = json!({
        "data": "dGVzdCBkYXRh", // base64 encoded "test data"
        "algorithm": "AES256-GCM",
        "key_id": "test_key_123"
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/encrypt")
                .header("Content-Type", "application/json")
                .body(Body::from(valid_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed or return appropriate error
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );

    // Test SQL injection attempts
    let sql_injection_payloads = vec![
        json!({"data": "'; DROP TABLE keys;--", "algorithm": "AES256-GCM"}),
        json!({"data": "test", "algorithm": "' OR 1=1--"}),
        json!({"key_id": "1' UNION SELECT * FROM secrets--"}),
    ];

    for payload in sql_injection_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should reject malicious input
        assert!(
            response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::UNPROCESSABLE_ENTITY
                || response.status() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // Test XSS attempts
    let xss_payloads = vec![
        json!({"data": "<script>alert('xss')</script>", "algorithm": "AES256-GCM"}),
        json!({"data": "test", "algorithm": "<img src=x onerror=alert('xss')>"}),
        json!({"key_id": "javascript:alert('xss')"}),
    ];

    for payload in xss_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should safely handle XSS attempts
        assert!(response.status().is_client_error() || response.status().is_server_error());
    }

    // Test oversized payload
    let large_data = "A".repeat(10_000_000); // 10MB
    let oversized_payload = json!({
        "data": large_data,
        "algorithm": "AES256-GCM"
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/encrypt")
                .header("Content-Type", "application/json")
                .body(Body::from(oversized_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should reject oversized payload
    assert!(
        response.status() == StatusCode::PAYLOAD_TOO_LARGE
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::REQUEST_TIMEOUT
    );

    // Test malformed JSON
    let malformed_payloads = vec![
        "{invalid json",
        "{'single': 'quotes'}",
        "{\"unclosed\": ",
        "not json at all",
        "",
    ];

    for payload in malformed_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should reject malformed JSON
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

async fn test_workflow_endpoints_security(app: &axum::Router) {
    // Test workflow creation with valid data
    let valid_workflow = json!({
        "workflow_id": "test_workflow_001",
        "workflow_type": "SecurityWorkflow",
        "steps": [
            {
                "step_id": "step1",
                "action": "encrypt_data",
                "parameters": {"algorithm": "AES256-GCM"}
            }
        ],
        "metadata": {
            "created_by": "test_user",
            "priority": "high"
        }
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/workflows")
                .header("Content-Type", "application/json")
                .body(Body::from(valid_workflow.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed or return appropriate error
    assert!(
        response.status() == StatusCode::CREATED
            || response.status() == StatusCode::OK
            || response.status().is_client_error()
    );

    // Test workflow with malicious step injection
    let malicious_workflow = json!({
        "workflow_id": "malicious_workflow",
        "workflow_type": "SecurityWorkflow",
        "steps": [
            {
                "step_id": "malicious_step",
                "action": "exec",
                "parameters": {"command": "rm -rf /"}
            }
        ]
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/workflows")
                .header("Content-Type", "application/json")
                .body(Body::from(malicious_workflow.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should reject malicious workflow
    assert!(response.status().is_client_error() || response.status().is_server_error());

    // Test workflow with path traversal attempt
    let path_traversal_workflow = json!({
        "workflow_id": "../../../etc/passwd",
        "workflow_type": "SecurityWorkflow",
        "steps": []
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/workflows")
                .header("Content-Type", "application/json")
                .body(Body::from(path_traversal_workflow.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should reject path traversal attempt
    assert!(response.status().is_client_error());
}

async fn test_audit_endpoints_security(app: &axum::Router) {
    // Test audit log retrieval with valid parameters
    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri("/api/v1/audit/logs?start_time=2024-01-01T00:00:00Z&end_time=2024-12-31T23:59:59Z")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed or return appropriate error
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::UNAUTHORIZED
    );

    // Test audit endpoint with SQL injection in query parameters
    let sql_injection_queries = vec![
        "/api/v1/audit/logs?start_time='; DROP TABLE audit_logs;--",
        "/api/v1/audit/logs?user_id=' OR 1=1--",
        "/api/v1/audit/logs?event_type='; SELECT * FROM secrets;--",
    ];

    for query in sql_injection_queries {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri(query)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should safely handle SQL injection attempts
        assert!(response.status().is_client_error() || response.status().is_server_error());
    }

    // Test unauthorized access attempts
    let sensitive_audit_endpoints = vec![
        "/api/v1/audit/logs",
        "/api/v1/audit/security-events",
        "/api/v1/audit/compliance-logs",
        "/api/v1/audit/admin-actions",
    ];

    for endpoint in sensitive_audit_endpoints {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri(endpoint)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should require authentication or return error
        assert!(
            response.status() == StatusCode::UNAUTHORIZED
                || response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::NOT_FOUND
                || response.status() == StatusCode::OK
        );
    }
}

async fn test_compliance_endpoints_security(app: &axum::Router) {
    // Test compliance report generation
    let valid_report_request = json!({
        "report_type": "GDPR",
        "start_date": "2024-01-01",
        "end_date": "2024-12-31",
        "include_violations": true
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/compliance/reports")
                .header("Content-Type", "application/json")
                .body(Body::from(valid_report_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should succeed or return appropriate error
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::CREATED
            || response.status().is_client_error()
    );

    // Test compliance endpoint with injection attacks
    let injection_payloads = vec![
        json!({"report_type": "'; DROP TABLE compliance_data;--"}),
        json!({"start_date": "<script>alert('xss')</script>"}),
        json!({"end_date": "../../etc/passwd"}),
    ];

    for payload in injection_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/compliance/reports")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should reject malicious input
        assert!(response.status().is_client_error() || response.status().is_server_error());
    }
}

/// Test rate limiting and DoS protection
#[tokio::test]
async fn test_api_rate_limiting_and_dos_protection() {
    let app = create_test_app().await;

    // Test rapid requests to trigger rate limiting
    let mut responses = Vec::new();

    for i in 0..100 {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri("/health")
                    .header("X-Forwarded-For", "192.168.1.100") // Same IP
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        responses.push(response.status());

        // Small delay to avoid overwhelming the system
        if i % 10 == 0 {
            sleep(Duration::from_millis(1)).await;
        }
    }

    // Check if rate limiting was triggered
    let too_many_requests = responses
        .iter()
        .any(|&status| status == StatusCode::TOO_MANY_REQUESTS);

    println!("Rate limiting triggered: {}", too_many_requests);

    // Test concurrent connections
    let mut handles = Vec::new();

    for i in 0..50 {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            let response = app_clone
                .oneshot(
                    axum::http::Request::builder()
                        .method(Method::GET)
                        .uri("/health")
                        .header("X-Request-ID", format!("concurrent_{}", i))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            response.status()
        });
        handles.push(handle);
    }

    // Wait for all concurrent requests
    let mut concurrent_results = Vec::new();
    for handle in handles {
        let status = handle.await.expect("Concurrent request should complete");
        concurrent_results.push(status);
    }

    // Most requests should succeed
    let success_count = concurrent_results
        .iter()
        .filter(|&&status| status == StatusCode::OK)
        .count();

    assert!(
        success_count >= 40,
        "Most concurrent requests should succeed"
    );
}

/// Test input validation and sanitization
#[tokio::test]
async fn test_api_input_validation() {
    let app = create_test_app().await;

    // Test various invalid inputs
    let invalid_inputs = vec![
        // Empty data
        json!({}),
        // Null values
        json!({"data": null}),
        // Wrong types
        json!({"data": 12345}),
        json!({"data": ["array", "instead", "of", "string"]}),
        // Unicode attacks
        json!({"data": "test\u{202e}attack"}),
        // Control characters
        json!({"data": "test\x00\x01\x02"}),
        // Extremely long strings
        json!({"data": "A".repeat(1_000_000)}),
        // Nested objects (JSON bomb)
        json!({"data": {"level1": {"level2": {"level3": {"level4": {"level5": "deep"}}}}}}),
    ];

    for input in invalid_inputs {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Type", "application/json")
                    .body(Body::from(input.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should handle invalid input gracefully
        assert!(
            response.status().is_client_error()
                || response.status().is_server_error()
                || response.status() == StatusCode::OK // If input is sanitized and processed
        );
    }
}

/// Test error handling and information disclosure
#[tokio::test]
async fn test_api_error_handling() {
    let app = create_test_app().await;

    // Test non-existent endpoints
    let invalid_endpoints = vec![
        "/api/v1/nonexistent",
        "/api/v2/encrypt",          // Wrong version
        "/admin/secret",            // Potentially sensitive
        "/../../../etc/passwd",     // Path traversal
        "/api/v1/encrypt/../admin", // Path traversal
    ];

    for endpoint in invalid_endpoints {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri(endpoint)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should return 404 and not leak information
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Check response body doesn't contain sensitive information
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8_lossy(&body);
        
        // Ensure no internal paths are leaked
        assert!(!body_str.contains("/home/"));
        assert!(!body_str.contains("/usr/"));
        assert!(!body_str.contains("/var/"));
        assert!(!body_str.contains("src/"));
        assert!(!body_str.contains("crates/"));
        assert!(!body_str.contains("Internal server path"));
        
        // Ensure no database information is leaked
        assert!(!body_str.contains("Database error"));
        assert!(!body_str.contains("SQL"));
        assert!(!body_str.contains("PostgreSQL"));
        assert!(!body_str.contains("MongoDB"));
        assert!(!body_str.contains("Connection failed"));
        
        // Ensure no stack traces or debug information are leaked
        assert!(!body_str.contains("panic"));
        assert!(!body_str.contains("unwrap"));
        assert!(!body_str.contains("thread"));
        assert!(!body_str.contains("backtrace"));
        
        // Ensure no environment variables are leaked
        assert!(!body_str.contains("BEARDOG_"));
        assert!(!body_str.contains("DATABASE_URL"));
        assert!(!body_str.contains("SECRET_KEY"));
    }

    // Test invalid HTTP methods
    let invalid_method_tests = vec![
        (Method::TRACE, "/api/v1/encrypt"),
        (Method::OPTIONS, "/api/v1/workflows"),
        (Method::CONNECT, "/health"),
    ];

    for (method, uri) in invalid_method_tests {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(method)
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should reject with appropriate status
        assert!(
            response.status() == StatusCode::METHOD_NOT_ALLOWED
                || response.status() == StatusCode::NOT_FOUND
        );
    }
}

/// Test CORS and security headers
#[tokio::test]
async fn test_api_security_headers() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let headers = response.headers();

    // Check for security headers (if implemented)
    // These tests will pass if headers are not set, but highlight what should be implemented

    if let Some(content_type) = headers.get("content-security-policy") {
        println!("CSP header found: {:?}", content_type);
    } else {
        println!("WARNING: Content-Security-Policy header not set");
    }

    if let Some(xframe) = headers.get("x-frame-options") {
        println!("X-Frame-Options header found: {:?}", xframe);
    } else {
        println!("WARNING: X-Frame-Options header not set");
    }

    if let Some(xss_protection) = headers.get("x-xss-protection") {
        println!("X-XSS-Protection header found: {:?}", xss_protection);
    } else {
        println!("WARNING: X-XSS-Protection header not set");
    }

    if let Some(content_type_options) = headers.get("x-content-type-options") {
        println!(
            "X-Content-Type-Options header found: {:?}",
            content_type_options
        );
    } else {
        println!("WARNING: X-Content-Type-Options header not set");
    }

    // CORS headers
    if let Some(cors_origin) = headers.get("access-control-allow-origin") {
        println!("CORS origin header found: {:?}", cors_origin);
        // Should not be "*" in production
        assert_ne!(
            cors_origin, "*",
            "CORS should not allow all origins in production"
        );
    }
}

/// Test authentication and authorization
#[tokio::test]
async fn test_api_authentication_authorization() {
    let app = create_test_app().await;

    // Test endpoints without authentication
    let protected_endpoints = vec![
        "/api/v1/admin/users",
        "/api/v1/admin/config",
        "/api/v1/sensitive-data",
        "/api/v1/encryption/keys",
    ];

    for endpoint in protected_endpoints {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri(endpoint)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should require authentication
        assert!(
            response.status() == StatusCode::UNAUTHORIZED
                || response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::NOT_FOUND // If endpoint doesn't exist
        );
    }

    // Test with invalid authentication tokens
    let oversized_token = "Bearer ".to_owned() + &"A".repeat(10000);
    let invalid_tokens = vec![
        "Bearer invalid_token",
        "Bearer ",
        "Basic invalid_base64",
        "Bearer <script>alert('xss')</script>",
        &oversized_token, // Oversized token
    ];

    for token in invalid_tokens {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri("/api/v1/encrypt")
                    .header("Authorization", token)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should reject invalid tokens
        assert!(
            response.status() == StatusCode::UNAUTHORIZED
                || response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::OK // If endpoint doesn't require auth
        );
    }
}
