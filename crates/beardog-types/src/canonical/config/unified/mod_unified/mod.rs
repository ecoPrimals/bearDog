//! # Modular Unified Configuration Implementation
//!
//! This module contains the actual implementation of the unified configuration system
//! split into manageable domain modules.

// Declare all domain modules
pub mod system;
pub mod network;
pub mod security;
pub mod hsm;
pub mod adapters;
pub mod genetics;
pub mod production;
pub mod performance;
pub mod auth;
pub mod compliance;
pub mod database;
pub mod cache;

// Main unified configuration implementation
pub mod mod_unified;

// Re-export everything from the main implementation
pub use self::mod_unified::*; 