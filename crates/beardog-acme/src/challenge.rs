// SPDX-License-Identifier: AGPL-3.0-or-later

//! HTTP-01 challenge solver and Gatehouse HTTP handler for ACME domain validation.
//!
//! Serves `/.well-known/acme-challenge/<token>` on port 80 to prove domain
//! control during certificate issuance. For all other requests, returns a
//! 301 redirect to HTTPS (Gatehouse policy: port 80 is the drawbridge entry
//! for ACME challenges, everything else crosses via :443).

use crate::error::AcmeError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// A pending ACME challenge token and its key authorization.
#[derive(Debug, Clone)]
pub struct ChallengeToken {
    /// The token from the ACME challenge object.
    pub token: String,
    /// The key authorization: `{token}.{account_thumbprint}`.
    pub key_authorization: String,
}

impl ChallengeToken {
    /// Create a new challenge token with its key authorization.
    #[must_use]
    pub fn new(token: String, account_thumbprint: &str) -> Self {
        let key_authorization = format!("{token}.{account_thumbprint}");
        Self {
            token,
            key_authorization,
        }
    }
}

/// HTTP-01 challenge solver that serves key authorizations on port 80.
///
/// Multiple challenges can be active simultaneously (e.g., SAN certificates).
#[derive(Debug, Clone)]
pub struct Http01Solver {
    /// Active challenges: token → `key_authorization`.
    challenges: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for Http01Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Http01Solver {
    /// Create a new solver with no active challenges.
    #[must_use]
    pub fn new() -> Self {
        Self {
            challenges: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a challenge token for serving.
    pub async fn add_challenge(&self, challenge: &ChallengeToken) {
        let mut challenges = self.challenges.write().await;
        challenges.insert(challenge.token.clone(), challenge.key_authorization.clone());
        info!(token = %challenge.token, "registered ACME HTTP-01 challenge");
    }

    /// Remove a challenge after validation completes.
    pub async fn remove_challenge(&self, token: &str) {
        let mut challenges = self.challenges.write().await;
        challenges.remove(token);
        debug!(token, "removed completed ACME challenge");
    }

    /// Get the key authorization for a token (if registered).
    pub async fn get_key_auth(&self, token: &str) -> Option<String> {
        let challenges = self.challenges.read().await;
        challenges.get(token).cloned()
    }

    /// Start the Gatehouse HTTP server on the given port.
    ///
    /// Dual-purpose: serves ACME HTTP-01 challenges at
    /// `/.well-known/acme-challenge/<token>`, and redirects all other
    /// HTTP requests to HTTPS (301 Moved Permanently). This eliminates
    /// the need for Caddy or any other HTTP→HTTPS redirect service.
    ///
    /// # Errors
    ///
    /// Returns an error if the TCP listener cannot bind to the specified port.
    pub async fn serve(&self, port: u16) -> Result<(), AcmeError> {
        let addr = format!("0.0.0.0:{port}");
        let listener = TcpListener::bind(&addr).await?;
        info!(port, "gatehouse HTTP server listening (ACME challenges + HTTPS redirect)");

        let challenges = Arc::clone(&self.challenges);

        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    warn!(error = %e, "ACME challenge server accept error");
                    continue;
                }
            };

            let challenges = Arc::clone(&challenges);
            tokio::spawn(async move {
                let mut reader = BufReader::new(stream);
                let mut request_line = String::new();
                if reader.read_line(&mut request_line).await.is_err() {
                    return;
                }

                let path = request_line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("/")
                    .to_string();

                let mut host = String::new();
                let mut header = String::new();
                loop {
                    header.clear();
                    match reader.read_line(&mut header).await {
                        Ok(0) | Err(_) => break,
                        Ok(_) if header.trim().is_empty() => break,
                        Ok(_) => {
                            if let Some(val) = header.strip_prefix("Host:").or_else(|| header.strip_prefix("host:")) {
                                host = val.trim().to_string();
                            }
                        }
                        #[allow(unreachable_patterns)]
                        _ => {}
                    }
                }

                let response = if let Some(token) =
                    path.strip_prefix("/.well-known/acme-challenge/")
                {
                    let challenges = challenges.read().await;
                    if let Some(key_auth) = challenges.get(token) {
                        debug!(token, peer = %peer, "serving ACME challenge response");
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                            key_auth.len(),
                            key_auth,
                        )
                    } else {
                        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_string()
                    }
                } else if host.is_empty() {
                    "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n".to_string()
                } else {
                    debug!(peer = %peer, host = %host, path = %path, "gatehouse: HTTP→HTTPS redirect");
                    format!(
                        "HTTP/1.1 301 Moved Permanently\r\nLocation: https://{host}{path}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    )
                };

                let stream = reader.into_inner();
                let mut stream = stream;
                let _ = stream.write_all(response.as_bytes()).await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn challenge_token_key_authorization_format() {
        let token = ChallengeToken::new("abc123".to_string(), "thumb456");
        assert_eq!(token.key_authorization, "abc123.thumb456");
    }

    #[tokio::test]
    async fn add_and_get_challenge() {
        let solver = Http01Solver::new();
        let token = ChallengeToken::new("tok".to_string(), "tp");
        solver.add_challenge(&token).await;

        let auth = solver.get_key_auth("tok").await;
        assert_eq!(auth.as_deref(), Some("tok.tp"));
    }

    #[tokio::test]
    async fn remove_challenge_clears_entry() {
        let solver = Http01Solver::new();
        let token = ChallengeToken::new("tok".to_string(), "tp");
        solver.add_challenge(&token).await;
        solver.remove_challenge("tok").await;

        assert!(solver.get_key_auth("tok").await.is_none());
    }

    #[tokio::test]
    async fn get_key_auth_returns_none_for_unknown_token() {
        let solver = Http01Solver::new();
        assert!(solver.get_key_auth("nonexistent").await.is_none());
    }

    #[tokio::test]
    async fn multiple_challenges_coexist() {
        let solver = Http01Solver::new();
        solver
            .add_challenge(&ChallengeToken::new("a".to_string(), "t"))
            .await;
        solver
            .add_challenge(&ChallengeToken::new("b".to_string(), "t"))
            .await;

        assert!(solver.get_key_auth("a").await.is_some());
        assert!(solver.get_key_auth("b").await.is_some());

        solver.remove_challenge("a").await;
        assert!(solver.get_key_auth("a").await.is_none());
        assert!(solver.get_key_auth("b").await.is_some());
    }

    // ── Integration tests: actual HTTP listener ─────────────────────────

    #[tokio::test]
    async fn serve_redirects_http_to_https() {
        let solver = Http01Solver::new();
        let port = portpicker::pick_unused_port().expect("free port");

        let solver_clone = solver.clone();
        tokio::spawn(async move {
            let _ = solver_clone.serve(port).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut stream = tokio::net::TcpStream::connect(format!("127.0.0.1:{port}"))
            .await
            .expect("connect");
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        stream
            .write_all(b"GET /some/path HTTP/1.1\r\nHost: example.com\r\n\r\n")
            .await
            .expect("write");

        let mut reader = BufReader::new(stream);
        let mut status_line = String::new();
        reader.read_line(&mut status_line).await.expect("read");
        assert!(
            status_line.contains("301"),
            "expected 301, got: {status_line}"
        );

        let mut headers = String::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).await.expect("read header");
            if line.trim().is_empty() {
                break;
            }
            headers.push_str(&line);
        }
        assert!(
            headers.contains("Location: https://example.com/some/path"),
            "expected redirect location, got: {headers}"
        );
    }

    #[tokio::test]
    async fn serve_returns_acme_challenge_token() {
        let solver = Http01Solver::new();
        let port = portpicker::pick_unused_port().expect("free port");

        solver
            .add_challenge(&ChallengeToken::new("test-auth-value".to_string(), "t"))
            .await;

        let solver_clone = solver.clone();
        tokio::spawn(async move {
            let _ = solver_clone.serve(port).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut stream = tokio::net::TcpStream::connect(format!("127.0.0.1:{port}"))
            .await
            .expect("connect");
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        stream
            .write_all(
                b"GET /.well-known/acme-challenge/test-auth-value HTTP/1.1\r\nHost: example.com\r\n\r\n",
            )
            .await
            .expect("write");

        let mut reader = BufReader::new(stream);
        let mut status_line = String::new();
        reader.read_line(&mut status_line).await.expect("read");
        assert!(
            status_line.contains("200"),
            "expected 200, got: {status_line}"
        );
    }

    #[tokio::test]
    async fn serve_returns_400_for_missing_host() {
        let solver = Http01Solver::new();
        let port = portpicker::pick_unused_port().expect("free port");

        let solver_clone = solver.clone();
        tokio::spawn(async move {
            let _ = solver_clone.serve(port).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut stream = tokio::net::TcpStream::connect(format!("127.0.0.1:{port}"))
            .await
            .expect("connect");
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        stream
            .write_all(b"GET /foo HTTP/1.1\r\n\r\n")
            .await
            .expect("write");

        let mut reader = BufReader::new(stream);
        let mut status_line = String::new();
        reader.read_line(&mut status_line).await.expect("read");
        assert!(
            status_line.contains("400"),
            "expected 400, got: {status_line}"
        );
    }
}
