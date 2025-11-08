//! Buffer Size Constants
//!
//! Memory buffer size constants for safe memory operations,
//! zero-copy patterns, and performance-critical paths.

/// Small buffer size (1 KB)
/// 
/// Use for: Small messages, headers, short strings
pub const BUFFER_SIZE_SMALL: usize = 1024;

/// Medium buffer size (4 KB)
/// 
/// Use for: Standard messages, typical payloads, general purpose
pub const BUFFER_SIZE_MEDIUM: usize = 4096;

/// Large buffer size (16 KB)
/// 
/// Use for: Large payloads, file chunks, bulk data transfer
pub const BUFFER_SIZE_LARGE: usize = 16384;

/// Extra large buffer size (64 KB)
/// 
/// Use for: Very large payloads, streaming data
pub const BUFFER_SIZE_XLARGE: usize = 65536;

/// Default buffer size (medium)
pub const BUFFER_SIZE_DEFAULT: usize = BUFFER_SIZE_MEDIUM;

/// Memory pool preallocation sizes
pub mod pool_sizes {
    /// Number of small buffers to preallocate
    pub const SMALL_POOL_COUNT: usize = 100;
    
    /// Number of medium buffers to preallocate
    pub const MEDIUM_POOL_COUNT: usize = 50;
    
    /// Number of large buffers to preallocate
    pub const LARGE_POOL_COUNT: usize = 10;
}

