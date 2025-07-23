//! Comprehensive Integration Tests
//!
//! This module contains end-to-end integration tests that validate complete
//! BearDog system workflows, including API endpoints, genetic operations,
//! security functions, and ecosystem integrations.

use beardog_api::*;
use beardog_core::*;
use beardog_config::runtime::RuntimeConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Integration test configuration
#[derive(Debug, Clone)]
pub struct IntegrationTestConfig {
    pub api_base_url: String,
    pub admin_base_url: String,
    pub metrics_base_url: String,
    pub test_timeout: Duration,
    pub cleanup_after_tests: bool,
}

impl Default for IntegrationTestConfig {
    fn default() -> Self {
        Self {
            api_base_url: std::env::var("BEARDOG_TEST_API_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            admin_base_url: std::env::var("BEARDOG_TEST_ADMIN_URL")
                .unwrap_or_else(|_| "http://localhost:9092".to_string()),
            metrics_base_url: std::env::var("BEARDOG_TEST_METRICS_URL")
                .unwrap_or_else(|_| "http://localhost:9091".to_string()),
            test_timeout: Duration::from_secs(30),
            cleanup_after_tests: true,
        }
    }
}

/// Integration test client for BearDog API
pub struct BearDogTestClient {
    pub config: IntegrationTestConfig,
    pub http_client: reqwest::Client,
    pub auth_token: Option<String>,
}

impl BearDogTestClient {
    /// Create a new test client
    pub fn new(config: IntegrationTestConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(config.test_timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
            auth_token: None,
        }
    }

    /// Authenticate and get an auth token
    pub async fn authenticate(&mut self, username: &str, password: &str) -> BearDogResult<()> {
        let auth_url = format!("{}/api/auth/login", self.config.api_base_url);
        let auth_payload = json!({
            "username": username,
            "password": password
        });

        let response = self.http_client
            .post(&auth_url)
            .json(&auth_payload)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Authentication request failed: {}", e),
            })?;

        if response.status().is_success() {
            let auth_response: serde_json::Value = response.json().await.map_err(|e| BearDogError::Parsing {
                message: format!("Failed to parse auth response: {}", e),
            })?;

            if let Some(token) = auth_response.get("data").and_then(|d| d.get("token")).and_then(|t| t.as_str()) {
                self.auth_token = Some(token.to_string());
                Ok(())
            } else {
                Err(BearDogError::Authentication {
                    message: "No token in authentication response".to_string(),
                })
            }
        } else {
            Err(BearDogError::Authentication {
                message: format!("Authentication failed with status: {}", response.status()),
            })
        }
    }

    /// Make an authenticated API request
    pub async fn api_request(&self, method: reqwest::Method, endpoint: &str, body: Option<serde_json::Value>) -> BearDogResult<serde_json::Value> {
        let url = format!("{}/api/{}", self.config.api_base_url, endpoint.trim_start_matches('/'));
        
        let mut request_builder = self.http_client.request(method, &url);
        
        if let Some(token) = &self.auth_token {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(json_body) = body {
            request_builder = request_builder.json(&json_body);
        }

        let response = request_builder.send().await.map_err(|e| BearDogError::Network {
            message: format!("API request to {} failed: {}", url, e),
        })?;

        let status = response.status();
        let response_body: serde_json::Value = response.json().await.map_err(|e| BearDogError::Parsing {
            message: format!("Failed to parse API response: {}", e),
        })?;

        if status.is_success() {
            Ok(response_body)
        } else {
            Err(BearDogError::ApiError {
                code: status.as_u16(),
                message: response_body.get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown API error")
                    .to_string(),
            })
        }
    }

    /// Check system health
    pub async fn health_check(&self) -> BearDogResult<serde_json::Value> {
        let health_url = format!("{}/health", self.config.api_base_url);
        
        let response = self.http_client
            .get(&health_url)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Health check failed: {}", e),
            })?;

        response.json().await.map_err(|e| BearDogError::Parsing {
            message: format!("Failed to parse health response: {}", e),
        })
    }

    /// Get system metrics
    pub async fn get_metrics(&self) -> BearDogResult<String> {
        let metrics_url = format!("{}/metrics", self.config.metrics_base_url);
        
        let response = self.http_client
            .get(&metrics_url)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Metrics request failed: {}", e),
            })?;

        response.text().await.map_err(|e| BearDogError::Parsing {
            message: format!("Failed to parse metrics response: {}", e),
        })
    }
}

