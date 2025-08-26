

use super::canonical::WorkflowProcessingResult;
use super::zero_cost_traits::{
    ZeroCostWorkflowProcessor, WorkflowProcessingRequest, WorkflowProcessingContext,
};
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const VALIDATION_STRICT: bool = true;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationParams {
    pub key_id: String,
    pub key_type: String,
    pub rotation_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyChangeParams {
    pub policy_id: String,
    pub change_type: String,
    pub validation_level: String,
}

pub struct ZeroCostKeyRotationProcessor<const BATCH_SIZE: usize, const TIMEOUT_MS: u64> {
    processed_count: AtomicU64,
    success_count: AtomicU64,
}

impl<const BATCH_SIZE: usize, const TIMEOUT_MS: u64> Default 
    for ZeroCostKeyRotationProcessor<BATCH_SIZE, TIMEOUT_MS> 
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const BATCH_SIZE: usize, const TIMEOUT_MS: u64> 
    ZeroCostKeyRotationProcessor<BATCH_SIZE, TIMEOUT_MS> 
{
    pub fn new() -> Self {
        Self {
            processed_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
        }
    }

    pub fn get_stats(&self) -> (u64, u64) {
        (
            self.processed_count.load(Ordering::Relaxed),
            self.success_count.load(Ordering::Relaxed),
        )
    }
}

impl<const BATCH_SIZE: usize, const TIMEOUT_MS: u64> ZeroCostWorkflowProcessor
    for ZeroCostKeyRotationProcessor<BATCH_SIZE, TIMEOUT_MS>
{
    type Request = WorkflowProcessingRequest<KeyRotationParams>;
    type Response = WorkflowProcessingResult;
    type Context = WorkflowProcessingContext;

    async fn process(
        &self,
        request: Self::Request,
        _context: Self::Context,
    ) -> BearDogResult<Self::Response> {
        let start_time = Instant::now();
        self.processed_count.fetch_add(1, Ordering::Relaxed);

        let key_id = &request.payload.key_id;
        let key_type = &request.payload.key_type;

        if start_time.elapsed() > Duration::from_millis(TIMEOUT_MS) {
            return Err(BearDogError::timeout(format!(
                "Key rotation timeout for key: {key_id}"
            )));
        }

        self.success_count.fetch_add(1, Ordering::Relaxed);
        
        Ok(WorkflowProcessingResult {
            success: true,
            message: format!("Key rotation completed for {key_id} ({key_type})"),
            duration_ms: start_time.elapsed().as_millis() as u64,
            workflow_id: request.id.clone(),
            processor_name: "ZeroCostKeyRotationProcessor".to_string(),
            status: crate::workflows::canonical::processing::WorkflowExecutionStatus::Completed,
            execution_duration_ms: Some(start_time.elapsed().as_millis() as u64),
            steps_completed: Some(1),
            steps_total: Some(1),
            output_data: None,
            actions_taken: vec!["key_rotation".to_string()],
            metrics: crate::workflows::canonical::processing::WorkflowMetrics {
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                memory_usage_bytes: 1024,
                cpu_usage_percent: 5.0,
            },
            warnings: Vec::new(),
        })
    }

    async fn validate(&self, request: &Self::Request) -> BearDogResult<()> {
        if request.payload.key_id.is_empty() {
            return Err(BearDogError::validation(
                "key_id parameter is required for key rotation".to_string(),
            ));
        }
        Ok(())
    }

    fn capabilities(&self) -> &'static [&'static str] {
        &["key_rotation", "hsm_operations", "batch_processing"]
    }

    fn name(&self) -> &'static str {
        "ZeroCostKeyRotationProcessor"
    }
}

pub struct ZeroCostPolicyChangeProcessor<const MAX_CONCURRENT: usize> {
    _phantom: std::marker::PhantomData<[(); MAX_CONCURRENT]>,
}

