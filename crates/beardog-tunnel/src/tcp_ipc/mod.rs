// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP IPC Transport for `BearDog`
//!
//! **Purpose**: Universal IPC transport that works on ALL platforms
//! **Primary Use**: Android (`SELinux` blocks Unix sockets for shell user)
//! **Secondary**: Windows, containers, cross-device
//!
//! ## Philosophy (Feb 2, 2026)
//!
//! Primals should ALWAYS function:
//! - **Tier 1** (Full system): JSON-RPC + Unix sockets (Linux, macOS)
//! - **Tier 2** (Degraded): TCP transport (Android shell, Windows)
//! - **Tier 3** (Elevated): App packaging with proper permissions (later)
//!
//! They function BETTER with more tech available, but MUST function in all environments.
//!
//! ## TRUE ecoBin v2.0 Compliance
//!
//! - ✅ Platform-agnostic (works everywhere)
//! - ✅ Fully memory-safe (pure Rust tokio)
//! - ✅ No C dependencies
//! - ✅ Auto-fallback (try Unix → TCP)
//!
//! ## Usage
//!
//! ```bash
//! # TCP mode (Android, universal)
//! beardog server --listen 127.0.0.1:9100
//!
//! # Auto port (OS assigns)
//! beardog server --listen 127.0.0.1:0
//!
//! # Unix socket (Linux, preferred)
//! beardog server --socket /run/user/1000/biomeos/beardog.sock
//! ```

pub mod client;
pub mod rate_limiter;
pub mod server;
#[cfg(feature = "tls-server")]
pub mod tls;

pub use client::TcpIpcClient;
pub use rate_limiter::{ConnectionRateLimiter, RateLimitConfig};
pub use server::TcpIpcServer;
#[cfg(feature = "tls-server")]
pub use tls::{TlsTerminationConfig, build_tls_acceptor};
