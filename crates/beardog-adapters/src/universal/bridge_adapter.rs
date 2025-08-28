pub struct BridgeAdapter {
    source_protocol: String,
    target_protocol: String,
}
impl BridgeAdapter {
    pub fn new(source_protocol: &str, target_protocol: &str) -> Self {
        Self {
            source_protocol: source_protocol.to_string(),
            target_protocol: target_protocol.to_string(),
        }
    }

    pub fn source_protocol(&self) -> &str {
        &self.source_protocol
    }

    pub fn target_protocol(&self) -> &str {
        &self.target_protocol
    }
}
