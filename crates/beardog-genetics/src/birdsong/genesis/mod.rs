// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genesis Lineage Establishment
//!
//! Provides cryptographic lineage creation for new nodes during physical
//! genesis ceremonies. Integrates with existing lineage infrastructure
//! while adding witness-based trust establishment.

mod provider;

#[cfg(test)]
mod tests;

pub use provider::GenesisLineageProvider;
