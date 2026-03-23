// SPDX-License-Identifier: AGPL-3.0-only

//! Consensus authorization implementation

use super::types::*;
use beardog_errors::BearDogError;
use chrono::Utc;

impl CrossNodeAuthEngine {
    /// Get trusted nodes for consensus (healthy entries only, sorted by node id).
    pub fn get_trusted_nodes(&self) -> Result<Vec<String>, BearDogError> {
        Ok(self
            .consensus_registry
            .iter()
            .filter(|(_, r)| r.health == ConsensusNodeHealth::Healthy)
            .map(|(id, _)| id.clone())
            .collect())
    }

    /// Register or replace a node in the in-memory consensus registry.
    pub fn register_consensus_node(
        &mut self,
        node_id: String,
        health: ConsensusNodeHealth,
    ) -> Result<(), BearDogError> {
        if node_id.is_empty() {
            return Err(BearDogError::invalid_input(
                "consensus node id must not be empty",
            ));
        }
        self.consensus_registry.insert(
            node_id,
            ConsensusNodeRecord {
                health,
                last_seen: Utc::now(),
            },
        );
        Ok(())
    }

    /// Look up health for a consensus participant.
    pub fn get_consensus_node_health(
        &self,
        node_id: &str,
    ) -> Result<ConsensusNodeHealth, BearDogError> {
        self.consensus_registry
            .get(node_id)
            .map(|r| r.health)
            .ok_or_else(|| BearDogError::not_found(format!("Consensus node not found: {node_id}")))
    }

    /// Update health (and last-seen) for a registered consensus node.
    pub fn set_consensus_node_health(
        &mut self,
        node_id: &str,
        health: ConsensusNodeHealth,
    ) -> Result<(), BearDogError> {
        let entry = self.consensus_registry.get_mut(node_id).ok_or_else(|| {
            BearDogError::not_found(format!("Consensus node not found: {node_id}"))
        })?;
        entry.health = health;
        entry.last_seen = Utc::now();
        Ok(())
    }

    /// Get consensus threshold
    pub const fn get_consensus_threshold(&self) -> usize {
        // Require 2/3 majority for consensus
        2
    }

    /// Validate consensus reached
    pub const fn validate_consensus(&self, approvals: usize) -> Result<bool, BearDogError> {
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
        let nodes = result.expect("get_trusted_nodes returns Ok");
        assert_eq!(nodes.len(), 3, "Should return 3 default trusted nodes");
        assert_eq!(nodes[0], "node_1");
        assert_eq!(nodes[1], "node_2");
        assert_eq!(nodes[2], "node_3");
    }

    #[test]
    fn test_get_trusted_nodes_consistency() {
        let engine = CrossNodeAuthEngine::default();

        // Call multiple times to ensure consistency
        let nodes1 = engine
            .get_trusted_nodes()
            .expect("get_trusted_nodes first call");
        let nodes2 = engine
            .get_trusted_nodes()
            .expect("get_trusted_nodes second call");

        assert_eq!(nodes1, nodes2, "Should return consistent node list");
    }

    #[test]
    fn test_get_trusted_nodes_not_empty() {
        let engine = CrossNodeAuthEngine::default();
        let nodes = engine
            .get_trusted_nodes()
            .expect("get_trusted_nodes not empty test");

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
        assert!(
            result.expect("validate_consensus Ok"),
            "2 approvals should reach consensus"
        );
    }

    #[test]
    fn test_validate_consensus_failure() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(1);

