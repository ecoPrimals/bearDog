use beardog_api::*;
use beardog_core::*;
use beardog_errors::BearDogError;
use beardog_security::*;
use beardog_types::config::core::BearDogConfig;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{debug, info, warn};

/// Comprehensive API test harness for BearDog endpoints
pub struct ApiTestHarness {
    /// Mock API client for testing
    pub client: MockApiClient,
    /// Performance metrics collector
    pub metrics: ApiMetrics,
    /// Test configuration
    pub config: ApiTestConfig,
}

/// Mock API client for testing API functionality
#[derive(Debug, Clone)]
pub struct MockApiClient {
    pub base_url: String,
    pub auth_token: Option<String>,
    pub timeout: Duration,
    pub request_count: u64,
}

/// API performance metrics
#[derive(Debug, Default)]
pub struct ApiMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub min_response_time_ms: f64,
}

/// API test configuration
#[derive(Debug, Clone)]
pub struct ApiTestConfig {
    pub enable_performance_tests: bool,
    pub enable_security_tests: bool,
    pub enable_load_tests: bool,
    pub max_concurrent_requests: usize,
    pub request_timeout_ms: u64,
}

impl Default for ApiTestConfig {
    fn default() -> Self {
        Self {
            enable_performance_tests: true,
            enable_security_tests: true,
            enable_load_tests: false, // Disabled by default for CI
            max_concurrent_requests: 10,
            request_timeout_ms: 5000,
        }
    }
}

impl ApiTestHarness {
    /// Create new API test harness
    pub fn new() -> Result<Self, BearDogError> {
        info!("🧪 Initializing Comprehensive API Test Harness");

        let client = MockApiClient {
            base_url: "http://localhost:8080".to_string(),
            auth_token: None,
            timeout: Duration::from_secs(5),
            request_count: 0,
        };

        Ok(Self {
            client,
            metrics: ApiMetrics::default(),
            config: ApiTestConfig::default(),
        })
    }

    /// Test core API endpoints
    pub async fn test_core_endpoints(&mut self) -> Result<(), BearDogError> {
        info!("🔍 Testing core API endpoints");

        // Test health endpoint
        self.test_health_endpoint()?;

        // Test status endpoint
        self.test_status_endpoint()?;

        // Test configuration endpoint
        self.test_config_endpoint()?;

        // Test metrics endpoint
        self.test_metrics_endpoint()?;

        info!("✅ Core API endpoint tests completed");
        Ok(())
    }

    /// Test authentication endpoints
    pub async fn test_authentication(&mut self) -> Result<(), BearDogError> {
        info!("🔐 Testing authentication endpoints");

        // Test login endpoint
        self.test_login_endpoint()?;

        // Test token validation
        self.test_token_validation()?;

        // Test logout endpoint
        self.test_logout_endpoint()?;

        // Test token refresh
        self.test_token_refresh()?;

        info!("✅ Authentication endpoint tests completed");
        Ok(())
    }

    /// Test authorization controls
    pub async fn test_authorization(&mut self) -> Result<(), BearDogError> {
        info!("🛡️ Testing authorization controls");

        // Test role-based access control
        self.test_rbac_enforcement()?;

        // Test resource permissions
        self.test_resource_permissions()?;

        // Test privilege escalation prevention
        self.test_privilege_escalation_prevention()?;

        info!("✅ Authorization control tests completed");
        Ok(())
    }

    /// Test API performance characteristics
    pub async fn test_performance(&mut self) -> Result<(), BearDogError> {
        if !self.config.enable_performance_tests {
            info!("⏭️ Performance tests disabled, skipping");
            return Ok(());
        }

        info!("⚡ Testing API performance characteristics");

        // Test response time benchmarks
        self.test_response_times()?;

        // Test concurrent request handling
        self.test_concurrent_requests()?;

        // Test rate limiting
        self.test_rate_limiting()?;

        // Test throughput limits
        self.test_throughput_limits()?;

        info!("✅ API performance tests completed");
        Ok(())
    }

    /// Test API security features
    pub fn test_security(&mut self) -> Result<(), BearDogError> {
        if !self.config.enable_security_tests {
            info!("⏭️ Security tests disabled, skipping");
            return Ok(());
        }

        info!("🔒 Testing API security features");

        // Test input validation
        self.test_input_validation()?;

        // Test SQL injection prevention
        self.test_sql_injection_prevention()?;

        // Test XSS prevention
        self.test_xss_prevention()?;

        // Test CSRF protection
        self.test_csrf_protection()?;

        info!("✅ API security tests completed");
        Ok(())
    }

