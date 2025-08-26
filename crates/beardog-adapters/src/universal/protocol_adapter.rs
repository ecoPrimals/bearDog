

pub struct ProtocolAdapter {
    protocols: Vec<String>,
}
impl ProtocolAdapter {

    pub fn new(protocols: Vec<&str>) -> Self {
        Self { protocols }
    }

    pub fn supported_protocols(&self) -> &[String] {
        &self.protocols
