// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nuclear lineage distance classification for enrollment proofs.

use beardog_config::env_keys;
use beardog_genetics::birdsong::lineage_proof::LineageProofManager;

/// Classify the nuclear lineage distance from an enrollment proof.
///
/// The enrollee's `lineage_proof.path` encodes their depth from root.
/// The verifier's own node ID is resolved from `BEARDOG_PRIMAL_NAME` / `NODE_ID`.
///
/// If the verifier is in the same chain, we compute true genetic distance:
/// `depth(enrollee) + depth(verifier) - 2 * depth(common_ancestor)`.
///
/// If the verifier's node is not in the enrollee's path (different branch
/// or verifier doesn't participate in chain proofs), we fall back to
/// classifying the enrollee's **depth from root** — still useful for
/// trust tiering since depth 1 (root's child) has more authority than depth 5.
pub fn classify_lineage_distance(
    enrollee_node_id: &str,
    proof: &beardog_types::btsp::EnrollmentLineageProof,
) -> (Option<String>, Option<u32>) {
    if proof.path.is_empty() {
        return (None, None);
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Lineage path lengths fit u32"
    )]
    let enrollee_depth = (proof.path.len() - 1) as u32;

    if proof.path.last().map(String::as_str) != Some(enrollee_node_id) {
        return (None, None);
    }

    let verifier_id = beardog_errors::process_env::var(env_keys::ENV_NODE_ID_PREFIXED)
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_NODE_ID))
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME_PREFIXED))
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME))
        .unwrap_or_default();

    let distance = if verifier_id.is_empty() {
        enrollee_depth
    } else if verifier_id == enrollee_node_id {
        0
    } else if let Some(verifier_pos) = proof.path.iter().position(|n| n == &verifier_id) {
        #[expect(clippy::cast_possible_truncation, reason = "Position in path fits u32")]
        let verifier_depth = verifier_pos as u32;
        enrollee_depth - verifier_depth
    } else {
        enrollee_depth
    };

    let tier = LineageProofManager::classify_enrollment_tier(distance);
    (Some(tier.wire_name().to_string()), Some(distance))
}
