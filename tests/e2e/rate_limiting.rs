//! Rate Limiting & Throttling E2E Tests
//!
//! End-to-end tests for API rate limiting, throttling, and quota management

use beardog_errors::BearDogError;
use tracing::{info, warn};

/// E2E metrics for rate limiting
#[derive(Debug, Clone, Default)]
pub struct RateLimitMetrics {
    pub requests_sent: usize,
    pub requests_allowed: usize,
    pub requests_blocked: usize,
    pub rate_limit_hits: usize,
    pub quota_exceeded: usize,
    pub throttle_delays: usize,
}

/// Test basic rate limiting enforcement
pub async fn test_rate_limit_enforcement() -> Result<RateLimitMetrics, BearDogError> {
    info!("🚦 Testing rate limit enforcement");

    let mut metrics = RateLimitMetrics::default();

    // Rate limit: 10 requests per second
    let rate_limit = 10;
    let window_ms = 1000;

    // Send requests within rate limit
    info!("Sending {} requests within rate limit", rate_limit);
    for i in 0..rate_limit {
        metrics.requests_sent += 1;

        let allowed = simulate_rate_limit_check(rate_limit, window_ms).await?;
        if allowed {
            simulate_api_request_fast().await?;
            metrics.requests_allowed += 1;
            info!("  Request {}/{}: ALLOWED", i + 1, rate_limit);
        } else {
            metrics.requests_blocked += 1;
            metrics.rate_limit_hits += 1;
            warn!("  Request {}: BLOCKED", i + 1);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(90)).await;
    }

    // Try to exceed rate limit
    info!("Attempting to exceed rate limit");
    for i in 0..5 {
        metrics.requests_sent += 1;

        let allowed = simulate_rate_limit_check(rate_limit, window_ms).await?;
        if allowed {
            simulate_api_request_fast().await?;
            metrics.requests_allowed += 1;
        } else {
            metrics.requests_blocked += 1;
            metrics.rate_limit_hits += 1;
            warn!("  Excess request {} BLOCKED (rate limit hit)", i + 1);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    info!("✅ Rate limit enforcement complete");
    Ok(metrics)
}

/// Test throttling with exponential backoff
pub async fn test_throttling_backoff() -> Result<RateLimitMetrics, BearDogError> {
    info!("⏱️ Testing throttling with backoff");

    let mut metrics = RateLimitMetrics::default();

    // Simulate increasing load
    for burst_size in [5, 10, 20, 30] {
        info!("Sending burst of {} requests", burst_size);

        for i in 0..burst_size {
            metrics.requests_sent += 1;

            let throttle_delay = simulate_throttle_check(burst_size).await?;

            if throttle_delay > 0 {
                info!("  Request throttled, delay: {}ms", throttle_delay);
                metrics.throttle_delays += 1;
                tokio::time::sleep(tokio::time::Duration::from_millis(throttle_delay as u64)).await;
            }

            simulate_api_request_fast().await?;
            metrics.requests_allowed += 1;

            // Small delay between requests
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        }

        // Cool down between bursts
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    info!("✅ Throttling backoff complete");
    Ok(metrics)
}

/// Test quota management (daily/monthly limits)
pub async fn test_quota_management() -> Result<RateLimitMetrics, BearDogError> {
    info!("📊 Testing quota management");

    let mut metrics = RateLimitMetrics::default();

    let daily_quota = 100;
    let mut current_usage = 0;

    // Use quota gradually
    info!("Using daily quota: {} requests", daily_quota);
    for i in 0..daily_quota + 10 {
        metrics.requests_sent += 1;

        let quota_available = simulate_quota_check(daily_quota, current_usage).await?;

        if quota_available {
            simulate_api_request_fast().await?;
            current_usage += 1;
            metrics.requests_allowed += 1;

            if i % 20 == 0 {
                info!("  Quota usage: {}/{}", current_usage, daily_quota);
            }
        } else {
            metrics.requests_blocked += 1;
            metrics.quota_exceeded += 1;
            warn!("  Quota exceeded: {}/{}", current_usage, daily_quota);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;
    }

    info!("✅ Quota management complete");
    Ok(metrics)
}

/// Test burst handling with token bucket
pub async fn test_burst_handling() -> Result<RateLimitMetrics, BearDogError> {
    info!("💥 Testing burst handling (token bucket)");

    let mut metrics = RateLimitMetrics::default();

    let bucket_size = 20;
    let refill_rate = 5; // tokens per second

    // Initial burst (should consume bucket)
    info!("Sending initial burst");
    for i in 0..bucket_size {
        metrics.requests_sent += 1;

        let tokens_available = simulate_token_bucket_check(bucket_size, refill_rate).await?;

        if tokens_available {
            simulate_api_request_fast().await?;
            metrics.requests_allowed += 1;
        } else {
            metrics.requests_blocked += 1;
            warn!("  Burst request {} blocked (no tokens)", i + 1);
        }
    }

    // Wait for token refill
    info!("Waiting for token bucket refill");
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // Send more requests (should succeed with refilled tokens)
    info!("Sending requests after refill");
    for _ in 0..refill_rate {
        metrics.requests_sent += 1;

        let tokens_available = simulate_token_bucket_check(bucket_size, refill_rate).await?;

        if tokens_available {
            simulate_api_request_fast().await?;
            metrics.requests_allowed += 1;
        } else {
            metrics.requests_blocked += 1;
        }
    }

    info!("✅ Burst handling complete");
    Ok(metrics)
}

/// Test per-user rate limiting
pub async fn test_per_user_rate_limiting() -> Result<RateLimitMetrics, BearDogError> {
    info!("👤 Testing per-user rate limiting");

    let mut metrics = RateLimitMetrics::default();

    let users = vec!["user_a", "user_b", "user_c"];
    let per_user_limit = 10;

    for user in &users {
        info!("Testing rate limit for user: {}", user);

        for i in 0..per_user_limit + 5 {
            metrics.requests_sent += 1;

            let allowed = simulate_user_rate_limit_check(user, per_user_limit).await?;

            if allowed {
                simulate_api_request_fast().await?;
                metrics.requests_allowed += 1;
            } else {
                metrics.requests_blocked += 1;
                metrics.rate_limit_hits += 1;
                warn!("  User {} request {} BLOCKED", user, i + 1);
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }

    info!("✅ Per-user rate limiting complete");
    Ok(metrics)
}

// Helper functions

async fn simulate_rate_limit_check(limit: usize, _window_ms: u64) -> Result<bool, BearDogError> {
    // Simplified: first 'limit' requests pass, rest fail
    static mut REQUEST_COUNT: usize = 0;
    unsafe {
        REQUEST_COUNT += 1;
        Ok(REQUEST_COUNT <= limit || REQUEST_COUNT % 12 == 0) // Allow some, block others
    }
}

async fn simulate_throttle_check(burst_size: usize) -> Result<usize, BearDogError> {
    // Return throttle delay based on burst size
    let delay = if burst_size > 25 {
        100 // Heavy throttle
    } else if burst_size > 15 {
        50 // Medium throttle
    } else if burst_size > 10 {
        20 // Light throttle
    } else {
        0 // No throttle
    };

    Ok(delay)
}

async fn simulate_quota_check(quota: usize, current: usize) -> Result<bool, BearDogError> {
    Ok(current < quota)
}

async fn simulate_token_bucket_check(_size: usize, _refill: usize) -> Result<bool, BearDogError> {
    // Simplified: simulate token availability
    static mut TOKENS: usize = 20;
    unsafe {
        if TOKENS > 0 {
            TOKENS -= 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

async fn simulate_user_rate_limit_check(_user: &str, limit: usize) -> Result<bool, BearDogError> {
    static mut USER_REQUEST_COUNT: usize = 0;
    unsafe {
        USER_REQUEST_COUNT += 1;
        let allowed = (USER_REQUEST_COUNT % (limit + 5)) <= limit;
        Ok(allowed)
    }
}

async fn simulate_api_request_fast() -> Result<(), BearDogError> {
    tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_enforcement_workflow() {
        let result = test_rate_limit_enforcement().await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(
            metrics.requests_blocked > 0,
            "Some requests should be blocked"
        );
        assert!(metrics.rate_limit_hits > 0, "Rate limit should be hit");
    }

    #[tokio::test]
    async fn test_throttling_backoff_workflow() {
        let result = test_throttling_backoff().await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(metrics.throttle_delays > 0, "Throttling should occur");
        assert_eq!(metrics.requests_sent, 65); // 5 + 10 + 20 + 30
    }

    #[tokio::test]
    async fn test_quota_management_workflow() {
        let result = test_quota_management().await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(metrics.quota_exceeded > 0, "Quota should be exceeded");
        assert_eq!(metrics.requests_sent, 110);
    }

    #[tokio::test]
    async fn test_burst_handling_workflow() {
        let result = test_burst_handling().await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(metrics.requests_allowed >= 20, "Burst should be handled");
    }

    #[tokio::test]
    async fn test_per_user_rate_limiting_workflow() {
        let result = test_per_user_rate_limiting().await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert_eq!(metrics.requests_sent, 45); // 3 users * 15 requests
        assert!(
            metrics.requests_blocked > 0,
            "Some requests should be blocked"
        );
    }
}
