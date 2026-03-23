// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive core tests (split by domain: core lifecycle, messaging, identity, coordination, storage).

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

mod coordination;
mod core_functionality;
mod identity;
mod messaging;
mod storage;
