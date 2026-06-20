// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ServerArgs;
use beardog_types::constants::domains::network::addresses::WILDCARD_IPV4;
use beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional;

/// Resolve TCP listen address from [`ServerArgs::port`] / [`ServerArgs::listen`] (`UniBin` v1.1).
///
/// Returns [`None`] when neither is set, or when both are set (normally prevented by `clap`
/// `conflicts_with`, but callers may construct [`ServerArgs`] programmatically).
#[must_use]
pub fn resolve_effective_tcp_listen(port: Option<u16>, listen: Option<&str>) -> Option<String> {
    match (port, listen) {
        (Some(p), None) => Some(format!("{WILDCARD_IPV4}:{p}")),
        (None, Some(addr)) => Some(addr.to_string()),
        _ => None,
    }
}

/// Resolve the effective socket path for server startup.
///
/// Bind-mode priority: `--bind-mode abstract` (or legacy `--abstract`) → abstract
/// namespace name. `--bind-mode tcp` → returns a placeholder (UDS is skipped
/// later). `--bind-mode filesystem` or `auto` → family-aware filesystem path.
pub fn resolve_server_socket_path(args: &ServerArgs) -> String {
    let use_abstract =
        args.bind_mode == crate::BindMode::Abstract || args.r#abstract;

    if use_abstract {
        let family = args.family_id.as_deref().unwrap_or("default");
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        format!("@{ns}_beardog_{family}")
    } else if args.bind_mode == crate::BindMode::Tcp {
        String::new()
    } else if let Some(ref family_id) = args.family_id {
        let family_sock = std::path::PathBuf::from(&args.socket);
        let parent = family_sock
            .parent()
            .unwrap_or_else(|| std::path::Path::new("/tmp"));
        parent
            .join(format!("beardog-{family_id}.sock"))
            .to_string_lossy()
            .to_string()
    } else {
        args.socket.clone()
    }
}

/// Address string advertised to the Neural API for registration (`TCP` vs Unix path).
#[must_use]
pub(super) fn neural_registration_address<'a>(
    tcp_listen: Option<&'a str>,
    unix_socket_path: &'a str,
) -> &'a str {
    tcp_listen.unwrap_or(unix_socket_path)
}
