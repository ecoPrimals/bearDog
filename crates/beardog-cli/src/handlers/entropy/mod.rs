// SPDX-License-Identifier: AGPL-3.0-or-later

//! Collect entropy into seed files and display seed metadata (`beardog entropy`).

mod collect;
mod helpers;
mod hsm_selection;
mod info;
mod types;

pub use collect::handle_entropy_collect;
// #[allow] rather than #[expect]: unused_imports fires only in bin target, not lib where #[expect] is checked.
#[allow(
    unused_imports,
    reason = "pub re-exports for downstream callers; unused only from bin target"
)]
pub use helpers::{
    base64_decode, base64_encode, calculate_entropy_quality, load_entropy_file, save_entropy_file,
};
pub use info::handle_entropy_info;
#[allow(
    unused_imports,
    reason = "pub re-export for downstream callers; unused only from bin target"
)]
pub use types::EntropySeedMetadata;

#[allow(
    unused_imports,
    reason = "used by crate-internal tests; unused only from bin target"
)]
pub(crate) use hsm_selection::{format_hsm_interface_type_label, select_hsm_by_preference};
#[allow(
    unused_imports,
    reason = "used by crate-internal tests; unused only from bin target"
)]
pub(crate) use types::HsmInfo;

#[cfg(test)]
mod tests;
