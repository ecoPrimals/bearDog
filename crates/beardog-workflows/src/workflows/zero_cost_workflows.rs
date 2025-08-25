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


/// # Zero-Cost Workflow Architecture
///
/// This module provides zero-cost abstractions for the `BearDog` workflow engine,
/// eliminating runtime overhead from async_trait and enum dispatch while maintaining
/// full flexibility and type safety.
/// ## Key Benefits:
/// - **No async_trait boxing** - Native async methods with zero overhead
/// - **No runtime dispatch** - All workflow processing monomorphized at compile time  
/// - **No enum matching** - Direct processor selection via generics
/// - **Perfect type safety** - Compile-time workflow validation
// Re-export main types and traits for convenience
pub use super::zero_cost_traits::{
    ZeroCostApprovalStore,
    ZeroCostWorkflowProcessor, ZeroCostWorkflowStore,
};

pub use super::zero_cost_engine::{
    DevelopmentWorkflowEngine, ProductionWorkflowEngine, ZeroCostWorkflowEngine,};


pub use super::zero_cost_storage::{
    ZeroCostMemoryApprovalStore, ZeroCostMemoryWorkflowStore,
};
/// Example usage and factory functions
pub mod examples {
    use super::{DevelopmentWorkflowEngine, ProductionWorkflowEngine};
    /// Create a production zero-cost workflow engine
    pub fn create_production_workflow_engine(
    ) -> beardog_errors::BearDogResult<ProductionWorkflowEngine> {
        Ok(ProductionWorkflowEngine::new(
            crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore::<5000>::new(),
            crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore::<1000>::new(),
            crate::workflows::zero_cost_processors::ZeroCostKeyRotationProcessor::<10, 5000>::new(),
        ))
    }
    /// Create a development zero-cost workflow engine
    pub fn create_development_workflow_engine(
    ) -> beardog_errors::BearDogResult<DevelopmentWorkflowEngine> {
        Ok(DevelopmentWorkflowEngine::new(
            crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore::<500>::new(),
            crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore::<1000>::new(),
            crate::workflows::zero_cost_processors::ZeroCostPolicyChangeProcessor::<1>::new(),
        ))
    }
}
#[cfg(test)]
mod tests {
    // Tests moved to individual modules
}
