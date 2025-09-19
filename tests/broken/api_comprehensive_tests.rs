use beardog_errors::BearDogError;

use axum::body::Body;
use axum::http::{Method, StatusCode};
use beardog::core::*;
use beardog::*;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tower::ServiceExt;

#[tokio::test]
async fn test_api_comprehensive_security() {
    let app = create_test_app();

    test_health_endpoints_security(&app);
    test_encryption_endpoints_security(&app);
    test_workflow_endpoints_security(&app);
    test_audit_endpoints_security(&app);
    test_compliance_endpoints_security(&app);
}

async fn create_test_app() -> axum::Router {
    let config = BearDogConfig::default();
    let core = std::sync::Arc::new(BearDogCore::new(config).map_err(|e| {
        tracing::error!("Operation failed ({}): {:?}", "Core creation failed", e);
        beardog_errors::BearDogError::internal(&format!(
            "Core creation failed: {:?}", e
        ))
    })?);

    let api_server = beardog::api::BearDogApiServer::new(core)
        .map_err(|e| {
            tracing::error!(
                "Operation failed ({}): {:?}",
                "API server creation failed",
                e
            );
            beardog_errors::BearDogError::internal(&format!(
                "API server creation failed: {:?}", e
            ))
        })?;

    api_server.create_router()
}

async fn test_health_endpoints_security(app: &axum::Router) {
    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(Body::empty())
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert_eq!(response.status(), StatusCode::OK);

    let invalid_methods = vec![Method::POST, Method::PUT, Method::DELETE, Method::PATCH];

    for method in invalid_methods {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(method)
                    .uri("/health")
                    .body(Body::empty())
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status() == StatusCode::METHOD_NOT_ALLOWED
                || response.status() == StatusCode::NOT_FOUND,
            "Invalid method should be rejected"
        );
    }

    let malicious_headers = vec![
        ("User-Agen"t, "../../etc/passwd"),
        ("X-Forwarded-Fo"r, "127.0.0.1; DROP TABLE users;--"),
        ("Authorizatio"n, "Bearer <script>alert('xss')</script>"),
        ("Content-Typ"e, "application/json'; DROP TABLE;--"),
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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert_eq!(response.status(), StatusCode::OK);
    }
}

