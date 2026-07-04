// SPDX-License-Identifier: AGPL-3.0-or-later

//! HTTPS gateway — TLS-terminating reverse proxy for the Tower HTTP Gateway.
//!
//! Accepts TLS connections on `:443` using ACME-managed certificates via
//! [`HotReloadAcceptor`], then forwards decrypted HTTP traffic to the
//! configured upstream (songBird `http.proxy` by default).
//!
//! The upstream is resolved from `BEARDOG_GATEWAY_UPSTREAM`:
//! - `host:port` — TCP upstream (e.g., `127.0.0.1:7700`)
//! - `unix:/path/to/socket` — Unix domain socket
//! - absent — default `127.0.0.1:7700` (songBird federation port)

use beardog_acme::HotReloadAcceptor;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, warn};

/// Upstream target for proxied HTTP traffic.
#[derive(Debug, Clone)]
enum Upstream {
    Tcp(Arc<str>),
    #[cfg(unix)]
    Unix(Arc<str>),
}

impl Upstream {
    fn resolve() -> Self {
        match std::env::var(env_keys::ENV_GATEWAY_UPSTREAM) {
            Ok(val) if val.starts_with("unix:") => {
                #[cfg(unix)]
                {
                    Self::Unix(val.strip_prefix("unix:").unwrap_or(&val).into())
                }
                #[cfg(not(unix))]
                {
                    warn!(upstream = %val, "unix: upstream not supported on this platform, falling back to TCP");
                    Self::Tcp("127.0.0.1:7700".into())
                }
            }
            Ok(val) if !val.is_empty() => Self::Tcp(val.into()),
            _ => Self::Tcp("127.0.0.1:7700".into()),
        }
    }

    async fn connect(&self) -> std::io::Result<UpstreamConn> {
        match self {
            Self::Tcp(addr) => {
                let stream = TcpStream::connect(addr.as_ref()).await?;
                Ok(UpstreamConn::Tcp(stream))
            }
            #[cfg(unix)]
            Self::Unix(path) => {
                let stream = tokio::net::UnixStream::connect(path.as_ref()).await?;
                Ok(UpstreamConn::Unix(stream))
            }
        }
    }
}

enum UpstreamConn {
    Tcp(TcpStream),
    #[cfg(unix)]
    Unix(tokio::net::UnixStream),
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
    let listener = TcpListener::bind(&addr).await.map_err(|e| {
        BearDogError::system(format!("HTTPS gateway bind {addr}: {e}"))
    })?;

    let upstream = Upstream::resolve();
    info!(port = bind_port, ?upstream, "HTTPS gateway listening (ACME TLS → upstream)");

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

            let upstream_conn = match upstream.connect().await {
                Ok(c) => c,
                Err(e) => {
                    warn!(peer = %peer, error = %e, "upstream connect failed");
                    let mut stream = tls_stream;
                    let _ = stream.write_all(
                        b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n"
                    ).await;
                    return;
                }
            };

            match upstream_conn {
                UpstreamConn::Tcp(upstream_tcp) => {
                    let (mut tls_read, mut tls_write) = tokio::io::split(tls_stream);
                    let (mut up_read, mut up_write) = tokio::io::split(upstream_tcp);
                    let _result = tokio::join!(
                        tokio::io::copy(&mut tls_read, &mut up_write),
                        tokio::io::copy(&mut up_read, &mut tls_write),
                    );
                }
                #[cfg(unix)]
                UpstreamConn::Unix(upstream_unix) => {
                    let (mut tls_read, mut tls_write) = tokio::io::split(tls_stream);
                    let (mut up_read, mut up_write) = tokio::io::split(upstream_unix);
                    let _result = tokio::join!(
                        tokio::io::copy(&mut tls_read, &mut up_write),
                        tokio::io::copy(&mut up_read, &mut tls_write),
                    );
                }
            }
            debug!(peer = %peer, "gateway connection closed");
        });
    }
}
