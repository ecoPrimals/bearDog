pub struct ProtocolAdapter {
    protocols: Vec<String>,
}
impl ProtocolAdapter {
    pub fn new(protocols: Vec<&str>) -> Self {
        Self {
            protocols: protocols.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn supported_protocols(&self) -> &[String] {
        &self.protocols
    }
}
