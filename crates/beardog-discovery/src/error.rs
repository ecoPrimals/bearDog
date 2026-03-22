// SPDX-License-Identifier: AGPL-3.0-only

//! Error types for discovery

use thiserror::Error;

/// Convenient [`Result`] alias for discovery operations that fail with [`DiscoveryError`].
pub type Result<T> = std::result::Result<T, DiscoveryError>;

/// Errors surfaced by discovery, announcement, and configuration loading.
///
/// Variants are intentionally granular so callers can distinguish user misconfiguration,
/// missing peers, and transient infrastructure failures without string-matching.
#[derive(Error, Debug)]
pub enum DiscoveryError {
    /// TOML or logical configuration could not be applied (invalid file, missing keys, etc.).
    #[error("Configuration error: {0}")]
    Config(String),

    /// A discovery pass failed after exhausting configured methods or retries.
    #[error("Discovery failed: {0}")]
    DiscoveryFailed(String),

    /// No service matched the requested identity or filters.
    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    /// No provider advertised the requested capability (or version/features).
    #[error("Capability not found: {0}")]
    CapabilityNotFound(String),

    /// Registering or refreshing local announcement metadata failed.
    #[error("Announcement failed: {0}")]
    AnnouncementFailed(String),

    /// Transport-level failure (socket, DNS resolution, connectivity).
    #[error("Network error: {0}")]
    Network(String),

    /// Local filesystem or pipe I/O failure while reading config or state.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to parse URLs, TXT records, or other structured text.
    #[error("Parse error: {0}")]
    Parse(String),

    /// An operation exceeded its deadline.
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Endpoint URL or binding information was malformed or unusable.
    #[error("Invalid endpoint: {0}")]
    InvalidEndpoint(String),

    // New variants for complete implementations
    /// Registry or mDNS query returned an error from the backing API.
    #[error("Query failed: {0}")]
    QueryFailed(String),

    /// Advertised service metadata was incomplete or internally inconsistent.
    #[error("Invalid service information: {0}")]
    InvalidServiceInfo(String),

    /// A subsystem (browser, resolver, registry client) failed to start.
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),

    /// Catch-all for OS or runtime failures not modeled elsewhere.
    #[error("System error: {0}")]
    SystemError(String),

    /// The requested code path is intentionally absent or still being integrated.
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// A specific discovery backend (e.g. mDNS, Consul) is down or misconfigured.
    #[error("Backend unavailable: {provider}, reason: {reason}")]
    BackendUnavailable {
        /// Human-readable backend name (`"mdns"`, `"consul"`, etc.).
        provider: String,
        /// Why the backend could not be used for this request.
        reason: String,
    },

    /// Wrapper for errors forwarded from [`anyhow::Error`] at integration boundaries.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<toml::de::Error> for DiscoveryError {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(err.to_string())
    }
}

// Tower Atomic Evolution: HTTP removed, all external calls via Songbird
// impl From<reqwest::Error> for DiscoveryError {
//     fn from(err: reqwest::Error) -> Self {
//         DiscoveryError::Network(err.to_string())
//     }
// }

impl From<url::ParseError> for DiscoveryError {
    fn from(err: url::ParseError) -> Self {
        Self::Parse(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::DiscoveryError;

    #[test]
    fn from_url_parse_error_maps_to_parse_variant() {
        let e: DiscoveryError = url::Url::parse("http://[").unwrap_err().into();
        assert!(matches!(e, DiscoveryError::Parse(_)));
        assert!(!e.to_string().is_empty());
    }
}
