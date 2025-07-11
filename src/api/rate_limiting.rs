//! High-Performance Rate Limiting for BearDog API
//!
//! Implements sophisticated rate limiting with:
//! - Token bucket algorithm for smooth rate limiting
//! - Per-endpoint and per-user limits
//! - Distributed rate limiting support
//! - Graceful degradation and burst handling

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, warn, info};

/// Rate limiter trait for different implementations
#[async_trait]
pub trait RateLimiter {
    /// Check if request is within rate limit
    async fn check_limit(&self, client_id: &str) -> bool;
    
    /// Check limit for specific endpoint
    async fn check_endpoint_limit(&self, client_id: &str, endpoint: &str) -> bool;
    
    /// Get remaining quota for client
    async fn get_quota(&self, client_id: &str) -> RateQuota;
    
    /// Reset limits for client (admin function)
    async fn reset_client(&self, client_id: &str) -> bool;
    
    /// Get rate limiting statistics
    async fn stats(&self) -> RateLimitStats;
}

/// Rate quota information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateQuota {
    /// Requests remaining in current window
    pub remaining: u32,
    /// Maximum requests per window
    pub limit: u32,
    /// Time until quota resets (seconds)
    pub reset_time_seconds: u64,
    /// Whether client is currently throttled
    pub throttled: bool,
}

/// Rate limiting statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStats {
    /// Total requests processed
    pub total_requests: u64,
    /// Requests blocked due to rate limiting
    pub blocked_requests: u64,
    /// Current active clients
    pub active_clients: u64,
    /// Block rate (0.0 - 1.0)
    pub block_rate: f64,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute for general API
    pub requests_per_minute: u32,
    /// Burst capacity (how many requests can be made at once)
    pub burst_capacity: u32,
    /// Endpoint-specific limits
    pub endpoint_limits: HashMap<String, EndpointLimit>,
    /// User tier limits
    pub user_tier_limits: HashMap<String, UserTierLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointLimit {
    /// Requests per minute for this endpoint
    pub requests_per_minute: u32,
    /// Burst capacity for this endpoint
    pub burst_capacity: u32,
    /// Whether this endpoint has priority
    pub priority: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTierLimit {
    /// Requests per minute for this user tier
    pub requests_per_minute: u32,
    /// Burst capacity for this user tier
    pub burst_capacity: u32,
    /// Daily request limit
    pub daily_limit: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut endpoint_limits = HashMap::new();
        
        // Health and info endpoints - more generous
        endpoint_limits.insert("/health".to_string(), EndpointLimit {
            requests_per_minute: 300,
            burst_capacity: 50,
            priority: true,
        });
        
        endpoint_limits.insert("/info".to_string(), EndpointLimit {
            requests_per_minute: 60,
            burst_capacity: 10,
            priority: true,
        });
        
        // Security endpoints - moderate limits due to processing overhead
        endpoint_limits.insert("/api/v1/security/analyze".to_string(), EndpointLimit {
            requests_per_minute: 30,
            burst_capacity: 5,
            priority: false,
        });
        
        // Genetics endpoints - conservative due to resource usage
        endpoint_limits.insert("/api/v1/genetics/spawn".to_string(), EndpointLimit {
            requests_per_minute: 10,
            burst_capacity: 2,
            priority: false,
        });
        
        // Compliance endpoints - moderate limits
        endpoint_limits.insert("/api/v1/compliance/audit".to_string(), EndpointLimit {
            requests_per_minute: 20,
            burst_capacity: 3,
            priority: false,
        });
        
        let mut user_tier_limits = HashMap::new();
        
        // Free tier
        user_tier_limits.insert("free".to_string(), UserTierLimit {
            requests_per_minute: 30,
            burst_capacity: 5,
            daily_limit: 1000,
        });
        
        // Pro tier
        user_tier_limits.insert("pro".to_string(), UserTierLimit {
            requests_per_minute: 120,
            burst_capacity: 20,
            daily_limit: 10000,
        });
        
        // Enterprise tier
        user_tier_limits.insert("enterprise".to_string(), UserTierLimit {
            requests_per_minute: 600,
            burst_capacity: 100,
            daily_limit: 100000,
        });
        
        Self {
            requests_per_minute: 60, // Default: 1 request per second
            burst_capacity: 10,
            endpoint_limits,
            user_tier_limits,
        }
    }
}

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
struct TokenBucket {
    /// Current number of tokens
    tokens: f64,
    /// Maximum number of tokens (bucket capacity)
    capacity: f64,
    /// Rate at which tokens are refilled (tokens per second)
    refill_rate: f64,
    /// Last refill timestamp
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate_per_minute: u32) -> Self {
        let capacity = capacity as f64;
        let refill_rate = refill_rate_per_minute as f64 / 60.0; // Convert to per-second
        
        Self {
            tokens: capacity, // Start with full bucket
            capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }
    
    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        
        if elapsed > 0.0 {
            let tokens_to_add = elapsed * self.refill_rate;
            self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
            self.last_refill = now;
        }
    }
    
    /// Try to consume tokens
    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill();
        
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
    
    /// Get remaining tokens
    fn remaining(&mut self) -> u32 {
        self.refill();
        self.tokens as u32
    }
    
    /// Get time until next token is available
    fn time_until_token(&mut self) -> Duration {
        self.refill();
        
        if self.tokens >= 1.0 {
            Duration::from_secs(0)
        } else {
            let time_for_one_token = 1.0 / self.refill_rate;
            Duration::from_secs_f64(time_for_one_token)
        }
    }
}

