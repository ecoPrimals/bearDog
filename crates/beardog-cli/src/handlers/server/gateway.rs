// SPDX-License-Identifier: AGPL-3.0-or-later

//! HTTPS gateway — TLS-terminating reverse proxy for the Tower HTTP Gateway.
//!
//! Accepts TLS connections on `:443` using ACME-managed certificates via
//! [`HotReloadAcceptor`], then forwards decrypted HTTP traffic to the
//! configured upstream.
//!
//! Upstream resolution (precedence order):
//! 1. `BEARDOG_GATEWAY_UPSTREAM` env var (`host:port` or `unix:/path`)
//! 2. `BEARDOG_GATEWAY_UPSTREAM_PORT` env var (localhost on that port)
//! 3. Error — no hardcoded defaults. The upstream must be explicitly
//!    configured or discovered at runtime.
//!
//! **Primal isolation**: bearDog has no compile-time knowledge of which
//! primal provides the upstream HTTP proxy. The operator configures it.

use beardog_acme::HotReloadAcceptor;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_types::btsp::TransportEndpoint;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tracing::{debug, info, warn};

/// Env var for specifying only the upstream port (localhost assumed).
const ENV_GATEWAY_UPSTREAM_PORT: &str = "BEARDOG_GATEWAY_UPSTREAM_PORT";

fn resolve_upstream() -> Result<TransportEndpoint, BearDogError> {
    if let Ok(val) = std::env::var(env_keys::ENV_GATEWAY_UPSTREAM) {
        if val.starts_with("unix:") {
            let path = val.strip_prefix("unix:").unwrap_or(&val);
            #[cfg(unix)]
            {
                return Ok(TransportEndpoint::Uds {
                    path: PathBuf::from(path),
                });
            }
            #[cfg(not(unix))]
            {
                return Err(BearDogError::system(format!(
                    "unix: upstream not supported on this platform: {val}"
                )));
            }
        }

        if !val.is_empty() {
            if let Some((host, port_str)) = val.rsplit_once(':')
                && let Ok(port) = port_str.parse::<u16>()
            {
                return Ok(TransportEndpoint::Tcp {
                    host: host.to_string(),
                    port,
                });
            }
            return Err(BearDogError::system(format!(
                "BEARDOG_GATEWAY_UPSTREAM must be host:port or unix:/path, got: {val}"
            )));
        }
    }

    if let Ok(port_str) = std::env::var(ENV_GATEWAY_UPSTREAM_PORT)
        && let Ok(port) = port_str.parse::<u16>()
    {
        return Ok(TransportEndpoint::Tcp {
            host: "127.0.0.1".to_string(),
            port,
        });
    }

    Err(BearDogError::system(
        "Gateway upstream not configured: set BEARDOG_GATEWAY_UPSTREAM (host:port or unix:/path) \
         or BEARDOG_GATEWAY_UPSTREAM_PORT"
            .to_string(),
    ))
}

/// Start the HTTPS gateway on the given port using ACME-managed certificates.
///
/// Terminates TLS and bidirectionally proxies cleartext HTTP to the upstream.
///
/// # Errors
///
/// Returns an error if the TCP listener cannot bind.
pub async fn serve_https_gateway(
    acceptor: HotReloadAcceptor,
    bind_port: u16,
) -> Result<(), BearDogError> {
    let addr = format!("0.0.0.0:{bind_port}");
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| BearDogError::system(format!("HTTPS gateway bind {addr}: {e}")))?;

    let upstream = resolve_upstream()?;
    info!(port = bind_port, upstream = %upstream, "HTTPS gateway listening (ACME TLS → upstream)");

    loop {
        let (tcp_stream, peer) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                warn!(error = %e, "HTTPS gateway accept error");
                continue;
            }
        };

        let tls_acceptor = acceptor.current();
        let upstream = upstream.clone();
        tokio::spawn(async move {
            let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                Ok(s) => s,
                Err(e) => {
                    debug!(peer = %peer, error = %e, "TLS handshake failed");
                    return;
                }
            };

            let mut upstream_conn = match beardog_ipc::connect_raw(&upstream).await {
                Ok(c) => c,
                Err(e) => {
                    warn!(peer = %peer, error = %e, "upstream connect failed");
                    let mut stream = tls_stream;
                    let _ = stream
                        .write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n")
                        .await;
                    return;
                }
            };

            let (mut tls_read, mut tls_write) = tokio::io::split(tls_stream);
            let (mut up_read, mut up_write) = tokio::io::split(&mut upstream_conn);
            let _result = tokio::join!(
                tokio::io::copy(&mut tls_read, &mut up_write),
                tokio::io::copy(&mut up_read, &mut tls_write),
            );
            debug!(peer = %peer, "gateway connection closed");
        });
    }
}
