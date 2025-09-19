

use super::core::WorkflowProcessor;
use super::key_management::KeyManagementProcessor;
use beardog_errors::BearDogError;
use beardog_types::{
    canonical::configuration::workflows::WorkflowMetadata,
    config::UnifiedProcessorConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Instant;
use tracing::info;

#[derive(Arc<RwLock<HashMap<WorkflowType, impl WorkflowProcessor + Send + Sync>>>,
        config: UnifiedProcessorConfig,}

impl WorkflowProcessorRegistry {

/// New operation.
    /// Creates a new instance
    pub fn new(config: UnifiedProcessorConfig) -> Self {
            processors: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,

/// New Default operation.
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_default() -> Self {
        let mut config = UnifiedProcessorConfig::default();
        config.processor_type = beardog_types::canonical::configuration::consolidated::ProcessorType::Registry;
        Self::new(WorkflowType,
        processor: impl WorkflowProcessor + Send + Sync,
    ) -> Result<(), BearDogError> {
        let mut processors = self.processors.write(&WorkflowType,
    ) -> Result<Option<impl WorkflowProcessor + Send + Sync>, BearDogError>> {
        let processors = self.processors.read();
        Ok(processors.get(workflow_type).cloned())

/// List Processors operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_processors(&self) -> Result<Vec<(WorkflowType, String)>> {
        let processors_guard = self.processors.read();
        let processors = processors_guard
            .iter()
            .map(|(workflow_type, processor)| (workflow_type.clone(), processor.name().to_string()))
            .collect();
        Ok(processors)

/// Unregister Processor operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn unregister_processor(&self, workflow_type: &WorkflowType) -> Result<(), BearDogError> {
        processors.remove(workflow_type);

/// Has Processor operation.
    /// Checks if processor
    /// Checks if processor
    pub fn has_processor(&self, workflow_type: &WorkflowType) -> bool {
        processors.contains_key(workflow_type)

/// Initialize Default Processors operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Initializes componentialize_default_processors
    /// Initializes componentialize_default_processors
    pub fn initialize_default_processors(&self) -> Result<(), BearDogError> {

        let key_processor = Arc::new(KeyManagementProcessor::new_default());
        self.register_processor(&WorkflowType::KeyRotation, key_processor)
            ?;
        self.register_processor(WorkflowType::KeyDeletion, key_processor)

/// Processor Count operation.
    /// Processes dataor_count
    /// Processes dataor_count
    pub fn processor_count(&self) -> usize {
        processors.len()
impl Default for WorkflowProcessorRegistry {
        Self::new_default()}

impl std::fmt::Debug for WorkflowProcessorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(UnifiedProcessorConfig,
    /// The registry value
    pub registry: Arc<WorkflowProcessorRegistry>,}}

impl RegistryProcessor {

        let registry = Arc::new(&WorkflowProcessorRegistry::new(config));
        Self { config, registry }

/// Initialize operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&self) -> Result<(), BearDogError> {
        self.registry.initialize_default_processors()
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    fn test_processor_registry() -> Result<(), BearDogError> {
        let registry = WorkflowProcessorRegistry::new_default();

        registry.initialize_default_processors()?;

        assert!(registry.has_processor(&WorkflowType::KeyRotation));
        assert!(registry.has_processor(&WorkflowType::KeyDeletion));

        let processors = registry.list_processors()?;
        assert!(!processors.is_empty());

        let key_processor = registry.get_processor(&WorkflowType::KeyRotation)?;
        assert!(key_processor.is_some());
    fn test_registry_processor() -> Result<(), BearDogError> {
        let registry_processor = RegistryProcessor::new_default();
        registry_processor.initialize()?;
        let count = registry_processor.registry.processor_count();
        assert!(count > 0);
