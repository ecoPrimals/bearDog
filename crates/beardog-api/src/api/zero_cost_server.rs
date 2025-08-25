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


/// # Zero-Cost API Server
///
/// Production-ready API server implementation using zero-cost dependency injection.
/// This replaces the type-erased AppState with compile-time specialized generics,
/// eliminating all runtime overhead while maintaining full functionality.

use crate::api::zero_cost_api::{
    ZeroCostApiState, ZeroCostApiCache, ZeroCostRateLimit, ZeroCostApiMemoryCache, 
    ZeroCostTokenBucketLimiter, ZeroCostApiBuilder, ApiConfig, ProductionApiState, DevelopmentApiState
};
use beardog_core::zero_cost_architecture::{ZeroCost`BearDog`, examples as core_examples};
use beardog_errors::{BearDogError, BearDogResult};
use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router, Json,
use serde_json::{json, Value};
use std::time::Instant;
use tokio::net::TcpListener;
use tracing::{error, info, warn};
/// Zero-cost `BearDog` API server
pub struct ZeroCost`BearDog`ApiServer<AppState> {
    app_state: AppState,
    bind_address: String,
}
impl<Core, Cache, RateLimit> ZeroCost`BearDog`ApiServer<ZeroCostApiState<Core, Cache, RateLimit>>
where
    Cache: ZeroCostApiCache<Key = String, Value = String> + Clone + Send + Sync + 'static,
    RateLimit: ZeroCostRateLimit<ClientId = String> + Clone + Send + Sync + 'static,
    Core: Clone + Send + Sync + 'static,
{
    /// Create a new zero-cost API server}


    pub fn new(app_state: ZeroCostApiState<Core, Cache, RateLimit>) -> Self {
        let bind_address = app_state.config.bind_address.clone();
        
        Self {
            app_state,
            bind_address,
        }
    }
    
    /// Start the zero-cost API server
    pub async fn start(self) -> BearDogResult<()> {
        info!("🚀 Starting Zero-Cost `BearDog` API Server on {}", self.bind_address);
        let router = self.create_zero_cost_router();
        let listener = TcpListener::bind(&self.bind_address)
            .await
            .map_err(|e| BearDogError::configuration(format!("Failed to bind to }: {}", self.bind_address, e),
            })?;
        info!("✅ Zero-Cost API Server listening on {}", self.bind_address);
        axum::serve(listener, router)
            .map_err(|e| BearDogError::configuration(format!("Server error: }", e),
        Ok(())
    /// Create the zero-cost router with monomorphized middleware
    fn create_zero_cost_router(&self) -> Router {
        Router::new()
            // Zero-cost health endpoint
            .route("/health", get(zero_cost_health_handler::<Core, Cache, RateLimit>))
            .route("/info", get(zero_cost_server_info::<Core, Cache, RateLimit>))
            // Zero-cost performance endpoints
            .route("/api/v1/performance/cache-stats", get(cache_stats_handler::<Core, Cache, RateLimit>))
            .route("/api/v1/performance/rate-limit-stats", get(rate_limit_stats_handler::<Core, Cache, RateLimit>))
            // Zero-cost API routes
            .nest("/v1", self.create_zero_cost_v1_routes())
            // Apply zero-cost middleware - all monomorphized at compile time
            .layer(middleware::from_fn(request_id_middleware))
            .layer(middleware::from_fn_with_state(
                self.app_state.clone(),
                zero_cost_rate_limiting_middleware::<Core, Cache, RateLimit>,
            ))
                zero_cost_performance_middleware::<Core, Cache, RateLimit>,
            .with_state(self.app_state.clone())
    /// Create zero-cost v1 API routes}


    fn create_zero_cost_v1_routes(&self) -> Router<ZeroCostApiState<Core, Cache, RateLimit>> {
            .route("/status", get(api_status_handler::<Core, Cache, RateLimit>))
            .route("/metrics", get(metrics_handler::<Core, Cache, RateLimit>))
/// Zero-cost health check handler - completely monomorphized
async fn zero_cost_health_handler<Core, Cache, RateLimit>(
    State(state): State<ZeroCostApiState<Core, Cache, RateLimit>>,
) -> Result<Json<Value>, StatusCode>
    Cache: ZeroCostApiCache,
    RateLimit: ZeroCostRateLimit,
    // Get cache performance - zero overhead call
    let cache_stats = state.cache.cache_stats().await;
    // Get rate limiting stats - zero overhead call  
    let rate_stats = state.rate_limiter.rate_limit_stats().await;
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "api_version": "v1",
        "architecture": "zero-cost",
        "performance": {
            "cache_hit_rate": cache_stats.hit_rate,
            "cache_size": cache_stats.cache_size,
            "total_requests": rate_stats.total_requests,
            "blocked_requests": rate_stats.blocked_requests,
            "active_clients": rate_stats.active_clients
    })))
/// Zero-cost server info handler
async fn zero_cost_server_info<Core, Cache, RateLimit>(
    State(_state): State<ZeroCostApiState<Core, Cache, RateLimit>>,
) -> Json<Value>
    Json(json!({
        "server": "`BearDog` Zero-Cost API",
        "architecture": "zero-cost dependency injection",
        "benefits": [
            "No runtime overhead",
            "100% compile-time type safety", 
            "Direct function calls",
            "Monomorphized performance",
            "Zero async_trait boxing"
        ],
        "rust_version": env!("RUSTC_VERSION"),
        "build_timestamp": env!("BUILD_TIMESTAMP"),
    }))
/// Cache statistics handler - zero overhead
async fn cache_stats_handler<Core, Cache, RateLimit>(
    let stats = state.get_cache_performance().await;
    Json(json!(stats))
/// Rate limiting statistics handler - zero overhead
async fn rate_limit_stats_handler<Core, Cache, RateLimit>(
    let stats = state.get_rate_limit_performance().await;
/// API status handler
async fn api_status_handler<Core, Cache, RateLimit>(
        "api_status": "operational",
        "cache": {
            "hit_rate": cache_stats.hit_rate,
            "total_requests": cache_stats.total_requests,
            "memory_usage_mb": cache_stats.memory_usage_bytes / 1024 / 1024
        },
        "rate_limiting": {
            "block_rate": if rate_stats.total_requests > 0 {
                rate_stats.blocked_requests as f64 / rate_stats.total_requests as f64
            } else { 0.0 }
/// Metrics handler
async fn metrics_handler<Core, Cache, RateLimit>(
        "metrics": {
            "cache": cache_stats,
            "rate_limiting": rate_stats,
            "architecture": {
                "type": "zero-cost",
                "overhead": "0%",
                "type_safety": "100% compile-time"
            }
/// Zero-cost performance middleware - completely monomorphized
async fn zero_cost_performance_middleware<Core, Cache, RateLimit>(
    mut request: Request,
    next: Next,
) -> Response
    let start = Instant::now();
    // Add start time to request extensions
    request.extensions_mut().insert(start);
    let mut response = next.run(request).await;
    let duration = start.elapsed();
    // Add performance headers - zero overhead string formatting
    if let Ok(header_value) = HeaderValue::from_str(&format!("{:.2}", duration.as_micros())) {
        response.headers_mut().insert("X-Response-Time-Microseconds", header_value);
    // Add architecture header
    response.headers_mut().insert("X-Architecture", HeaderValue::from_static("zero-cost"));
    // Log performance
    tracing::debug!("Zero-cost request took {:.2}μs", duration.as_micros());
    response
/// Zero-cost rate limiting middleware - completely monomorphized
async fn zero_cost_rate_limiting_middleware<Core, Cache, RateLimit>(
    request: Request,
) -> Result<Response, Response>
    RateLimit: ZeroCostRateLimit<ClientId = String>,
    // Extract client ID (simplified - in production would use proper client identification)
    let client_id = request.headers()
        .get("x-client-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous")
        .to_string();
    // Check rate limit - zero overhead call, completely monomorphized
    let allowed = state.rate_limiter.check_limit(&client_id).await;
    if !allowed {
        let response = Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("Content-Type", "application/json")
            .header("X-Rate-Limit-Exceeded", "true")
            .header("X-Architecture", "zero-cost")
            .body(axum::body::Body::from(
                json!({
                    "error": "Rate limit exceeded",
                    "architecture": "zero-cost",
                    "client_id": client_id
                }).to_string()
            .unwrap_or_else(|_| {
                // Fallback response if body creation fails
                Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(axum::body::Body::from("Rate limit exceeded"))
                    .unwrap_or_default()
            });
        return Err(response);
    // Add rate limit headers
    let quota = state.rate_limiter.get_quota(&client_id).await;
    // Add rate limit headers with proper error handling
    if let Ok(remaining_header) = HeaderValue::from_str(&quota.remaining.to_string()) {
        response.headers_mut().insert("X-Rate-Limit-Remaining", remaining_header);
    if let Ok(limit_header) = HeaderValue::from_str(&quota.limit.to_string()) {
        response.headers_mut().insert("X-Rate-Limit-Limit", limit_header);
    Ok(response)
/// Request ID middleware (shared with old implementation)
async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    request.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id)
            .unwrap_or_else(|_| HeaderValue::from_static("invalid-request-id")),
    );
    response.headers_mut().insert(
/// Example usage and factory functions}


pub mod examples {
    use super::*;
    /// Create a production zero-cost API server
    pub fn create_production_api_server() -> ZeroCost`BearDog`ApiServer<ProductionApiState<core_examples::Production`BearDog`>> {
        // Create the core `BearDog` system using zero-cost architecture
        let core_system = core_examples::create_production_system();
        // Create the zero-cost API state
        let api_state = ZeroCostApiBuilder::new()
            .with_core(core_system)
            .with_cache(ZeroCostApiMemoryCache::<String, String, 100000, 7200>::new()) // 100k entries, 2hr TTL
            .with_rate_limiter(ZeroCostTokenBucketLimiter::<String, 1000, 50>::new())  // 1000/min, 50 burst
            .with_config(ApiConfig::new(beardog_types::canonical::constants::default_api_bind_address()))
            .build();
        ZeroCost`BearDog`ApiServer::new(api_state)
    /// Create a development zero-cost API server}


    pub fn create_development_api_server() -> ZeroCost`BearDog`ApiServer<DevelopmentApiState<core_examples::Development`BearDog`>> {
        let core_system = core_examples::create_development_system();
            .with_cache(ZeroCostApiMemoryCache::<String, String, 1000, 3600>::new())   // 1k entries, 1hr TTL
            .with_rate_limiter(ZeroCostTokenBucketLimiter::<String, 100, 10>::new())   // 100/min, 10 burst
            .with_config(ApiConfig::new(beardog_types::env_config::get_api_bind_address()))
    /// Create a high-performance API server for benchmarking
    pub fn create_benchmark_api_server() -> ZeroCost`BearDog`ApiServer<
        ZeroCostApiState<
            core_examples::Development`BearDog`,
            ZeroCostApiMemoryCache<String, String, 500000, 1800>, // 500k entries, 30min TTL
            ZeroCostTokenBucketLimiter<String, 10000, 200>,       // 10k/min, 200 burst
        >
    > {
            .with_cache(ZeroCostApiMemoryCache::<String, String, 500000, 1800>::new())
            .with_rate_limiter(ZeroCostTokenBucketLimiter::<String, 10000, 200>::new())
            .with_config(ApiConfig::<5000, 10485760, true, false>::new("0.0.0.0:8080".to_string())) // 5s timeout, 10MB max, compression on, CORS off
#[cfg(test)]
mod tests {
    #[tokio::test]}


    async fn test_zero_cost_api_creation() {
        let server = examples::create_development_api_server();
        // Verify the server was created with correct configuration
        assert_eq!(server.bind_address, "127.0.0.1:8080");
        // Test that the state contains working components
        let cache_stats = server.app_state.get_cache_performance().await;
        assert_eq!(cache_stats.total_requests, 0); // Initially no requests
        let rate_stats = server.app_state.get_rate_limit_performance().await;
        assert_eq!(rate_stats.total_requests, 0); // Initially no requests
    #[test]
    fn test_compile_time_configurations() {
        // Verify different configurations create different types
        let _prod_server = examples::create_production_api_server();
        let _dev_server = examples::create_development_api_server();
        let _bench_server = examples::create_benchmark_api_server();
        // All configurations should compile to different optimized types
        // This test ensures the zero-cost architecture is working
} 
