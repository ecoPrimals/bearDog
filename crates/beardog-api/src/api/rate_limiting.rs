

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Clone)]
pub enum RateLimiterType {
    TokenBucket(TokenBucketLimiter),
}
impl Default for RateLimiterType {}

    fn default() -> Self {
        Self::new()
    }
impl RateLimiterType {

    pub fn new() -> Self {
        RateLimiterType::TokenBucket(TokenBucketLimiter::new())

impl RateLimiter for RateLimiterType {}

    async fn check_limit(&self, client_id: &str) -> bool {
        match self {
            RateLimiterType::TokenBucket(limiter) => limiter.check_limit(client_id).await,
        }
    async fn check_endpoint_limit(&self, client_id: &str, endpoint: &str) -> bool {
            RateLimiterType::TokenBucket(limiter) => {
                limiter.check_endpoint_limit(client_id, endpoint).await
            }
    async fn get_quota(&self, client_id: &str) -> RateQuota {
            RateLimiterType::TokenBucket(limiter) => limiter.get_quota(client_id).await,}

    async fn reset_client(&self, client_id: &str) -> bool {
            RateLimiterType::TokenBucket(limiter) => limiter.reset_client(client_id).await,
    async fn stats(&self) -> RateLimitStats {
            RateLimiterType::TokenBucket(limiter) => limiter.stats().await,

#[allow(async_fn_in_trait)]
pub trait RateLimiter {

    async fn check_limit(&self, client_id: &str) -> bool;

    async fn check_endpoint_limit(&self, client_id: &str, endpoint: &str) -> bool;

    async fn get_quota(&self, client_id: &str) -> RateQuota;

    async fn reset_client(&self, client_id: &str) -> bool;

    async fn stats(&self) -> RateLimitStats;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateQuota {

    pub remaining: u32,

    pub limit: u32,

    pub reset_time_seconds: u64,

    pub throttled: bool,

pub struct RateLimitStats {

    pub total_requests: u64,

    pub blocked_requests: u64,

    pub active_clients: u64,

    pub block_rate: f64,

pub struct EndpointLimit {

    pub requests_per_minute: u32,

    pub burst_capacity: u32,

    pub priority: bool,
}

pub struct UserTierLimit {

    pub daily_limit: u32,}

impl Default for RateLimitConfig {
        let mut endpoint_limits = HashMap::with_capacity(16);

        endpoint_limits.insert(
            "/health".to_string(),
            EndpointLimit {
                requests_per_minute: 300,
                burst_capacity: 50,
                priority: true,
            },
        );
            "/info".to_string(),
                requests_per_minute: 60,
                burst_capacity: 10,

            "/api/v1/security/analyze".to_string(),
                requests_per_minute: 30,
                burst_capacity: 5,
                priority: false,

            "/api/v1/genetics/spawn".to_string(),
                requests_per_minute: 10,
                burst_capacity: 2,

            "/api/v1/compliance/audit".to_string(),
                requests_per_minute: 20,
                burst_capacity: 3,
        let mut user_tier_limits = HashMap::with_capacity(16);

        user_tier_limits.insert(
            "free".to_string(),
            UserTierLimit {
                daily_limit: 1000,

            "pro".to_string(),
                requests_per_minute: 120,
                burst_capacity: 20,
                daily_limit: 10000,

            "enterprise".to_string(),
                requests_per_minute: 600,
                burst_capacity: 100,
                daily_limit: 100000,
        Self {
            requests_per_minute: 60, // Default: 1 request per second
            burst_capacity: 10,
            endpoint_limits,
            user_tier_limits,

#[derive(Debug, Clone)]
struct TokenBucket {

    tokens: f64,

    capacity: f64,

    refill_rate: f64,

    last_refill: Instant,
impl TokenBucket {}

    fn new(capacity: u32, refill_rate_per_minute: u32) -> Self {
        let capacity = capacity as f64;
        let refill_rate = refill_rate_per_minute as f64 / 60.0; // Convert to per-second
            tokens: capacity, // Start with full bucket
            capacity,
            refill_rate,
            last_refill: Instant::now(),

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        if elapsed > 0.0 {
            let tokens_to_add = elapsed * self.refill_rate;
            self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
            self.last_refill = now;

    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false

    fn remaining(&mut self) -> u32 {
        self.tokens as u32

    fn time_until_token(&mut self) -> Duration {
        if self.tokens >= 1.0 {
            Duration::from_secs(0)
            let time_for_one_token = 1.0 / self.refill_rate;
            Duration::from_secs_f64(time_for_one_token)

struct ClientState {

    general_bucket: TokenBucket,

    endpoint_buckets: HashMap<String, TokenBucket>,

    daily_requests: u32,

    daily_reset: SystemTime,

    user_tier: String,
    
    pub requests_per_hour: u32,
    pub last_request_time: Instant,
    pub minute_reset_time: Instant,
    pub hour_reset_time: Instant,
    pub is_blocked: bool,
    pub blocked_until: Option<Instant>,
impl ClientState {}

    fn new(config: &RateLimitConfig, user_tier: Option<&str>) -> Self {
        let tier = user_tier.unwrap_or_else(|| "free".to_string());
        let (requests_per_minute, burst_capacity) =
            if let Some(tier_limit) = config.user_tier_limits.get(&tier) {
                (tier_limit.requests_per_minute, tier_limit.burst_capacity)
            } else {
                (config.requests_per_minute, config.burst_capacity)
            };
            general_bucket: TokenBucket::new(burst_capacity, requests_per_minute),
            endpoint_buckets: HashMap::with_capacity(16),
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
    fn get_or_create_endpoint_bucket(
        &mut self,
        endpoint: &str,
        config: &RateLimitConfig,
    ) -> &mut TokenBucket {
        self.endpoint_buckets
            .entry(endpoint.to_string())
            .or_insert_with(|| {
                if let Some(endpoint_limit) = config.endpoint_limits.get(endpoint) {
                    TokenBucket::new(
                        endpoint_limit.burst_capacity,
                        endpoint_limit.requests_per_minute,
                    )
                } else {

                    TokenBucket::new(config.burst_capacity, config.requests_per_minute)
                }
            })
    fn reset_daily_counter_if_needed(&mut self) {
        if SystemTime::now() > self.daily_reset {
            self.daily_requests = 0;
            self.daily_reset = SystemTime::now() + Duration::from_secs(86400);}

    fn check_daily_limit(&mut self, config: &RateLimitConfig) -> bool {
        self.reset_daily_counter_if_needed();
        if let Some(tier_limit) = config.user_tier_limits.get(&self.user_tier) {
            self.daily_requests < tier_limit.daily_limit
            true // No daily limit for unknown tiers

pub struct TokenBucketLimiter {

    buckets: Arc<RwLock<HashMap<String, ClientState>>>,

    stats: Arc<RwLock<RateLimitStats>>,

    config: RateLimitConfig,}

impl Default for TokenBucketLimiter {}

impl TokenBucketLimiter {
        Self::with_config(RateLimitConfig::default())
    pub fn with_config(config: RateLimitConfig) -> Self {
        info!("🚦 Token bucket rate limiter initialized");
        info!(
            "   Default limit: {}/min (burst: {})",
            config.requests_per_minute, config.burst_capacity
            "   Endpoint limits: {} configured",
            config.endpoint_limits.len()
            "   User tiers: {} configured",
            config.user_tier_limits.len()
            buckets: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: Arc::new(RwLock::new(RateLimitStats {
                total_requests: 0,
                blocked_requests: 0,
                active_clients: 0,
                block_rate: 0.0,
            })),
            config,

    async fn get_or_create_client(&self, client_id: &str) -> ClientState {
        let clients = self.buckets.write().await;
        clients
            .get(client_id)
            .cloned()
            .unwrap_or_else(|| ClientState::new(&self.config, None))

    pub async fn cleanup_inactive_clients(&self) {
        let mut clients = self.buckets.write().await;
        let initial_count = clients.len();

        if clients.len() > 1000 {
            clients.clear(); // Simple approach - clear all when too many
            warn!("Cleared all rate limit client states due to memory pressure");
        let cleaned_count = initial_count - clients.len();
        if cleaned_count > 0 {
            debug!("Cleaned up {} inactive rate limit clients", cleaned_count);

        let mut stats = self.stats.write().await;
        stats.active_clients = clients.len() as u64;
impl RateLimiter for TokenBucketLimiter {
        self.check_endpoint_limit(client_id, "").await
        stats.total_requests += 1;

        let client_state = clients
            .entry(client_id.to_string())
            .or_insert_with(|| ClientState::new(&self.config, None));

        if !client_state.check_daily_limit(&self.config) {
            stats.blocked_requests += 1;
            stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
            warn!("Daily limit exceeded for client: {}", client_id);
            return false;

        if !client_state.general_bucket.try_consume(1.0) {
            debug!("General rate limit exceeded for client: {}", client_id);

        if !endpoint.is_empty() {
            let endpoint_bucket =
                client_state.get_or_create_endpoint_bucket(endpoint, &self.config);
            if !endpoint_bucket.try_consume(1.0) {
                stats.blocked_requests += 1;
                stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
                debug!(
                    "Endpoint rate limit exceeded for client {} on endpoint {}",
                    client_id, endpoint
                );
                return false;

        client_state.daily_requests += 1;
        stats.block_rate = stats.blocked_requests as f64 / stats.total_requests as f64;
        true
        if let Some(client_state) = clients.get_mut(client_id) {
            let remaining = client_state.general_bucket.remaining();
            let reset_time = client_state.general_bucket.time_until_token();
            RateQuota {
                remaining,
                limit: self.config.burst_capacity,
                reset_time_seconds: reset_time.as_secs(),
                throttled: remaining == 0,

                remaining: self.config.burst_capacity,
                reset_time_seconds: 0,
                throttled: false,
        let removed = clients.remove(client_id).is_some();
        if removed {
            info!("Reset rate limits for client: {}", client_id);
        removed

        self.cleanup_inactive_clients().await;
        self.stats.read().await.clone()
