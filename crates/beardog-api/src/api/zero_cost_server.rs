

use crate::api::zero_cost_api::{
    ZeroCostApiState, ZeroCostApiCache, ZeroCostRateLimit, ZeroCostApiMemoryCache, 
    ZeroCostTokenBucketLimiter, ZeroCostApiBuilder, ApiConfig, ProductionApiState, DevelopmentApiState
};
use beardog_core::zero_cost_architecture::{ZeroCost`BearDog`, examples as core_examples};
use beardog_errors::BearDogError;
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

    pub fn new(app_state: ZeroCostApiState<Core, Cache, RateLimit>) -> Self {
        let bind_address = app_state.config.bind_address.clone();
        
        Self {
            app_state,
            bind_address,
        }
    }

    pub async fn start(self) -> Result<(), BearDogError> {
        info!("🚀 Starting Zero-Cost `BearDog` API Server on {}", self.bind_address);
        let router = self.create_zero_cost_router();
        let listener = TcpListener::bind(&self.bind_address)
            .await
            .map_err(|e| BearDogError::configuration(format_args!("Failed to bind to }: {}", self.bind_address, e).to_string(),
            })?;
        info!("✅ Zero-Cost API Server listening on {}", self.bind_address);
        axum::serve(listener, router)
            .map_err(|e| BearDogError::configuration(format_args!("Server error: }", e).to_string(),
        Ok(())

    fn create_zero_cost_router(&self) -> Router {
        Router::new()

            .route("/health", get(zero_cost_health_handler::<Core, Cache, RateLimit>))
            .route("/info", get(zero_cost_server_info::<Core, Cache, RateLimit>))

            .route("/api/v1/performance/cache-stats", get(cache_stats_handler::<Core, Cache, RateLimit>))
            .route("/api/v1/performance/rate-limit-stats", get(rate_limit_stats_handler::<Core, Cache, RateLimit>))

            .nest("/v1", self.create_zero_cost_v1_routes())

            .layer(middleware::from_fn(request_id_middleware))
            .layer(middleware::from_fn_with_state(
                self.app_state.clone(),
                zero_cost_rate_limiting_middleware::<Core, Cache, RateLimit>,
            ))
                zero_cost_performance_middleware::<Core, Cache, RateLimit>,
            .with_state(self.app_state.clone())

    fn create_zero_cost_v1_routes(&self) -> Router<ZeroCostApiState<Core, Cache, RateLimit>> {
            .route("/status", get(api_status_handler::<Core, Cache, RateLimit>))
            .route("/metrics", get(metrics_handler::<Core, Cache, RateLimit>))

async fn zero_cost_health_handler<Core, Cache, RateLimit>(
    State(state): State<ZeroCostApiState<Core, Cache, RateLimit>>,
) -> Result<Json<Value>, StatusCode>
    Cache: ZeroCostApiCache,
    RateLimit: ZeroCostRateLimit,

    let cache_stats = state.cache.cache_stats().await;

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

async fn cache_stats_handler<Core, Cache, RateLimit>(
    let stats = state.get_cache_performance().await;
    Json(json!(stats))

async fn rate_limit_stats_handler<Core, Cache, RateLimit>(
    let stats = state.get_rate_limit_performance().await;

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

async fn metrics_handler<Core, Cache, RateLimit>(
        "metrics": {
            "cache": cache_stats,
            "rate_limiting": rate_stats,
            "architecture": {
                "type": "zero-cost",
                "overhead": "0%",
                "type_safety": "100% compile-time"
            }

async fn zero_cost_performance_middleware<Core, Cache, RateLimit>(
    mut request: Request,
    next: Next,
) -> Response
    let start = Instant::now();

    request.extensions_mut().insert(start);
    let mut response = next.run(request).await;
    let duration = start.elapsed();

    if let Ok(header_value) = HeaderValue::from_str(&format_args!("{:.2}", duration.as_micros().to_string())) {
        response.headers_mut().insert("X-Response-Time-Microseconds", header_value);

    response.headers_mut().insert("X-Architecture", HeaderValue::from_static("zero-cost"));

    tracing::debug!("Zero-cost request took {:.2}μs", duration.as_micros());
    response

async fn zero_cost_rate_limiting_middleware<Core, Cache, RateLimit>(
    request: Request,
) -> Result<Response, Response>
    RateLimit: ZeroCostRateLimit<ClientId = String>,

    let client_id = request.headers()
        .get("x-client-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous")
        .to_string();

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

                Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(axum::body::Body::from("Rate limit exceeded"))
                    .unwrap_or_default()
            });
        return Err(response);

    let quota = state.rate_limiter.get_quota(&client_id).await;

    if let Ok(remaining_header) = HeaderValue::from_str(&quota.remaining.to_string()) {
        response.headers_mut().insert("X-Rate-Limit-Remaining", remaining_header);
    if let Ok(limit_header) = HeaderValue::from_str(&quota.limit.to_string()) {
        response.headers_mut().insert("X-Rate-Limit-Limit", limit_header);
    Ok(response)

async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    request.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id)
            .unwrap_or_else(|_| HeaderValue::from_static("invalid-request-id")),
    );
    response.headers_mut().insert(

pub mod examples {
    use super::*;

    pub fn create_production_api_server() -> ZeroCost`BearDog`ApiServer<ProductionApiState<core_examples::Production`BearDog`>> {

        let core_system = core_examples::create_production_system();

        let api_state = ZeroCostApiBuilder::new()
            .with_core(core_system)
            .with_cache(ZeroCostApiMemoryCache::<String, String, 100000, 7200>::new()) // 100k entries, 2hr TTL
            .with_rate_limiter(ZeroCostTokenBucketLimiter::<String, 1000, 50>::new())  // 1000/min, 50 burst
            .with_config(ApiConfig::new(beardog_types::canonical::constants::default_api_bind_address()))
            .build();
        ZeroCost`BearDog`ApiServer::new(api_state)

    pub fn create_development_api_server() -> ZeroCost`BearDog`ApiServer<DevelopmentApiState<core_examples::Development`BearDog`>> {
        let core_system = core_examples::create_development_system();
            .with_cache(ZeroCostApiMemoryCache::<String, String, 1000, 3600>::new())   // 1k entries, 1hr TTL
            .with_rate_limiter(ZeroCostTokenBucketLimiter::<String, 100, 10>::new())   // 100/min, 10 burst
            .with_config(ApiConfig::new(beardog_types::env_config::get_api_bind_address()))

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

        assert_eq!(server.bind_address, "127.0.0.1:8080");

        let cache_stats = server.app_state.get_cache_performance().await;
        assert_eq!(cache_stats.total_requests, 0); // Initially no requests
        let rate_stats = server.app_state.get_rate_limit_performance().await;
        assert_eq!(rate_stats.total_requests, 0); // Initially no requests
    #[test]
    fn test_compile_time_configurations() {

        let _prod_server = examples::create_production_api_server();
        let _dev_server = examples::create_development_api_server();
        let _bench_server = examples::create_benchmark_api_server();

} 
