// SPDX-License-Identifier: AGPL-3.0-or-later

//! Rate limit algorithms, defaults, and related headers.

use std::time::Duration;

/// Rate limiting algorithms
pub const TOKEN_BUCKET: &str = "token_bucket";
/// Configuration constant: leaky bucket
pub const LEAKY_BUCKET: &str = "leaky_bucket";
/// Configuration constant: fixed window
pub const FIXED_WINDOW: &str = "fixed_window";
/// Configuration constant: sliding window
pub const SLIDING_WINDOW: &str = "sliding_window";

/// Default limits
pub const DEFAULT_REQUESTS_PER_SECOND: u32 = 100;
/// Configuration constant: default burst size
pub const DEFAULT_BURST_SIZE: u32 = 200;
/// Configuration constant: default window size
pub const DEFAULT_WINDOW_SIZE: Duration = Duration::from_secs(60);

/// Rate limiting headers
pub const X_RATELIMIT_LIMIT: &str = "X-RateLimit-Limit";
/// Configuration constant: x ratelimit remaining
pub const X_RATELIMIT_REMAINING: &str = "X-RateLimit-Remaining";
/// Configuration constant: x ratelimit reset
pub const X_RATELIMIT_RESET: &str = "X-RateLimit-Reset";
/// Configuration constant: x ratelimit retry after
pub const X_RATELIMIT_RETRY_AFTER: &str = "X-RateLimit-Retry-After";

/// Key extractors
pub const KEY_IP_ADDRESS: &str = "ip_address";
/// Configuration constant: key user id
pub const KEY_USER_ID: &str = "user_id";
/// Configuration constant: key api key
pub const KEY_API_KEY: &str = "api_key";
/// Configuration constant: key session
pub const KEY_SESSION: &str = "session";
