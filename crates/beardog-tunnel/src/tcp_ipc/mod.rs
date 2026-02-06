//! TCP IPC Transport for BearDog
//!
//! **Purpose**: Universal IPC transport that works on ALL platforms
//! **Primary Use**: Android (SELinux blocks Unix sockets for shell user)
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
//! - ✅ Zero unsafe code (pure Rust tokio)
//! - ✅ No C dependencies
//! - ✅ Auto-fallback (try Unix → TCP)
//!
//! ## Usage
//!
//! ```bash
//! # TCP mode (Android, universal)
//! beardog server --listen 127.0.0.1:9900
//!
//! # Auto port (OS assigns)
//! beardog server --listen 127.0.0.1:0
//!
//! # Unix socket (Linux, preferred)
//! beardog server --socket /run/user/1000/biomeos/beardog.sock
//! ```

pub mod client;
pub mod server;

pub use client::TcpIpcClient;
pub use server::TcpIpcServer;
