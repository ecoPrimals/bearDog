// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Basic Integration Tests for BearDog API
///
/// This test suite validates basic API functionality

use beardog_api::api::{
    ai_interface::create_ai_router,
    auth::models::TokenValidationRequest,
    cache::CacheConfig,
    compliance::ComplianceStatusResponse,
    rate_limiting::{EndpointLimit, RateLimitConfig},
    security::models::ThreatIntelRequest,
};
use beardog_types::config::BearDogConfig;
use beardog_core::core::BearDogCore;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
#[tokio::test]
async fn test_api_module_structure() -> Result<(), Box<dyn std::error::Error>> {
    // Test that all API modules are accessible
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);
    let _app: axum::Router<Arc<BearDogCore>> = create_ai_router().with_state(core);
    println!("✅ AI router created successfully");
    Ok(())
}
async fn test_security_api_models() -> Result<(), Box<dyn std::error::Error>> {
    // Test security API request models with actual fields
    let threat_request = ThreatIntelRequest {
        indicators: vec!["192.168.1.100".to_string()],
        indicator_type: "ip".to_string(),
    };
    // Test serialization
    let serialized = serde_json::to_string(&threat_request)?;
    assert!(serialized.contains("192.168.1.100"));
    assert!(serialized.contains("ip"));
    // Test deserialization
    let deserialized: ThreatIntelRequest = serde_json::from_str(&serialized)?;
    assert_eq!(threat_request.indicators, deserialized.indicators);
    assert_eq!(threat_request.indicator_type, deserialized.indicator_type);
    println!("✅ Security API models serialize/deserialize correctly");
async fn test_compliance_api_models() -> Result<(), Box<dyn std::error::Error>> {
    // Test compliance API models with actual structure
    let status_response = ComplianceStatusResponse {
        overall_compliance_score: 0.95,
        status: "compliant".to_string(),
        last_assessment: chrono::Utc::now().to_string(),
        frameworks: vec![],
        pending_audits: 0,
        active_violations: 0,
        remediation_tasks: 0,
    let serialized = serde_json::to_string(&status_response)?;
    assert!(serialized.contains("compliant"));
    // Verify response structure
    assert!(status_response.overall_compliance_score > 0.9);
    assert_eq!(status_response.status, "compliant");
    assert_eq!(status_response.pending_audits, 0);
    println!("✅ Compliance API models support regulatory frameworks");}


async fn test_auth_api_models() -> Result<(), Box<dyn std::error::Error>> {
    // Test authentication API models with actual structure
    let token_validation = TokenValidationRequest {
        token: "jwt_token_example".to_string(),
    let serialized = serde_json::to_string(&token_validation)?;
    assert!(serialized.contains("jwt_token_example"));
    println!("✅ Authentication API models support secure access patterns");
async fn test_api_rate_limiting() -> Result<(), Box<dyn std::error::Error>> {
    // Test API rate limiting functionality with actual structure
    let endpoint_limit = EndpointLimit {
        requests_per_minute: 100,
        burst_capacity: 150,
        priority: true,
    let config = RateLimitConfig {
        requests_per_minute: 1000,
        burst_capacity: 1500,
        endpoint_limits: std::collections::HashMap::from([(
            "/api/v1/security/encrypt".to_string(),
            endpoint_limit,
        )]),
        user_tier_limits: std::collections::HashMap::new(),
    // Test rate limit configuration
    assert_eq!(config.requests_per_minute, 1000);
    assert_eq!(config.burst_capacity, 1500);
    assert!(!config.endpoint_limits.is_empty());
    println!("✅ Rate limiting configuration structure validated");}


async fn test_api_caching() -> Result<(), Box<dyn std::error::Error>> {
    // Test API response caching with actual structure
    let cache_config = CacheConfig {
        max_entries: 1000,
        default_ttl: Duration::from_secs(300),
        compression_enabled: true,
        cleanup_interval: Duration::from_secs(600),
    // Test cache configuration
    assert_eq!(cache_config.max_entries, 1000);
    assert_eq!(cache_config.default_ttl, Duration::from_secs(300));
    assert!(cache_config.compression_enabled);
    println!("✅ API caching configuration structure validated");
async fn test_api_model_serialization() -> Result<(), Box<dyn std::error::Error>> {
    // Test various API model serialization
    let models = vec![
        json!({"type": "threat_intel", "query": "test"}),
        json!({"type": "compliance", "framework": "GDPR"}),
        json!({"type": "auth", "token": "example"}),
    ];
    for model in models {
        let serialized = serde_json::to_string(&model)?;
        let deserialized: serde_json::Value = serde_json::from_str(&serialized)?;
        assert_eq!(model, deserialized);
    }
    println!("✅ API model serialization working correctly");
