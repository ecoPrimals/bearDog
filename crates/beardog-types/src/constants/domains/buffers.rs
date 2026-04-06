// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Buffer Size Constants - UNIFIED
//!
//! **CANONICAL BUFFER CONSTANTS** - Single source of truth for all buffer sizes.
//!
//! This module consolidates scattered buffer size constants across the `BearDog` ecosystem,
//! providing a unified location for all memory buffer configurations used in:
//! - Network I/O operations
//! - File system operations
//! - Cryptographic operations
//! - HSM communications
//! - Zero-copy patterns
//! - Performance-critical paths
//!
//! ## Design Philosophy
//!
//! Buffer sizes follow powers of 2 for optimal memory alignment and CPU cache efficiency:
//! - 1 KB (2^10) - Small messages, headers
//! - 4 KB (2^12) - Page-aligned, standard messages
//! - 8 KB (2^13) - Streaming, moderate payloads
//! - 16 KB (2^14) - Large messages, file chunks
//! - 64 KB (2^16) - Network MTU-friendly, bulk transfer
//! - 128 KB (2^17) - High-throughput operations
//! - 1 MB (2^20) - Maximum safe single allocation
//!
//! ## Usage Guidelines
//!
//! ```rust
//! use beardog_types::constants::domains::buffers::*;
//!
//! // Network operations
//! let mut network_buf = vec![0u8; NETWORK_BUFFER_SIZE];
//!
//! // File I/O
//! let mut file_buf = vec![0u8; FILE_BUFFER_SIZE];
//!
//! // Cryptographic operations
//! let mut crypto_buf = vec![0u8; CRYPTO_BUFFER_SIZE];
//! ```

// ============================================================================
// GENERAL PURPOSE BUFFER SIZES
// ============================================================================

/// Small buffer size (1 KB / 1,024 bytes)
///
/// **Use for**:
/// - Small messages (< 1 KB)
/// - HTTP/gRPC headers
/// - Short strings and metadata
/// - Control messages
///
/// **Memory alignment**: Fits in L1 cache (typically 32-64 KB)
pub const BUFFER_SIZE_SMALL: usize = 1_024;

/// Medium buffer size (4 KB / 4,096 bytes)
///
/// **Use for**:
/// - Standard messages (1-4 KB)
/// - Typical payloads
/// - General purpose buffering
/// - Page-aligned operations
///
/// **Memory alignment**: Matches OS page size (4 KB on most systems)
pub const BUFFER_SIZE_MEDIUM: usize = 4_096;

/// Large buffer size (16 KB / 16,384 bytes)
///
/// **Use for**:
/// - Large payloads (4-16 KB)
/// - File chunks
/// - Bulk data transfer
/// - Batched operations
pub const BUFFER_SIZE_LARGE: usize = 16_384;

/// Extra large buffer size (64 KB / 65,536 bytes)
///
/// **Use for**:
/// - Very large payloads (16-64 KB)
/// - Streaming data
/// - High-throughput operations
/// - Network MTU-friendly transfers
pub const BUFFER_SIZE_XLARGE: usize = 65_536;

/// Default buffer size (medium - 4 KB)
///
/// **Use for**: General purpose when size is unknown
pub const BUFFER_SIZE_DEFAULT: usize = BUFFER_SIZE_MEDIUM;

// ============================================================================
// NETWORK-SPECIFIC BUFFER SIZES
// ============================================================================

/// Network buffer size (64 KB / 65,536 bytes)
///
/// **Use for**:
/// - TCP socket buffers
/// - HTTP request/response bodies
/// - gRPC streaming
/// - WebSocket frames
///
/// **Rationale**: Matches typical network MTU and socket buffer sizes
pub const NETWORK_BUFFER_SIZE: usize = 65_536;

/// TCP buffer size (128 KB / 131,072 bytes)
///
/// **Use for**:
/// - High-throughput TCP connections
/// - Large message streaming
/// - Bulk data transfer
///
/// **Rationale**: Optimal for high-bandwidth, low-latency networks
pub const TCP_BUFFER_SIZE: usize = 131_072;

/// UDP packet buffer size (8 KB / 8,192 bytes)
///
/// **Use for**:
/// - UDP datagram buffers
/// - Service discovery broadcasts
/// - Health check packets
///
/// **Rationale**: Well below typical MTU (1500 bytes), allows for fragmentation
pub const UDP_PACKET_SIZE: usize = 8_192;

/// HTTP/gRPC header buffer size (8 KB / 8,192 bytes)
///
/// **Use for**:
/// - HTTP request/response headers
/// - gRPC metadata
/// - WebSocket handshakes
///
/// **Rationale**: Large enough for typical headers, prevents overflow attacks
pub const HTTP_HEADER_BUFFER_SIZE: usize = 8_192;

// ============================================================================
// FILE I/O BUFFER SIZES
// ============================================================================

/// File buffer size (128 KB / 131,072 bytes)
///
/// **Use for**:
/// - File reading/writing operations
/// - Configuration file loading
/// - Log file operations
///
/// **Rationale**: Balances memory usage with I/O efficiency
pub const FILE_BUFFER_SIZE: usize = 131_072;

/// Stream buffer size (8 KB / 8,192 bytes)
///
/// **Use for**:
/// - Streaming file reads
/// - Progressive parsing
/// - Line-by-line processing
///
/// **Rationale**: Small enough for memory efficiency, large enough to reduce syscalls
pub const STREAM_BUFFER_SIZE: usize = 8_192;

