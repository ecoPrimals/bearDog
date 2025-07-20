//! Rate Limiting Module
//!
//! Handles request rate limiting and throttling to prevent abuse.

use super::*;
use serde::{Deserialize, Serialize};

impl BearDogSecurityProvider {
    /// Check if rate limiting allows the request
    pub async fn check_rate_limit(&mut self, user_id: &str) -> BearDogResult<bool> {
        if !self.config.rate_limiting_enabled {
            return Ok(true);
        }

        let now = Utc::now();
        let window_start = now - Duration::seconds(self.rate_limiter.config.window_seconds as i64);

        // Get or create user state
        let user_state = self
            .rate_limiter
            .state
            .entry(user_id.to_string())
            .or_insert_with(|| RateLimiterState {
                count: 0,
                window_start,
                last_operation: now,
            });

        // Reset window if needed
        if now
            >= user_state.window_start
                + Duration::seconds(self.rate_limiter.config.window_seconds as i64)
        {
            user_state.count = 0;
            user_state.window_start = now;
        }

        // Check if under limit
        if user_state.count < self.rate_limiter.config.max_operations {
            user_state.count += 1;
            user_state.last_operation = now;
            Ok(true)
        } else {
            self.metrics.rate_limited_requests += 1;
            self.metrics.rate_limit_violations += 1;
            *self
                .metrics
                .rate_limit_violations_per_user
                .entry(user_id.to_string())
                .or_insert(0) += 1;
            Ok(false)
        }
    }

    /// Reset rate limiting for a user (admin function)
    pub async fn reset_rate_limit(&mut self, user_id: &str) -> BearDogResult<()> {
        self.rate_limiter.state.remove(user_id);
        Ok(())
    }

    /// Get current rate limiting status for a user
    pub async fn get_rate_limit_status(&self, user_id: &str) -> RateLimitStatus {
        if let Some(user_state) = self.rate_limiter.state.get(user_id) {
            let now = Utc::now();
            let window_end = user_state.window_start
                + Duration::seconds(self.rate_limiter.config.window_seconds as i64);

            RateLimitStatus {
                current_count: user_state.count,
                max_operations: self.rate_limiter.config.max_operations,
                window_start: user_state.window_start,
                window_end,
                remaining_operations: self
                    .rate_limiter
                    .config
                    .max_operations
                    .saturating_sub(user_state.count),
                reset_time: if now >= window_end { now } else { window_end },
            }
        } else {
            RateLimitStatus {
                current_count: 0,
                max_operations: self.rate_limiter.config.max_operations,
                window_start: Utc::now(),
                window_end: Utc::now()
                    + Duration::seconds(self.rate_limiter.config.window_seconds as i64),
                remaining_operations: self.rate_limiter.config.max_operations,
                reset_time: Utc::now()
                    + Duration::seconds(self.rate_limiter.config.window_seconds as i64),
            }
        }
    }
}

/// Rate limiting status for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    pub current_count: u32,
    pub max_operations: u32,
    pub window_start: chrono::DateTime<Utc>,
    pub window_end: chrono::DateTime<Utc>,
    pub remaining_operations: u32,
    pub reset_time: chrono::DateTime<Utc>,
}