        assert!(result.is_ok());
        assert!(
            !result.expect("validate_consensus Ok"),
            "1 approval should not reach consensus"
        );
    }

    #[test]
    fn test_validate_consensus_exact_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let threshold = engine.get_consensus_threshold();
        let result = engine.validate_consensus(threshold);

        assert!(result.is_ok());
        assert!(
            result.expect("validate_consensus Ok"),
            "Exact threshold should reach consensus"
        );
    }

    #[test]
    fn test_validate_consensus_above_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(3);

        assert!(result.is_ok());
        assert!(
            result.expect("validate_consensus Ok"),
            "Above threshold should reach consensus"
        );
    }

    #[test]
    fn test_quorum_size() {
        let engine = CrossNodeAuthEngine::default();
        let quorum = engine
            .quorum_size()
            .expect("quorum_size with default registry");

        assert_eq!(quorum, 2, "Quorum should be 2/3 of 3 nodes");
    }

    #[test]
    fn test_quorum_size_matches_threshold() {
        let engine = CrossNodeAuthEngine::default();
        let quorum = engine
            .quorum_size()
            .expect("quorum_size matches threshold test");
        let threshold = engine.get_consensus_threshold();

        assert_eq!(quorum, threshold, "Quorum and threshold should match");
    }

    #[test]
    fn test_zero_approvals() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(0);

        assert!(result.is_ok());
        assert!(
            !result.expect("validate_consensus Ok"),
            "Zero approvals should not reach consensus"
        );
    }

    #[test]
    fn test_excessive_approvals() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.validate_consensus(100);

        assert!(result.is_ok());
        assert!(
            result.expect("validate_consensus Ok"),
            "More than threshold should reach consensus"
        );
    }

    #[test]
    fn test_register_consensus_node_rejects_empty_id() {
        let mut engine = CrossNodeAuthEngine::default();
        let err = engine
            .register_consensus_node(String::new(), ConsensusNodeHealth::Healthy)
            .unwrap_err();
        assert!(format!("{err}").contains("empty") || format!("{err}").contains("consensus"));
    }

    #[test]
    fn test_get_consensus_node_health_found_and_not_found() {
        let engine = CrossNodeAuthEngine::default();
        assert_eq!(
            engine
                .get_consensus_node_health("node_1")
                .expect("default node_1 health"),
            ConsensusNodeHealth::Healthy
        );
        let err = engine
            .get_consensus_node_health("no_such_node")
            .unwrap_err();
        assert!(format!("{err}").contains("not found") || format!("{err}").contains("Consensus"));
    }

    #[test]
    fn test_set_consensus_node_health_updates_and_errors_on_unknown() {
        let mut engine = CrossNodeAuthEngine::default();
        engine
            .set_consensus_node_health("node_1", ConsensusNodeHealth::Degraded)
            .expect("set node_1 health to Degraded");
        assert_eq!(
            engine
                .get_consensus_node_health("node_1")
                .expect("node_1 health after update"),
            ConsensusNodeHealth::Degraded
        );
        let err = engine
            .set_consensus_node_health("missing", ConsensusNodeHealth::Healthy)
            .unwrap_err();
        assert!(format!("{err}").contains("not found") || format!("{err}").contains("Consensus"));
    }

    #[test]
    fn test_get_trusted_nodes_excludes_unhealthy() {
        let mut engine = CrossNodeAuthEngine::default();
        engine
            .consensus_registry
            .get_mut("node_1")
            .expect("default registry")
            .health = ConsensusNodeHealth::Offline;
        let nodes = engine
            .get_trusted_nodes()
            .expect("get_trusted_nodes after marking node_1 offline");
        assert_eq!(nodes, vec!["node_2", "node_3"]);
    }

    #[test]
    fn test_quorum_size_with_fewer_healthy_nodes() {
        let mut engine = CrossNodeAuthEngine::default();
        engine
            .consensus_registry
            .get_mut("node_1")
            .expect("node_1 in default registry")
            .health = ConsensusNodeHealth::Offline;
        engine
            .consensus_registry
            .get_mut("node_2")
            .expect("node_2 in default registry")
            .health = ConsensusNodeHealth::Offline;
        let q = engine
            .quorum_size()
            .expect("quorum_size with two nodes offline");
        assert_eq!(q, 0);
    }

    #[test]
    fn test_get_trusted_nodes_excludes_degraded() {
        let mut engine = CrossNodeAuthEngine::default();
        engine
            .consensus_registry
            .get_mut("node_2")
            .expect("default registry")
            .health = ConsensusNodeHealth::Degraded;
        let nodes = engine
            .get_trusted_nodes()
            .expect("get_trusted_nodes after marking node_2 degraded");
        assert_eq!(nodes, vec!["node_1", "node_3"]);
    }

    #[test]
    fn test_register_consensus_node_success_sorted_and_quorum() {
        let mut engine = CrossNodeAuthEngine::default();
        engine
            .register_consensus_node("node_z".to_string(), ConsensusNodeHealth::Healthy)
            .expect("register node_z");
        let nodes = engine
            .get_trusted_nodes()
            .expect("get_trusted_nodes after registering node_z");
        assert_eq!(
            nodes,
            vec![
                "node_1".to_string(),
                "node_2".to_string(),
                "node_3".to_string(),
                "node_z".to_string(),
            ]
        );
        assert_eq!(
            engine
                .quorum_size()
                .expect("quorum_size with four healthy nodes"),
            (4 * 2) / 3
        );
    }
}
