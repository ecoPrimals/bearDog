// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod platform;
pub mod platform_access;
pub mod safe_memory_enhanced;
pub mod safe_ops;

pub use platform::resolve_uid_from_proc;
pub use platform_access::PlatformAccess;
