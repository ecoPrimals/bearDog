//! Zero-Copy API Request/Response Handlers for BearDog
//!
//! **High-Performance HTTP Processing with Minimal Allocations**
//!
//! This module provides zero-copy HTTP request and response handling that
//! minimizes memory allocations and data copying in hot API paths.

pub mod buffer_pool;
pub mod json_serializer;
pub mod request_parser;
pub mod response_builder;
pub mod types;

// Re-export commonly used types for convenience
pub use buffer_pool::{HttpBufferPool, HttpBufferPoolStats};
pub use json_serializer::{ZeroCopyJsonSerializer, ZeroCopySerializerStats};
pub use request_parser::{ZeroCopyRequestParser, ZeroCopyRequestStats};
pub use response_builder::{ZeroCopyResponseBuilder, ZeroCopyResponseStats};
pub use types::*;
