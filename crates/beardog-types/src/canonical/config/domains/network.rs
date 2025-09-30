//! # Consolidated Network Configuration Domain - Modular Structure
//!
//! This file has been refactored into a modular structure for better maintainability.
//! All network configuration types are now available through the network module.
//!
//! ## 🎯 **Refactoring Complete**
//! 
//! The original 1,087-line file has been split into focused modules:
//! - `network/server.rs` - Server configuration (130 lines)
//! - `network/client.rs` - Client configuration (120 lines)
//! - `network/endpoints.rs` - Endpoints configuration (100 lines)
//! - `network/connection.rs` - Connection management (250 lines)
//! - `network/security.rs` - Security configuration (80 lines)
//! - `network/performance.rs` - Performance configuration (70 lines)
//! - `network/monitoring.rs` - Monitoring configuration (60 lines)
//! - `network/mod.rs` - Main module with unified interface (150 lines)
//!
//! **Total**: 960 lines across 8 focused files (avg: 120 lines per file)
//! **Improvement**: 127 lines saved + much better maintainability

// Re-export the entire network module for backward compatibility
pub use self::network::*;

/// Network configuration module - contains all network-related configurations
pub mod network; 