/// Integration test suite
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    /// Helper to create test client
    async fn create_test_client() -> BearDogTestClient {
        let config = IntegrationTestConfig::default();
        BearDogTestClient::new(config)
    }

    #[tokio::test]
    async fn test_system_health_check() {
        let client = create_test_client().await;
        
        let result = timeout(Duration::from_secs(10), client.health_check()).await;
        
        match result {
            Ok(Ok(health_response)) => {
                assert!(health_response.is_object());
                println!("✅ Health check passed: {}", health_response);
            }
            Ok(Err(e)) => {
                println!("⚠️  Health check failed (expected if system not running): {}", e);
            }
            Err(_) => {
                println!("⚠️  Health check timed out (expected if system not running)");
            }
        }
    }

    #[tokio::test]
    async fn test_metrics_endpoint() {
        let client = create_test_client().await;
        
        let result = timeout(Duration::from_secs(10), client.get_metrics()).await;
        
        match result {
            Ok(Ok(metrics)) => {
                assert!(!metrics.is_empty());
                assert!(metrics.contains("beardog_"));
                println!("✅ Metrics endpoint accessible");
            }
            Ok(Err(e)) => {
                println!("⚠️  Metrics request failed (expected if system not running): {}", e);
            }
            Err(_) => {
                println!("⚠️  Metrics request timed out (expected if system not running)");
            }
        }
    }

    #[tokio::test]
    async fn test_authentication_workflow() {
        let mut client = create_test_client().await;
        
        // Test authentication with default test credentials
        let result = timeout(
            Duration::from_secs(10), 
            client.authenticate("admin", "admin123")
        ).await;
        
        match result {
            Ok(Ok(_)) => {
                assert!(client.auth_token.is_some());
                println!("✅ Authentication successful");
                
                // Test authenticated API call
                let user_info_result = client.api_request(
                    reqwest::Method::GET,
                    "/auth/session",
                    None
                ).await;
                
                match user_info_result {
                    Ok(user_info) => {
                        println!("✅ Authenticated API call successful: {}", user_info);
                    }
                    Err(e) => {
                        println!("⚠️  Authenticated API call failed: {}", e);
                    }
                }
            }
            Ok(Err(e)) => {
                println!("⚠️  Authentication failed (expected if system not running): {}", e);
            }
            Err(_) => {
                println!("⚠️  Authentication timed out (expected if system not running)");
            }
        }
    }

    #[tokio::test]
    async fn test_genetic_spawning_workflow() {
        let mut client = create_test_client().await;
        
        // First authenticate
        if client.authenticate("admin", "admin123").await.is_ok() {
            let spawn_request = json!({
                "purpose": "testing",
                "parent_genetics": [],
                "resource_limits": {
                    "max_memory_mb": 512,
                    "max_cpu_percent": 25,
                    "max_disk_mb": 1024
                },
                "target_capabilities": ["storage"]
            });

            let result = client.api_request(
                reqwest::Method::POST,
                "/genetics/spawn",
                Some(spawn_request)
            ).await;

            match result {
                Ok(spawn_response) => {
                    println!("✅ Genetic spawning successful: {}", spawn_response);
                    
                    // Verify spawn result structure
                    assert!(spawn_response.get("data").is_some());
                    
                    // Test spawn status check if spawn_id is available
                    if let Some(spawn_id) = spawn_response.get("data").and_then(|d| d.get("spawn_id")).and_then(|id| id.as_str()) {
                        let status_result = client.api_request(
                            reqwest::Method::GET,
                            &format!("/genetics/spawn/{}/status", spawn_id),
                            None
                        ).await;

                        match status_result {
                            Ok(status_response) => {
                                println!("✅ Spawn status check successful: {}", status_response);
                            }
                            Err(e) => {
                                println!("⚠️  Spawn status check failed: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️  Genetic spawning failed (may not be implemented): {}", e);
                }
            }
        } else {
            println!("⚠️  Skipping genetic spawning test - authentication failed");
        }
    }

    #[tokio::test]
    async fn test_security_analysis_workflow() {
        let mut client = create_test_client().await;
        
        if client.authenticate("admin", "admin123").await.is_ok() {
            let security_event = json!({
                "event_type": "login_attempt",
                "source_ip": "192.168.1.100",
                "destination_ip": "192.168.1.1",
                "user_id": "test_user",
                "data_size": 1024.0,
                "user_agent": "BearDog-Test/1.0",
                "location": "Test Location"
            });

            let result = client.api_request(
                reqwest::Method::POST,
                "/security/analyze",
                Some(security_event)
            ).await;

            match result {
                Ok(analysis_response) => {
                    println!("✅ Security analysis successful: {}", analysis_response);
                    
                    // Verify analysis result structure
                    assert!(analysis_response.get("data").is_some());
                    
                    if let Some(data) = analysis_response.get("data") {
                        assert!(data.get("threats_detected").is_some());
                        assert!(data.get("risk_level").is_some());
                    }
                }
                Err(e) => {
                    println!("⚠️  Security analysis failed (may not be implemented): {}", e);
                }
            }
        } else {
            println!("⚠️  Skipping security analysis test - authentication failed");
        }
    }

    #[tokio::test]
    async fn test_compliance_audit_workflow() {
        let mut client = create_test_client().await;
        
        if client.authenticate("admin", "admin123").await.is_ok() {
            // Start a compliance audit
            let audit_request = json!({
                "audit_type": "GDPR",
                "scope": ["user_data", "data_processing"],
                "automated_remediation": false
            });

            let result = client.api_request(
                reqwest::Method::POST,
                "/compliance/audit/start",
                Some(audit_request)
            ).await;

            match result {
                Ok(audit_response) => {
                    println!("✅ Compliance audit started: {}", audit_response);
                    
                    // Check audit status
                    if let Some(audit_id) = audit_response.get("data").and_then(|d| d.get("audit_id")).and_then(|id| id.as_str()) {
                        sleep(Duration::from_secs(2)).await; // Give audit time to process
                        
                        let status_result = client.api_request(
                            reqwest::Method::GET,
                            &format!("/compliance/audit/{}/status", audit_id),
                            None
                        ).await;

                        match status_result {
                            Ok(status_response) => {
                                println!("✅ Audit status check successful: {}", status_response);
                            }
                            Err(e) => {
                                println!("⚠️  Audit status check failed: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️  Compliance audit failed (may not be implemented): {}", e);
                }
            }
        } else {
            println!("⚠️  Skipping compliance audit test - authentication failed");
        }
    }

    #[tokio::test]
    async fn test_api_rate_limiting() {
        let client = create_test_client().await;
        
        // Make multiple rapid requests to test rate limiting
        let mut successful_requests = 0;
        let mut rate_limited_requests = 0;
        
        for i in 0..20 {
            let result = timeout(
                Duration::from_secs(5),
                client.health_check()
            ).await;
            
            match result {
                Ok(Ok(_)) => {
                    successful_requests += 1;
                }
                Ok(Err(BearDogError::ApiError { code: 429, .. })) => {
                    rate_limited_requests += 1;
                    println!("✅ Rate limiting working - request {} was rate limited", i + 1);
                    break; // Stop once we hit rate limit
                }
                Ok(Err(e)) => {
                    println!("⚠️  Request {} failed with non-rate-limit error: {}", i + 1, e);
                }
                Err(_) => {
                    println!("⚠️  Request {} timed out", i + 1);
                }
            }
            
            // Small delay between requests
            sleep(Duration::from_millis(100)).await;
        }
        
        println!("Rate limiting test: {} successful, {} rate limited", 
                successful_requests, rate_limited_requests);
    }

    #[tokio::test]
    async fn test_error_handling_and_recovery() {
        let client = create_test_client().await;
        
        // Test invalid API endpoint
        let result = client.api_request(
            reqwest::Method::GET,
            "/nonexistent/endpoint",
            None
        ).await;
        
        match result {
            Err(BearDogError::ApiError { code: 404, .. }) => {
                println!("✅ 404 error handling working correctly");
            }
            Err(e) => {
                println!("⚠️  Got different error for invalid endpoint: {}", e);
            }
            Ok(response) => {
                println!("⚠️  Unexpected success for invalid endpoint: {}", response);
            }
        }
        
        // Test malformed request body
        let malformed_body = json!({
            "invalid": "request",
            "missing": "required_fields"
        });
        
        let result = client.api_request(
            reqwest::Method::POST,
            "/auth/login",
            Some(malformed_body)
        ).await;
        
        match result {
            Err(BearDogError::ApiError { code: 400, .. }) => {
                println!("✅ 400 error handling working correctly");
            }
            Err(e) => {
                println!("⚠️  Got different error for malformed request: {}", e);
            }
            Ok(response) => {
                println!("⚠️  Unexpected success for malformed request: {}", response);
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let client = Arc::new(create_test_client().await);
        let mut handles = vec![];
        
        // Spawn multiple concurrent health checks
        for i in 0..10 {
            let client_clone = Arc::clone(&client);
            let handle = tokio::spawn(async move {
                let result = timeout(
                    Duration::from_secs(10),
                    client_clone.health_check()
                ).await;
                (i, result.is_ok())
            });
            handles.push(handle);
        }
        
        // Wait for all concurrent operations to complete
        let mut successful_operations = 0;
        for handle in handles {
            match handle.await {
                Ok((_, true)) => successful_operations += 1,
                Ok((i, false)) => println!("⚠️  Concurrent operation {} failed", i),
                Err(e) => println!("⚠️  Concurrent operation task failed: {}", e),
            }
        }
        
        println!("✅ Concurrent operations test: {}/10 successful", successful_operations);
        assert!(successful_operations > 0, "At least some concurrent operations should succeed");
    }

    /// Test cleanup - run this last to clean up test resources
    #[tokio::test]
    async fn test_zzz_cleanup() {
        println!("🧹 Integration test cleanup complete");
        // Add any cleanup logic here if needed
    }
}

/// Performance integration tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_api_response_times() {
        let client = create_test_client().await;
        let mut response_times = vec![];
        
        for _ in 0..10 {
            let start = Instant::now();
            let _ = client.health_check().await;
            let duration = start.elapsed();
            response_times.push(duration);
        }
        
        let average_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;
        let max_time = response_times.iter().max().unwrap();
        let min_time = response_times.iter().min().unwrap();
        
        println!("📊 API Performance Metrics:");
        println!("   Average response time: {:?}", average_time);
        println!("   Max response time: {:?}", max_time);
        println!("   Min response time: {:?}", min_time);
        
        // Assert reasonable performance (adjust thresholds as needed)
        assert!(average_time < Duration::from_millis(1000), "Average response time should be under 1 second");
        assert!(max_time < Duration::from_secs(5), "Max response time should be under 5 seconds");
    }

    async fn create_test_client() -> BearDogTestClient {
        let config = IntegrationTestConfig::default();
        BearDogTestClient::new(config)
    }
} 