

use beardog_errors::BearDogResult;
use super::canonical::{CanonicalWorkflow, WorkflowId, WorkflowStatus};
use super::zero_cost_traits::{
    ZeroCostWorkflowStore, ZeroCostApprovalStore, ZeroCostWorkflowProcessor,
    ZeroCostEngineStats, WorkflowProcessingContext,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::marker::PhantomData;

pub struct ZeroCostWorkflowEngine<W, A, P, const APPROVAL_CAPACITY: usize = 1000> 
where
    W: ZeroCostWorkflowStore,
    A: ZeroCostApprovalStore<APPROVAL_CAPACITY>,
    P: ZeroCostWorkflowProcessor,
{

    workflow_store: W,

    approval_store: A,

    processor: P,

    stats: ZeroCostEngineStats,

    start_time: Instant,

    operations: AtomicU64,

    _phantom: PhantomData<(W, A, P)>,
}

impl<W, A, P, const APPROVAL_CAPACITY: usize> ZeroCostWorkflowEngine<W, A, P, APPROVAL_CAPACITY>
where
    W: ZeroCostWorkflowStore,
    A: ZeroCostApprovalStore<APPROVAL_CAPACITY>,
    P: ZeroCostWorkflowProcessor,
{

    pub fn new(workflow_store: W, approval_store: A, processor: P) -> Self {
        Self {
            workflow_store,
            approval_store,
            processor,
            stats: ZeroCostEngineStats::default(),
            start_time: Instant::now(),
            operations: AtomicU64::new(0),
            _phantom: PhantomData,
        }
    }

    pub async fn submit_workflow(&self, workflow: CanonicalWorkflow) -> BearDogResult<WorkflowId> {
        let start = Instant::now();
        self.operations.fetch_add(1, Ordering::Relaxed);

        let workflow_id = workflow.id.clone();

        let _context = WorkflowProcessingContext::default();

        let duration = start.elapsed();
        self.update_stats(duration, true);
        
        Ok(workflow_id)
    }

    pub async fn get_workflow_status(&self, _workflow_id: &WorkflowId) -> BearDogResult<WorkflowStatus> {

        Ok(WorkflowStatus::Completed)
    }

    pub fn get_stats(&self) -> ZeroCostEngineStats {
        let operations = self.operations.load(Ordering::Relaxed);
        let uptime = self.start_time.elapsed().as_secs();
        
        ZeroCostEngineStats {
            workflows_processed: operations,
            avg_processing_time_ms: 50.0, // Simplified for now
            active_workflows: 0,
            error_rate: 0.0,
            memory_usage_bytes: std::mem::size_of::<Self>(),
            uptime_seconds: uptime,
        }
    }

    fn update_stats(&self, duration: Duration, success: bool) {

        let _ = (duration, success); // Suppress unused warnings for now
    }
}

#[allow(async_fn_in_trait)]
pub trait WorkflowEngine: Send + Sync {

    fn submit_workflow(&self, _workflow: CanonicalWorkflow) -> impl std::future::Future<Output = BearDogResult<WorkflowId>> + Send;

    fn get_workflow_status(&self, _workflow_id: &WorkflowId) -> impl std::future::Future<Output = BearDogResult<WorkflowStatus>> + Send;

    fn get_engine_stats(&self) -> ZeroCostEngineStats;
}

pub trait WorkflowEngineInterface: Send + Sync {

    fn submit_workflow_boxed(&self, _workflow: CanonicalWorkflow) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<WorkflowId>> + Send + '_>>;

    fn get_workflow_status_boxed<'a>(&'a self, workflow_id: &'a WorkflowId) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<WorkflowStatus>> + Send + 'a>>;

    fn get_engine_stats(&self) -> ZeroCostEngineStats;
}

impl<T: WorkflowEngine> WorkflowEngineInterface for T {
    fn submit_workflow_boxed(&self, workflow: CanonicalWorkflow) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<WorkflowId>> + Send + '_>> {
        Box::pin(self.submit_workflow(workflow))
    }
    
    fn get_workflow_status_boxed<'a>(&'a self, workflow_id: &'a WorkflowId) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<WorkflowStatus>> + Send + 'a>> {
        Box::pin(self.get_workflow_status(workflow_id))
    }
    
    fn get_engine_stats(&self) -> ZeroCostEngineStats {
        WorkflowEngine::get_engine_stats(self)
    }
}

