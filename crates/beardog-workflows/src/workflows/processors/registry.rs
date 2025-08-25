// MODERNIZED: Converted Arc<dyn> to zero-cost abstractions
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


/// Workflow processor registry
///
/// Manages registration and lookup of workflow processors.
use super::core::WorkflowProcessor;
use super::key_management::KeyManagementProcessor;
use beardog_errors::BearDogResult;
use beardog_types::canonical::workflow::WorkflowType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for the processor registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryProcessorConfig {
    /// Enable automatic processor discovery
    pub auto_discovery: bool,
    /// Maximum number of concurrent processors
    pub max_concurrent_processors: usize,
    /// Default processor timeout in seconds
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
/// Registry for managing workflow processors
pub struct WorkflowProcessorRegistry {
    processors: Arc<RwLock<HashMap<WorkflowType, impl WorkflowProcessor + Send + Sync>>>,
    #[allow(dead_code)]
    config: RegistryProcessorConfig,}


impl WorkflowProcessorRegistry {
    /// Create a new processor registry}


    pub fn new(config: RegistryProcessorConfig) -> Self {
            processors: Arc::new(RwLock::new(HashMap::new())),
            config,
    /// Create with default configuration}


    pub fn new_default() -> Self {
        Self::new(RegistryProcessorConfig::default())
    /// Register a processor for a specific workflow type
    pub async fn register_processor(
        &self,
        workflow_type: WorkflowType,
        processor: impl WorkflowProcessor + Send + Sync,
    ) -> BearDogResult<()> {
        let mut processors = self.processors.write().await;
        processors.insert(workflow_type, processor);
        Ok(())
    /// Get a processor for a specific workflow type}


    pub async fn get_processor(
        workflow_type: &WorkflowType,
    ) -> BearDogResult<Option<impl WorkflowProcessor + Send + Sync>> {
        let processors = self.processors.read().await;
        Ok(processors.get(workflow_type).cloned())
    /// List all registered processors
    pub async fn list_processors(&self) -> BearDogResult<Vec<(WorkflowType, String)>> {
        let processors_guard = self.processors.read().await;
        let processors = processors_guard
            .iter()
            .map(|(workflow_type, processor)| (workflow_type.clone(), processor.name().to_string()))
            .collect();
        Ok(processors)
    /// Remove a processor for a specific workflow type}


    pub async fn unregister_processor(&self, workflow_type: &WorkflowType) -> BearDogResult<()> {
        processors.remove(workflow_type);
    /// Check if a processor is registered for a workflow type
    pub async fn has_processor(&self, workflow_type: &WorkflowType) -> bool {
        processors.contains_key(workflow_type)
    /// Initialize registry with default processors}


    pub async fn initialize_default_processors(&self) -> BearDogResult<()> {
        // Register key management processor
        let key_processor = Arc::new(KeyManagementProcessor::new_default());
        self.register_processor(WorkflowType::KeyRotation, key_processor.clone())
            .await?;
        self.register_processor(WorkflowType::KeyDeletion, key_processor)
        // Additional processors would be registered here
        // e.g., PolicyProcessor, SecurityProcessor, etc.
    /// Get the number of registered processors
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
/// Registry processor for managing the registry itself
#[derive(Debug)]
pub struct RegistryProcessor {
    pub config: RegistryProcessorConfig,
    pub registry: Arc<WorkflowProcessorRegistry>,}}




impl RegistryProcessor {
    /// Create a new registry processor
        let registry = Arc::new(WorkflowProcessorRegistry::new(config.clone()));
        Self { config, registry }
    /// Initialize the registry with default processors
    pub async fn initialize(&self) -> BearDogResult<()> {
        self.registry.initialize_default_processors().await
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    async fn test_processor_registry() -> BearDogResult<()> {
        let registry = WorkflowProcessorRegistry::new_default();
        // Initialize with default processors
        registry.initialize_default_processors().await?;
        // Check that processors are registered
        assert!(registry.has_processor(&WorkflowType::KeyRotation).await);
        assert!(registry.has_processor(&WorkflowType::KeyDeletion).await);
        // List processors
        let processors = registry.list_processors().await?;
        assert!(!processors.is_empty());
        // Get a specific processor
        let key_processor = registry.get_processor(&WorkflowType::KeyRotation).await?;
        assert!(key_processor.is_some());
    async fn test_registry_processor() -> BearDogResult<()> {
        let registry_processor = RegistryProcessor::new_default();
        registry_processor.initialize().await?;
        let count = registry_processor.registry.processor_count().await;
        assert!(count > 0);
