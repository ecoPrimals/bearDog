// SPDX-License-Identifier: AGPL-3.0-or-later

//! `enrollment.verify` — two-layer genetic enrollment verification.
//!
//! ## Two-layer genetic model
//!
//! Enrollment mirrors the biological two-DNA system:
//!
//! 1. **Mitochondrial gate** — shared `FAMILY_SEED` HMAC proves the enrollee
//!    can "hear the birdsong" (same family). This is the mito-beacon layer:
//!    if you don't share the family seed, you can't even produce a valid proof.
//!
//! 2. **Nuclear lineage distance** — when a `lineage_proof` is attached, the
//!    verifier computes genetic distance (tree hops) and classifies the
//!    enrollee into a trust tier (`identity`, `kin`, `sibling`, `extended`,
//!    `distant`). Like nuclear DNA, this determines authority and permissions.
//!
//! ```text
//! enrollment_key(gen) = HKDF-SHA256(
//!     ikm  = FAMILY_SEED,
//!     salt = FAMILY_ID (or "default"),
//!     info = "enrollment-v{gen}"
//! )
//! proof = HMAC-SHA256(enrollment_key(gen), node_id|public_key|timestamp|gen)
//! ```
//!
//! ## Seed rotation
//!
//! - `BEARDOG_ENROLLMENT_SEED_GENERATION` sets the current generation (default 0).
//! - Grace period accepts N and N−1 (same as `BirdSong` key rotation).
//!
//! ## Security hardening
//!
//! - **Timestamp window**: ±`BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW` seconds.
//! - **Replay tracking**: BLAKE3-digest dedup cache, bounded and self-pruning.

mod config;
mod crypto;
mod handler;
mod lineage;
pub mod replay_cache;

pub use replay_cache::ReplayCache;

#[cfg(test)]
mod tests;
