// SPDX-License-Identifier: AGPL-3.0-or-later

//! Authentication Module - Identity and Access Management
//!
//! Provides authentication, authorization, and consensus mechanisms for the
//! BearDog ecosystem including:
//!
//! - **Handlers**: Authentication request processing
//! - **Node Registry**: Cross-node authentication management
//! - **Proof Verification**: Cryptographic proof validation
//! - **Genetics**: Lineage-based authentication
//! - **Workflow**: Multi-step authentication flows

pub use handlers::AuthenticationHandler;
pub use types::ConsensusResult as HandlerConsensusResult;
pub use types::ConsensusResult as TypesConsensusResult;

pub use types::{
    AccessCondition, AlgorithmFamily, ApprovalMode, AuthMethod, AuthProof, AuthorizationProof,
    AutomatedCheck, BearDogGenetics, BearDogWorkflowType, CapabilityMutation, ConsensusConfig,
    ConsensusNodeHealth, ConsensusNodeRecord, ConsensusResult, CrossNodeAuthConfig,
    CrossNodeAuthEngine, CrossNodeAuthorization, CrossNodeOperation, CrossNodeWorkflowRequest,
    CryptoChromosome, EscalationCondition, MutationTrigger, NodeCapability, NodeInfo, NodeRegistry,
    NodeSpecialization, OperationType, ProofVerifier, ResourceLimits, ResourcePermission,
    SecurityClearance, SecurityTraits, SpawnPurpose, SpawnRequest, SpawnRestriction, SpawnStatus,
    SpawnedBearDog, SpawningMode, TaskType, VerificationMode, WorkflowEngine, WorkflowStatus,
    default_consensus_registry,
};

mod consensus;
mod core; // CrossNodeAuthEngine implementation
mod ecosystem;
mod genetics;
mod verification;

/// Password-based authentication, session issuance, and rate limiting for `BearDog` services.
pub mod handlers;
/// In-memory [`crate::auth::types::NodeRegistry`] used in tests and lightweight deployments.
pub mod node_registry;
/// Default [`crate::auth::types::ProofVerifier`] that performs structural checks on authorization proofs.
pub mod proof_verifier;
#[cfg(test)]
mod tests;
/// Shared auth DTOs: cross-node authorization, genetics, spawning, and workflow types.
pub mod types;
