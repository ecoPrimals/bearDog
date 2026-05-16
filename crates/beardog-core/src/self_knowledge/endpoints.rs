// SPDX-License-Identifier: AGPL-3.0-or-later

//! Network endpoint discovery (OS-assigned or env-configured).

use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::path::PathBuf;

use beardog_config::domains::network_addresses::NetworkAddressesConfig;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Sentinel [`SocketAddr`] for [`Protocol::UnixSocket`] (`Endpoint::address` is unused for UDS).
#[inline]
fn unix_socket_sentinel_addr() -> SocketAddr {
    SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))
}

/// Inputs for endpoint discovery (pure data, no env reads).
#[derive(Debug, Clone, Default)]
pub struct EndpointInputs {
    /// `BEARDOG_LISTEN_ADDR`
    pub beardog_listen_addr: Option<String>,
    /// `BEARDOG_PORT`
    pub beardog_port: Option<String>,
}

impl EndpointInputs {
    /// Read endpoint inputs from the process environment.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            beardog_listen_addr: std::env::var("BEARDOG_LISTEN_ADDR").ok(),
            beardog_port: std::env::var("BEARDOG_PORT").ok(),
        }
    }
}

/// Network endpoint (where this primal listens).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    /// Protocol (HTTP, gRPC, etc.).
    pub protocol: Protocol,
    /// Socket address (OS-assigned or configured).
    ///
    /// For [`Protocol::UnixSocket`], this is a sentinel value; use [`Self::unix_socket_path`].
    pub address: SocketAddr,
    /// Unix domain socket path when `protocol` is [`Protocol::UnixSocket`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unix_socket_path: Option<PathBuf>,
}

impl Endpoint {
    /// Parse an endpoint from a string.
    ///
    /// Supports formats:
    /// - `127.0.0.1:8900` (defaults to HTTP)
    /// - `http://127.0.0.1:8900`
    /// - `grpc://127.0.0.1:8900`
    /// - `unix:///run/user/1000/biomeos/foo.sock` (PRIMAL IPC)
    /// - On Unix, an absolute path `/run/.../foo.sock` is treated as a Unix socket
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the unix path is empty or the host:port cannot be parsed.
    pub fn parse(s: &str) -> Result<Self, BearDogError> {
        let trimmed = s.trim();
        if let Some(rest) = trimmed
            .strip_prefix("unix://")
            .or_else(|| trimmed.strip_prefix("unix:"))
        {
            let path = PathBuf::from(rest);
            if path.as_os_str().is_empty() {
                return Err(BearDogError::network(
                    "Empty unix:// path in endpoint".to_string(),
                ));
            }
            let sentinel = unix_socket_sentinel_addr();
            return Ok(Self {
                protocol: Protocol::UnixSocket,
                address: sentinel,
                unix_socket_path: Some(path),
            });
        }

        #[cfg(unix)]
        if trimmed.starts_with('/') {
            let path = PathBuf::from(trimmed);
            let sentinel = unix_socket_sentinel_addr();
            return Ok(Self {
                protocol: Protocol::UnixSocket,
                address: sentinel,
                unix_socket_path: Some(path),
            });
        }

        let (protocol, addr_str) = if let Some(rest) = trimmed.strip_prefix("http://") {
            (Protocol::Http, rest)
        } else if let Some(rest) = trimmed.strip_prefix("grpc://") {
            (Protocol::Grpc, rest)
        } else {
            (Protocol::Http, trimmed)
        };

        let address = addr_str.parse::<SocketAddr>().map_err(|e| {
            BearDogError::network(format!("Invalid endpoint address '{addr_str}': {e}"))
        })?;

        Ok(Self {
            protocol,
            address,
            unix_socket_path: None,
        })
    }
}

/// Network protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    /// HTTP/HTTPS.
    Http,
    /// gRPC.
    Grpc,
    /// Unix domain socket.
    UnixSocket,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http => write!(f, "HTTP"),
            Self::Grpc => write!(f, "gRPC"),
            Self::UnixSocket => write!(f, "Unix Socket"),
        }
    }
}

/// Discover endpoints where this primal listens.
///
/// Priority order:
/// 1. Explicit `beardog_listen_addr`
/// 2. `beardog_port` (with loopback from [`NetworkAddressesConfig`])
/// 3. Default: loopback with port `0` (OS-assigned ephemeral port)
///
/// The loopback address comes from environment-driven [`NetworkAddressesConfig`] (not a fixed
/// literal). Peer URLs and remote service endpoints for the wider ecosystem should come from
/// capability discovery / registry data rather than being assumed here.
///
/// # Errors
///
/// Returns [`BearDogError`] when listen addresses or ports are invalid.
pub fn discover_endpoints_from_inputs(
    inputs: &EndpointInputs,
) -> Result<Vec<Endpoint>, BearDogError> {
    let mut endpoints = Vec::new();
    let network_addrs = NetworkAddressesConfig::from_env();
    let bind_loopback = network_addrs.localhost_ipv4;

    if let Some(ref addr_str) = inputs.beardog_listen_addr {
        debug!("Using BEARDOG_LISTEN_ADDR: {}", addr_str);

        let addr = addr_str
            .to_socket_addrs()
            .map_err(|e| {
                BearDogError::network(format!("Invalid BEARDOG_LISTEN_ADDR '{addr_str}': {e}"))
            })?
            .next()
            .ok_or_else(|| {
                BearDogError::network(format!(
                    "Could not resolve BEARDOG_LISTEN_ADDR '{addr_str}'"
                ))
            })?;

        endpoints.push(Endpoint {
            protocol: Protocol::Http,
            address: addr,
            unix_socket_path: None,
        });

        return Ok(endpoints);
    }

    if let Some(ref port_str) = inputs.beardog_port {
        let port: u16 = port_str.parse().map_err(|e| {
            BearDogError::network(format!("Invalid BEARDOG_PORT '{port_str}': {e}"))
        })?;

        debug!("Using BEARDOG_PORT: {}", port);

        endpoints.push(Endpoint {
            protocol: Protocol::Http,
            address: SocketAddr::new(bind_loopback, port),
            unix_socket_path: None,
        });

        return Ok(endpoints);
    }

    debug!("No explicit endpoint configured, using OS-assigned port on loopback");
    endpoints.push(Endpoint {
        protocol: Protocol::Http,
        // Default bind for local development: ephemeral port on configured loopback (see
        // `NetworkAddressesConfig::from_env`). Remote peers must be resolved via capability discovery.
        address: SocketAddr::new(bind_loopback, 0),
        unix_socket_path: None,
    });

    Ok(endpoints)
}

/// Discover endpoints using [`EndpointInputs::from_env`].
///
/// # Errors
///
/// Same as [`discover_endpoints_from_inputs`].
pub fn discover_endpoints_from_env() -> Result<Vec<Endpoint>, BearDogError> {
    discover_endpoints_from_inputs(&EndpointInputs::from_env())
}
