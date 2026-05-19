// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME-specific error types.

/// Errors that can occur during ACME operations.
#[derive(Debug, thiserror::Error)]
pub enum AcmeError {
    /// HTTP request to ACME server failed.
    #[error("ACME HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// ACME server returned an error response.
    #[error("ACME server error: {status} — {detail}")]
    Server {
        /// HTTP status code.
        status: u16,
        /// Error detail from ACME response.
        detail: String,
    },

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Certificate storage I/O error.
    #[error("storage I/O: {0}")]
    Storage(#[from] std::io::Error),

    /// Certificate parsing error.
    #[error("certificate parse error: {0}")]
    CertParse(String),

    /// Challenge validation failed.
    #[error("challenge validation failed: {0}")]
    ChallengeFailed(String),

    /// Account key error.
    #[error("account key error: {0}")]
    AccountKey(String),

    /// Order is in an unexpected state.
    #[error("order state error: expected {expected}, got {actual}")]
    OrderState {
        /// Expected state.
        expected: String,
        /// Actual state.
        actual: String,
    },

    /// Certificate has expired.
    #[error("certificate expired: {0}")]
    Expired(String),

    /// Configuration error.
    #[error("configuration: {0}")]
    Config(String),
}
