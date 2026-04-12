// SPDX-License-Identifier: AGPL-3.0-or-later

//! Environment variable parsing helpers for port discovery.

use beardog_errors::process_env;

pub(super) fn parse_u16_env(key: &str, fallback: u16) -> u16 {
    process_env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback)
}
