// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Unified Config Utils System
//!
//! This module consolidates ALL scattered config utility modules across the BearDog ecosystem
//! into a single, comprehensive, maintainable location for configuration operations.
//!
//! ## 🎯 **Complete Config Consolidation Strategy**
//!
//! This module consolidates and replaces:
//! - `beardog-utils/src/utils/config_utils.rs` - Basic config utilities
//! - `beardog-utils/src/zero_copy/shared_config.rs` - Shared config management
//! - Scattered config helpers across multiple crates
//!
//! ## 🏗️ **Architecture Benefits**
//!
//! - **Single Source of Truth**: All config operations in one canonical location
//! - **Zero Fragmentation**: No duplicate config utility definitions
//! - **Performance Optimized**: Efficient implementations with caching
//! - **Type Safety**: Comprehensive error handling and validation
//! - **Memory Efficient**: Smart caching and shared configurations
//! - **File System Safe**: Proper permissions and path handling

mod serde_arc;
mod shared_manager;
mod types;
mod unified_impl;

#[cfg(test)]
mod tests;

pub use serde_arc::{deserialize_arc_str, serialize_arc_str};
pub use types::{ConfigPerformanceMetrics, SharedConfigStats};

/// **UNIFIED CONFIG UTILS** - Single source of truth for all config operations
pub struct UnifiedConfigUtils;

/// Shared configuration manager implementation
pub use shared_manager::SharedConfigManager;

// =============================================================================
// LEGACY COMPATIBILITY REMOVED - Nov 11, 2025
// =============================================================================
// All legacy functions removed - Zero active usage confirmed
// Migration: Use UnifiedConfigUtils methods directly