impl<W, A, P, const APPROVAL_CAPACITY: usize> WorkflowEngine for ZeroCostWorkflowEngine<W, A, P, APPROVAL_CAPACITY>
where
    W: ZeroCostWorkflowStore,
    A: ZeroCostApprovalStore<APPROVAL_CAPACITY>,
    P: ZeroCostWorkflowProcessor,
{
    async fn submit_workflow(&self, _workflow: CanonicalWorkflow) -> BearDogResult<WorkflowId> {

        let workflow_id = WorkflowId::new();

        Ok(workflow_id)
    }
    
    async fn get_workflow_status(&self, _workflow_id: &WorkflowId) -> BearDogResult<WorkflowStatus> {

        Ok(beardog_types::canonical::workflow::WorkflowStatus::Pending)
    }
    
    fn get_engine_stats(&self) -> ZeroCostEngineStats {
        self.get_stats()
    }
}

pub type ProductionWorkflowEngine = ZeroCostWorkflowEngine<
    crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore<5000>,
    crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore<1000>,
    crate::workflows::zero_cost_processors::ZeroCostKeyRotationProcessor<10, 5000>,
    1000,
>;

pub type DevelopmentWorkflowEngine = ZeroCostWorkflowEngine<
    crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore<500>,
    crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore<1000>,
    crate::workflows::zero_cost_processors::ZeroCostPolicyChangeProcessor<1>,
    1000,
>;

pub async fn create_production_engine() -> BearDogResult<ProductionWorkflowEngine> {
    let workflow_store = super::zero_cost_storage::ZeroCostMemoryWorkflowStore::new();
    let approval_store = super::zero_cost_storage::ZeroCostMemoryApprovalStore::new();
    let processor = super::zero_cost_processors::ZeroCostKeyRotationProcessor::new();
    
    Ok(ZeroCostWorkflowEngine::new(workflow_store, approval_store, processor))
}

pub async fn create_development_engine() -> BearDogResult<DevelopmentWorkflowEngine> {
    let workflow_store = super::zero_cost_storage::ZeroCostMemoryWorkflowStore::new();
    let approval_store = super::zero_cost_storage::ZeroCostMemoryApprovalStore::new();
    let processor = super::zero_cost_processors::ZeroCostPolicyChangeProcessor::new();
    
    Ok(ZeroCostWorkflowEngine::new(workflow_store, approval_store, processor))
}

#[derive(Debug, Clone)]
pub struct EngineConfig {

    pub max_concurrent_workflows: usize,

    pub approval_enabled: bool,

    pub processing_timeout_seconds: u64,

    pub max_retry_attempts: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_workflows: 1000,
            approval_enabled: true,
            processing_timeout_seconds: 300,
            max_retry_attempts: 3,
        }
    }
}

pub struct ZeroCostEngineBuilder<W, A, P, const APPROVAL_CAPACITY: usize = 1000> {
    workflow_store: Option<W>,
    approval_store: Option<A>,
    processor: Option<P>,
    config: EngineConfig,
}

impl<W, A, P, const APPROVAL_CAPACITY: usize> ZeroCostEngineBuilder<W, A, P, APPROVAL_CAPACITY>
where
    W: ZeroCostWorkflowStore,
    A: ZeroCostApprovalStore<APPROVAL_CAPACITY>,
    P: ZeroCostWorkflowProcessor,
{

    pub fn new() -> Self {
        Self {
            workflow_store: None,
            approval_store: None,
            processor: None,
            config: EngineConfig::default(),
        }
    }

    pub fn with_workflow_store(mut self, store: W) -> Self {
        self.workflow_store = Some(store);
        self
    }

    pub fn with_approval_store(mut self, store: A) -> Self {
        self.approval_store = Some(store);
        self
    }

    pub fn with_processor(mut self, processor: P) -> Self {
        self.processor = Some(processor);
        self
    }

    pub fn with_config(mut self, config: EngineConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> BearDogResult<ZeroCostWorkflowEngine<W, A, P, APPROVAL_CAPACITY>> {
        let workflow_store = self.workflow_store
            .ok_or_else(|| beardog_errors::BearDogError::validation("Workflow store required"))?;
        let approval_store = self.approval_store
            .ok_or_else(|| beardog_errors::BearDogError::validation("Approval store required"))?;
        let processor = self.processor
            .ok_or_else(|| beardog_errors::BearDogError::validation("Processor required"))?;
            
        Ok(ZeroCostWorkflowEngine::<W, A, P, APPROVAL_CAPACITY>::new(workflow_store, approval_store, processor))
    }
}

impl<W, A, P> Default for ZeroCostEngineBuilder<W, A, P>
where
    W: ZeroCostWorkflowStore,
    A: ZeroCostApprovalStore<10000>,
    P: ZeroCostWorkflowProcessor,
{
    fn default() -> Self {
        Self {
            workflow_store: None,
            approval_store: None,
            processor: None,
            config: EngineConfig::default(),
        }
    }
}
