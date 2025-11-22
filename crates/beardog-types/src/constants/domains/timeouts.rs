//! # Timeout Constants - UNIFIED
//!
//! **CANONICAL TIMEOUT CONSTANTS** - Single source of truth for all timeout durations.
//!
//! This module consolidates scattered timeout constants across the BearDog ecosystem,
//! providing a unified location for all timeout configurations used in:
//! - Network operations (connection, read, write)
//! - Health checks and monitoring
//! - Service discovery
//! - HSM operations
//! - AI/ML operations
//! - Database operations
//! - Cache operations
//!
//! ## Design Philosophy
//!
//! Timeout durations are organized by:
//! 1. **Operation Type**: What kind of operation (connection, read, write)
//! 2. **System Component**: Which system uses it (network, HSM, discovery)
//! 3. **Performance Tier**: Fast (< 1s), Standard (1-30s), Long (> 30s)
//!
//! ## Usage Guidelines
//!
//! ```rust,no_run
//! use beardog_types::constants::domains::timeouts::*;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Network operation with timeout
//! let timeout = NETWORK_CONNECTION_TIMEOUT;
//! // tokio::time::timeout(timeout, connect_to_server()).await?;
//!
//! // Health check with appropriate timeout
//! let health_timeout = HEALTH_CHECK_TIMEOUT;
//! // tokio::time::timeout(health_timeout, check_service_health()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Timeout Categories
//!
//! - **Connection Timeouts** (5-30s): Establishing connections
//! - **Read/Write Timeouts** (10-60s): I/O operations
//! - **Operation Timeouts** (30-120s): Complex operations
//! - **Health Check Timeouts** (1-10s): Fast responsiveness checks
//! - **Discovery Timeouts** (5-30s): Service discovery operations
//! - **Long-Running Timeouts** (60s-10min): Batch operations, AI inference

use std::time::Duration;

// ============================================================================
// GENERAL NETWORK TIMEOUTS
// ============================================================================

/// Default connection timeout (10 seconds)
///
/// **Use for**:
/// - TCP connection establishment
/// - Initial handshake operations
/// - General purpose connections
///
/// **Rationale**: Balances responsiveness with network variability
pub const NETWORK_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Default read timeout (30 seconds)
///
/// **Use for**:
/// - Socket read operations
/// - Receiving responses
/// - Stream reading
///
/// **Rationale**: Allows for reasonable response times under load
pub const NETWORK_READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Default write timeout (30 seconds)
///
/// **Use for**:
/// - Socket write operations
/// - Sending requests
/// - Stream writing
///
/// **Rationale**: Matches read timeout for symmetry
pub const NETWORK_WRITE_TIMEOUT: Duration = Duration::from_secs(30);

/// Default operation timeout (60 seconds)
///
/// **Use for**:
/// - Complete request-response cycles
/// - End-to-end operations
/// - General timeouts when specific timeout is unknown
///
/// **Rationale**: 1 minute is standard for most operations
pub const DEFAULT_OPERATION_TIMEOUT: Duration = Duration::from_secs(60);

/// Idle connection timeout (5 minutes / 300 seconds)
///
/// **Use for**:
/// - Keep-alive connections
/// - Connection pool idle timeout
/// - Long-lived connection management
///
/// **Rationale**: Balances resource usage with connection reuse
pub const IDLE_CONNECTION_TIMEOUT: Duration = Duration::from_secs(300);

// ============================================================================
// HTTP/GRPC SPECIFIC TIMEOUTS
// ============================================================================

/// HTTP request timeout (30 seconds)
///
/// **Use for**:
/// - HTTP/HTTPS requests
/// - RESTful API calls
/// - Web service communications
///
/// **Rationale**: Standard web timeout
pub const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// gRPC request timeout (60 seconds)
///
/// **Use for**:
/// - gRPC unary calls
/// - Standard RPC operations
/// - Service-to-service communication
///
/// **Rationale**: Allows for complex server-side processing
pub const GRPC_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Streaming timeout (5 minutes / 300 seconds)
///
/// **Use for**:
/// - gRPC streaming operations
/// - WebSocket connections
/// - Server-Sent Events (SSE)
///
/// **Rationale**: Long-lived streams need extended timeout
pub const STREAMING_TIMEOUT: Duration = Duration::from_secs(300);

// ============================================================================
// HEALTH CHECK & MONITORING TIMEOUTS
// ============================================================================

/// Health check timeout (5 seconds)
///
/// **Use for**:
/// - Service health checks
/// - Liveness probes
/// - Quick responsiveness checks
///
/// **Rationale**: Fast detection of unhealthy services
pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

/// Health check interval (30 seconds)
///
/// **Use for**:
/// - Periodic health check scheduling
/// - Monitoring intervals
/// - Heartbeat intervals
///
/// **Rationale**: Balances monitoring overhead with detection speed
pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);

