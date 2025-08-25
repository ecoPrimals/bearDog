// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Zero-Copy API Request/Response Handlers for BearDog
///
/// **High-Performance HTTP Processing with Minimal Allocations**
/// This module provides zero-copy HTTP request and response handling that
/// minimizes memory allocations and data copying in hot API paths.

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