/// Client rate limiting state
#[derive(Debug, Clone)]
struct ClientState {
    /// General API bucket
    general_bucket: TokenBucket,
    /// Endpoint-specific buckets
    endpoint_buckets: HashMap<String, TokenBucket>,
    /// Daily request counter
    daily_requests: u32,
    /// Daily counter reset time
    daily_reset: SystemTime,
    /// User tier
    user_tier: String,
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub last_request_time: Instant,
    pub minute_reset_time: Instant,
    pub hour_reset_time: Instant,
    pub is_blocked: bool,
    pub blocked_until: Option<Instant>,
}

impl ClientState {
    fn new(config: &RateLimitConfig, user_tier: Option<String>) -> Self {
        let tier = user_tier.unwrap_or_else(|| "free".to_string());
        
        let (requests_per_minute, burst_capacity) = if let Some(tier_limit) = config.user_tier_limits.get(&tier) {
            (tier_limit.requests_per_minute, tier_limit.burst_capacity)
        } else {
            (config.requests_per_minute, config.burst_capacity)
        };
        
        Self {
            general_bucket: TokenBucket::new(burst_capacity, requests_per_minute),
            endpoint_buckets: HashMap::new(),
            daily_requests: 0,
            daily_reset: SystemTime::now() + Duration::from_secs(86400), // 24 hours
            user_tier: tier,
            requests_per_minute,
            requests_per_hour: 0,
            last_request_time: Instant::now(),
            minute_reset_time: Instant::now(),
            hour_reset_time: Instant::now(),
            is_blocked: false,
            blocked_until: None,
        }
    }
    
    fn get_or_create_endpoint_bucket(&mut self, endpoint: &str, config: &RateLimitConfig) -> &mut TokenBucket {
        self.endpoint_buckets.entry(endpoint.to_string()).or_insert_with(|| {
            if let Some(endpoint_limit) = config.endpoint_limits.get(endpoint) {
                TokenBucket::new(endpoint_limit.burst_capacity, endpoint_limit.requests_per_minute)
            } else {
                // Use general limits for unknown endpoints
                TokenBucket::new(config.burst_capacity, config.requests_per_minute)
            }
        })
    }
    
    fn reset_daily_counter_if_needed(&mut self) {
        if SystemTime::now() > self.daily_reset {
            self.daily_requests = 0;
            self.daily_reset = SystemTime::now() + Duration::from_secs(86400);
        }
    }
    
    fn check_daily_limit(&mut self, config: &RateLimitConfig) -> bool {
        self.reset_daily_counter_if_needed();
        
        if let Some(tier_limit) = config.user_tier_limits.get(&self.user_tier) {
            self.daily_requests < tier_limit.daily_limit
        } else {
            true // No daily limit for unknown tiers
        }
    }
}

/// Token bucket rate limiter implementation
pub struct TokenBucketLimiter {
    /// Client states
    clients: Arc<RwLock<HashMap<String, ClientState>>>,
    /// Configuration
    config: RateLimitConfig,
    /// Statistics
    stats: Arc<RwLock<RateLimitStats>>,
}

