// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Unix Socket Configuration
//!
//! Robust, BTSP-compliant socket path resolution with a 5-tier fallback system:
//!
//! 1. **Primal-Specific** (highest priority): `BEARDOG_SOCKET`
//! 2. **Generic Orchestrator**: `BIOMEOS_SOCKET_PATH` or `BIOMEOS_SOCKET_DIR`
//! 3. **Primal IPC Protocol Standard**: `/primal/{PRIMAL_NAME}` (discovery-based)
//! 4. **XDG Runtime Directory**: `/run/user/<uid>/biomeos/{socket_filename}` (biomeOS standard)
//! 5. **Temp Directory** (last resort): `{temp}/{PRIMAL_NAME}-<family>-<node>.sock`
//!
//! ## Family-Scoped Sockets (BTSP Production Mode)
//!
//! When `FAMILY_ID` is set (and not `"default"`), tiers 2-4 produce family-scoped
//! socket filenames (`beardog-{family_id}.sock`) per `BTSP_PROTOCOL_STANDARD.md`.
//! This is the BTSP activation signal: all incoming connections MUST authenticate
//! via the BTSP handshake before any JSON-RPC methods are exposed.
//!
//! When `FAMILY_ID` is unset or `"default"`, tiers produce `beardog.sock` (development mode).
//!
//! ## Security Guards
//!
//! Setting both `FAMILY_ID` (non-default) and `BIOMEOS_INSECURE=1` is a fatal
//! configuration conflict — you cannot claim a family AND skip authentication.
//! `SocketConfig::from_inputs` returns `Err(SocketConfigError::InsecureWithFamily)`.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use beardog_core::socket_config::SocketConfig;
//!
//! let config = SocketConfig::from_env().expect("socket config conflict");
//! let socket_path = config.socket_path();
//! println!("Socket: {} (production={})", socket_path.display(), config.production_mode());
//! ```

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::self_knowledge::SimpleCapability;
use beardog_types::constants::domains::network::ipc_discovery::BIOMEOS_RUNTIME_SOCKET_SUBDIR;
use beardog_types::primal_identity::resolve_node_id_from_env_or_ephemeral;
use tracing::warn;

/// Default primal identifier when `PRIMAL_NAME` is unset — used for tier 3–5 socket path resolution.
pub const DEFAULT_PRIMAL_NAME: &str = "beardog";

/// Resolve the real UID by reading `/proc/self/status` (Linux) without `unsafe`.
///
/// Returns `None` on non-Linux or on parse failure, allowing callers to fall back.
fn resolve_uid_from_proc() -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                return rest.split_whitespace().next()?.parse().ok();
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// All inputs needed to resolve a [`SocketConfig`] without reading the process environment.
///
/// Use [`SocketPathInputs::from_env`] at process boundaries; tests should construct values
/// directly for concurrency-safe, deterministic behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketPathInputs {
    /// Tier 1: `BEARDOG_SOCKET` (empty string is treated as unset).
    pub beardog_socket: Option<String>,
    /// Tier 2: `BIOMEOS_SOCKET_PATH`
    pub biomeos_socket_path: Option<String>,
    /// Tier 2: `BIOMEOS_SOCKET_DIR` (joined with the resolved socket filename)
    pub biomeos_socket_dir: Option<String>,
    /// Used for tier 3–5 path construction; defaults to [`DEFAULT_PRIMAL_NAME`] in resolution.
    pub primal_name: Option<String>,
    /// Resolved family id (`BEARDOG_FAMILY_ID` / `FAMILY_ID`); default `"default"`.
    pub family_id: Option<String>,
    /// Resolved node id (`BEARDOG_NODE_ID` / `NODE_ID`); if unset or empty, [`SocketConfig::from_inputs`]
    /// uses the same ephemeral `standalone-{uuid}` as [`beardog_types::primal_identity::PrimalIdentity`].
    pub node_id: Option<String>,
    /// User id for tier 4 (`/run/user/<uid>/...`); resolved from the process in [`Default`].
    pub uid: u32,
    /// When `true`, tier 3 uses `/primal/<primal_name>` if tier 1–2 do not apply.
    pub primal_namespace_root_exists: bool,
    /// `BIOMEOS_INSECURE` env var; when `true` AND `family_id` is set (non-default),
    /// the primal MUST refuse to start per `BTSP_PROTOCOL_STANDARD.md`.
    pub biomeos_insecure: bool,
}

