// SPDX-License-Identifier: AGPL-3.0-only

//! # Limits & Constraints Constants - UNIFIED
//!
//! **CANONICAL LIMIT CONSTANTS** - Single source of truth for all system limits.
//!
//! This module consolidates scattered limit and constraint constants across the
//! `BearDog` ecosystem, providing a unified location for all maximum/minimum values:
//! - Connection limits
//! - Request/response size limits
//! - Retry limits
//! - Concurrency limits
//! - Resource usage limits
//! - Rate limiting thresholds
//!
//! ## Design Philosophy
//!
//! Limits are organized by:
//! 1. **Resource Type**: Connections, memory, concurrency
//! 2. **Operation Type**: Requests, retries, batches
//! 3. **Safety Tier**: Conservative (safe), Standard (balanced), Aggressive (performance)
//!
//! ## Usage Guidelines
//!
//! ```rust
//! use beardog_types::constants::domains::limits::*;
//!
//! # fn example(active_connections: usize, request_size: usize, retry_count: u32) -> Result<(), &'static str> {
//! // Validate connection count
//! if active_connections > MAX_CONNECTIONS {
//!     return Err("Too many connections");
//! }
//!
//! // Limit request size
//! if request_size > MAX_REQUEST_SIZE {
//!     return Err("Request too large");
//! }
//!
//! // Enforce retry limit
//! if retry_count >= MAX_RETRIES {
//!     return Err("Max retries exceeded");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Safety Considerations
//!
//! These limits protect against:
//! - Denial of Service (`DoS`) attacks
//! - Resource exhaustion
//! - Memory leaks
//! - Runaway loops
//! - Excessive retries

use std::time::Duration;

// ============================================================================
// CONNECTION LIMITS
// ============================================================================

/// Maximum concurrent connections (1,000)
///
/// **Use for**:
/// - Total active connections limit
/// - Connection pool maximum size
/// - Server connection cap
///
/// **Rationale**: Balances throughput with resource usage
pub const MAX_CONNECTIONS: usize = 1000;

/// Maximum connections per client (100)
///
/// **Use for**:
/// - Per-client connection limit
/// - Client-specific throttling
/// - Fair usage enforcement
///
/// **Rationale**: Prevents single client monopolization
pub const MAX_CONNECTIONS_PER_CLIENT: usize = 100;

/// Minimum connection pool size (5)
///
/// **Use for**:
/// - Connection pool minimum
/// - Always-ready connections
/// - Warm pool size
///
/// **Rationale**: Ensures connections always available
pub const MIN_CONNECTION_POOL_SIZE: usize = 5;

/// Maximum connection pool size (100)
///
/// **Use for**:
/// - Connection pool maximum
/// - Resource cap
/// - Pool size limit
///
/// **Rationale**: Prevents excessive resource usage
pub const MAX_CONNECTION_POOL_SIZE: usize = 100;

// ============================================================================
// REQUEST/RESPONSE SIZE LIMITS
// ============================================================================

/// Maximum request size (10 MB / 10,485,760 bytes)
///
/// **Use for**:
/// - HTTP/gRPC request body limit
/// - API request size validation
/// - Input size checks
///
/// **Rationale**: Prevents memory exhaustion attacks
pub const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024;

/// Maximum response size (100 MB / 104,857,600 bytes)
///
/// **Use for**:
/// - HTTP/gRPC response body limit
/// - API response size validation
/// - Output size checks
///
/// **Rationale**: Allows large responses, prevents abuse
pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024;

/// Maximum message size (16 MB / 16,777,216 bytes)
///
/// **Use for**:
/// - Message queue message size
/// - Internal communication limit
/// - Event size validation
///
/// **Rationale**: Standard message size limit
pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

/// Maximum header size (8 KB / 8,192 bytes)
///
/// **Use for**:
/// - HTTP header size limit
/// - Metadata size validation
/// - Header overflow prevention
///
/// **Rationale**: Prevents header-based attacks
pub const MAX_HEADER_SIZE: usize = 8192;

// ============================================================================
// RETRY & BACKOFF LIMITS
// ============================================================================

/// Maximum retry attempts (3)
///
/// **Use for**:
/// - Operation retry limit
/// - Transient failure handling
/// - Network request retries
///
/// **Rationale**: 3 retries is industry standard
pub const MAX_RETRIES: u32 = 3;

/// Maximum aggressive retry attempts (5)
///
/// **Use for**:
/// - Critical operations
/// - High-importance retries
/// - Extended retry scenarios
///
/// **Rationale**: Some operations warrant more attempts
pub const MAX_RETRIES_AGGRESSIVE: u32 = 5;

/// Maximum conservative retry attempts (1)
///
/// **Use for**:
/// - Fast-fail operations
/// - Idempotency concerns
/// - Quick failure detection
///
/// **Rationale**: Some operations should fail fast
pub const MAX_RETRIES_CONSERVATIVE: u32 = 1;

