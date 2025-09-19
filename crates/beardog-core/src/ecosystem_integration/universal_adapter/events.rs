// Event System for Universal Adapter

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Event subscriber
#[derive(Debug, Clone)]
pub struct EventSubscriber {
    pub id: Uuid,
    /// Name of the item
    pub name: String,
    /// Collection of event types
    pub event_types: Vec<String>,
}

impl EventSubscriber {
    /// Creates a new instance
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
