// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constants from tor-spec section 5.1.4.

/// Protocol identifier for ntor
pub(super) const NTOR_PROTOID: &[u8] = b"ntor-curve25519-sha256-1";

/// Key derivation tweak for `KEY_SEED` extraction
pub(super) const NTOR_T_KEY: &[u8] = b"ntor-curve25519-sha256-1:key_extract";

/// Key derivation tweak for verification MAC
pub(super) const NTOR_T_VERIFY: &[u8] = b"ntor-curve25519-sha256-1:verify";

/// Key expansion tweak for HKDF
pub(super) const NTOR_T_EXPAND: &[u8] = b"ntor-curve25519-sha256-1:key_expand";

/// MAC tweak (for auth computation)
pub(super) const NTOR_T_MAC: &[u8] = b"ntor-curve25519-sha256-1:mac";

/// Server string constant
pub(super) const NTOR_SERVER: &[u8] = b"Server";
