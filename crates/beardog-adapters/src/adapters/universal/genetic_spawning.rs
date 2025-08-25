// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Cross-Ecosystem Genetic Spawning
///
/// **Revolutionary genetic algorithm system for creating hybrid nodes**
/// This module provides a comprehensive system for creating hybrid nodes through
/// genetic algorithms, enabling unprecedented capabilities by combining features
/// from multiple ecosystem components.
/// ## Features
/// - Cross-ecosystem genetic recombination
/// - Hybrid capability emergence
/// - Advanced spawning operations management
/// - Comprehensive statistics and monitoring
/// - Configurable genetic algorithms
/// ## Example
/// ```rust
/// use beardog::adapters::universal::genetic_spawning::*;
/// use beardog::BearDogCore;
/// use std::sync::Arc;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let core = Arc::new(BearDogCore::new().await?);
/// let spawner = CrossEcosystemGeneticSpawner::new(core).await?;
/// let parent_nodes = vec![/* parent nodes */];
/// let target_capabilities = vec![HybridCapability::MultiNodeAuthentication];
/// let security_requirements = SecurityRequirements::default();
/// let resource_constraints = ResourceConstraints::default();
/// let hybrid_node = spawner.spawn_hybrid_node(
///     parent_nodes,
///     target_capabilities,
///     security_requirements,
///     resource_constraints,
/// ).await?;
/// # Ok(())
/// # }
/// ```
/// This module is organized into focused sub-modules:
/// - `core` - Main spawner implementation and core logic
/// - `operations` - Spawning operation management and status tracking
/// - `genetics` - Genetic algorithms and blueprint types
/// - `config` - Configuration and statistics types

pub mod config;
pub mod core;
pub mod genetics;
pub mod operations;
// Re-export all types for backward compatibility
pub use config::*;
pub use core::*;
pub use genetics::*;
pub use operations::*; 
