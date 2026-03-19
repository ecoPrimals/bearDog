// SPDX-License-Identifier: AGPL-3.0-only

// Event System for Universal Adapter

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Event subscriber
/// Event subscriber for adapter events
///
/// Subscribes to specific event types from the universal adapter event system.
#[derive(Debug, Clone)]
pub struct EventSubscriber {
    /// Unique subscriber identifier
    pub id: Uuid,
    /// Subscriber name
    pub name: String,
    /// Event types to subscribe to
    pub event_types: Vec<String>,
}

impl EventSubscriber {
    /// Creates a new instance
    #[must_use]
    pub fn new(name: String, event_types: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            event_types,
        }
    }
}

/// Adapter event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterEvent {
    /// Event identifier
    pub event_id: Uuid,
    /// Event type
    /// The event type value
    pub event_type: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event source
    /// The source value
    pub source: String,
    /// Event payload
    /// The payload value
    pub payload: serde_json::Value,
    /// Event metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
