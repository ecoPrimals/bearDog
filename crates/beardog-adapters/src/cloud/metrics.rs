

#[derive(Debug, Clone)]
    /// Number of successful_requests
    pub successful_requests: u64,

    /// Number of failed_requests
    pub failed_requests: u64,


    pub avg_response_time_ms: f64,
}

impl CloudMetrics {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default(bool, response_time_ms: f64) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        self.avg_response_time_ms = (self.avg_response_time_ms * (self.total_requests - 1) as f64
            + response_time_ms)
            / self.total_requests as f64;
    }

/// Success Rate operation.
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.successful_requests as f64 / self.total_requests as f64
        }
    }
}
