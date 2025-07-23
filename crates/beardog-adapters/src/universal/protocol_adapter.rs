//! Protocol Adapter for Universal Service Integration
//!
//! Provides protocol-agnostic connectivity to service meshes.

/// Protocol adapter for universal service mesh integration
pub struct ProtocolAdapter {
    protocols: Vec<String>,
}

impl ProtocolAdapter {
    /// Create new protocol adapter
    pub fn new(protocols: Vec<String>) -> Self {
        Self { protocols }
    }

    /// Get supported protocols
    pub fn supported_protocols(&self) -> &[String] {
        &self.protocols
    }
}
