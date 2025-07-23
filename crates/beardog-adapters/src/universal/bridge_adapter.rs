//! Bridge Adapter for Universal Service Integration
//!
//! Provides bridging between different service mesh protocols.

/// Bridge adapter for connecting different service mesh types
pub struct BridgeAdapter {
    source_protocol: String,
    target_protocol: String,
}

impl BridgeAdapter {
    /// Create new bridge adapter
    pub fn new(source_protocol: String, target_protocol: String) -> Self {
        Self {
            source_protocol,
            target_protocol,
        }
    }

    /// Get source protocol
    pub fn source_protocol(&self) -> &str {
        &self.source_protocol
    }

    /// Get target protocol  
    pub fn target_protocol(&self) -> &str {
        &self.target_protocol
    }
}
