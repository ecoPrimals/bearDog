// SPDX-License-Identifier: AGPL-3.0-or-later

//! Environment-backed configuration loaders for enrollment verification.

use beardog_config::env_keys;

use crate::unix_socket_ipc::handlers::HandlerError;

/// Default timestamp validity window: ±300 seconds (5 minutes).
const DEFAULT_TIMESTAMP_WINDOW_SECS: u64 = 300;

/// Load the family seed from environment variables.
///
/// Checks `BEARDOG_FAMILY_SEED` first, then `FAMILY_SEED`.
pub fn load_family_seed() -> Result<Vec<u8>, HandlerError> {
    if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED)
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED)
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    Err(
        "enrollment.verify requires FAMILY_SEED or BEARDOG_FAMILY_SEED env var"
            .to_string()
            .into(),
    )
}

/// Load the current enrollment seed generation from env (default 0).
pub fn load_seed_generation() -> u32 {
    beardog_errors::process_env::var(env_keys::ENV_ENROLLMENT_SEED_GENERATION)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

/// Load the enrollment timestamp window from env, defaulting to 300 seconds.
pub fn load_timestamp_window() -> u64 {
    beardog_errors::process_env::var(env_keys::ENV_ENROLLMENT_TIMESTAMP_WINDOW)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TIMESTAMP_WINDOW_SECS)
}

/// Current wall-clock time as Unix seconds.
pub fn current_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}