impl Default for SocketPathInputs {
    fn default() -> Self {
        Self {
            beardog_socket: None,
            biomeos_socket_path: None,
            biomeos_socket_dir: None,
            primal_name: None,
            family_id: None,
            node_id: None,
            uid: resolve_uid_from_proc().unwrap_or(1000),
            primal_namespace_root_exists: false,
            biomeos_insecure: false,
        }
    }
}

impl SocketPathInputs {
    /// Read configuration from the process environment (read-only, thread-safe `std::env::var`).
    #[must_use]
    pub fn from_env() -> Self {
        let family_id = std::env::var("BEARDOG_FAMILY_ID")
            .ok()
            .or_else(|| std::env::var("FAMILY_ID").ok());
        let node_id = std::env::var("BEARDOG_NODE_ID")
            .ok()
            .or_else(|| std::env::var("NODE_ID").ok());
        let uid = std::env::var("UID")
            .ok()
            .or_else(|| std::env::var("EUID").ok())
            .and_then(|s| s.parse().ok())
            .or_else(resolve_uid_from_proc)
            .unwrap_or(1000);
        let biomeos_insecure = std::env::var("BIOMEOS_INSECURE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Self {
            beardog_socket: std::env::var("BEARDOG_SOCKET").ok(),
            biomeos_socket_path: std::env::var("BIOMEOS_SOCKET_PATH").ok(),
            biomeos_socket_dir: std::env::var("BIOMEOS_SOCKET_DIR").ok(),
            primal_name: std::env::var("PRIMAL_NAME").ok(),
            family_id,
            node_id,
            uid,
            primal_namespace_root_exists: Path::new("/primal").exists(),
            biomeos_insecure,
        }
    }

    /// Returns `true` when `FAMILY_ID` is set to a non-default value,
    /// indicating production mode (BTSP handshake required on all connections).
    #[must_use]
    pub fn is_production_mode(&self) -> bool {
        matches!(self.family_id.as_deref(), Some(fid) if !fid.is_empty() && fid != "default")
    }
}

/// Fatal configuration conflicts detected during socket resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketConfigError {
    /// `FAMILY_ID` (non-default) and `BIOMEOS_INSECURE=1` are both set.
    /// Per `BTSP_PROTOCOL_STANDARD.md`: you cannot claim a family AND skip authentication.
    InsecureWithFamily {
        /// The non-default family id that was set.
        family_id: String,
    },
}

impl std::fmt::Display for SocketConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsecureWithFamily { family_id } => write!(
                f,
                "FATAL: FAMILY_ID={family_id} and BIOMEOS_INSECURE are both set. \
                 Cannot claim a family AND skip BTSP authentication. \
                 Unset BIOMEOS_INSECURE for production or unset FAMILY_ID for development."
            ),
        }
    }
}

impl std::error::Error for SocketConfigError {}

/// Socket configuration with 5-tier fallback logic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketConfig {
    /// Final resolved socket path
    socket_path: PathBuf,
    /// Family ID for this instance
    family_id: String,
    /// Node ID for this instance
    node_id: String,
    /// Which tier was used (for diagnostics)
    source: SocketPathSource,
    /// `true` when `FAMILY_ID` is set to a non-default value (BTSP handshake required)
    production_mode: bool,
}

