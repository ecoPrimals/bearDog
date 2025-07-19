//! Command-line interface commands for BearDog
//!
//! This module contains all CLI commands for BearDog operations.

pub mod ai;

// Re-export AI commands for main CLI
pub use ai::{execute_ai_command, AiCommand};
