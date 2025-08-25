// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Security Event Types
///
/// This module contains types for security events that can be analyzed
/// for threat detection and correlation.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security event structure
/// Represents a security event that can be analyzed
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
impl SecurityEvent {
    /// Create a new SecurityEvent}


    pub fn new(
        event_id: String,
        event_type: String,
        source_ip: String,
        destination_ip: String,
        user_id: String,
    ) -> Self {
        Self {
            event_id,
            timestamp: Utc::now(),
            event_type,
            source_ip,
            destination_ip,
            user_id,
            user_agent: None,
            data_size: 0.0,
            location: None,
            file_hash: None,
            additional_data: HashMap::new(),
        }
    }
    /// Create a new SecurityEvent with additional data
    pub fn new_with_data(
        event_type: String,
        source_ip: String,
        additional_data: HashMap<String, String>,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            source_ip,
            timestamp: Utc::now(),
            severity: 1.0,
            data_size: 0.0,
            user_id: String::new(),
            additional_data,
        }
    }

    /// Check if the event is from an internal source
    pub fn is_internal_source(&self) -> bool {
        self.source_ip.starts_with("192.168.")
            || self.source_ip.starts_with("10.")
            || self.source_ip.starts_with("172.16.")
            || self.source_ip == "127.0.0.1"
    }

    /// Check if the event is recent (within specified seconds)


    pub fn is_recent(&self, seconds: u64) -> bool {
        let now = Utc::now();
        let diff = now.signed_duration_since(self.timestamp);
        diff.num_seconds() < seconds as i64
    }

    /// Check if the event involves a large data transfer
    pub fn is_large_data_transfer(&self) -> bool {
        self.data_size > 10_000_000.0 // 10MB threshold
    }

    /// Calculate severity score based on event attributes


    pub fn severity_score(&self) -> f64 {
        let mut score: f64 = 0.0;
        // Base score by event type
        match self.event_type.as_str() {
            "login_failure" => score += 0.3,
            "suspicious_activity" => score += 0.6,
            "malware_detected" => score += 0.9,
            "data_exfiltration" => score += 0.8,
            _ => score += 0.1,
        }
        
        // Add score for large data transfers
        if self.is_large_data_transfer() {
            score += 0.3;
        }
        
        // Add score for external sources
        if !self.is_internal_source() {
            score += 0.2;
        }
        
        score.min(1.0)
    }
}


impl Default for SecurityEvent {
    fn default() -> Self {
        Self {
            event_id: String::new(),
            event_type: String::new(),
            source_ip: String::new(),
            timestamp: Utc::now(),
            severity: 0.0,
            data_size: 0.0,
            user_id: String::new(),
            additional_data: HashMap::new(),
        }
    }
}