/// Indicates which tier of fallback logic was used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketPathSource {
    /// Primal-specific environment variable (`BEARDOG_SOCKET`, highest priority)
    PrimalEnvVar,
    /// Generic orchestrator environment variable (`BIOMEOS_SOCKET_PATH`)
    OrchestratorEnvVar,
    /// Primal IPC Protocol standard namespace (/primal/beardog)
    PrimalNamespace,
    /// XDG Runtime Directory (preferred)
    XdgRuntime,
    /// Temp directory (last resort)
    TempDir,
}

impl SocketConfig {
    /// Resolve socket configuration from explicit inputs (no environment reads).
    ///
    /// Implements 5-tier fallback (Primal IPC Protocol + BTSP compliant):
    /// 1. `beardog_socket` (primal-specific, highest priority)
    /// 2. `biomeos_socket_path` or `biomeos_socket_dir` (generic orchestrator)
    /// 3. `/primal/{primal-name}` when `primal_namespace_root_exists`
    /// 4. `/run/user/<uid>/biomeos/{socket_filename}` when the XDG runtime dir exists
    /// 5. Platform temp dir + `{primal-name}-{family}-{node}.sock` (fallback; root from `BEARDOG_SOCKET_TMP_DIR`)
    ///
    /// When `FAMILY_ID` is set (production mode), tiers 2-4 produce family-scoped
    /// filenames (`beardog-{family_id}.sock`) per `BTSP_PROTOCOL_STANDARD.md`.
    ///
    /// # Errors
    ///
    /// Returns `Err` when both `FAMILY_ID` (non-default) and `BIOMEOS_INSECURE` are set —
    /// this is a fatal configuration conflict per BTSP spec.
    pub fn from_inputs(inputs: &SocketPathInputs) -> Result<Self, SocketConfigError> {
        let family_id = inputs
            .family_id
            .clone()
            .unwrap_or_else(|| "default".to_string());
        let node_id = resolve_node_id_from_env_or_ephemeral(inputs.node_id.as_deref());

        let production_mode = inputs.is_production_mode();

        if production_mode && inputs.biomeos_insecure {
            return Err(SocketConfigError::InsecureWithFamily { family_id });
        }

        let primal_name = inputs
            .primal_name
            .clone()
            .unwrap_or_else(|| DEFAULT_PRIMAL_NAME.to_string());

        let socket_filename = if production_mode {
            format!("{primal_name}-{family_id}.sock")
        } else {
            format!("{primal_name}.sock")
        };

        // Tier 1: primal-specific BEARDOG_SOCKET
        if let Some(ref socket_path) = inputs.beardog_socket
            && !socket_path.is_empty()
        {
            return Ok(Self {
                socket_path: PathBuf::from(socket_path),
                family_id,
                node_id,
                source: SocketPathSource::PrimalEnvVar,
                production_mode,
            });
        }

        // Tier 2: BIOMEOS_SOCKET_PATH or BIOMEOS_SOCKET_DIR
        if let Some(ref socket_path) = inputs.biomeos_socket_path {
            if !socket_path.is_empty() {
                return Ok(Self {
                    socket_path: PathBuf::from(socket_path),
                    family_id,
                    node_id,
                    source: SocketPathSource::OrchestratorEnvVar,
                    production_mode,
                });
            }
        } else if let Some(ref socket_dir) = inputs.biomeos_socket_dir
            && !socket_dir.is_empty()
        {
            return Ok(Self {
                socket_path: PathBuf::from(socket_dir).join(&socket_filename),
                family_id,
                node_id,
                source: SocketPathSource::OrchestratorEnvVar,
                production_mode,
            });
        }

        // Tier 3: Primal IPC Protocol standard namespace
        if inputs.primal_namespace_root_exists {
            return Ok(Self {
                socket_path: PathBuf::from(format!("/primal/{primal_name}")),
                family_id,
                node_id,
                source: SocketPathSource::PrimalNamespace,
                production_mode,
            });
        }

        // Tier 4: XDG Runtime Directory
        if let Some(xdg_path) = Self::try_xdg_runtime(inputs.uid, &socket_filename) {
            return Ok(Self {
                socket_path: xdg_path,
                family_id,
                node_id,
                source: SocketPathSource::XdgRuntime,
                production_mode,
            });
        }

        // Tier 5: platform temp dir fallback (`BEARDOG_SOCKET_TMP_DIR` overrides root)
        let tmp_root = std::env::var("BEARDOG_SOCKET_TMP_DIR")
            .map_or_else(|_| std::env::temp_dir(), PathBuf::from);
        let tmp_path = tmp_root
            .join(format!("{primal_name}-{family_id}-{node_id}.sock"))
            .display()
            .to_string();
        Ok(Self {
            socket_path: PathBuf::from(tmp_path),
            family_id,
            node_id,
            source: SocketPathSource::TempDir,
            production_mode,
        })
    }

