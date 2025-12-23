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

    /// Get consensus threshold
    pub fn get_consensus_threshold(&self) -> usize {
        // Require 2/3 majority for consensus
        2
    }

    /// Validate consensus reached
    pub fn validate_consensus(&self, approvals: usize) -> Result<bool, BearDogError> {
        let threshold = self.get_consensus_threshold();
        Ok(approvals >= threshold)
    }

    /// Get quorum size
    pub fn quorum_size(&self) -> Result<usize, BearDogError> {
        let nodes = self.get_trusted_nodes()?;
        Ok((nodes.len() * 2) / 3)
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

    #[test]
    fn test_consensus_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let threshold = engine.get_consensus_threshold();

        assert_eq!(threshold, 2, "Should require 2/3 majority");
        assert!(threshold > 0, "Threshold must be positive");
    }

    #[test]
    fn test_validate_consensus_success() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(2);

        assert!(result.is_ok());
        assert!(result.unwrap(), "2 approvals should reach consensus");
    }

    #[test]
    fn test_validate_consensus_failure() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(1);

        assert!(result.is_ok());
        assert!(!result.unwrap(), "1 approval should not reach consensus");
    }

    #[test]
    fn test_validate_consensus_exact_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let threshold = engine.get_consensus_threshold();
        let result = engine.validate_consensus(threshold);

        assert!(result.is_ok());
        assert!(result.unwrap(), "Exact threshold should reach consensus");
    }

    #[test]
    fn test_validate_consensus_above_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(3);

        assert!(result.is_ok());
        assert!(result.unwrap(), "Above threshold should reach consensus");
    }

    #[test]
    fn test_quorum_size() {
        let engine = CrossNodeAuthEngine::default();
        let quorum = engine.quorum_size().unwrap();

        assert_eq!(quorum, 2, "Quorum should be 2/3 of 3 nodes");
    }

    #[test]
    fn test_quorum_size_matches_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let quorum = engine.quorum_size().unwrap();
        let threshold = engine.get_consensus_threshold();

        assert_eq!(quorum, threshold, "Quorum and threshold should match");
    }

    #[test]
    fn test_zero_approvals() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(0);

        assert!(result.is_ok());
        assert!(
            !result.unwrap(),
            "Zero approvals should not reach consensus"
        );
    }

    #[test]
    fn test_excessive_approvals() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(100);

        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "More than threshold should reach consensus"
        );
    }
}
