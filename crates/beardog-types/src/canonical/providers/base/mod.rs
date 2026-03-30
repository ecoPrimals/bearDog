// SPDX-License-Identifier: AGPL-3.0-only

// Base Provider Traits - Foundation of Unified Provider System
//
// This module defines the core BaseProvider trait that all provider implementations
// must implement. It establishes the fundamental interface for provider lifecycle,
// health monitoring, capabilities, and ecosystem integration.

mod performance;
mod configuration;
mod schema;
mod provider_trait;
mod defaults;

pub use configuration::*;
pub use performance::*;
pub use provider_trait::BaseProvider;
pub use schema::*;
