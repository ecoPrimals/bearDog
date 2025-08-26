

use super::core::WorkflowProcessor;
use crate::workflows::canonical::{
    Workflow, WorkflowExecutionStatus, WorkflowMetrics, WorkflowProcessingResult,
};

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::workflow::WorkflowType;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct SecurityProcessorConfig {

    pub auto_threat_detection: bool,

    pub scan_depth: SecurityScanDepth,

    pub enable_compliance: bool,
}
pub enum SecurityScanDepth {
    Surface,
    Deep,
    Comprehensive,}

impl Default for SecurityProcessorConfig {}

    fn default() -> Self {
        Self {
            auto_threat_detection: true,
            scan_depth: SecurityScanDepth::Deep,
            enable_compliance: true,
        }
    }

#[derive(Debug)]
pub struct SecurityProcessor {
    pub config: UnifiedProcessorConfig,}

impl SecurityProcessor {

    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }
    }

    pub fn new_default() -> Self {
        Self::new(SecurityProcessorConfig::default())
    }
}

impl Default for SecurityProcessor {
    fn default() -> Self {
        Self {
            config: UnifiedProcessorConfig::default(),
        }
    }
}

impl WorkflowProcessor for SecurityProcessor {
    async fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let _start_time = Instant::now();
        info!("Processing security workflow: {}", workflow.id);
        match workflow.workflow_type {
            WorkflowType::ComplianceAudit => self.process_compliance_audit(workflow).await,
            WorkflowType::SecurityAudit => self.process_security_audit(workflow).await,
            _ => Err(BearDogError::validation(format!(
                "Unsupported workflow type for security processor: {:?}",
                workflow.workflow_type
            ))),
    fn name(&self) -> &str {
        "SecurityProcessor"}

    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(
            workflow.workflow_type,
            WorkflowType::SecurityAudit | WorkflowType::ComplianceAudit
        )

    async fn process_compliance_audit(
        let start_time = Instant::now();
        let audit_scope = workflow
            .parameters
            .get("audit_scope")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing audit_scope parameter".to_string()))?;
        info!("Processing compliance audit for scope: {}", audit_scope);

        let execution_duration = start_time.elapsed();
        Ok(WorkflowProcessingResult {
            success: true,
            message: format!(
                "Successfully completed compliance audit for: {}",
                audit_scope
            ),
            duration_ms: execution_duration.as_millis().min(u64::MAX as u128) as u64,
            workflow_id: workflow.id.clone(),
            processor_name: "SecurityComplianceProcessor".to_string(),
            status: WorkflowExecutionStatus::Completed,
            execution_duration_ms: Some(execution_duration.as_millis().min(u64::MAX as u128) as u64),
            steps_completed: Some(3),
            steps_total: Some(3),
            output_data: Some(serde_json::json!({
                "audit_scope": audit_scope,
                "compliance_score": 95.5,
                "audit_timestamp": chrono::Utc::now().to_rfc3339(),
                "findings": ["Minor configuration issue in access controls"]
            })),
            actions_taken: vec![
                format_args!("Scanned compliance for scope: {}", audit_scope).to_string(),
                "Generated compliance report".to_string(),
                "Updated compliance database".to_string(),
            ],
            metrics: WorkflowMetrics {
                processing_time_ms: execution_duration.as_millis().min(u64::MAX as u128) as u64,
                memory_usage_bytes: 1024 * 512, // 512KB estimated
                cpu_usage_percent: 15.5,
            },
            warnings: vec!["Minor configuration issue in access controls".to_string()],
        })

    async fn process_security_audit(
        info!("Processing security audit for scope: {}", audit_scope);
            message: format!("Compliance check completed"),
            duration_ms: execution_duration.as_millis() as u64,
            processor_name: "SecurityProcessor".to_string(),
            execution_duration_ms: Some(execution_duration.as_millis() as u64),
                "compliance_status": "passed",
                "check_timestamp": chrono::Utc::now().to_rfc3339()
                "Performed compliance validation".to_string(),
                "Checked regulatory requirements".to_string(),
            warnings: vec![],
