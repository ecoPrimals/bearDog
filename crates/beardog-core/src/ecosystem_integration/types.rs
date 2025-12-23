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
/// An event in the ecosystem event stream
///
/// Represents messages and notifications flowing between ecosystem services,
/// including metadata, payload, priority, and routing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemEvent {
    /// Unique event identifier
    pub id: Uuid,
    /// Type of the event (e.g., "service.started", "task.completed")
    pub event_type: String,
    /// Source service that generated the event
    pub source: String,
    /// Optional target service for directed events
    pub target: Option<String>,
    /// Event payload data as key-value pairs
    pub data: HashMap<String, serde_json::Value>,
    /// When the event was created
    pub timestamp: DateTime<Utc>,
    /// Priority level for event processing
    pub priority: EventPriority,
    /// Current processing status of the event
    pub status: EventStatus,
}

/// A node in the ecosystem
///
/// Represents a service, compute resource, or other participant in the ecosystem,
/// including its identity, capabilities, health, and connection information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemNode {
    /// Unique node identifier
    pub id: Uuid,
    /// Human-readable name of the node
    pub name: String,
    /// Type of the node (service, compute, HSM, etc.)
    pub node_type: NodeType,
    /// Connection endpoint URL or address
    pub endpoint: String,
    /// Current health status
    pub health_status: HealthStatus,
    /// Capabilities provided by this node
    pub capabilities: Vec<String>,
    /// Additional node metadata
    pub metadata: HashMap<String, String>,
    /// Last time the node was seen alive
    pub last_seen: DateTime<Utc>,
}

/// Priority level for ecosystem events
///
/// Determines the urgency and importance of events for routing,
/// handling, and alerting purposes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// Low priority event that can be handled asynchronously
    Low,
    /// Medium priority event requiring timely handling
    Medium,
    /// High priority event requiring immediate attention
    High,
    /// Critical priority event requiring urgent action
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
    #[must_use]
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
    #[must_use]
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
