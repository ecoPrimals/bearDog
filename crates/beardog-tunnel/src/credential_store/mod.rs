// SPDX-License-Identifier: AGPL-3.0-or-later

//! Credential store backends implementing [`beardog_traits::unified::CredentialStore`].
//!
//! Silicon Atheism pattern: one trait, multiple backends dispatched via
//! [`CredentialStoreBackend`] enum. No `#[cfg]`-gated exclusion — every
//! backend compiles on every platform and reports availability at runtime.

mod backend;
mod file_vault;
mod in_memory;

pub use backend::CredentialStoreBackend;
pub use file_vault::FileVaultCredentialStore;
pub use in_memory::InMemoryCredentialStore;
