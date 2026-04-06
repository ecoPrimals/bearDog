// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for the software HSM core (`core/`).
//!
//! Broader `RustSoftwareHsm` coverage lives under `software_hsm/tests.rs`.

#[test]
fn module_compiles_with_core_split() {
    let _ = std::any::type_name::<super::RustSoftwareHsm>();
}
