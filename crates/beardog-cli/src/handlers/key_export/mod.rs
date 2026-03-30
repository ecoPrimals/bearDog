// SPDX-License-Identifier: AGPL-3.0-only

//! Export and import keys for inter-primal sharing (JSON [`ExportedKey`] format).

mod crypto;
mod export;
mod import;
mod types;

// Re-exports are the module's public API; `*_with_home` and `ExportedKey` are primarily for tests / embedders.
#[allow(unused_imports)]
pub use export::{handle_key_export, handle_key_export_with_home};
#[allow(unused_imports)]
pub use import::{handle_key_import, handle_key_import_with_home};
#[allow(unused_imports)]
pub use types::ExportedKey;

#[cfg(test)]
mod tests;
