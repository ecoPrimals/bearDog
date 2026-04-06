// SPDX-License-Identifier: AGPL-3.0-or-later

// Zero-cost abstractions for BearDog
//
// This module provides zero-cost abstractions for performance-critical operations.
// All implementations are memory-safe and use modern Rust patterns.
//
// **MIGRATION NOTICE**: Many types have been migrated to the unified provider system.
// Use `canonical::providers_unified::traits` for new implementations.

// Core zero-cost types
/// Core zero-cost type definitions
pub mod types;
/// Workflow zero-cost abstractions
pub mod workflow;

// Safe implementations - production ready
/// Safe memory management using modern Rust patterns
pub mod memory_safe;

// Re-export core types that actually exist
pub use types::{HealthStatus, HsmKey, KeyType, PerformanceMetrics, SecurityLevel};

// Re-export safe memory management types
pub use memory_safe::{SafeRingBuffer, SafeSimdCapabilities, SafeZeroCopyMemoryPool};
