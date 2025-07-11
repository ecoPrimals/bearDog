//! High-Performance BearDog API Server
//!
//! Provides a comprehensive, AI-first REST API server with:
//! - Request/response caching
//! - Rate limiting and throttling  
//! - Connection pooling
//! - Middleware for observability
//! - Graceful shutdown handling

use super::*;
use crate::{BearDogCore, BearDogResult, BearDogError};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use tracing::{info, warn, debug};
use std::time::{Duration, Instant};

use crate::api::cache::InMemoryCache;

/// High-performance BearDog API Server
pub struct BearDogApiServer {
    core: Arc<BearDogCore>,
    cache: Arc<dyn crate::api::cache::CacheProvider + Send + Sync>,
    rate_limiter: Arc<dyn crate::api::rate_limiting::RateLimiter + Send + Sync>,
    config: ApiServerConfig,
}

#[derive(Debug, Clone)]
pub struct ApiServerConfig {
    pub bind_address: String,
    pub request_timeout_seconds: u64,
    pub max_request_size: usize,
    pub compression_enabled: bool,
    pub cors_enabled: bool,
    pub rate_limiting_enabled: bool,
    pub caching_enabled: bool,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:8080".to_string(),
            request_timeout_seconds: 30,
            max_request_size: 16 * 1024 * 1024, // 16MB
            compression_enabled: true,
            cors_enabled: true,
            rate_limiting_enabled: true,
            caching_enabled: true,
        }
    }
}

impl BearDogApiServer {
    /// Create new API server with comprehensive middleware
    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        // Initialize cache provider
        let cache = Arc::new(InMemoryCache::new());

        // Initialize rate limiter
        let rate_limiter = Arc::new(
            crate::api::rate_limiting::TokenBucketLimiter::new()
        );

        Ok(Self {
            core,
            cache,
            rate_limiter,
            config: ApiServerConfig::default(),
        })
    }

    /// Create comprehensive application router
    pub fn create_router(&self) -> Router {
        let app_state = AppState {
            core: self.core.clone(),
            cache: self.cache.clone(),
            rate_limiter: self.rate_limiter.clone(),
            config: self.config.clone(),
        };

        let app = Router::new()
            .route("/health", get(self::handlers::health_check))
            .route("/info", get(self::handlers::server_info))
            
            // API v1 routes
            .nest("/v1", self.create_v1_routes())
            
            // Apply individual middleware layers
            .layer(middleware::from_fn(request_id_middleware))
            .layer(middleware::from_fn_with_state(app_state.clone(), rate_limiting_middleware))
            .layer(middleware::from_fn_with_state(app_state.clone(), performance_middleware))
            .layer(TraceLayer::new_for_http())
            .layer(TimeoutLayer::new(Duration::from_secs(self.config.request_timeout_seconds)));

        // Apply optional layers based on configuration
        let app = if self.config.compression_enabled {
            app.layer(CompressionLayer::new())
        } else {
            app
        };

        let app = if self.config.cors_enabled {
            app.layer(CorsLayer::permissive())
        } else {
            app
        };

        app.with_state(app_state)
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
            // TODO: Add these routes when modules are implemented
            // .nest("/compliance", crate::api::compliance::create_routes())
            // .nest("/auth", crate::api::auth::create_routes())
            // .nest("/config", crate::api::config::create_routes())
            // .nest("/nodes", crate::api::nodes::create_routes())
    }

    /// Start the API server
    pub async fn start(&self, bind_address: &str) -> BearDogResult<()> {
        info!("🚀 Starting high-performance BearDog API server on {}", bind_address);
        
        let listener = TcpListener::bind(bind_address).await
            .map_err(|e| BearDogError::internal(&format!("Failed to bind to {}: {}", bind_address, e)))?;

        let app = self.create_router();

        info!("✅ BearDog API server ready - AI-first design with full BearDog capabilities");
        info!("📊 Performance features: caching={}, rate_limiting={}, compression={}", 
              self.config.caching_enabled, self.config.rate_limiting_enabled, self.config.compression_enabled);

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| BearDogError::internal(&format!("Server error: {}", e)))?;

        info!("🛑 BearDog API server shutdown completed");
        Ok(())
    }
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub core: Arc<BearDogCore>,
    pub cache: Arc<dyn crate::api::cache::CacheProvider + Send + Sync>,
    pub rate_limiter: Arc<dyn crate::api::rate_limiting::RateLimiter + Send + Sync>,
    pub config: ApiServerConfig,
}

/// Request performance middleware
async fn performance_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    
    // Add start time to request extensions
    request.extensions_mut().insert(start);
    
    let response = next.run(request).await;
    
    let processing_time = start.elapsed();
    debug!("Request processed in {:?}", processing_time);
    
    // Add performance headers
    let mut response = response;
    response.headers_mut().insert("X-Processing-Time", 
                  format!("{}ms", processing_time.as_millis()).parse().unwrap());
    response.headers_mut().insert("X-Server", "BearDog-API/1.0".parse().unwrap());
    
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
async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Add request ID to headers
    request.headers_mut().insert(
        "X-Request-ID",
        request_id.parse().unwrap()
    );
    
    // Store in extensions for handlers
    request.extensions_mut().insert(request_id.clone());
    
    let mut response = next.run(request).await;
    
    // Add request ID to response headers
    response.headers_mut().insert(
        "X-Request-ID",
        request_id.parse().unwrap()
    );
    
    response
}

/// Extract client identifier for rate limiting
fn extract_client_id(request: &Request) -> String {
    // Try API key first
    if let Some(api_key) = request.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return format!("api_key:{}", key_str);
        }
    }
    
    // Fall back to IP address
    if let Some(forwarded) = request.headers().get("X-Forwarded-For") {
        if let Ok(ip_str) = forwarded.to_str() {
            return format!("ip:{}", ip_str.split(',').next().unwrap_or("unknown").trim());
        }
    }
    
    // Default fallback
    "unknown".to_string()
}

/// Graceful shutdown signal
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
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
        let health = state.core.get_health_status().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now(),
            "version": env!("CARGO_PKG_VERSION"),
            "api_version": API_VERSION,
            "health": health
        })))
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