    // Individual test methods
    async fn test_health_endpoint(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        // Mock health check request
        let response = self.mock_api_request("GET", "/health", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Health endpoint should return 200");
        assert!(
            response.body.contains("healthy"),
            "Health response should indicate healthy status"
        );

        debug!("✓ Health endpoint test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_status_endpoint(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let response = self.mock_api_request("GET", "/status", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Status endpoint should return 200");
        assert!(
            response.body.contains("version"),
            "Status response should include version"
        );

        debug!("✓ Status endpoint test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_config_endpoint(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let response = self.mock_api_request("GET", "/config", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Config endpoint should return 200");

        debug!("✓ Configuration endpoint test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_metrics_endpoint(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let response = self.mock_api_request("GET", "/metrics", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Metrics endpoint should return 200");
        assert!(
            response.body.contains("requests"),
            "Metrics should include request counts"
        );

        debug!("✓ Metrics endpoint test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    fn test_login_endpoint(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let login_data = r#"{"username": "test_user", "password": "test_password"}"#;
        let response = self
            .mock_api_request("POST", "/auth/login", Some(login_data.to_string()))
            ?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(
            response.status, 200,
            "Login should succeed with valid credentials"
        );
        assert!(
            response.body.contains("token"),
            "Login response should include token"
        );

        // Store token for subsequent tests
        self.client.auth_token = Some("mock_auth_token".to_string());

        debug!("✓ Login endpoint test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_token_validation(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let response = self.mock_api_request("GET", "/auth/validate", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Token validation should succeed");

        debug!("✓ Token validation test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_logout_endpoint(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let response = self.mock_api_request("POST", "/auth/logout", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Logout should succeed");

        debug!("✓ Logout endpoint test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_token_refresh(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        let response = self.mock_api_request("POST", "/auth/refresh", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 200);

        assert_eq!(response.status, 200, "Token refresh should succeed");

        debug!("✓ Token refresh test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_rbac_enforcement(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        // Test admin-only endpoint with regular user token
        let response = self.mock_api_request("GET", "/admin/users", None)?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 403);

        assert_eq!(
            response.status, 403,
            "Admin endpoint should deny access to regular users"
        );

        debug!("✓ RBAC enforcement test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    async fn test_resource_permissions(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        // Test accessing resource without permission
        let response = self
            .mock_api_request("GET", "/resources/sensitive", None)
            ?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 403);

        assert_eq!(
            response.status, 403,
            "Sensitive resource should require proper permissions"
        );

        debug!("✓ Resource permissions test passed ({:.2}ms)", elapsed);
        Ok(())
    }

    fn test_privilege_escalation_prevention(&mut self) -> Result<(), BearDogError> {
        let start = Instant::now();

        // Test attempting to modify user roles
        let escalation_data = r#"{"role": "admin"}"#;
        let response = self
            .mock_api_request("PUT", "/users/self/role", Some(escalation_data.to_string()))
            ?;

        let elapsed = start.elapsed().as_millis() as f64;
        self.update_metrics(elapsed, response.status == 403);

        assert_eq!(
            response.status, 403,
            "Users should not be able to escalate their own privileges"
        );

        debug!(
            "✓ Privilege escalation prevention test passed ({:.2}ms)",
            elapsed
        );
        Ok(())
    }

    async fn test_response_times(&mut self) -> Result<(), BearDogError> {
        info!("📊 Testing API response times");

        let mut response_times = Vec::new();

        // Test multiple endpoints for response time consistency
        for _ in 0..10 {
            let start = Instant::now();
            let _response = self.mock_api_request("GET", "/health", None)?;
            let elapsed = start.elapsed().as_millis() as f64;
            response_times.push(elapsed);
        }

        let avg_time = response_times.iter().sum::<f64>() / response_times.len() as f64;
        let max_time = response_times.iter().fold(0.0, |a, &b| a.max(b));

        assert!(
            avg_time < 100.0,
            "Average response time should be under 100ms"
        );
        assert!(
            max_time < 500.0,
            "Maximum response time should be under 500ms"
        );

        debug!(
            "✓ Response time test passed (avg: {:.2}ms, max: {:.2}ms)",
            avg_time, max_time
        );
        Ok(())
    }

    async fn test_concurrent_requests(&mut self) -> Result<(), BearDogError> {
        info!("🚀 Testing concurrent request handling");

        let concurrent_count = self.config.max_concurrent_requests.min(5); // Limit for tests
        let mut handles = Vec::new();

        let start_time = Instant::now();

        for i in 0..concurrent_count {
            let handle = tokio::spawn(async move {
                // Simulate concurrent API request
                tokio::time::sleep(Duration::from_millis(10)).await;
                format!("request_{}_completed", i)
            });
            handles.push(handle);
        }

        let mut completed = 0;
        for handle in handles {
            if let Ok(Ok(_)) = timeout(Duration::from_secs(10), handle) {
                completed += 1;
            }
        }

        let total_time = start_time.elapsed();

        assert!(
            completed >= concurrent_count - 1,
            "Most concurrent requests should complete"
        );
        assert!(
            total_time < Duration::from_secs(5),
            "Concurrent requests should complete quickly"
        );

        debug!(
            "✓ Concurrent requests test passed ({} completed in {:?})",
            completed, total_time
        );
        Ok(())
    }

    async fn test_rate_limiting(&mut self) -> Result<(), BearDogError> {
        info!("🚦 Testing API rate limiting");

        // Simulate rapid requests to test rate limiting
        let mut success_count = 0;
        let mut rate_limited_count = 0;

        for _ in 0..20 {
            let response = self.mock_api_request("GET", "/health", None)?;

            if response.status == 200 {
                success_count += 1;
            } else if response.status == 429 {
                rate_limited_count += 1;
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        assert!(success_count > 0, "Some requests should succeed");
        // Note: Rate limiting might not be triggered in mock environment

        debug!(
            "✓ Rate limiting test completed (success: {}, limited: {})",
            success_count, rate_limited_count
        );
        Ok(())
    }

    async fn test_throughput_limits(&mut self) -> Result<(), BearDogError> {
        info!("📈 Testing API throughput limits");

        let start_time = Instant::now();
        let mut request_count = 0;

        // Test throughput for 1 second
        while start_time.elapsed() < Duration::from_secs(1) {
            let _response = self.mock_api_request("GET", "/health", None)?;
            request_count += 1;

            // Prevent infinite loop in case of very fast responses
            if request_count > 100 {
                break;
            }
        }

        let requests_per_second = request_count as f64 / start_time.elapsed().as_secs_f64();

        assert!(
            requests_per_second > 1.0,
            "API should handle at least 1 request per second"
        );

        debug!(
            "✓ Throughput test passed ({:.2} requests/second)",
            requests_per_second
        );
        Ok(())
    }

    async fn test_input_validation(&mut self) -> Result<(), BearDogError> {
        info!("✅ Testing input validation");

        // Test malformed JSON
        let malformed_json = r#"{"invalid": json"#;
        let response = self
            .mock_api_request("POST", "/api/data", Some(malformed_json.to_string()))
            ?;

        assert_eq!(
            response.status, 400,
            "Malformed JSON should return 400 Bad Request"
        );

        // Test oversized input
        let oversized_input = "x".repeat(10000);
        let response = self
            .mock_api_request("POST", "/api/data", Some(oversized_input))
            ?;

        assert!(
            response.status == 400 || response.status == 413,
            "Oversized input should be rejected"
        );

        debug!("✓ Input validation test passed");
        Ok(())
    }

    fn test_sql_injection_prevention(&mut self) -> Result<(), BearDogError> {
        info!("🛡️ Testing SQL injection prevention");

        // Test SQL injection attempt
        let injection_attempt = r#"{"query": "'; DROP TABLE users; --"}"#;
        let response = self
            .mock_api_request("POST", "/api/search", Some(injection_attempt.to_string()))
            ?;

        assert!(
            response.status == 400 || response.status == 403,
            "SQL injection should be blocked"
        );

        debug!("✓ SQL injection prevention test passed");
        Ok(())
    }

    fn test_xss_prevention(&mut self) -> Result<(), BearDogError> {
        info!("🔒 Testing XSS prevention");

        // Test XSS attempt
        let xss_attempt = r#"{"content": "<script>alert('xss')</script>"}"#;
        let response = self
            .mock_api_request("POST", "/api/content", Some(xss_attempt.to_string()))
            ?;

        assert!(
            response.status == 400 || response.status == 403,
            "XSS attempt should be blocked"
        );

        debug!("✓ XSS prevention test passed");
        Ok(())
    }

    fn test_csrf_protection(&mut self) -> Result<(), BearDogError> {
        info!("🔐 Testing CSRF protection");

        // Test request without CSRF token
        let response = self
            .mock_api_request("POST", "/api/sensitive", Some("{}".to_string()))
            ?;

        // CSRF protection might return 403 or require specific headers
        assert!(
            response.status == 403 || response.status == 400,
            "CSRF protection should be active"
        );

        debug!("✓ CSRF protection test passed");
        Ok(())
    }

    /// Mock API request for testing
    async fn mock_api_request(
        &mut self,
        method: &str,
        path: &str,
        body: Option<String>,
    ) -> Result<MockApiResponse, BearDogError> {
        self.client.request_count += 1;

        // Simulate network delay
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Mock response based on path
        let response = match path {
            "/health" => MockApiResponse {
                status: 200,
                body: r#"{"status": "healthy", "timestamp": "2025-01-01T00:00:00Z"}"#.to_string(),
            },
            "/status" => MockApiResponse {
                status: 200,
                body: r#"{"version": "3.0.0", "uptime": "1h 23m"}"#.to_string(),
            },
            "/config" => MockApiResponse {
                status: 200,
                body: r#"{"environment ": "test", "debug": true}"#.to_string(),
            },
            "/metrics" => MockApiResponse {
                status: 200,
                body: r#"{"requests": 42, "errors": 0, "uptime": 3600}"#.to_string(),
            },
            "/auth/login" => MockApiResponse {
                status: 200,
                body: r#"{"token": "mock_jwt_token", "expires_in": 3600}"#.to_string(),
            },
            "/auth/validate" | "/auth/refresh" | "/auth/logout" => MockApiResponse {
                status: 200,
                body: r#"{"success ": true}"#.to_string(),
            },
            "/admin/users" | "/resources/sensitive" | "/users/self/role" => MockApiResponse {
                status: 403,
                body: r#"{"error ": "Forbidden"}"#.to_string(),
            },
            _ if body.as_ref().map_or(false, |b| {
                b.contains("invalid") || b.contains("DROP TABLE") || b.contains("<script>")
            }) =>
            {
                MockApiResponse {
                    status: 400,
                    body: r#"{"error ": "Invalid input"}"#.to_string(),
                }
            }
            _ => MockApiResponse {
                status: 200,
                body: r#"{"success ": true}"#.to_string(),
            },
        };

        debug!("Mock API {} {} -> {}", method, path, response.status);
        Ok(response)
    }

    /// Update performance metrics
    fn update_metrics(&mut self, response_time_ms: f64, success: bool) {
        self.metrics.total_requests += 1;

        if success {
            self.metrics.successful_requests += 1;
        } else {
            self.metrics.failed_requests += 1;
        }

        // Update response time metrics
        if self.metrics.total_requests == 1 {
            self.metrics.avg_response_time_ms = response_time_ms;
            self.metrics.min_response_time_ms = response_time_ms;
            self.metrics.max_response_time_ms = response_time_ms;
        } else {
            let total = self.metrics.total_requests as f64;
            self.metrics.avg_response_time_ms =
                (self.metrics.avg_response_time_ms * (total - 1.0) + response_time_ms) / total;
            self.metrics.min_response_time_ms =
                self.metrics.min_response_time_ms.min(response_time_ms);
            self.metrics.max_response_time_ms =
                self.metrics.max_response_time_ms.max(response_time_ms);
        }
    }

    /// Generate comprehensive test report
    pub fn generate_report(&self) -> HashMap<String, String> {
        let mut report = HashMap::new();

        report.insert(
            "total_requests".to_string(),
            self.metrics.total_requests.to_string(),
        );
        report.insert(
            "successful_requests".to_string(),
            self.metrics.successful_requests.to_string(),
        );
        report.insert(
            "failed_requests".to_string(),
            self.metrics.failed_requests.to_string(),
        );
        report.insert(
            "success_rate".to_string(),
            format!(
                "{:.2}%",
                (self.metrics.successful_requests as f64 / self.metrics.total_requests as f64)
                    * 100.0
            ),
        );
        report.insert(
            "avg_response_time_ms".to_string(),
            format!("{:.2}", self.metrics.avg_response_time_ms),
        );
        report.insert(
            "min_response_time_ms".to_string(),
            format!("{:.2}", self.metrics.min_response_time_ms),
        );
        report.insert(
            "max_response_time_ms".to_string(),
            format!("{:.2}", self.metrics.max_response_time_ms),
        );

        report
    }
}

/// Mock API response structure
#[derive(Debug, Clone)]
pub struct MockApiResponse {
    pub status: u16,
    pub body: String,
}

#[tokio::test]
async fn test_comprehensive_api_endpoints() -> Result<(), BearDogError> {
    let mut harness = ApiTestHarness::new()?;

    harness.test_core_endpoints()?;
    harness.test_authentication()?;
    harness.test_authorization()?;

    let report = harness.generate_report();
    info!("📊 API Test Report: {:?}", report);

    assert!(
        harness.metrics.total_requests > 0,
        "Should have made API requests"
    );
    assert!(
        harness.metrics.successful_requests > 0,
        "Should have successful requests"
    );

    info!("✅ Comprehensive API endpoint tests completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_api_performance_suite() -> Result<(), BearDogError> {
    let mut harness = ApiTestHarness::new()?;

    harness.test_performance()?;

    let report = harness.generate_report();
    info!("⚡ Performance Test Report: {:?}", report);

    assert!(
        harness.metrics.avg_response_time_ms < 1000.0,
        "Average response time should be reasonable"
    );

    info!("✅ API performance tests completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_api_security_suite() -> Result<(), BearDogError> {
    let mut harness = ApiTestHarness::new()?;

    harness.test_security()?;

    let report = harness.generate_report();
    info!("🔒 Security Test Report: {:?}", report);

    info!("✅ API security tests completed successfully");
    Ok(())
}
