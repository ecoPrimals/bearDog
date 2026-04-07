// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export and import keys for inter-primal sharing (JSON [`ExportedKey`](crate::handlers::key_export::ExportedKey) format).

mod crypto;
mod export;
mod import;
mod types;

// Re-exports form the module's public API; `_with_home` variants and `ExportedKey` are for tests / embedders.
// #[allow] rather than #[expect]: unused_imports fires only in bin target, not lib where #[expect] is checked.
#[allow(
    unused_imports,
    reason = "pub re-exports for downstream callers; unused only from bin target"
)]
pub use export::{handle_key_export, handle_key_export_with_home};
#[allow(
    unused_imports,
    reason = "pub re-exports for downstream callers; unused only from bin target"
)]
pub use import::{handle_key_import, handle_key_import_with_home};
#[allow(
    unused_imports,
    reason = "pub re-export for downstream callers; unused only from bin target"
)]
pub use types::ExportedKey;

#[cfg(test)]
mod tests;
