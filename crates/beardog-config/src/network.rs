//! Network and API configuration types
//!
//! Contains all networking, API, TLS, CORS, and authentication configuration structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network configuration for BearDog
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    /// HTTP server configuration
    pub http: HttpConfig,
    /// HTTPS server configuration
    pub https: HttpsConfig,
    /// WebSocket configuration
    pub websocket: WebSocketConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// CORS configuration
    pub cors: CorsConfig,
    /// Network timeout configuration
    pub timeout: TimeoutConfig,
    /// Load balancer configuration
    pub load_balancer: LoadBalancerConfig,
    /// Node communication configuration
    pub node_communication: NodeCommunicationConfig,
    /// Ecosystem federation configuration
    pub federation: FederationConfig,
}

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Enable HTTP server
    pub enabled: bool,
    /// HTTP server bind address
    pub bind_address: String,
    /// HTTP server port
    pub port: u16,
    /// Maximum request body size
    pub max_request_size: usize,
    /// Keep alive timeout
    pub keep_alive_timeout: u64,
}

/// HTTPS server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpsConfig {
    /// Enable HTTPS server
    pub enabled: bool,
    /// HTTPS server bind address
    pub bind_address: String,
    /// HTTPS server port
    pub port: u16,
    /// TLS certificate path
    pub cert_path: String,
    /// TLS private key path
    pub key_path: String,
    /// TLS certificate chain path
    pub cert_chain_path: Option<String>,
    /// Client certificate verification
    pub client_cert_verification: bool,
}

/// WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// Enable WebSocket server
    pub enabled: bool,
    /// WebSocket server bind address
    pub bind_address: String,
    /// WebSocket server port
    pub port: u16,
    /// Maximum frame size
    pub max_frame_size: usize,
    /// Ping interval in seconds
    pub ping_interval: u64,
    /// Pong timeout in seconds
    pub pong_timeout: u64,
}

/// Comprehensive rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Basic requests per minute
    pub requests_per_minute: u32,
    /// Burst capacity (how many requests can be made at once)
    pub burst_size: u32,
    /// Enable per-user rate limiting
    pub per_user_limiting: bool,
    /// Rate limit window in seconds
    pub window_seconds: u32,
    /// Maximum requests per minute (alias for requests_per_minute for backward compatibility)
    pub max_requests_per_minute: Option<u32>,
    /// Endpoint-specific limits
    pub endpoint_limits: HashMap<String, EndpointLimit>,
    /// User tier limits
    pub user_tier_limits: HashMap<String, UserTierLimit>,
}

/// Endpoint-specific rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointLimit {
    /// Requests per minute for this endpoint
    pub requests_per_minute: u32,
    /// Burst capacity for this endpoint
    pub burst_capacity: u32,
    /// Whether this endpoint has priority
    pub priority: bool,
}

/// User tier rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTierLimit {
    /// Requests per minute for this user tier
    pub requests_per_minute: u32,
    /// Burst capacity for this user tier
    pub burst_capacity: u32,
    /// Daily request limit
    pub daily_limit: u32,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Exposed headers
    pub exposed_headers: Vec<String>,
    /// Max age for preflight requests
    pub max_age: u32,
    /// Allow credentials
    pub allow_credentials: bool,
}

/// Network timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Read timeout in seconds
    pub read_timeout: u64,
    /// Write timeout in seconds
    pub write_timeout: u64,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Enable load balancer
    pub enabled: bool,
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Upstream servers
    pub upstream_servers: Vec<UpstreamServer>,
}

/// Load balancing algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round-robin distribution
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// IP hash
    IpHash,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check path
    pub path: String,
    /// Health check interval in seconds
    pub interval: u64,
    /// Health check timeout in seconds
    pub timeout: u64,
    /// Number of successful checks to mark healthy
    pub healthy_threshold: u32,
    /// Number of failed checks to mark unhealthy
    pub unhealthy_threshold: u32,
}

/// Upstream server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamServer {
    /// Server address
    pub address: String,
    /// Server weight for load balancing
    pub weight: u32,
    /// Maximum number of connections
    pub max_connections: u32,
    /// Server enabled status
    pub enabled: bool,
}

/// Node communication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCommunicationConfig {
    /// Enable node communication
    pub enabled: bool,
    /// Communication protocol
    pub protocol: CommunicationProtocol,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Authentication method
    pub auth_method: AuthMethod,
    /// API key for authentication (when using SharedSecret or Ed25519)
    pub api_key: Option<String>,
    /// Shared secret for authentication
    pub secret: Option<String>,
    /// Retry configuration
    pub retry: RetryConfig,
}

