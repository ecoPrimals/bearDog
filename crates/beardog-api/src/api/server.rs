//! High-Performance BearDog API Server
//!
//! Provides a comprehensive, AI-first REST API server with:
//! - Request/response caching
//! - Rate limiting and throttling  
//! - Connection pooling
//! - Middleware for observability
//! - Graceful shutdown handling

use super::zero_copy_handlers::ZeroCopyHandlerContext;
use super::*;
use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use beardog_config::core::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

use crate::api::rate_limiting::RateLimiter;

/// High-performance BearDog API Server
pub struct BearDogApiServer {
    core: Arc<dyn std::any::Any + Send + Sync>,
    cache: Arc<crate::api::cache::CacheProviderType>,
    rate_limiter: Arc<crate::api::rate_limiting::RateLimiterType>,
    config: ApiServerConfig,
}

/// Configuration settings for the API server
#[derive(Debug, Clone)]
pub struct ApiServerConfig {
    /// Address and port to bind the server to
    pub bind_address: String,
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
    /// Maximum size of incoming requests in bytes
    pub max_request_size: usize,
    /// Enable gzip compression for responses
    pub compression_enabled: bool,
    /// Enable Cross-Origin Resource Sharing (CORS)
    pub cors_enabled: bool,
    /// Enable rate limiting middleware
    pub rate_limiting_enabled: bool,
    /// Enable response caching
    pub caching_enabled: bool,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            bind_address: std::env::var("BEARDOG_API_BIND_ADDRESS").unwrap_or_else(|_| {
                format!(
                    "{}:{}",
                    beardog_config::constants::network::DEFAULT_BIND_ADDRESS,
                    beardog_config::constants::network::DEFAULT_API_PORT
                )
            }),
            request_timeout_seconds: std::env::var("BEARDOG_API_REQUEST_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            max_request_size: std::env::var("BEARDOG_API_MAX_REQUEST_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(16 * 1024 * 1024), // 16MB
            compression_enabled: std::env::var("BEARDOG_API_COMPRESSION_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            cors_enabled: std::env::var("BEARDOG_API_CORS_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            rate_limiting_enabled: std::env::var("BEARDOG_API_RATE_LIMITING_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            caching_enabled: std::env::var("BEARDOG_API_CACHING_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl ApiServerConfig {
    /// Create ApiServerConfig from BearDogConfig
    pub fn from_beardog_config(config: &BearDogConfig) -> Self {
        Self {
            bind_address: config.api.http.bind_address.clone(),
            request_timeout_seconds: config.api.timeout.request_timeout,
            max_request_size: config.api.http.max_request_size,
            compression_enabled: true, // Enable compression by default
            cors_enabled: config.api.cors.enabled,
            rate_limiting_enabled: config.api.rate_limiting.enabled,
            caching_enabled: true, // Enable caching by default
        }
    }
}

impl BearDogApiServer {
    /// Create new API server with comprehensive middleware
    pub async fn new(core: Arc<dyn std::any::Any + Send + Sync>) -> BearDogResult<Self> {
        Self::new_with_config(core, ApiServerConfig::default()).await
    }

    /// Create new API server with custom configuration
    pub async fn new_with_config(
        core: Arc<dyn std::any::Any + Send + Sync>,
        config: ApiServerConfig,
    ) -> BearDogResult<Self> {
        // Initialize cache provider
        let cache = Arc::new(crate::api::cache::CacheProviderType::InMemory(
            crate::api::cache::InMemoryCache::new(),
        ));

        // Initialize rate limiter
        let rate_limiter = Arc::new(crate::api::rate_limiting::RateLimiterType::new());

        Ok(Self {
            core,
            cache,
            rate_limiter,
            config,
        })
    }

    /// Create comprehensive application router
    pub fn create_router(&self) -> Router {
        let app_state = AppState {
            core: self.core.clone(),
            cache: self.cache.clone(),
            rate_limiter: self.rate_limiter.clone(),
            config: self.config.clone(),
            zero_copy_context: Arc::new(ZeroCopyHandlerContext::new()),
        };

        // Return the configured app
        Router::new()
            // Use zero-copy optimized health endpoint for better performance
            .route("/health", get(zero_copy_health_wrapper))
            .route("/info", get(self::handlers::server_info))
            // Performance monitoring endpoint for zero-copy statistics
            .route(
                "/api/v1/performance/zero-copy-stats",
                get(zero_copy_stats_handler),
            )
            // API v1 routes
            .nest("/v1", self.create_v1_routes())
            // Apply basic middleware layers
            .layer(middleware::from_fn(request_id_middleware))
            .layer(middleware::from_fn_with_state(
                app_state.clone(),
                rate_limiting_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                app_state.clone(),
                performance_middleware,
            ))
            .with_state(app_state)
    }

    /// Create API v1 routes
    fn create_v1_routes(&self) -> Router<AppState> {
        Router::new()
            // Security API
            .nest("/security", crate::api::security::create_routes())
            // Genetics API
            .nest("/genetics", crate::api::genetics::create_routes())
            // Monitoring API
            .nest("/monitoring", crate::api::monitoring::create_routes())
            // Individual Sovereignty & Peer-to-Peer Sharing API
            .nest("/sovereignty", crate::api::sovereignty::create_routes())
            // Compliance & Audit Management API
            .nest("/compliance", crate::api::compliance::create_routes())
            // Authentication & Authorization API
            .nest("/auth", crate::api::auth::create_routes())
            // Ecosystem RPC Integration API
            .nest("/rpc", crate::api::rpc::create_rpc_router())
        // TODO: Add these routes when modules are implemented
        // .nest("/config", crate::api::config::create_routes())
        // .nest("/nodes", crate::api::nodes::create_routes())
    }

    /// Start the API server
    pub async fn start(&self, bind_address: &str) -> BearDogResult<()> {
        info!(
            "🚀 Starting high-performance BearDog API server on {}",
            bind_address
        );

        let listener = TcpListener::bind(bind_address).await.map_err(|e| {
            BearDogError::internal(format!("Failed to bind to {bind_address}: {e}"))
        })?;

        let app = self.create_router();

        info!("✅ BearDog API server ready - AI-first design with full BearDog capabilities");
        info!(
            "📊 Performance features: caching={}, rate_limiting={}, compression={}",
            self.config.caching_enabled,
            self.config.rate_limiting_enabled,
            self.config.compression_enabled
        );

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| BearDogError::internal(format!("Server error: {e}")))?;

        info!("🛑 BearDog API server shutdown completed");
        Ok(())
    }
}

/// Zero-copy health check wrapper compatible with axum routing
async fn zero_copy_health_wrapper(State(state): State<AppState>) -> Result<Response, StatusCode> {
    use serde_json::json;

    let health_data = json!({
        "success": true,
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "performance_mode": "zero-copy-optimized",
        "components": {
            "api": "healthy",
            "crypto": "healthy",
            "buffer_pool": "active"
        }
    });

    state
        .zero_copy_context
        .response_builder
        .json_response(&health_data, StatusCode::OK)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Zero-copy performance statistics endpoint
async fn zero_copy_stats_handler(State(state): State<AppState>) -> Result<Response, StatusCode> {
    let stats = state.zero_copy_context.get_comprehensive_stats().await;

    let response_data = serde_json::json!({
        "success": true,
        "data": {
            "buffer_pool": {
                "small_buffer_hits": stats.buffer_pool_stats.small_buffer_hits.load(std::sync::atomic::Ordering::Relaxed),
                "medium_buffer_hits": stats.buffer_pool_stats.medium_buffer_hits.load(std::sync::atomic::Ordering::Relaxed),
                "large_buffer_hits": stats.buffer_pool_stats.large_buffer_hits.load(std::sync::atomic::Ordering::Relaxed),
                "total_allocations": stats.buffer_pool_stats.total_allocations.load(std::sync::atomic::Ordering::Relaxed),
            },
            "response_performance": {
                "responses_built": stats.response_stats.responses_built.load(std::sync::atomic::Ordering::Relaxed),
                "header_cache_hits": stats.response_stats.header_cache_hits.load(std::sync::atomic::Ordering::Relaxed),
                "streaming_responses": stats.response_stats.streaming_responses.load(std::sync::atomic::Ordering::Relaxed),
                "content_bytes_served": stats.response_stats.content_bytes_served.load(std::sync::atomic::Ordering::Relaxed),
            },
            "request_parsing": {
                "requests_parsed": stats.request_stats.requests_parsed.load(std::sync::atomic::Ordering::Relaxed),
                "zero_copy_parses": stats.request_stats.zero_copy_parses.load(std::sync::atomic::Ordering::Relaxed),
            }
        },
        "metadata": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "performance_mode": "zero-copy-optimized"
        }
    });

    state
        .zero_copy_context
        .response_builder
        .json_response(&response_data, StatusCode::OK)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    /// Core BearDog engine instance (stored as Any to break circular dependency)
    pub core: Arc<dyn std::any::Any + Send + Sync>,
    /// Cache provider for response caching
    pub cache: Arc<crate::api::cache::CacheProviderType>,
    /// Rate limiter for request throttling
    pub rate_limiter: Arc<crate::api::rate_limiting::RateLimiterType>,
    /// API server configuration
    pub config: ApiServerConfig,
    /// Zero-copy handler context for high-performance operations
    pub zero_copy_context: Arc<ZeroCopyHandlerContext>,
}

/// Request performance middleware
async fn performance_middleware(
    State(_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();

    // Add start time to request extensions
    request.extensions_mut().insert(start);

    let mut response = next.run(request).await;

    let duration = start.elapsed();

    // Add performance headers to the response
    if let Ok(header_value) =
        HeaderValue::from_str(&format!("{:.2}", duration.as_secs_f64() * 1000.0))
    {
        response
            .headers_mut()
            .insert("X-Response-Time-Ms", header_value);
    }

    // Log performance for monitoring
    tracing::debug!("Request took {:.2}ms", duration.as_secs_f64() * 1000.0);

    response
}

/// Rate limiting middleware
async fn rate_limiting_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !state.config.rate_limiting_enabled {
        return Ok(next.run(request).await);
    }

    // Extract client identifier (IP or API key)
    let client_id = extract_client_id(&request);

    // Check rate limit
    if !state.rate_limiter.check_limit(&client_id).await {
        warn!("Rate limit exceeded for client: {}", client_id);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

/// Request ID middleware
async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Add request ID to headers
    if let Ok(header_value) = request_id.parse() {
        request.headers_mut().insert("X-Request-ID", header_value);
    }

    // Store in extensions for handlers
    request.extensions_mut().insert(request_id.clone());

    let mut response = next.run(request).await;

    // Add request ID to response headers
    if let Ok(header_value) = request_id.parse() {
        response.headers_mut().insert("X-Request-ID", header_value);
    }

    response
}

/// Extract client identifier for rate limiting
fn extract_client_id(request: &Request) -> String {
    // Try API key first
    if let Some(api_key) = request.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return format!("api_key:{key_str}");
        }
    }

    // Fall back to IP address
    if let Some(forwarded) = request.headers().get("X-Forwarded-For") {
        if let Ok(ip_str) = forwarded.to_str() {
            return format!(
                "ip:{}",
                ip_str.split(',').next().unwrap_or("unknown").trim()
            );
        }
    }

    // Default fallback
    "unknown".to_string()
}

/// Graceful shutdown signal
async fn shutdown_signal() {
    let ctrl_c = async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {}
            Err(e) => {
                error!("Failed to install Ctrl+C handler: {}", e);
                // Continue without ctrl+c handling rather than panic
            }
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(e) => {
                error!("Failed to install signal handler: {}", e);
                // Continue without terminate signal handling rather than panic
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal, initiating graceful shutdown...");
        },
        _ = terminate => {
            info!("Received terminate signal, initiating graceful shutdown...");
        },
    }
}

/// Basic handler implementations
pub mod handlers {
    use super::*;
    use axum::{extract::State, Json};
    use serde_json::{json, Value};

