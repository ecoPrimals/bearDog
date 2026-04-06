// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared request metadata for adapter certificate classification.

/// Request context for classification
///
/// Contains information about the requesting entity that helps determine
/// whether this is human use or commercial extraction.
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Stable id for logging and license lookup (e.g. hashed tenant id)
    pub requester_id: String,
    /// User agent string
    pub user_agent: String,
    /// Automation score (0.0 = human, 1.0 = fully automated)
    pub automation_score: f64,
    /// Pattern consistency (0.0 = random, 1.0 = perfectly consistent)
    pub pattern_consistency: f64,
    /// Number of requests in the last hour
    pub request_rate: u32,
    /// Whether TLS was used
    pub tls_enabled: bool,
    /// Source IP address
    pub source_ip: String,
    /// Optional: License key if present
    pub license_key: Option<String>,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            requester_id: "anonymous".to_string(),
            user_agent: "unknown".to_string(),
            automation_score: 0.0,
            pattern_consistency: 0.0,
            request_rate: 0,
            tls_enabled: false,
            source_ip: std::net::Ipv4Addr::UNSPECIFIED.to_string(),
            license_key: None,
        }
    }
}