/// Large file transfer buffer (1 MB / 1,048,576 bytes)
///
/// **Use for**:
/// - Large file uploads/downloads
/// - Backup operations
/// - Bulk file transfers
///
/// **Rationale**: Maximizes throughput for large files
pub const LARGE_FILE_BUFFER_SIZE: usize = 1_048_576;

// ============================================================================
// CRYPTOGRAPHIC OPERATION BUFFER SIZES
// ============================================================================

/// Cryptographic buffer size (16 KB / 16,384 bytes)
///
/// **Use for**:
/// - Encryption/decryption operations
/// - Hash computations
/// - Signature verification
/// - Key derivation
///
/// **Rationale**: Optimal for block cipher operations and hash functions
pub const CRYPTO_BUFFER_SIZE: usize = 16_384;

/// HSM buffer size (4 KB / 4,096 bytes)
///
/// **Use for**:
/// - HSM command/response buffers
/// - PKCS#11 operations
/// - Hardware security token communications
///
/// **Rationale**: Matches typical HSM message size limits
pub const HSM_BUFFER_SIZE: usize = 4_096;

/// Entropy buffer size (1 KB / 1,024 bytes)
///
/// **Use for**:
/// - Random number generation
/// - Entropy collection
/// - Nonce generation
///
/// **Rationale**: Sufficient for most entropy needs, efficient collection
pub const ENTROPY_BUFFER_SIZE: usize = 1_024;

// ============================================================================
// ZERO-COPY & PERFORMANCE BUFFERS
// ============================================================================

/// Zero-copy shared buffer size (64 KB / 65,536 bytes)
///
/// **Use for**:
/// - Arc-wrapped shared buffers
/// - Zero-copy message passing
/// - Shared cache entries
///
/// **Rationale**: Large enough for most messages, efficient for sharing
pub const ZERO_COPY_BUFFER_SIZE: usize = 65_536;

/// DMA-aligned buffer size (4 KB / 4,096 bytes)
///
/// **Use for**:
/// - Direct Memory Access operations
/// - Hardware-accelerated I/O
/// - Page-aligned allocations
///
/// **Rationale**: Matches page size for DMA efficiency
pub const DMA_BUFFER_SIZE: usize = 4_096;

// ============================================================================
// PROTOCOL-SPECIFIC BUFFER SIZES
// ============================================================================

/// BSTP (`BearDog` Secure Transport Protocol) buffer size (32 KB / 32,768 bytes)
///
/// **Use for**:
/// - BSTP message frames
/// - Sovereign protocol communications
///
/// **Rationale**: Optimized for BSTP message size limits
pub const BSTP_BUFFER_SIZE: usize = 32_768;

/// Discovery protocol buffer size (8 KB / 8,192 bytes)
///
/// **Use for**:
/// - Service discovery messages
/// - Capability announcements
/// - Health check responses
///
/// **Rationale**: Sufficient for typical discovery payloads
pub const DISCOVERY_BUFFER_SIZE: usize = 8_192;

// ============================================================================
// MAXIMUM SIZE LIMITS
// ============================================================================

/// Maximum safe buffer size (10 MB / 10,485,760 bytes)
///
/// **Use for**: Upper limit validation, prevents excessive allocations
///
/// **Rationale**: Large enough for legitimate use cases, prevents `DoS` attacks
pub const MAX_BUFFER_SIZE: usize = 10 * 1024 * 1024;

/// Minimum buffer size (256 bytes)
///
/// **Use for**: Lower limit validation, ensures efficiency
///
/// **Rationale**: Below this, overhead exceeds benefit
pub const MIN_BUFFER_SIZE: usize = 256;

// ============================================================================
// MEMORY POOL PREALLOCATION SIZES
// ============================================================================

/// Memory pool preallocation sizes
///
/// Pre-allocating buffers in pools reduces allocation overhead and improves
/// performance in hot paths. These constants define pool sizes for different
/// buffer tiers.
pub mod pool_sizes {
    /// Number of small buffers to preallocate (1 KB each)
    ///
    /// **Total memory**: ~100 KB
    pub const SMALL_POOL_COUNT: usize = 100;

    /// Number of medium buffers to preallocate (4 KB each)
    ///
    /// **Total memory**: ~200 KB
    pub const MEDIUM_POOL_COUNT: usize = 50;

    /// Number of large buffers to preallocate (16 KB each)
    ///
    /// **Total memory**: ~160 KB
    pub const LARGE_POOL_COUNT: usize = 10;

    /// Number of network buffers to preallocate (64 KB each)
    ///
    /// **Total memory**: ~320 KB
    pub const NETWORK_POOL_COUNT: usize = 5;
}

// ============================================================================
// BACKWARD COMPATIBILITY ALIASES
// ============================================================================

/// Deprecated: Use `NETWORK_BUFFER_SIZE` instead
#[deprecated(since = "3.1.0", note = "Use NETWORK_BUFFER_SIZE for clarity")]
pub const DEFAULT_BUFFER_SIZE_NETWORK: usize = NETWORK_BUFFER_SIZE;

/// Deprecated: Use `FILE_BUFFER_SIZE` instead
#[deprecated(since = "3.1.0", note = "Use FILE_BUFFER_SIZE for clarity")]
pub const DEFAULT_BUFFER_SIZE_FILE: usize = FILE_BUFFER_SIZE;
