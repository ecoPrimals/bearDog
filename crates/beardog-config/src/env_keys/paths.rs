// SPDX-License-Identifier: AGPL-3.0-or-later

//! Path, XDG, and build metadata environment variable keys.

// ── Paths ────────────────────────────────────────────────────────────

/// Override the configuration directory.
pub const ENV_CONFIG_DIR: &str = "BEARDOG_CONFIG_DIR";
/// Explicit configuration file path.
pub const ENV_CONFIG_PATH: &str = "BEARDOG_CONFIG_PATH";
/// Override the data directory.
pub const ENV_DATA_DIR: &str = "BEARDOG_DATA_DIR";
/// Override the log directory.
pub const ENV_LOG_DIR: &str = "BEARDOG_LOG_DIR";
/// Base directory for `BearDog` data and config layout.
pub const ENV_BASE_DIR: &str = "BEARDOG_BASE_DIR";
/// PKCS#11 library path override.
pub const ENV_PKCS11_LIBRARY: &str = "BEARDOG_PKCS11_LIBRARY";
/// PKCS#11 search paths (colon-separated).
pub const ENV_PKCS11_SEARCH_PATHS: &str = "BEARDOG_PKCS11_SEARCH_PATHS";

// ── XDG / standard paths ─────────────────────────────────────────────

/// XDG config home directory (unprefixed).
pub const ENV_XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
/// XDG data home directory (unprefixed).
pub const ENV_XDG_DATA_HOME: &str = "XDG_DATA_HOME";
/// XDG cache home directory (unprefixed).
pub const ENV_XDG_CACHE_HOME: &str = "XDG_CACHE_HOME";
/// XDG runtime directory (unprefixed).
pub const ENV_XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
/// User home directory (unprefixed).
pub const ENV_HOME: &str = "HOME";
/// Windows application data directory (unprefixed).
pub const ENV_APPDATA: &str = "APPDATA";
/// Override the cache directory.
pub const ENV_CACHE_DIR: &str = "BEARDOG_CACHE_DIR";
/// Override the temp directory.
pub const ENV_TEMP_DIR: &str = "BEARDOG_TEMP_DIR";
/// IPC port file path override.
pub const ENV_IPC_PORT_FILE: &str = "BEARDOG_IPC_PORT_FILE";
/// Software key storage directory override.
pub const ENV_KEY_STORAGE_DIR: &str = "BEARDOG_KEY_STORAGE_DIR";

// ── Build metadata ───────────────────────────────────────────────────

/// Build timestamp injected at compile time.
pub const ENV_BUILD_TIMESTAMP: &str = "BUILD_TIMESTAMP";
/// Git commit hash injected at compile time.
pub const ENV_GIT_COMMIT: &str = "GIT_COMMIT";
// ── Build / toolchain ────────────────────────────────────────────────────

/// Cargo compilation target triple.
pub const ENV_TARGET: &str = "TARGET";
/// Tower Atomic peer socket path.
pub const ENV_TOWER_ATOMIC_PEER: &str = "TOWER_ATOMIC_PEER";
