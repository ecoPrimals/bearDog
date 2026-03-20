// SPDX-License-Identifier: AGPL-3.0-only

//! HTTP client defaults and content types.

pub use super::defaults::{DEFAULT_HTTP_VERSION, DEFAULT_TLS_VERSION};
pub use super::timeouts::{HTTP_REQUEST_TIMEOUT, HTTP_RESPONSE_TIMEOUT};

/// HTTP headers
pub const CONTENT_TYPE_JSON: &str = "application/json";
/// Configuration constant: content type binary
pub const CONTENT_TYPE_BINARY: &str = "application/octet-stream";
/// Configuration constant: user agent
pub const USER_AGENT: &str = "BearDog/3.0";
