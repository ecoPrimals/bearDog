// **MODERNIZED**: Ecosystem integration types for BearDog
//
// This module provides comprehensive types for ecosystem integration,
// including service discovery, compute orchestration, and HSM management.

// Removed unused import: beardog_errors::BearDogError
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents an event in the ecosystem integration system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemEvent {
    pub id: Uuid,
    /// Type of the event
    /// The event type value
    pub event_type: String,
    /// Source service that generated the event
    /// The source value
    pub source: String,
    /// Optional target
    pub target: Option<String>,
    /// Event payload data
    /// Mapping of data
    pub data: HashMap<String, serde_json::Value>,
    /// Timestamp when the event was created
    pub timestamp: DateTime<Utc>,
    /// Priority level of the event
    /// The priority value
    pub priority: EventPriority,
    /// Current status of the event
    /// Current status of the component
    pub status: EventStatus,
}

/// Represents a node in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemNode {
    pub id: Uuid,
    /// Human-readable name of the node
    /// Name of the item
    pub name: String,
    /// Type of the node (service, compute, hsm, etc.)
    /// The node type value
    pub node_type: NodeType,
    /// The endpoint value
    pub endpoint: String,
    /// Current health status
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Node capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Node metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Last seen timestamp
    /// The last seen value
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// Low priority event
    Low,
    /// Medium priority event
    Medium,
    /// High priority event
    High,
    /// Critical priority event
    Critical,
}

/// Status of ecosystem events
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventStatus {
    /// Event is pending processing
    Pending,
    /// Event is currently being processed
    Processing,
    /// Event has been completed successfully
    Completed,
    /// Event processing failed
    Failed,
    /// Event was cancelled
    Cancelled,
}

/// Types of nodes in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of node
pub enum NodeType {
    /// Service node
    Service,
    /// Compute node
    Compute,
    /// HSM node
    Hsm,
    /// Storage node
    Storage,
    /// Gateway node
    Gateway,
    /// Custom node type
    Custom(String),
}

impl Default for EcosystemEvent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: "unknown".to_string(),
            source: "system".to_string(),
            target: None,
            data: HashMap::new(),
            timestamp: Utc::now(),
            priority: EventPriority::Medium,
            status: EventStatus::Pending,
        }
    }
}

impl Default for EcosystemNode {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "unnamed-node".to_string(),
            node_type: NodeType::Service,
            endpoint: std::env::var("BEARDOG_ENDPOINT").unwrap_or_else(|_| {
                std::env::var("ECOSYSTEM_BASE_URL")
                    .unwrap_or_else(|_| "https://beardog.ecoprimals.com".to_string())
            }),
            health_status: HealthStatus::Healthy,
            capabilities: vec!["security".to_string(), "hsm".to_string()],
            metadata: HashMap::new(),
            last_seen: Utc::now(),
        }
    }
}

impl EcosystemEvent {
    /// Creates a new ecosystem event
    /// Creates a new instance
    pub fn new(event_type: String, source: String) -> Self {
        Self {
            event_type,
            source,
            ..Default::default()
        }
    }

    /// Creates instance with target
    #[must_use]
    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Creates instance with priority
    #[must_use]
    pub const fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Adds data to the event
    /// Creates instance with data
    #[must_use]
    pub fn with_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.data.insert(key, value);
        self
    }
}

impl EcosystemNode {
    /// Creates a new ecosystem node
    /// Creates a new instance
    pub fn new(name: String, node_type: NodeType, endpoint: String) -> Self {
        Self {
            name,
            node_type,
            endpoint,
            ..Default::default()
        }
    }

    /// Adds a capability to the node
    /// Creates instance with capability
    #[must_use]
    pub fn with_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Adds metadata to the node
    /// Creates instance with metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Updates the health status of the node
    /// Updates health
    /// Updates health
    pub fn update_health(&mut self, status: HealthStatus) {
        self.health_status = status;
        self.last_seen = Utc::now();
    }
}