/// Maximum retry delay (60 seconds)
///
/// **Use for**:
/// - Exponential backoff cap
/// - Maximum wait between retries
/// - Backoff upper limit
///
/// **Rationale**: Prevents excessive wait times
pub const MAX_RETRY_DELAY: Duration = Duration::from_secs(60);

// ============================================================================
// CONCURRENCY & PARALLELISM LIMITS
// ============================================================================

/// Maximum concurrent requests (100)
///
/// **Use for**:
/// - In-flight request limit
/// - Concurrent operation cap
/// - Parallelism control
///
/// **Rationale**: Balances throughput with resource usage
pub const MAX_CONCURRENT_REQUESTS: usize = 100;

/// Maximum concurrent operations per client (10)
///
/// **Use for**:
/// - Per-client concurrency limit
/// - Fair resource allocation
/// - Client-specific throttling
///
/// **Rationale**: Prevents single client monopolization
pub const MAX_CONCURRENT_OPERATIONS_PER_CLIENT: usize = 10;

/// Maximum parallel tasks (16)
///
/// **Use for**:
/// - Thread pool size
/// - Parallel processing limit
/// - CPU-bound task cap
///
/// **Rationale**: 16 is a reasonable default (works for 4-8 core systems)
/// **Note**: Adjust based on `std::thread::available_parallelism()` at runtime if needed
pub const MAX_PARALLEL_TASKS: usize = 16;

/// Minimum parallel tasks (2)
///
/// **Use for**:
/// - Minimum parallelism
/// - Thread pool minimum
/// - Always-available workers
///
/// **Rationale**: At least 2 for responsiveness
pub const MIN_PARALLEL_TASKS: usize = 2;

// ============================================================================
// BATCH & BULK OPERATION LIMITS
// ============================================================================

/// Maximum batch size (1,000 items)
///
/// **Use for**:
/// - Batch operation item limit
/// - Bulk processing cap
/// - Array operation maximum
///
/// **Rationale**: Balances efficiency with memory usage
pub const MAX_BATCH_SIZE: usize = 1000;

/// Maximum bulk operation size (10,000 items)
///
/// **Use for**:
/// - Large bulk operations
/// - Mass updates
/// - Bulk data transfer
///
/// **Rationale**: Allows large operations, prevents abuse
pub const MAX_BULK_OPERATION_SIZE: usize = 10000;

/// Recommended batch size (100 items)
///
/// **Use for**:
/// - Optimal batch size
/// - Default batching
/// - Efficient processing
///
/// **Rationale**: Good balance for most use cases
pub const RECOMMENDED_BATCH_SIZE: usize = 100;

// ============================================================================
// RATE LIMITING THRESHOLDS
// ============================================================================

/// Maximum requests per second (100)
///
/// **Use for**:
/// - Rate limiting threshold
/// - QPS (Queries Per Second) limit
/// - Throughput cap
///
/// **Rationale**: Prevents service overload
pub const MAX_REQUESTS_PER_SECOND: u32 = 100;

/// Maximum requests per minute (1,000)
///
/// **Use for**:
/// - Per-minute rate limit
/// - Burst tolerance
/// - Sustained rate cap
///
/// **Rationale**: Allows bursts while limiting sustained load
pub const MAX_REQUESTS_PER_MINUTE: u32 = 1000;

/// Maximum requests per hour (10,000)
///
/// **Use for**:
/// - Hourly rate limit
/// - Long-term throttling
/// - Fair usage enforcement
///
/// **Rationale**: Reasonable hourly limit
pub const MAX_REQUESTS_PER_HOUR: u32 = 10000;

// ============================================================================
// MEMORY & STORAGE LIMITS
// ============================================================================

/// Maximum cache size (50 MB / 52,428,800 bytes)
///
/// **Use for**:
/// - In-memory cache size limit
/// - Cache memory cap
/// - Cache eviction threshold
///
/// **Rationale**: Balances performance with memory usage
pub const MAX_CACHE_SIZE: usize = 50 * 1024 * 1024;

/// Maximum log file size (100 MB / 104,857,600 bytes)
///
/// **Use for**:
/// - Log file rotation threshold
/// - Maximum single log file size
/// - Log storage cap
///
/// **Rationale**: Prevents log files from growing unbounded
pub const MAX_LOG_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Maximum temporary file size (1 GB / 1,073,741,824 bytes)
///
/// **Use for**:
/// - Temp file size limit
/// - Scratch space cap
/// - Temporary storage validation
///
/// **Rationale**: Allows large temp files, prevents abuse
pub const MAX_TEMP_FILE_SIZE: u64 = 1024 * 1024 * 1024;

// ============================================================================
// QUEUE & BUFFER LIMITS
// ============================================================================

/// Maximum queue size (10,000 items)
///
/// **Use for**:
/// - Message queue capacity
/// - Task queue limit
/// - Buffer queue maximum
///
/// **Rationale**: Large enough for bursts, prevents memory exhaustion
pub const MAX_QUEUE_SIZE: usize = 10000;

/// Maximum pending operations (500)
///
/// **Use for**:
/// - Pending operation limit
/// - Backlog cap
/// - In-flight operation maximum
///
/// **Rationale**: Prevents operation backlog buildup
pub const MAX_PENDING_OPERATIONS: usize = 500;

