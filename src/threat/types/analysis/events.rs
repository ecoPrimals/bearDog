//! Security Event Types
//!
//! This module contains types for security events that can be analyzed
//! for threat detection and correlation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security event structure
///
/// Represents a security event that can be analyzed
/// for threat detection and correlation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Unique event identifier
    pub event_id: String,

    /// Event timestamp
    pub timestamp: DateTime<Utc>,

    /// Event type
    pub event_type: String,

    /// Source IP address
    pub source_ip: String,

    /// Destination IP address
    pub destination_ip: String,

    /// User ID associated with the event
    pub user_id: String,

    /// User agent string (if available)
    pub user_agent: Option<String>,

    /// Size of data involved in the event
    pub data_size: f64,

    /// Geographic location (if available)
    pub location: Option<String>,

    /// File hash (if applicable)
    pub file_hash: Option<String>,

    /// Additional event-specific data
    pub additional_data: HashMap<String, String>,
}

impl Default for SecurityEvent {
    fn default() -> Self {
        Self {
            event_id: String::new(),
            timestamp: Utc::now(),
            event_type: String::new(),
            source_ip: String::new(),
            destination_ip: String::new(),
            user_id: String::new(),
            user_agent: None,
            data_size: 0.0,
            location: None,
            file_hash: None,
            additional_data: HashMap::new(),
        }
    }
} 