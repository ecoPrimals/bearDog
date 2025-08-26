

use beardog_errors::BearDogResult;

use super::super::core::types::{CanonicalWorkflow, WorkflowAuditEntry};

pub trait WorkflowStore: Send + Sync {

    fn store_workflow(&self, workflow: CanonicalWorkflow) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    fn get_workflow(&self, workflow_id: &str) -> impl std::future::Future<Output = BearDogResult<Option<CanonicalWorkflow>>> + Send;

    fn update_workflow(&self, workflow: CanonicalWorkflow) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    fn delete_workflow(&self, workflow_id: &str) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    fn list_workflows(&self) -> impl std::future::Future<Output = BearDogResult<Vec<CanonicalWorkflow>>> + Send;

    fn store_audit_entry(&self, entry: WorkflowAuditEntry) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
}

#[allow(async_fn_in_trait)]
pub trait WorkflowNotificationEngine: Send + Sync {

    async fn notify_workflow_created(&self, workflow: &CanonicalWorkflow) -> BearDogResult<()>;

    async fn notify_workflow_completed(&self, workflow: &CanonicalWorkflow) -> BearDogResult<()>;

    async fn notify_workflow_failed(&self, workflow: &CanonicalWorkflow, error: &str) -> BearDogResult<()>;

    async fn notify_approval_required(&self, workflow: &CanonicalWorkflow) -> BearDogResult<()>;

    async fn notify_approval_granted(&self, workflow: &CanonicalWorkflow, approver: &str) -> BearDogResult<()>;
} 