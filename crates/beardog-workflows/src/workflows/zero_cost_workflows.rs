

pub use super::zero_cost_traits::{
    ZeroCostApprovalStore,
    ZeroCostWorkflowProcessor, ZeroCostWorkflowStore,
};

pub use super::zero_cost_engine::{
    DevelopmentWorkflowEngine, ProductionWorkflowEngine, ZeroCostWorkflowEngine,};

pub use super::zero_cost_storage::{
    ZeroCostMemoryApprovalStore, ZeroCostMemoryWorkflowStore,
};

pub mod examples {
    use super::{DevelopmentWorkflowEngine, ProductionWorkflowEngine};

    pub fn create_production_workflow_engine(
    ) -> beardog_errors::BearDogResult<ProductionWorkflowEngine> {
        Ok(ProductionWorkflowEngine::new(
            crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore::<5000>::new(),
            crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore::<1000>::new(),
            crate::workflows::zero_cost_processors::ZeroCostKeyRotationProcessor::<10, 5000>::new(),
        ))
    }

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

}