    /// Create socket configuration from environment variables.
    ///
    /// Thin wrapper: [`SocketPathInputs::from_env`] then [`Self::from_inputs`].
    ///
    /// # Errors
    ///
    /// Returns `Err` when `FAMILY_ID` and `BIOMEOS_INSECURE` are both set.
    pub fn from_env() -> Result<Self, SocketConfigError> {
        Self::from_inputs(&SocketPathInputs::from_env())
    }

    /// Try to use XDG Runtime Directory.
    ///
    /// Returns `Some(path)` if `/run/user/<uid>/` exists, otherwise `None`.
    /// The socket filename is caller-determined and already family-scoped when applicable.
    fn try_xdg_runtime(uid: u32, socket_filename: &str) -> Option<PathBuf> {
        let xdg_runtime_dir = format!("/run/user/{uid}");
        if Path::new(&xdg_runtime_dir).exists() {
            Some(
                PathBuf::from(&xdg_runtime_dir)
                    .join(BIOMEOS_RUNTIME_SOCKET_SUBDIR)
                    .join(socket_filename),
            )
        } else {
            None
        }
    }

    /// Get the resolved socket path
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Get socket path as string
    #[must_use]
    pub fn socket_path_string(&self) -> String {
        self.socket_path.display().to_string()
    }

    /// Get family ID
    #[must_use]
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    /// Get node ID
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Get the source tier that was used
    #[must_use]
    pub const fn source(&self) -> SocketPathSource {
        self.source
    }

    /// Returns `true` when `FAMILY_ID` is non-default, indicating BTSP production mode.
    #[must_use]
    pub const fn production_mode(&self) -> bool {
        self.production_mode
    }

