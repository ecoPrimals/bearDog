// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub struct BridgeAdapter {
    source_protocol: String,
    target_protocol: String,
}
impl BridgeAdapter {
    /// New operation.
    /// Creates a new instance
    pub fn new(&str, target_protocol: &str) -> Self {
        Self {
            source_protocol: source_protocol.to_string(),
            target_protocol: target_protocol.to_string(),
        }
    }

    /// Source Protocol operation.
    pub fn source_protocol(&self) -> &str {
        &self.source_protocol
    }

    /// Target Protocol operation.
    pub fn target_protocol(&self) -> &str {
        &self.target_protocol
    }
}