/// Communication protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationProtocol {
    /// HTTP/HTTPS protocol
    Http,
    /// gRPC protocol
    Grpc,
    /// WebSocket protocol
    WebSocket,
    /// TCP protocol
    Tcp,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Enable encryption
    pub enabled: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key exchange method
    pub key_exchange: String,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Private key path
    pub key_path: Option<String>,
}

/// Federation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    /// Enable federation
    pub enabled: bool,
    /// Federation discovery method
    pub discovery: FederationDiscovery,
    /// Trust configuration
    pub trust: TrustConfig,
    /// Sync configuration
    pub sync: SyncConfig,
}

/// Federation discovery method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationDiscovery {
    /// Static configuration
    Static,
    /// DNS discovery
    Dns,
    /// Consul discovery
    Consul,
    /// Kubernetes discovery
    Kubernetes,
}

/// Trust configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustConfig {
    /// Trust level threshold
    pub threshold: f32,
    /// Trust decay rate
    pub decay_rate: f32,
    /// Trust boost factor
    pub boost_factor: f32,
}

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Sync interval in seconds
    pub interval: u64,
    /// Sync timeout in seconds
    pub timeout: u64,
    /// Batch size for sync operations
    pub batch_size: u32,
}

/// Authentication method for node communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Shared secret authentication using a pre-configured key
    SharedSecret,
    /// Ed25519 authentication
    Ed25519,
    /// Mutual TLS authentication where both parties verify certificates
    Mutual,
}

/// Retry configuration for network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts before giving up
    pub max_attempts: u32,
    /// Initial delay in milliseconds before first retry
    pub initial_delay_ms: u32,
    /// Maximum delay in milliseconds between retries
    pub max_delay_ms: u32,
    /// Multiplier applied to delay after each failed attempt
    pub backoff_multiplier: f32,
}

// Default implementations

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: std::env::var("BEARDOG_HTTP_BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            port: std::env::var("BEARDOG_HTTP_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            max_request_size: std::env::var("BEARDOG_HTTP_MAX_REQUEST_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(16 * 1024 * 1024), // 16MB - increased from 1MB
            keep_alive_timeout: std::env::var("BEARDOG_HTTP_KEEP_ALIVE_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(60), // 60 seconds
        }
    }
}

impl Default for HttpsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bind_address: "127.0.0.1:443".to_string(),
            port: 443,
            cert_path: "certs/server.crt".to_string(),
            key_path: "certs/server.key".to_string(),
            cert_chain_path: None,
            client_cert_verification: false,
        }
    }
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bind_address: "127.0.0.1:8081".to_string(),
            port: 8081,
            max_frame_size: 1024 * 1024, // 1MB
            ping_interval: 30,           // 30 seconds
            pong_timeout: 10,            // 10 seconds
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 60,
            burst_size: 10,
            per_user_limiting: false,
            window_seconds: 60,
            max_requests_per_minute: None,
            endpoint_limits: HashMap::new(),
            user_tier_limits: HashMap::new(),
        }
    }
}

impl Default for EndpointLimit {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_capacity: 10,
            priority: false,
        }
    }
}

impl Default for UserTierLimit {
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            burst_capacity: 100,
            daily_limit: 10000,
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            exposed_headers: vec![],
            max_age: 3600,
            allow_credentials: false,
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            request_timeout: 30,
            connection_timeout: 30,
            read_timeout: 30,
            write_timeout: 30,
        }
    }
}

impl Default for LoadBalancerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_check: HealthCheckConfig::default(),
            upstream_servers: vec![],
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: "/health".to_string(),
            interval: 30,
            timeout: 10,
            healthy_threshold: 1,
            unhealthy_threshold: 3,
        }
    }
}

impl Default for UpstreamServer {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8080".to_string(),
            weight: 1,
            max_connections: 100,
            enabled: true,
        }
    }
}

impl Default for NodeCommunicationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            protocol: CommunicationProtocol::Http,
            encryption: EncryptionConfig::default(),
            auth_method: AuthMethod::SharedSecret,
            api_key: None,
            secret: None,
            retry: RetryConfig::default(),
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "AES-256-GCM".to_string(),
            key_exchange: "ECDHE-RSA-AES256-GCM-SHA384".to_string(),
            cert_path: None,
            key_path: None,
        }
    }
}

impl Default for FederationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            discovery: FederationDiscovery::Static,
            trust: TrustConfig::default(),
            sync: SyncConfig::default(),
        }
    }
}

impl Default for TrustConfig {
    fn default() -> Self {
        Self {
            threshold: 0.7,
            decay_rate: 0.1,
            boost_factor: 1.0,
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            interval: 60,
            timeout: 30,
            batch_size: 100,
        }
    }
}

impl Default for AuthMethod {
    fn default() -> Self {
        Self::SharedSecret
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 1000,
            backoff_multiplier: 2.0,
        }
    }
}
