//! Consensus authorization implementation

use super::types::*;
use beardog_errors::BearDogError;

impl CrossNodeAuthEngine {
    /// Get trusted nodes for consensus
    pub fn get_trusted_nodes(&self) -> Result<Vec<String>, BearDogError> {
        // Stub implementation - would query node registry
        Ok(vec![
            "node_1".to_string(),
            "node_2".to_string(),
            "node_3".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trusted_nodes_returns_default_nodes() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.get_trusted_nodes();

        assert!(result.is_ok(), "Should successfully return trusted nodes");
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 3, "Should return 3 default trusted nodes");
        assert_eq!(nodes[0], "node_1");
        assert_eq!(nodes[1], "node_2");
        assert_eq!(nodes[2], "node_3");
    }

    #[test]
    fn test_get_trusted_nodes_consistency() {
        let engine = CrossNodeAuthEngine::default();

        // Call multiple times to ensure consistency
        let nodes1 = engine.get_trusted_nodes().unwrap();
        let nodes2 = engine.get_trusted_nodes().unwrap();

        assert_eq!(nodes1, nodes2, "Should return consistent node list");
    }

    #[test]
    fn test_get_trusted_nodes_not_empty() {
        let engine = CrossNodeAuthEngine::default();
        let nodes = engine.get_trusted_nodes().unwrap();

        assert!(!nodes.is_empty(), "Trusted nodes list should not be empty");
        for node in nodes {
            assert!(!node.is_empty(), "Node IDs should not be empty strings");
        }
    }
}