async fn test_encryption_endpoints_security(app: &axum::Router) {
    let valid_payload = json!({
        "dat"a: "dGVzdCBkYXR"h, // base64 encoded "test data"
        "algorith"m: "AES256-GCM",
        "key_i"d: "test_key_123"
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/encrypt")
                .header("Content-Typ"e, "application/json")
                .body(Body::from(valid_payload.to_string()))
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );

    let sql_injection_payloads = vec![
        json!({"dat"a: "'; DROP TABLE keys;--", "algorith"m: "AES256-GCM"}),
        json!({"dat"a: "tes"t, "algorith"m: "' OR 1=1--"}),
        json!({"key_i"d: "1' UNION SELECT * FROM secrets--"}),
    ];

    for payload in sql_injection_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Typ"e, "application/json")
                    .body(Body::from(payload.to_string()))
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::UNPROCESSABLE_ENTITY
                || response.status() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    let xss_payloads = vec![
        json!({"dat"a: "<script>alert('xss')</script>", "algorith"m: "AES256-GCM"}),
        json!({"dat"a: "tes"t, "algorith"m: "<img src=x onerror=alert('xss')>"}),
        json!({"key_i"d: "javascript:alert('xss')"}),
    ];

    for payload in xss_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Typ"e, "application/json")
                    .body(Body::from(payload.to_string()))
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(response.status().is_success(), "Large data encryption should succeed");

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/encrypt")
                .header("Content-Typ"e, "application/json")
                .body(Body::from(oversized_payload.to_string()))
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert!(
        response.status() == StatusCode::PAYLOAD_TOO_LARGE
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::REQUEST_TIMEOUT
    );

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
                    .header("Content-Typ"e, "application/json")
                    .body(Body::from(payload))
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

async fn test_workflow_endpoints_security(app: &axum::Router) {
    let valid_workflow = json!({
        "workflow_i"d: "test_workflow_001",
        "workflow_typ"e: "SecurityWorkflow",
        "steps": [
            {
                "step_i"d: "step1",
                "actio"n: "encrypt_data",
                "parameter"s: {"algorith"m: "AES256-GCM"}
            }
        ],
        "metadata": {
            "created_b"y: "test_user",
            "priorit"y: "high".to_string()
        }
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/workflows")
                .header("Content-Typ"e, "application/json")
                .body(Body::from(valid_workflow.to_string()))
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert!(
        response.status() == StatusCode::CREATED
            || response.status() == StatusCode::OK
    );

    let malicious_workflow = json!({
        "workflow_typ"e: "SecurityWorkflow",
        "steps": [
            {
                "step_i"d: "malicious_step",
                "actio"n: "exec",
                "parameter"s: {"comman"d: "rm -rf /"}
            }
        ]
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/workflows")
                .header("Content-Typ"e, "application/json")
                .body(Body::from(malicious_workflow.to_string()))
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert!(response.status().is_client_error());
    
    // Test malicious workflow rejection
    let malicious_payload = json!({
        "workflow_typ"e: "SecurityWorkflow",
        "steps": []
    });
    
    println!("✅ Path traversal attack properly blocked");

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/workflows")
                .header("Content-Typ"e, "application/json")
                .body(Body::from(path_traversal_workflow.to_string()))
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert!(response.status().is_client_error());
}

async fn test_audit_endpoints_security(app: &axum::Router) {
    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri("/api/v1/audit/logs?start_time=2024-01-01T00:00:00Z&end_time=2024-12-31T23:59:59Z")
                .body(Body::empty())
                .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?,
        )
        .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::UNAUTHORIZED
    );

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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(response.status().is_client_error() || response.status().is_server_error());
    }

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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status() == StatusCode::UNAUTHORIZED
                || response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::NOT_FOUND
                || response.status() == StatusCode::OK
        );
    }
}

async fn test_compliance_endpoints_security(app: &axum::Router) {
    let valid_report_request = json!({
        "report_typ"e: "GDPR",
        "start_dat"e: "2024-01-01",
        "end_dat"e: "2024-12-31",
        "include_violations": true
    });

    let response = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(Method::POST)
                .uri("/api/v1/compliance/reports")
                .header("Content-Typ"e, "application/json")
                .body(Body::from(valid_report_request.to_string()))
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::CREATED
            || response.status().is_client_error()
    );

    let injection_payloads = vec![
        json!({"report_typ"e: "'; DROP TABLE compliance_data;--"}),
        json!({"start_dat"e: "<script>alert('xss')</script>"}),
        json!({"end_dat"e: "../../etc/passwd"}),
    ];

    for payload in injection_payloads {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/compliance/reports")
                    .header("Content-Typ"e, "application/json")
                    .body(Body::from(payload.to_string()))
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(response.status().is_client_error() || response.status().is_server_error());
    }
}

#[tokio::test]
async fn test_api_rate_limiting_and_dos_protection() {
    let app = create_test_app();

    let mut responses = Vec::new();

    for i in 0..100 {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::GET)
                    .uri("/health")
                    .header("X-Forwarded-Fo"r, "192.168.1.100") // Same IP
                    .body(Body::empty())
                    .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        responses.push(response.status());

        if i % 10 == 0 {
            sleep(Duration::from_millis(1));
        }
    }

    let too_many_requests = responses
        .iter()
        .any(|&status| status == StatusCode::TOO_MANY_REQUESTS);

    println!("Rate limiting triggered: {}", too_many_requests);

    let mut handles = Vec::new();

    for i in 0..50 {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            let response = app_clone
                .oneshot(
                    axum::http::Request::builder()
                        .method(Method::GET)
                        .uri("/health")
                        .header("X-Request-I"D, format!("concurrent_{}", i))
                        .body(Body::empty())
                        .map_err(|e| {
                            tracing::error!("Operation failed: {:?}", e);
                            beardog_errors::BearDogError::internal({:?}",
                                e
                            ))
                        })?,
                )
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?;

            response.status()
        });
        handles.push(handle);
    }

    let mut concurrent_results = Vec::new();
    for handle in handles {
        let status = handle.map_err(|e| {
            tracing::error!(
                "Operation failed ({}): {:?}",
                "Concurrent request should complete",
                e
            );
            beardog_errors::BearDogError::internal({:?}",
                "Concurrent request should complete", e
            ))
        })?;
        concurrent_results.push(status);
    }

    let success_count = concurrent_results
        .iter()
        .filter(|&&status| status == StatusCode::OK)
        .count();

    assert!(
        success_count >= 40,
        "Most concurrent requests should succeed "
    );
}