impl TokenBucketLimiter {
    pub fn new() -> Self {
        Self::with_config(RateLimitConfig::default())
    }
    
    pub fn with_config(config: RateLimitConfig) -> Self {
        info!("🚦 Token bucket rate limiter initialized");
        info!("   Default limit: {}/min (burst: {})", config.requests_per_minute, config.burst_capacity);
        info!("   Endpoint limits: {} configured", config.endpoint_limits.len());
        info!("   User tiers: {} configured", config.user_tier_limits.len());
        
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(RateLimitStats {
                total_requests: 0,
                blocked_requests: 0,
                active_clients: 0,
                block_rate: 0.0,
            })),
        }
    }
    
    /// Get or create client state
    async fn get_or_create_client(&self, client_id: &str) -> ClientState {
        let clients = self.clients.write().await;
        
        clients.get(client_id).cloned().unwrap_or_else(|| {
            ClientState::new(&self.config, None)
        })
    }
    
    /// Cleanup inactive clients periodically
    pub async fn cleanup_inactive_clients(&self) {
        let mut clients = self.clients.write().await;
        let initial_count = clients.len();
        
        // Remove clients that haven't been active for 1 hour
        // In a more sophisticated implementation, you'd track last activity
        if clients.len() > 1000 {
            clients.clear(); // Simple approach - clear all when too many
            warn!("Cleared all rate limit client states due to memory pressure");
        }
        
        let cleaned_count = initial_count - clients.len();
        if cleaned_count > 0 {
            debug!("Cleaned up {} inactive rate limit clients", cleaned_count);
        }
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.active_clients = clients.len() as u64;
    }
}

#[async_trait]
impl RateLimiter for TokenBucketLimiter {
    async fn check_limit(&self, client_id: &str) -> bool {
        self.check_endpoint_limit(client_id, "").await
    }
    
    async fn check_endpoint_limit(&self, client_id: &str, endpoint: &str) -> bool {
        let mut clients = self.clients.write().await;
        let mut stats = self.stats.write().await;
        
        stats.total_requests += 1;
        
        // Get or create client state
        let client_state = clients.entry(client_id.to_string())
            .or_insert_with(|| ClientState::new(&self.config, None));
        
        // Check daily limit first
        if !client_state.check_daily_limit(&self.config) {
            stats.blocked_requests += 1;
            stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
            warn!("Daily limit exceeded for client: {}", client_id);
            return false;
        }
        
        // Check general rate limit
        if !client_state.general_bucket.try_consume(1.0) {
            stats.blocked_requests += 1;
            stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
            debug!("General rate limit exceeded for client: {}", client_id);
            return false;
        }
        
        // Check endpoint-specific limit if endpoint is specified
        if !endpoint.is_empty() {
            let endpoint_bucket = client_state.get_or_create_endpoint_bucket(endpoint, &self.config);
            if !endpoint_bucket.try_consume(1.0) {
                stats.blocked_requests += 1;
                stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
                debug!("Endpoint rate limit exceeded for client {} on endpoint {}", client_id, endpoint);
                return false;
            }
        }
        
        // Update daily counter
        client_state.daily_requests += 1;
        
        // Update stats
        stats.active_clients = clients.len() as u64;
        stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
        
        true
    }
    
    async fn get_quota(&self, client_id: &str) -> RateQuota {
        let mut clients = self.clients.write().await;
        
        if let Some(client_state) = clients.get_mut(client_id) {
            let remaining = client_state.general_bucket.remaining();
            let reset_time = client_state.general_bucket.time_until_token();
            
            RateQuota {
                remaining,
                limit: self.config.burst_capacity,
                reset_time_seconds: reset_time.as_secs(),
                throttled: remaining == 0,
            }
        } else {
            // New client - full quota
            RateQuota {
                remaining: self.config.burst_capacity,
                limit: self.config.burst_capacity,
                reset_time_seconds: 0,
                throttled: false,
            }
        }
    }
    
    async fn reset_client(&self, client_id: &str) -> bool {
        let mut clients = self.clients.write().await;
        let removed = clients.remove(client_id).is_some();
        
        if removed {
            info!("Reset rate limits for client: {}", client_id);
        }
        
        removed
    }
    
    async fn stats(&self) -> RateLimitStats {
        // Trigger cleanup before returning stats
        self.cleanup_inactive_clients().await;
        self.stats.read().await.clone()
    }
} 