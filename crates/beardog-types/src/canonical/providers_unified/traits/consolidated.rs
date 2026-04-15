// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Consolidated Provider Trait System - CANONICAL LOCATION
//!
//! This module provides the **single source of truth** for all provider traits across
//! the `BearDog` ecosystem. It consolidates and replaces all scattered provider trait
//! definitions with a unified, hierarchical trait system.
//!
//! ## 🎯 **Consolidation Strategy**
//!
//! This module unifies and replaces:
//! - `beardog-traits/src/unified/providers.rs` - Original unified provider traits
//! - `beardog-traits/src/unified/providers_consolidated.rs` - Consolidated provider traits
//! - `beardog-traits/src/canonical/*Provider` - Legacy canonical provider traits
//! - Scattered provider trait definitions across all crates
//!
//! ## 🏗️ **Architecture Principles**
//!
//! - **Single Source of Truth**: All provider traits defined in one location
//! - **Hierarchical Design**: Clear inheritance from base to specialized traits
//! - **Native Async**: All operations use native async/await (no `async_trait`)
//! - **Type Safety**: Strong typing with associated types and comprehensive error handling
//! - **Zero Fragmentation**: No duplicate trait definitions anywhere in the ecosystem
//! - **Performance Optimized**: Zero-cost abstractions with efficient implementations
//!
//! ## 📊 **Provider Hierarchy**
//!
//! ```text
//! ConsolidatedProvider (base trait)
//! ├── SecurityProvider
//! │   ├── CryptoProvider
//! │   └── HsmProvider
//! ├── MonitoringProvider
//! ├── StorageProvider
//! ├── NetworkProvider
//! ├── GeneticsProvider
//! ├── AdapterProvider
//! └── WorkflowProvider
//! ```

#[path = "consolidated_traits.rs"]
mod consolidated_traits;
#[path = "consolidated_types.rs"]
mod consolidated_types;

#[allow(clippy::wildcard_imports)]
pub use consolidated_traits::*;
#[allow(clippy::wildcard_imports)]
pub use consolidated_types::*;
