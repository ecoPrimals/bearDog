pub mod bridge_adapter;
pub mod capability_adapter;
pub mod http_adapter;
pub mod protocol_adapter;
pub mod security_provider_bridge;

pub mod commercial_extraction;
pub mod service_registration;

pub use bridge_adapter::BridgeAdapter;
pub use capability_adapter::{BearDogCapabilityAdapter, ServiceCapability, ServiceMeshConnector};
pub use http_adapter::HttpAdapter;
pub use protocol_adapter::ProtocolAdapter;
pub use security_provider_bridge::{BridgeConfig, SecurityProviderBridge};

pub use commercial_extraction::*;
pub use service_registration::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