/// Readiness check timeout (10 seconds)
///
/// **Use for**:
/// - Kubernetes readiness probes
/// - Service startup checks
/// - Dependency readiness verification
///
/// **Rationale**: Allows time for initialization
pub const READINESS_CHECK_TIMEOUT: Duration = Duration::from_secs(10);

/// Metrics collection timeout (15 seconds)
///
/// **Use for**:
/// - Prometheus scrapes
/// - Metrics endpoint calls
/// - Performance data collection
///
/// **Rationale**: Allows time for metric calculation
pub const METRICS_COLLECTION_TIMEOUT: Duration = Duration::from_secs(15);

// ============================================================================
// SERVICE DISCOVERY TIMEOUTS
// ============================================================================

/// Service discovery timeout (15 seconds)
///
/// **Use for**:
/// - Service registry queries
/// - Capability discovery
/// - Provider lookup operations
///
/// **Rationale**: Allows for multi-source lookups
pub const SERVICE_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(15);

/// Provider discovery timeout (10 seconds)
///
/// **Use for**:
/// - Provider capability queries
/// - Adapter discovery
/// - Plugin detection
///
/// **Rationale**: Quick provider enumeration
pub const PROVIDER_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(10);

/// DNS resolution timeout (5 seconds)
///
/// **Use for**:
/// - DNS lookups
/// - Hostname resolution
/// - Service name resolution
///
/// **Rationale**: Fast DNS should respond quickly
pub const DNS_RESOLUTION_TIMEOUT: Duration = Duration::from_secs(5);

/// Registration timeout (20 seconds)
///
/// **Use for**:
/// - Service registration
/// - Provider registration
/// - Capability announcement
///
/// **Rationale**: Allows for validation and processing
pub const REGISTRATION_TIMEOUT: Duration = Duration::from_secs(20);

// ============================================================================
// HSM & CRYPTOGRAPHIC OPERATION TIMEOUTS
// ============================================================================

/// HSM operation timeout (30 seconds)
///
/// **Use for**:
/// - Hardware Security Module operations
/// - PKCS#11 calls
/// - TPM operations
///
/// **Rationale**: Hardware operations can be slower
pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

/// HSM probe timeout (5 seconds)
///
/// **Use for**:
/// - HSM device detection
/// - Hardware enumeration
/// - Quick availability checks
///
/// **Rationale**: Fast hardware detection
pub const HSM_PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// Cryptographic operation timeout (60 seconds)
///
/// **Use for**:
/// - Encryption/decryption operations
/// - Key derivation functions
/// - Signature generation/verification
///
/// **Rationale**: Complex crypto can take time
pub const CRYPTO_OPERATION_TIMEOUT: Duration = Duration::from_secs(60);

/// Key rotation timeout (2 minutes / 120 seconds)
///
/// **Use for**:
/// - Key rotation operations
/// - Certificate renewal
/// - Credential refresh
///
/// **Rationale**: Key operations involve multiple steps
pub const KEY_ROTATION_TIMEOUT: Duration = Duration::from_secs(120);

// ============================================================================
// AI/ML OPERATION TIMEOUTS
// ============================================================================

/// AI decision timeout (10 seconds)
///
/// **Use for**:
/// - AI-assisted decision making
/// - Quick inference operations
/// - Real-time AI responses
///
/// **Rationale**: AI should respond quickly for UX
pub const AI_DECISION_TIMEOUT: Duration = Duration::from_secs(10);

/// AI request timeout (30 seconds)
///
/// **Use for**:
/// - Standard AI/ML inference requests
/// - Model predictions
/// - AI API calls
///
/// **Rationale**: Standard AI operation time
pub const AI_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// AI batch timeout (5 minutes / 300 seconds)
///
/// **Use for**:
/// - Batch inference operations
/// - Multiple predictions
/// - Complex AI workflows
///
/// **Rationale**: Batch operations need more time
pub const AI_BATCH_TIMEOUT: Duration = Duration::from_secs(300);

// ============================================================================
// DATABASE & STORAGE TIMEOUTS
// ============================================================================

/// Database query timeout (30 seconds)
///
/// **Use for**:
/// - SQL queries
/// - NoSQL operations
/// - Database reads
///
/// **Rationale**: Standard database operation time
pub const DATABASE_QUERY_TIMEOUT: Duration = Duration::from_secs(30);

/// Database connection timeout (10 seconds)
///
/// **Use for**:
/// - Database connection establishment
/// - Connection pool acquisition
/// - Initial database handshake
///
/// **Rationale**: Fast connection required
pub const DATABASE_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Transaction timeout (60 seconds)
///
/// **Use for**:
/// - Database transactions
/// - Multi-operation atomic operations
/// - ACID compliance checks
///
/// **Rationale**: Transactions can span multiple operations
pub const TRANSACTION_TIMEOUT: Duration = Duration::from_secs(60);

/// File operation timeout (30 seconds)
///
/// **Use for**:
/// - File reads/writes
/// - Configuration loading
/// - Log file operations
///
/// **Rationale**: File I/O should be fast
pub const FILE_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

