// SPDX-License-Identifier: AGPL-3.0-or-later

//! HTTPS gateway — TLS-terminating reverse proxy for the Tower HTTP Gateway.
//!
//! Accepts TLS connections on `:443` using ACME-managed certificates via
//! [`HotReloadAcceptor`], then forwards decrypted HTTP traffic to the
//! configured upstream (songBird `http.proxy` by default).
//!
//! The upstream is resolved from `BEARDOG_GATEWAY_UPSTREAM`:
//! - `host:port` — TCP upstream (e.g., `127.0.0.1:7780`)
//! - `unix:/path/to/socket` — Unix domain socket
//! - absent — default `127.0.0.1:7780` (songBird drawbridge HTTP listener)

use beardog_acme::HotReloadAcceptor;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_types::btsp::TransportEndpoint;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tracing::{debug, info, warn};

fn resolve_upstream() -> TransportEndpoint {
    match std::env::var(env_keys::ENV_GATEWAY_UPSTREAM) {
        Ok(val) if val.starts_with("unix:") => {
            let path = val.strip_prefix("unix:").unwrap_or(&val);
            #[cfg(unix)]
            {
                TransportEndpoint::Uds {
                    path: PathBuf::from(path),
                }
            }
            #[cfg(not(unix))]
            {
                warn!(upstream = %val, "unix: upstream not supported on this platform, falling back to TCP");
                TransportEndpoint::Tcp {
                    host: "127.0.0.1".to_string(),
                    port: 7780,
                }
            }
        }
        Ok(val) if !val.is_empty() => {
            if let Some((host, port_str)) = val.rsplit_once(':')
                && let Ok(port) = port_str.parse::<u16>()
            {
                return TransportEndpoint::Tcp {
                    host: host.to_string(),
                    port,
                };
            }
            TransportEndpoint::Tcp {
                host: val,
                port: 7780,
            }
        }
        _ => TransportEndpoint::Tcp {
            host: "127.0.0.1".to_string(),
            port: 7780,
        },
    }
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

    let upstream = resolve_upstream();
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