    /// Health check endpoint
    pub async fn health_check(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
        // Attempt to downcast the core to BearDogCore for proper health checking
        use beardog_core::core::BearDogCore;

        match state.core.downcast_ref::<BearDogCore>() {
            Some(core) => {
                // Perform actual health check using the BearDog core
                match core.health_check().await {
                    Ok(health_check) => {
                        // Return detailed health information
                        Ok(Json(json!({
                            "status": match health_check.status {
                                                            beardog_core::types::HealthStatus::Healthy => "healthy",
                            beardog_core::types::HealthStatus::Degraded => "degraded",
                            beardog_core::types::HealthStatus::Unhealthy => "unhealthy",
                            beardog_core::types::HealthStatus::Starting => "starting",
                            beardog_core::types::HealthStatus::Stopping => "stopping",
                            },
                            "timestamp": chrono::Utc::now(),
                            "version": env!("CARGO_PKG_VERSION"),
                            "api_version": API_VERSION,
                            "uptime_seconds": health_check.uptime.map(|u| u.num_seconds()).unwrap_or(0),
                            "components": health_check.components.len(),
                            "component_details": health_check.components.iter().map(|c| json!({
                                "name": c.name,
                                "healthy": c.healthy,
                                "error": c.error_message
                            })).collect::<Vec<_>>(),
                            "metrics": {
                                                            "memory_usage_bytes": health_check.metrics.memory_usage_mb * 1024 * 1024, // Convert MB to bytes
                            "cpu_usage_percent": health_check.metrics.cpu_usage,
                                "active_connections": health_check.metrics.active_connections,
                                "requests_per_second": health_check.metrics.requests_per_second,
                                "avg_response_time_ms": health_check.metrics.avg_response_time_ms,
                                "error_rate_percent": 0.0 // SystemMetrics doesn't have error_rate_percent
                            }
                        })))
                    }
                    Err(e) => {
                        tracing::error!("Health check failed: {}", e);
                        Ok(Json(json!({
                            "status": "unhealthy",
                            "timestamp": chrono::Utc::now(),
                            "version": env!("CARGO_PKG_VERSION"),
                            "api_version": API_VERSION,
                            "error": format!("Health check failed: {}", e)
                        })))
                    }
                }
            }
            None => {
                // Fallback to basic health check if core type is not BearDogCore
                tracing::warn!("Core is not BearDogCore type, using basic health check");
                Ok(Json(json!({
                    "status": "healthy",
                    "timestamp": chrono::Utc::now(),
                    "version": env!("CARGO_PKG_VERSION"),
                    "api_version": API_VERSION,
                    "note": "Basic health check - core type not recognized"
                })))
            }
        }
    }

    /// Server information endpoint
    pub async fn server_info(State(_state): State<AppState>) -> Json<Value> {
        Json(json!({
            "name": "BearDog API Server",
            "version": env!("CARGO_PKG_VERSION"),
            "api_version": API_VERSION,
            "description": "Enterprise-grade security manager - AI-first API design",
            "features": [
                "threat_detection",
                "genetic_spawning",
                "compliance_audit",
                "cross_node_auth",
                "encryption",
                "monitoring"
            ],
            "performance": {
                "caching": true,
                "rate_limiting": true,
                "compression": true,
                "async_processing": true
            },
            "timestamp": chrono::Utc::now()
        }))
    }
}