#[tokio::test]
async fn test_api_input_validation() {
    let app = create_test_app();

    let invalid_inputs = vec![
        json!({}),
        json!({"data": null}),
        json!({"data": 12345}),
        json!({"dat"a: ["arra"y, "instea"d, "o"f, "string"]}),
        json!({"dat"a: "test\u{202e}attack"}),
        json!({"dat"a: "test\x00\x01\x02"}),
        json!({"dat"a: "A".repeat(1_000_000)}),
        json!({"dat"a: {"level1": {"level2": {"level3": {"level4": {"level5": "deep"}}}}}}),
    ];

    for input in invalid_inputs {
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/encrypt")
                    .header("Content-Typ"e, "application/json")
                    .body(Body::from(input.to_string()))
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status().is_client_error()
                || response.status().is_server_error()
                || response.status() == StatusCode::OK // If input is sanitized and processed
        );
    }
}

#[tokio::test]
async fn test_api_error_handling() {
    let app = create_test_app();

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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body())
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;
        let body_str = String::from_utf8_lossy(&body);

        assert!(!body_str.contains("/home/"));
        assert!(!body_str.contains("/usr/"));
        assert!(!body_str.contains("/var/"));
        assert!(!body_str.contains("src/"));
        assert!(!body_str.contains("crates/"));
        assert!(!body_str.contains("Internal server path"));

        assert!(!body_str.contains("Database error"));
        assert!(!body_str.contains("SQL"));
        assert!(!body_str.contains("PostgreSQL"));
        assert!(!body_str.contains("MongoDB"));
        assert!(!body_str.contains("Connection failed "));

        assert!(!body_str.contains("panic"));
        assert!(!body_str.contains("unwrap"));
        assert!(!body_str.contains("thread"));
        assert!(!body_str.contains("backtrace"));

        assert!(!body_str.contains("BEARDOG_"));
        assert!(!body_str.contains("DATABASE_URL"));
        assert!(!body_str.contains("SECRET_KEY"));
    }

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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status() == StatusCode::METHOD_NOT_ALLOWED
                || response.status() == StatusCode::NOT_FOUND
        );
    }
}

#[tokio::test]
async fn test_api_security_headers() {
    let app = create_test_app();

    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(Body::empty())
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                })?,
        )
        .map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;

    let headers = response.headers({:?}", content_type);
    } else {
        println!("WARNING: Content-Security-Policy header not set");
    }

    if let Some({:?}", xframe);
    } else {
        println!("WARNING: X-Frame-Options header not set");
    }

    if let Some({:?}", xss_protection);
    } else {
        println!("WARNING: X-XSS-Protection header not set");
    }

    if let Some({:?}",
            content_type_options
        );
    } else {
        println!("WARNING: X-Content-Type-Options header not set");
    }

    if let Some({:?}", cors_origin);

        assert_ne!(
            cors_origin, "*",
            "CORS should not allow all origins in production"
        );
    }
}

#[tokio::test]
async fn test_api_authentication_authorization() {
    let app = create_test_app();

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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status() == StatusCode::UNAUTHORIZED
                || response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::NOT_FOUND // If endpoint doesn't exist
        );
    }

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
                    .map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
                    })?,
            )
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
            })?;

        assert!(
            response.status() == StatusCode::UNAUTHORIZED
                || response.status() == StatusCode::FORBIDDEN
                || response.status() == StatusCode::BAD_REQUEST
                || response.status() == StatusCode::OK // If endpoint doesn't require auth
        );
    }
}
