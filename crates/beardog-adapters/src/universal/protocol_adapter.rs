// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub struct ProtocolAdapter {
    protocols: Vec<String>,
}
impl ProtocolAdapter {
    /// New operation.
    /// Creates a new instance
    pub fn new(protocols: Vec<&str>) -> Self {
        Self {
            protocols: protocols
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        }
    }

    /// Supported Protocols operation.
    pub fn supported_protocols(&self) -> &[String] {
        &self.protocols
    }
}
