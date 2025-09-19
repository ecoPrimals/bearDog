// Core Types for Universal Adapter System

use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Supported protocol types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of protocol
pub enum ProtocolType {
    /// HTTP/HTTPS protocol
    Http,
    /// WebSocket protocol
    WebSocket,
    /// gRPC protocol
    Grpc,
    /// MQTT protocol
    Mqtt,
    /// TCP socket
    Tcp,
    /// UDP socket
    Udp,
    /// Custom protocol
    Custom(String),
}

/// Adapter operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdapterOperation {
    /// Connect to service
    Connect,
    /// Disconnect from service
    Disconnect,
    /// Send request
    Request,
    /// Subscribe to events
    Subscribe,
    /// Unsubscribe from events
    Unsubscribe,
    /// Health check
    HealthCheck,
    /// Custom operation
    Custom(String),
}

/// Adapter request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterRequest {
    /// Request identifier
    pub request_id: Uuid,
    /// Operation type
    /// The operation value
    pub operation: AdapterOperation,
    /// Target endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Request headers
    /// Mapping of headers
    pub headers: HashMap<String, String>,
    /// Request payload
    /// Optional payload
    pub payload: Option<serde_json::Value>,
    /// Request metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Request timeout override
    pub timeout_ms: Option<u64>,
}

/// Adapter response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterResponse {
    /// Request identifier
    pub request_id: Uuid,
    /// Response status
    /// Current status of the component
    pub status: ResponseStatus,
    /// Response headers
    /// Mapping of headers
    pub headers: HashMap<String, String>,
    /// Response payload
    /// Optional payload
    pub payload: Option<serde_json::Value>,
    /// Response metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Processing duration in milliseconds
    /// Number of duration_ms
    pub duration_ms: u64,
}

/// Response status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    /// Successful response
    Success,
    /// Partial success
    PartialSuccess,
    /// Request failed
    Failed,
    /// Request timed out
    Timeout,
    /// Service unavailable
    ServiceUnavailable,
    /// Authentication failed
    AuthenticationFailed,
    /// Authorization failed
    AuthorizationFailed,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// Unknown error
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// Connection identifier
    pub connection_id: Uuid,
    /// Target endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Protocol type
    /// The protocol value
    pub protocol: ProtocolType,
    /// Connection status
    /// Current status of the component
    pub status: ConnectionStatus,
    /// Connection established timestamp
    /// The established at value
    pub established_at: DateTime<Utc>,
    /// Last activity timestamp
    /// The last activity value
    pub last_activity: DateTime<Utc>,
    /// Connection metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Connection is active
    Active,
    /// Connection is idle
    Idle,
    /// Connection is being established
    Connecting,
    /// Connection is being closed
    Closing,
    /// Connection is closed
    Closed,
    /// Connection failed
    Failed,
    /// Connection is in error state
    Error,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint identifier
    pub endpoint_id: String,
    /// Endpoint URL
    /// The url value
    pub url: String,
    /// Protocol type
    /// The protocol value
    pub protocol: ProtocolType,
    /// Health status
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Last health check timestamp
    /// The last health check value
    pub last_health_check: DateTime<Utc>,
    /// Endpoint metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl AdapterRequest {
    /// Create a new adapter request
    /// Creates a new instance
    pub fn new(operation: AdapterOperation, endpoint: String) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            operation,
            endpoint,
            headers: HashMap::new(),
            payload: None,
            metadata: HashMap::new(),
            timeout_ms: None,
        }
    }

    /// Add a header to the request
    /// Creates instance with header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Set the request payload
    /// Creates instance with payload
    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Add metadata to the request
    /// Creates instance with metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set request timeout
    /// Creates instance with timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }
}
