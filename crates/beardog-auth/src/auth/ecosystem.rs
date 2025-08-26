

use std::collections::HashMap;
use beardog_errors::BearDogResult;
use super::types::*;
impl CrossNodeAuthEngine {

    pub fn get_ecosystem_capabilities(&self, node_id: &str) -> Vec<NodeCapability> {

        let mut capabilities = Vec::new();

        capabilities.push(NodeCapability::EncryptionStrength(256));
        capabilities.push(NodeCapability::SecurityAnalysis);

        if let Some(node_info) = self.known_nodes.get(node_id) {

            match node_info.node_type.as_str() {
                "storage" => {
                    capabilities.push(NodeCapability::StorageProvider);
                },
                "compute" => {
                    capabilities.push(NodeCapability::ComputeProvider);
                    capabilities.push(NodeCapability::HighThroughput);
                "security" => {
                    capabilities.push(NodeCapability::QuantumResistant);
                _ => {

                }
            }
        }
        capabilities
    }

    pub async fn evaluate_network_effects(
        &self,
        operation: &NetworkOperation,
    ) -> BearDogResult<NetworkEffectAnalysis> {

        let mut affected_nodes = Vec::new();
        let mut resource_requirements = HashMap::with_capacity(16);
        let mut security_implications = Vec::new();

        let predicted_impact = match operation.operation_type.as_str() {
            "key_generation" => {
                affected_nodes.extend(self.known_nodes.keys().cloned());
                resource_requirements.insert("cpu".to_string(), 0.3);
                resource_requirements.insert("memory".to_string(), 0.1);
                security_implications.push("new_key_distribution".to_string());
                0.6
            },
            "consensus_operation" => {

                affected_nodes.extend(self.known_nodes.keys().take(5).cloned());
                resource_requirements.insert("network".to_string(), 0.5);
                resource_requirements.insert("consensus".to_string(), 0.8);
                security_implications.push("consensus_validation_load".to_string());
                0.9
            _ => {

                resource_requirements.insert("cpu".to_string(), 0.1);
                0.2
        };
        let performance_impact = resource_requirements.values().sum::<f64>() / resource_requirements.len() as f64;
        Ok(NetworkEffectAnalysis {
            predicted_impact,
            affected_nodes,
            resource_requirements,
            performance_impact,
            security_implications,
        })

    pub async fn discover_node_capabilities(&self, node_id: &str) -> BearDogResult<Vec<NodeCapability>> {

        let capabilities = if node_id.starts_with("hsm_") {
            vec![
                NodeCapability::HsmOperations,
                NodeCapability::KeyGeneration,
                NodeCapability::DigitalSigning,
            ]
        } else if node_id.starts_with("storage_") {
                NodeCapability::DataStorage,
                NodeCapability::DataRetrieval,
        } else {
                NodeCapability::BasicOperations,
                NodeCapability::NetworkCommunication,
        Ok(capabilities)

    pub async fn analyze_network_effects(&self, capabilities: &[NodeCapability]) -> BearDogResult<f64> {

        let base_score = capabilities.len() as f64 * 0.1;

        let critical_bonus = capabilities.iter().map(|cap| match cap {
            NodeCapability::HsmOperations => 0.3,
            NodeCapability::KeyGeneration => 0.25,
            NodeCapability::DigitalSigning => 0.2,
            NodeCapability::DataStorage => 0.15,
            _ => 0.05,
        }).sum::<f64>();
        let network_effect = (base_score + critical_bonus).min(1.0);
        Ok(network_effect)
}