// ============================================================================
// CACHE OPERATION TIMEOUTS
// ============================================================================

/// Cache operation timeout (5 seconds)
///
/// **Use for**:
/// - Cache reads/writes
/// - Cache invalidation
/// - Quick cache operations
///
/// **Rationale**: Cache should be very fast
pub const CACHE_OPERATION_TIMEOUT: Duration = Duration::from_secs(5);

/// Cache warm-up timeout (2 minutes / 120 seconds)
///
/// **Use for**:
/// - Cache preloading
/// - Warm-up operations
/// - Bulk cache population
///
/// **Rationale**: Warming can involve many operations
pub const CACHE_WARMUP_TIMEOUT: Duration = Duration::from_secs(120);

// ============================================================================
// WORKFLOW & ORCHESTRATION TIMEOUTS
// ============================================================================

/// Workflow step timeout (5 minutes / 300 seconds)
///
/// **Use for**:
/// - Individual workflow steps
/// - Task execution
/// - Job processing
///
/// **Rationale**: Steps can be complex
pub const WORKFLOW_STEP_TIMEOUT: Duration = Duration::from_secs(300);

/// Workflow total timeout (30 minutes / 1800 seconds)
///
/// **Use for**:
/// - Complete workflow execution
/// - End-to-end process timeout
/// - Maximum workflow duration
///
/// **Rationale**: Complex workflows need extended time
pub const WORKFLOW_TOTAL_TIMEOUT: Duration = Duration::from_secs(1800);

/// Background job timeout (10 minutes / 600 seconds)
///
/// **Use for**:
/// - Background task execution
/// - Async job processing
/// - Scheduled operations
///
/// **Rationale**: Background jobs can be long-running
pub const BACKGROUND_JOB_TIMEOUT: Duration = Duration::from_secs(600);

// ============================================================================
// RETRY & BACKOFF TIMEOUTS
// ============================================================================

/// Initial retry delay (1 second)
///
/// **Use for**:
/// - First retry attempt delay
/// - Initial backoff duration
/// - Quick retry operations
///
/// **Rationale**: Fast first retry
pub const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(1);

/// Maximum retry delay (60 seconds)
///
/// **Use for**:
/// - Maximum backoff duration
/// - Retry delay cap
/// - Exponential backoff limit
///
/// **Rationale**: Prevents excessive wait times
pub const MAX_RETRY_DELAY: Duration = Duration::from_secs(60);

/// Retry timeout (5 minutes / 300 seconds)
///
/// **Use for**:
/// - Total retry operation timeout
/// - Maximum retry duration
/// - Retry abandonment threshold
///
/// **Rationale**: Eventually give up on retries
pub const RETRY_TIMEOUT: Duration = Duration::from_secs(300);

// ============================================================================
// GRACEFUL SHUTDOWN TIMEOUTS
// ============================================================================

/// Graceful shutdown timeout (30 seconds)
///
/// **Use for**:
/// - Service graceful shutdown
/// - In-flight request completion
/// - Resource cleanup
///
/// **Rationale**: Allow active operations to complete
pub const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

/// Force shutdown timeout (10 seconds)
///
/// **Use for**:
/// - Forced termination after graceful timeout
/// - Emergency shutdown
/// - Hard stop operations
///
/// **Rationale**: Must eventually force stop
pub const FORCE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

// ============================================================================
// BACKWARD COMPATIBILITY ALIASES
// ============================================================================

/// Deprecated: Use `NETWORK_CONNECTION_TIMEOUT` instead
#[deprecated(since = "3.1.0", note = "Use NETWORK_CONNECTION_TIMEOUT for clarity")]
pub const DEFAULT_TIMEOUT: Duration = NETWORK_CONNECTION_TIMEOUT;

/// Deprecated: Use `DEFAULT_OPERATION_TIMEOUT` instead
#[deprecated(since = "3.1.0", note = "Use DEFAULT_OPERATION_TIMEOUT for clarity")]
pub const DEFAULT_REQUEST_TIMEOUT: Duration = DEFAULT_OPERATION_TIMEOUT;

/// Deprecated: Use `HTTP_REQUEST_TIMEOUT` instead
#[deprecated(since = "3.1.0", note = "Use HTTP_REQUEST_TIMEOUT for clarity")]
pub const REQUEST_TIMEOUT: Duration = HTTP_REQUEST_TIMEOUT;

/// Deprecated: Use `NETWORK_CONNECTION_TIMEOUT` instead
#[deprecated(since = "3.1.0", note = "Use NETWORK_CONNECTION_TIMEOUT for clarity")]
pub const CONNECTION_TIMEOUT: Duration = NETWORK_CONNECTION_TIMEOUT;

/// Deprecated: Use `DEFAULT_OPERATION_TIMEOUT` instead
#[deprecated(since = "3.1.0", note = "Use DEFAULT_OPERATION_TIMEOUT for clarity")]
pub const OPERATION_TIMEOUT: Duration = DEFAULT_OPERATION_TIMEOUT;