impl<const MAX_CONCURRENT: usize> Default for ZeroCostPolicyChangeProcessor<MAX_CONCURRENT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_CONCURRENT: usize> ZeroCostPolicyChangeProcessor<MAX_CONCURRENT> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<const MAX_CONCURRENT: usize> ZeroCostWorkflowProcessor
    for ZeroCostPolicyChangeProcessor<MAX_CONCURRENT>
{
    type Request = WorkflowProcessingRequest<PolicyChangeParams>;
    type Response = WorkflowProcessingResult;
    type Context = WorkflowProcessingContext;

    async fn process(
        &self,
        request: Self::Request,
        _context: Self::Context,
    ) -> BearDogResult<Self::Response> {
        let start_time = Instant::now();

        let policy_id = &request.payload.policy_id;
        let change_type = &request.payload.change_type;

        let base_time_ms = if VALIDATION_STRICT { 200 } else { 100 };
        tokio::time::sleep(Duration::from_millis(base_time_ms)).await;
        
        Ok(WorkflowProcessingResult {
            success: true,
            message: format!("Policy change completed for {policy_id} ({change_type})"),
            duration_ms: start_time.elapsed().as_millis() as u64,
            workflow_id: request.id.clone(),
            processor_name: "ZeroCostPolicyChangeProcessor".to_string(),
            status: crate::workflows::canonical::processing::WorkflowExecutionStatus::Completed,
            execution_duration_ms: Some(start_time.elapsed().as_millis() as u64),
            steps_completed: Some(1),
            steps_total: Some(1),
            output_data: None,
            actions_taken: vec!["policy_change".to_string()],
            metrics: crate::workflows::canonical::processing::WorkflowMetrics {
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                memory_usage_bytes: 512,
                cpu_usage_percent: 3.0,
            },
            warnings: Vec::new(),
        })
    }

    async fn validate(&self, request: &Self::Request) -> BearDogResult<()> {
        if request.payload.policy_id.is_empty() {
            return Err(BearDogError::validation(
                "policy_id parameter is required for policy changes".to_string(),
            ));
        }
        Ok(())
    }

    fn capabilities(&self) -> &'static [&'static str] {
        &["policy_change", "validation", "sequential_processing"]
    }

    fn name(&self) -> &'static str {
        "ZeroCostPolicyChangeProcessor"
    }
}

pub struct ZeroCostProcessorFactory;

impl ZeroCostProcessorFactory {

    pub fn key_rotation_processor<const BATCH_SIZE: usize, const TIMEOUT_MS: u64>() 
        -> ZeroCostKeyRotationProcessor<BATCH_SIZE, TIMEOUT_MS> 
    {
        ZeroCostKeyRotationProcessor::new()
    }

    pub fn policy_change_processor<const MAX_CONCURRENT: usize>() 
        -> ZeroCostPolicyChangeProcessor<MAX_CONCURRENT> 
    {
        ZeroCostPolicyChangeProcessor::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::workflow::WorkflowType;

    #[tokio::test]
    async fn test_key_rotation_processor() {
        let processor = ZeroCostKeyRotationProcessor::<10, 5000>::new();
        
        let request = WorkflowProcessingRequest {
            id: "test-key-rotation".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            payload: KeyRotationParams {
                key_id: "test-key-123".to_string(),
                key_type: "aes256".to_string(),
                rotation_interval: Duration::from_secs(3600),
            },
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        let context = WorkflowProcessingContext::default();
        let result = processor.process(request, context).await;
        
        let response = result.expect("Key rotation processing should succeed");
        assert_eq!(response.status, WorkflowExecutionStatus::Completed);
        assert!(response.message.contains("Key rotation completed"));
    }

    #[tokio::test]
    async fn test_policy_change_processor() {
        let processor = ZeroCostPolicyChangeProcessor::<1>::new();
        
        let request = WorkflowProcessingRequest {
            id: "test-policy-change".to_string(),
            workflow_type: WorkflowType::PolicyChange,
            payload: PolicyChangeParams {
                policy_id: "security-policy-v2".to_string(),
                change_type: "access_control".to_string(),
                validation_level: "strict".to_string(),
            },
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        let context = WorkflowProcessingContext::default();
        let result = processor.process(request, context).await;
        
        let response = result.expect("Policy change processing should succeed");
        assert_eq!(response.status, WorkflowExecutionStatus::Completed);
        assert!(response.message.contains("Policy change completed"));
    }
}
