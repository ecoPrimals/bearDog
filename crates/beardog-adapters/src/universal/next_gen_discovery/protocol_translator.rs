

use super::types::{EcosystemAnalysis, ProtocolBridge, ProtocolTranslatorConfig, ServiceEndpoint};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
    translation_cache: HashMap<String, Vec<ServiceEndpoint>>,
}

impl ProtocolTranslator {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: ProtocolTranslatorConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config,
            translation_cache: HashMap::with_capacity(Vec<ServiceEndpoint>,
    ) -> Result<Vec<ServiceEndpoint>, BearDogError> {
        let mut compatible_services = Vec::new(&EcosystemAnalysis,
    ) -> Result<Vec<ProtocolBridge>, BearDogError> {
        let mut bridges = Vec::new();

        for incompatibility in &analysis.incompatibilities {
            let bridge = ProtocolBridge {
                id: Uuid::new_v4(&incompatibility.source_protocol,
                target_protocol: &incompatibility.target_protocol,
                bridge_type: "adaptive".to_string(),
                ),
            };
            bridges.push(bridge);
        }

        Ok(bridges)
    }

    /// Checks if protocol supported
    fn is_protocol_supported(&self, protocol: &str) -> bool {
        self.config
            .supported_protocols
            .contains(&ServiceEndpoint,
    ) -> Result<ServiceEndpoint, BearDogError> {

        let target_protocol = self.select_best_target_protocol(&endpoint.protocol)?;

        let mut translated = &endpoint;
        translated.protocol = target_protocol;

        if endpoint.protocol == "grpc" && translated.protocol == "http" {
            translated.endpoint_url = translated.endpoint_url.replace("grpc://", "https://");
        } else if endpoint.protocol == "http" && translated.protocol == "grpc" {
            translated.endpoint_url = translated.endpoint_url.replace("https://", "grpc://");
        }

        Ok(translated)
    }


    fn select_best_target_protocol(&self, source_protocol: &str) -> Result<String, BearDogError> {
        match source_protocol {
            "grpc" => Ok("http".to_string()),
            "http" => Ok("grpc".to_string()),
            "websocket" => Ok("http".to_string()),
            _ => {
                if let Some(fallback) = self.config.supported_protocols.first() {
                    Ok(fallback)
                } else {
                    Err(BearDogError::configuration(&str, target: &str) -> HashMap<String, String> {
        let mut config = HashMap::with_capacity(16);
        config.insert("source_protocol".to_string(), source);
        config.insert("target_protocol".to_string(), target);
        config.insert("translation_mode".to_string(), "adaptive");
        config.insert("cache_enabled".to_string(), "true".to_string());
        config
    }
}
