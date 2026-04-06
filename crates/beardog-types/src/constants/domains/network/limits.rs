// SPDX-License-Identifier: AGPL-3.0-or-later

//! Message size, rate, and bandwidth limits.

/// Connection limits
pub const MAX_CONNECTIONS: usize = 10000;
/// Configuration constant: min connections
pub const MIN_CONNECTIONS: usize = 1;
/// Configuration constant: max connections per ip
pub const MAX_CONNECTIONS_PER_IP: usize = 100;
/// Configuration constant: max concurrent connections
pub const MAX_CONCURRENT_CONNECTIONS: usize = 1000;

/// Buffer and message limits
pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024; // 16MB
/// Configuration constant: min message size
pub const MIN_MESSAGE_SIZE: usize = 1;
/// Configuration constant: max header size
pub const MAX_HEADER_SIZE: usize = 8192;
/// Configuration constant: max url length
pub const MAX_URL_LENGTH: usize = 2048;
/// Configuration constant: max query string length
pub const MAX_QUERY_STRING_LENGTH: usize = 4096;

/// Request limits
pub const MAX_REQUEST_SIZE: usize = 100 * 1024 * 1024; // 100MB
/// Configuration constant: max response size
pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024; // 100MB
/// Configuration constant: max upload size
pub const MAX_UPLOAD_SIZE: usize = 1024 * 1024 * 1024; // 1GB
/// Configuration constant: max download size
pub const MAX_DOWNLOAD_SIZE: usize = 1024 * 1024 * 1024; // 1GB

/// Rate limiting
pub const MAX_REQUESTS_PER_SECOND: u32 = 1000;
/// Configuration constant: max requests per minute
pub const MAX_REQUESTS_PER_MINUTE: u32 = 60000;
/// Configuration constant: max requests per hour
pub const MAX_REQUESTS_PER_HOUR: u32 = 3_600_000;
/// Configuration constant: default rate limit
pub const DEFAULT_RATE_LIMIT: u32 = 100;

/// Bandwidth limits
pub const MAX_BANDWIDTH_BYTES_PER_SEC: u64 = 100 * 1024 * 1024; // 100MB/s
/// Configuration constant: default bandwidth limit
pub const DEFAULT_BANDWIDTH_LIMIT: u64 = 10 * 1024 * 1024; // 10MB/s

/// Connection pool limits
pub const MAX_CONNECTION_POOL_SIZE: usize = 100;
/// Configuration constant: min connection pool size
pub const MIN_CONNECTION_POOL_SIZE: usize = 1;
/// Configuration constant: max idle connections
pub const MAX_IDLE_CONNECTIONS: usize = 10;
/// Configuration constant: min idle connections
pub const MIN_IDLE_CONNECTIONS: usize = 0;
