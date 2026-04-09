// SPDX-License-Identifier: AGPL-3.0-or-later

//! Authentication DTOs: authorization proofs, genetics, spawning, workflows, and node directory types.
//!
//! Public re-exports mirror historical `beardog_auth::auth::*` paths for downstream crates.

/// Cross-node grants, resource permissions, and proof envelopes.
pub mod authorization;
/// Genetic/capability models attached to nodes and keys.
pub mod genetics;
/// Concrete behavior for [`genetics::BearDogGenetics`] split out for readability.
pub mod genetics_impl;
/// Node registry traits, proof verification hooks, workflow engine trait, and `CrossNodeAuthEngine`.
pub mod node_registry;
/// Spawn metadata, resource limits, and lifecycle enums.
pub mod spawning;
/// Declarative multi-node workflow requests and execution status.
pub mod workflow;

pub use authorization::{
    AccessCondition, ApprovalMode, AuthMethod, AuthProof, AuthorizationProof, ConsensusConfig,
    ConsensusResult, CrossNodeAuthConfig, CrossNodeAuthorization, CrossNodeOperation,
    OperationType, ResourcePermission, SpawningMode, VerificationMode,
};
pub use genetics::{
    AlgorithmFamily, BearDogGenetics, CapabilityMutation, CryptoChromosome, MutationTrigger,
    NodeCapability, NodeSpecialization, SecurityClearance, SecurityTraits, SpawnRestriction,
};
pub use node_registry::{
    ConsensusNodeHealth, ConsensusNodeRecord, CrossNodeAuthEngine, NodeInfo, NodeRegistry,
    ProofVerifier, WorkflowEngine, default_consensus_registry,
};
pub use spawning::{
    ResourceLimits, SpawnPurpose, SpawnRequest, SpawnStatus, SpawnedBearDog, TaskType,
};
pub use workflow::{
    AutomatedCheck, BearDogWorkflowType, CrossNodeWorkflowRequest, EscalationCondition,
    WorkflowStatus,
};
