// SPDX-License-Identifier: AGPL-3.0-only

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

pub use authorization::*;
pub use genetics::*;
pub use node_registry::*;
pub use spawning::*;
pub use workflow::*;
