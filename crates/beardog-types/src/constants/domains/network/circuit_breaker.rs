// SPDX-License-Identifier: AGPL-3.0-or-later

//! Circuit breaker thresholds and monitoring windows.

use std::time::Duration;

/// Circuit breaker states
pub const STATE_CLOSED: &str = "closed";
/// Configuration constant: state open
pub const STATE_OPEN: &str = "open";
/// Configuration constant: state half open
pub const STATE_HALF_OPEN: &str = "half_open";

/// Default thresholds
pub const DEFAULT_FAILURE_THRESHOLD: u32 = 5;
/// Configuration constant: default success threshold
pub const DEFAULT_SUCCESS_THRESHOLD: u32 = 3;
/// Configuration constant: default timeout
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
/// Configuration constant: default half open max calls
pub const DEFAULT_HALF_OPEN_MAX_CALLS: u32 = 3;

/// Monitoring intervals
pub const METRICS_WINDOW: Duration = Duration::from_secs(60);
/// Configuration constant: reset timeout
pub const RESET_TIMEOUT: Duration = Duration::from_secs(60);
/// Configuration constant: state check interval
pub const STATE_CHECK_INTERVAL: Duration = Duration::from_secs(10);
