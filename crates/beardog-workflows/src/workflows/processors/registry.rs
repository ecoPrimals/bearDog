

use super::core::WorkflowProcessor;
use super::key_management::KeyManagementProcessor;
use beardog_errors::BearDogResult;
use beardog_types::canonical::workflow::WorkflowType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct RegistryProcessorConfig {

    pub auto_discovery: bool,

    pub max_concurrent_processors: usize,

    pub default_timeout_seconds: u64,
}
impl Default for RegistryProcessorConfig {}

    fn default() -> Self {
        Self {
            auto_discovery: true,
            max_concurrent_processors: 10,
            default_timeout_seconds: 300, // 5 minutes
        }
    }

pub struct WorkflowProcessorRegistry {
    processors: Arc<RwLock<HashMap<WorkflowType, impl WorkflowProcessor + Send + Sync>>>,
        config: UnifiedProcessorConfig,}

impl WorkflowProcessorRegistry {

    pub fn new(config: UnifiedProcessorConfig) -> Self {
            processors: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,

    pub fn new_default() -> Self {
        Self::new(RegistryProcessorConfig::default())

    pub async fn register_processor(
        &self,
        workflow_type: WorkflowType,
        processor: impl WorkflowProcessor + Send + Sync,
    ) -> BearDogResult<()> {
        let mut processors = self.processors.write().await;
        processors.insert(workflow_type, processor);
        Ok(())

    pub async fn get_processor(
        workflow_type: &WorkflowType,
    ) -> BearDogResult<Option<impl WorkflowProcessor + Send + Sync>> {
        let processors = self.processors.read().await;
        Ok(processors.get(workflow_type).cloned())

    pub async fn list_processors(&self) -> BearDogResult<Vec<(WorkflowType, String)>> {
        let processors_guard = self.processors.read().await;
        let processors = processors_guard
            .iter()
            .map(|(workflow_type, processor)| (workflow_type.clone(), processor.name().to_string()))
            .collect();
        Ok(processors)

    pub async fn unregister_processor(&self, workflow_type: &WorkflowType) -> BearDogResult<()> {
        processors.remove(workflow_type);

    pub async fn has_processor(&self, workflow_type: &WorkflowType) -> bool {
        processors.contains_key(workflow_type)

    pub async fn initialize_default_processors(&self) -> BearDogResult<()> {

        let key_processor = Arc::new(KeyManagementProcessor::new_default());
        self.register_processor(WorkflowType::KeyRotation, key_processor.clone())
            .await?;
        self.register_processor(WorkflowType::KeyDeletion, key_processor)

    pub async fn processor_count(&self) -> usize {
        processors.len()
impl Default for WorkflowProcessorRegistry {
        Self::new_default()}

impl std::fmt::Debug for WorkflowProcessorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkflowProcessorRegistry")
            .field(
                "processors",
                &"Arc<RwLock<HashMap<WorkflowType, Arc<dyn WorkflowProvider>>>>",
            )
            .finish()

#[derive(Debug)]
pub struct RegistryProcessor {
    pub config: UnifiedProcessorConfig,
    pub registry: Arc<WorkflowProcessorRegistry>,}}

impl RegistryProcessor {

        let registry = Arc::new(WorkflowProcessorRegistry::new(config.clone()));
        Self { config, registry }

    pub async fn initialize(&self) -> BearDogResult<()> {
        self.registry.initialize_default_processors().await
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    async fn test_processor_registry() -> BearDogResult<()> {
        let registry = WorkflowProcessorRegistry::new_default();

        registry.initialize_default_processors().await?;

        assert!(registry.has_processor(&WorkflowType::KeyRotation).await);
        assert!(registry.has_processor(&WorkflowType::KeyDeletion).await);

        let processors = registry.list_processors().await?;
        assert!(!processors.is_empty());

        let key_processor = registry.get_processor(&WorkflowType::KeyRotation).await?;
        assert!(key_processor.is_some());
    async fn test_registry_processor() -> BearDogResult<()> {
        let registry_processor = RegistryProcessor::new_default();
        registry_processor.initialize().await?;
        let count = registry_processor.registry.processor_count().await;
        assert!(count > 0);
