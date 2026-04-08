// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key lifecycle handlers: generate, list, info, delete (wired to local key store).
#![allow(
    unused_imports,
    reason = "thin hub: not every re-export is referenced from main/lib"
)]

mod generate;
mod list;
mod storage;

pub use generate::handle_key_generate_v2;
pub(crate) use generate::select_cli_hsm_for_preference;
pub use list::{
    handle_key_info, handle_key_info_with_home, handle_key_list, handle_key_list_with_home,
};
pub use storage::{handle_key_delete, handle_key_delete_with_home};
