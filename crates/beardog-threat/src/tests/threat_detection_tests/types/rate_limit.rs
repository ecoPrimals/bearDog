//! Rate Limiting Test Types

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct RateLimitDetector {
    requests: HashMap<String, Vec<Instant>>,
    limit: usize,
    window: Duration,
}

impl RateLimitDetector {
    pub fn new(limit: usize, window: Duration) -> Self {
        Self {
            requests: HashMap::new(),
            limit,
            window,
        }
    }

    pub fn check_rate_limit(&mut self, identifier: &str) -> bool {
        let now = Instant::now();
        let cutoff = now - self.window;

        let requests = self.requests.entry(identifier.to_string()).or_default();

        // Remove old requests
        requests.retain(|&time| time > cutoff);

        // Check if limit exceeded
        if requests.len() >= self.limit {
            return false;
        }

        // Record new request
        requests.push(now);
        true
    }

    pub fn check_rate(&mut self, identifier: &str) -> bool {
        // Returns true if rate limit is EXCEEDED (flagged), false if OK
        !self.check_rate_limit(identifier)
    }
}