/// Queue warning threshold (70% of max)
///
/// **Use for**:
/// - Queue capacity warning
/// - Backpressure trigger
/// - Load shedding threshold
///
/// **Rationale**: Early warning before queue full
pub const QUEUE_WARNING_THRESHOLD: usize = (MAX_QUEUE_SIZE * 70) / 100;

// ============================================================================
// TIMEOUT-RELATED LIMITS
// ============================================================================

/// Maximum timeout duration (10 minutes / 600 seconds)
///
/// **Use for**:
/// - Upper timeout limit
/// - Maximum wait time
/// - Timeout validation
///
/// **Rationale**: Prevents indefinite waits
pub const MAX_TIMEOUT_DURATION: Duration = Duration::from_secs(600);

/// Minimum timeout duration (1 second)
///
/// **Use for**:
/// - Lower timeout limit
/// - Minimum wait time
/// - Timeout validation
///
/// **Rationale**: Ensures meaningful timeout
pub const MIN_TIMEOUT_DURATION: Duration = Duration::from_secs(1);

// ============================================================================
// STRING & COLLECTION LIMITS
// ============================================================================

/// Maximum string length (1 MB / 1,048,576 chars)
///
/// **Use for**:
/// - String size validation
/// - Text field limit
/// - Maximum text size
///
/// **Rationale**: Prevents excessive string allocations
pub const MAX_STRING_LENGTH: usize = 1024 * 1024;

/// Maximum array length (100,000 items)
///
/// **Use for**:
/// - Array size validation
/// - Collection limit
/// - List maximum size
///
/// **Rationale**: Allows large collections, prevents abuse
pub const MAX_ARRAY_LENGTH: usize = 100_000;

/// Maximum map size (10,000 entries)
///
/// **Use for**:
/// - HashMap/BTreeMap size limit
/// - Dictionary maximum
/// - Key-value store cap
///
/// **Rationale**: Reasonable map size limit
pub const MAX_MAP_SIZE: usize = 10000;

// ============================================================================
// SECURITY & VALIDATION LIMITS
// ============================================================================

/// Maximum password length (128 chars)
///
/// **Use for**:
/// - Password validation
/// - Credential size limit
/// - Secret maximum length
///
/// **Rationale**: Generous limit, prevents abuse
pub const MAX_PASSWORD_LENGTH: usize = 128;

/// Minimum password length (12 chars)
///
/// **Use for**:
/// - Password strength requirement
/// - Minimum secure length
/// - Security policy enforcement
///
/// **Rationale**: Modern security standard
pub const MIN_PASSWORD_LENGTH: usize = 12;

/// Maximum username length (64 chars)
///
/// **Use for**:
/// - Username validation
/// - Account name limit
/// - Identity maximum length
///
/// **Rationale**: Reasonable username limit
pub const MAX_USERNAME_LENGTH: usize = 64;

/// Maximum API key length (256 chars)
///
/// **Use for**:
/// - API key validation
/// - Token size limit
/// - Credential maximum
///
/// **Rationale**: Allows long keys, prevents abuse
pub const MAX_API_KEY_LENGTH: usize = 256;

// ============================================================================
// PROTOCOL-SPECIFIC LIMITS
// ============================================================================

/// Maximum HSM operations per second (50)
///
/// **Use for**:
/// - HSM rate limiting
/// - Hardware operation throttling
/// - HSM load management
///
/// **Rationale**: Hardware limitations
pub const MAX_HSM_OPERATIONS_PER_SECOND: u32 = 50;

/// Maximum discovery retries (5)
///
/// **Use for**:
/// - Service discovery retry limit
/// - Provider discovery attempts
/// - Discovery failure handling
///
/// **Rationale**: Discovery warrants more retries
pub const MAX_DISCOVERY_RETRIES: u32 = 5;

/// Maximum workflow steps (100)
///
/// **Use for**:
/// - Workflow complexity limit
/// - Maximum pipeline stages
/// - Step count validation
///
/// **Rationale**: Prevents runaway workflows
pub const MAX_WORKFLOW_STEPS: usize = 100;

// ============================================================================
// BACKWARD COMPATIBILITY ALIASES
// ============================================================================

/// Deprecated: Use `MAX_CONNECTIONS` instead
#[deprecated(since = "3.1.0", note = "Use MAX_CONNECTIONS for clarity")]
pub const DEFAULT_MAX_CONNECTIONS: usize = MAX_CONNECTIONS;

/// Deprecated: Use `MAX_RETRIES` instead
#[deprecated(since = "3.1.0", note = "Use MAX_RETRIES for clarity")]
pub const DEFAULT_MAX_RETRIES: u32 = MAX_RETRIES;

/// Deprecated: Use `MAX_REQUEST_SIZE` instead
#[deprecated(since = "3.1.0", note = "Use MAX_REQUEST_SIZE for clarity")]
pub const DEFAULT_MAX_REQUEST_SIZE: usize = MAX_REQUEST_SIZE;