    /// Prepare the socket path for binding
    ///
    /// This ensures:
    /// 1. Parent directory exists (creates if needed)
    /// 2. Old socket file is removed (prevents "address already in use")
    ///
    /// Call this before binding to the socket.
    ///
    /// # Errors
    ///
    /// Returns `Err` with a message when the parent directory cannot be created or the stale socket
    /// file cannot be removed.
    pub fn prepare(&self) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = self.socket_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create socket directory {}: {}",
                    parent.display(),
                    e
                )
            })?;
        }

        // Remove old socket file if it exists
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path).map_err(|e| {
                format!(
                    "Failed to remove old socket {}: {}",
                    self.socket_path.display(),
                    e
                )
            })?;
        }

        Ok(())
    }

    /// Filename suffix for wateringHole IPC v3.1 capability symlinks (`{stem}{suffix}`), aligned with
    /// the primary socket name (`beardog.sock` vs `beardog-{family}.sock`).
    #[must_use]
    pub fn ipc_symlink_filename_suffix(&self) -> String {
        if self.production_mode {
            format!("-{}.sock", self.family_id)
        } else {
            ".sock".to_string()
        }
    }

    /// Install capability-domain symlinks next to the bound primal socket.
    ///
    /// Per `PRIMAL_IPC_PROTOCOL.md` v3.1, each discovered capability domain gets
    /// `{domain}{suffix} -> <primal-socket-basename>` (e.g. `crypto.sock -> beardog.sock`).
    ///
    /// Call after `prepare()` and a successful `bind()`. On failure for a single domain, logs a
    /// warning and continues. Returns paths successfully created (for diagnostics).
    #[must_use]
    pub fn install_ipc_capability_symlinks(&self, domain_stems: &[String]) -> Vec<PathBuf> {
        let Some(basename) = self.socket_path.file_name() else {
            return Vec::new();
        };
        install_ipc_symlinks_at(
            &self.socket_path,
            basename,
            &self.ipc_symlink_filename_suffix(),
            domain_stems,
        )
    }

    /// Remove symlinks for the given domain stems (same suffix as [`Self::ipc_symlink_filename_suffix`].
    pub fn remove_ipc_capability_symlinks(&self, domain_stems: &[String]) {
        remove_ipc_symlinks_at(
            &self.socket_path,
            &self.ipc_symlink_filename_suffix(),
            domain_stems,
        );
    }

    /// Get a descriptive string for logging
    #[must_use]
    pub fn description(&self) -> String {
        match self.source {
            SocketPathSource::PrimalEnvVar => {
                format!(
                    "{} (from BEARDOG_SOCKET env var ⭐ Tier 1)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::OrchestratorEnvVar => {
                format!(
                    "{} (from BIOMEOS_SOCKET_PATH env var ⭐ Tier 2 - Neural API)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::PrimalNamespace => {
                format!(
                    "{} (Primal IPC Protocol standard namespace - Tier 3)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::XdgRuntime => {
                format!(
                    "{} (XDG Runtime Directory - Tier 4)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::TempDir => {
                format!(
                    "{} (fallback to platform temp dir - Tier 5)",
                    self.socket_path_string()
                )
            }
        }
    }

    /// Create a custom socket configuration (for testing)
    pub fn custom(socket_path: impl Into<PathBuf>, family_id: String, node_id: String) -> Self {
        let production_mode = !family_id.is_empty() && family_id != "default";
        Self {
            socket_path: socket_path.into(),
            family_id,
            node_id,
            source: SocketPathSource::PrimalEnvVar,
            production_mode,
        }
    }
}

impl std::fmt::Display for SocketConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Configuration for wateringHole IPC v3.1 capability-domain symlinks next to the primal socket.
#[derive(Debug, Clone)]
pub struct IpcCapabilitySymlinksConfig {
    /// Symlink filename suffix (e.g. `.sock`, or `-<family>.sock` in production).
    pub symlink_suffix: String,
    /// Capability domain stems (e.g. `crypto`, `security`, `btsp`) — symlink `{stem}{symlink_suffix}` → primal basename.
    pub domain_stems: Vec<String>,
}

impl Default for IpcCapabilitySymlinksConfig {
    fn default() -> Self {
        Self {
            symlink_suffix: ".sock".to_string(),
            domain_stems: Vec::new(),
        }
    }
}

impl IpcCapabilitySymlinksConfig {
    /// Build from a resolved [`SocketConfig`] and discovered [`SimpleCapability`] set.
    #[must_use]
    pub fn from_socket_config_and_capabilities(
        socket_config: &SocketConfig,
        caps: &[SimpleCapability],
    ) -> Self {
        Self {
            symlink_suffix: socket_config.ipc_symlink_filename_suffix(),
            domain_stems: ipc_capability_domain_stems_resolved(caps),
        }
    }
}

/// Map [`SimpleCapability`] to wateringHole IPC domain symlink stems (unordered).
///
/// Security umbrella covers both `SimpleCapability::Cryptography` and `SimpleCapability::HsmIntegration` (operator
/// may enable either or both). `ed25519` / `x25519` stems are included when lineage / tunneling caps are present.
fn capability_stems_for_ipc(caps: &[SimpleCapability]) -> HashSet<String> {
    let mut stems = HashSet::new();
    for cap in caps {
        match cap {
            SimpleCapability::Cryptography => {
                stems.insert("crypto".into());
                stems.insert("ed25519".into());
                stems.insert("x25519".into());
                stems.insert("security".into());
            }
            SimpleCapability::SecureTunneling => {
                stems.insert("btsp".into());
                stems.insert("security".into());
            }
            SimpleCapability::GeneticLineage => {
                stems.insert("ed25519".into());
                stems.insert("x25519".into());
                stems.insert("crypto".into());
            }
            SimpleCapability::HsmIntegration => {
                stems.insert("security".into());
            }
            SimpleCapability::Discovery => {}
        }
    }
    stems
}

/// Preferred sort order for capability domain stems (stable ordering for tests and deterministic layout).
fn sort_ipc_capability_stems(stems: &mut [String]) {
    const PREFERRED: &[&str] = &["crypto", "btsp", "ed25519", "x25519", "security"];
    stems.sort_by_key(|s| {
        PREFERRED
            .iter()
            .position(|&p| p == s.as_str())
            .unwrap_or(100 + s.len())
    });
}

/// Resolve capability domain stems for symlink creation: env override first, else derived from `caps`.
#[must_use]
pub fn ipc_capability_domain_stems_resolved(caps: &[SimpleCapability]) -> Vec<String> {
    if let Ok(raw) = std::env::var("BEARDOG_IPC_CAPABILITY_STEMS") {
        let mut stems: Vec<String> = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        if !stems.is_empty() {
            sort_ipc_capability_stems(&mut stems);
            return stems;
        }
    }
    let mut stems: Vec<String> = capability_stems_for_ipc(caps).into_iter().collect();
    sort_ipc_capability_stems(&mut stems);
    stems
}

/// Derive symlink stems from [`SimpleCapability`] set (no env override).
#[must_use]
pub fn ipc_capability_domain_stems_from_capabilities(caps: &[SimpleCapability]) -> Vec<String> {
    let mut stems: Vec<String> = capability_stems_for_ipc(caps).into_iter().collect();
    sort_ipc_capability_stems(&mut stems);
    stems
}

/// Install wateringHole capability-domain symlinks beside `primal_socket_path` (e.g. `crypto.sock` → `beardog.sock`).
///
/// `target_basename` is the filename of the primal socket (e.g. `OsStr` of `beardog.sock`).
/// Symlink names are `{stem}{symlink_suffix}` (e.g. `crypto` + `.sock` → `crypto.sock`).
///
/// # Errors
///
/// Returns I/O errors from symlink creation; caller should treat partial success as acceptable (warn-only).
#[must_use]
pub fn install_ipc_symlinks_at(
    primal_socket_path: &Path,
    target_basename: &OsStr,
    symlink_suffix: &str,
    domain_stems: &[String],
) -> Vec<PathBuf> {
    let mut created = Vec::new();
    let Some(parent) = primal_socket_path.parent() else {
        return created;
    };
    for stem in domain_stems {
        let name = format!("{stem}{symlink_suffix}");
        let link_path = parent.join(&name);
        #[cfg(unix)]
        {
            if link_path.exists() {
                let _ = fs::remove_file(&link_path);
            }
            if let Err(e) = std::os::unix::fs::symlink(target_basename, &link_path) {
                warn!(
                    target = %target_basename.to_string_lossy(),
                    link = %link_path.display(),
                    err = %e,
                    "failed to create wateringHole capability symlink"
                );
            } else {
                created.push(link_path);
            }
        }
    }
    created
}

/// Remove wateringHole capability symlinks beside `primal_socket_path` for `domain_stems`.
pub fn remove_ipc_symlinks_at(
    primal_socket_path: &Path,
    symlink_suffix: &str,
    domain_stems: &[String],
) {
    let Some(parent) = primal_socket_path.parent() else {
        return;
    };
    for stem in domain_stems {
        let path = parent.join(format!("{stem}{symlink_suffix}"));
        if path.exists() {
            let _ = fs::remove_file(&path);
        }
    }
}

#[cfg(test)]
#[path = "socket_config_tests.rs"]
mod tests;
