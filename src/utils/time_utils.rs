//! Time utilities for BearDog security operations

use chrono::{DateTime, Utc};

/// Get the current UTC timestamp
/// 
/// Returns the current time in UTC timezone as a DateTime object.
/// This is used throughout BearDog for consistent timestamping of
/// security events, audit logs, and other time-sensitive operations.
/// 
/// # Returns
/// 
/// Current UTC timestamp as `DateTime<Utc>`
/// 
/// # Example
/// 
/// ```rust
/// use beardog::utils::time_utils::current_timestamp;
/// 
/// let timestamp = current_timestamp();
/// println!("Current time: {}", timestamp);
/// ```
pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
} 