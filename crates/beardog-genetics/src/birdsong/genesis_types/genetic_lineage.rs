// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lineage integrity checks and lineage-derived hints.

use crate::birdsong::LineageHint;

use super::core::GeneticLineage;

impl GeneticLineage {
    /// Verify this lineage's integrity
    ///
    /// # Errors
    ///
    /// Returns [`beardog_errors::BearDogError`] when the lineage chain has no child node or witness signature
    /// verification fails with a cryptographic error.
    pub fn verify(&self) -> Result<bool, beardog_errors::BearDogError> {
        // 1. Verify witness signature
        // Find the child node (non-root) in the lineage
        let child_node = self
            .lineage_chain
            .nodes
            .values()
            .find(|n| n.parent_id.is_some())
            .ok_or_else(|| {
                beardog_errors::BearDogError::business("No child node in lineage chain".into())
            })?;

        let node_id = child_node.node_id.clone();

        if !self.genesis_witness.verify_signature(&node_id)? {
            return Ok(false);
        }

        // 2. Verify timestamps are consistent
        if self.birth_timestamp < self.genesis_witness.timestamp {
            return Ok(false);
        }

        // 3. Verify lineage chain integrity
        // (This will be implemented by LineageChainManager)

        Ok(true)
    }

    /// Get lineage depth (number of generations from genesis)
    pub fn depth(&self) -> usize {
        self.lineage_chain.nodes.len()
    }

    /// Get lineage hint for broadcast encryption
    #[expect(
        clippy::cast_possible_truncation,
        reason = "lineage depth fits u32 for broadcast hint"
    )]
    pub fn lineage_hint(&self) -> LineageHint {
        LineageHint {
            root_id: self.lineage_chain.root_node.node_id.clone(),
            min_depth: 0,
            max_depth: self.depth() as u32,
            biome_filter: Some("genesis".to_string()),
            version: 1,
        }
    }
}
