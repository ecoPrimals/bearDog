//! Dependency resolution system for capability management
//!
//! This module provides comprehensive dependency resolution capabilities,
//! including circular dependency detection, resolution caching, and
//! complex dependency chain management.

use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::BearDogResult;

/// Dependency resolver for capability management
pub struct DependencyResolver {
    /// Dependency graph mapping capability IDs to their dependencies
    pub dependency_graph: Arc<RwLock<HashMap<String, Vec<String>>>>,

    /// Circular dependency detection cache
    pub circular_dependencies: Arc<RwLock<HashSet<String>>>,

    /// Resolution cache for performance optimization
    pub resolution_cache: Arc<RwLock<HashMap<String, ResolutionResult>>>,
}

/// Result of dependency resolution
#[derive(Debug, Clone)]
pub struct ResolutionResult {
    /// Full dependency chain discovered
    pub dependency_chain: Vec<String>,
    /// Optimal resolution order
    pub resolution_order: Vec<String>,
    /// Whether circular dependencies were detected
    pub circular_detected: bool,
    /// Dependencies that could not be resolved
    pub unresolved_dependencies: Vec<String>,
    /// Estimated complexity of resolution
    pub estimated_complexity: u32,
    /// When this resolution was performed
    pub resolution_timestamp: DateTime<Utc>,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            dependency_graph: Arc::new(RwLock::new(HashMap::new())),
            circular_dependencies: Arc::new(RwLock::new(HashSet::new())),
            resolution_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Resolve dependencies for a set of capabilities
    pub async fn resolve_dependencies(
        &self,
        required_capabilities: &[String],
    ) -> BearDogResult<ResolutionResult> {
        info!(
            "🔗 Resolving dependencies for {} capabilities",
            required_capabilities.len()
        );

        let mut dependency_chain = Vec::new();
        let mut resolution_order = Vec::new();
        let mut unresolved_dependencies = Vec::new();
        let mut visited = HashSet::new();
        let mut in_progress = HashSet::new();

        // Check cache first
        let cache_key = required_capabilities.join("+");
        if let Some(cached_result) = self.resolution_cache.read().await.get(&cache_key) {
            debug!("📋 Using cached resolution result");
            return Ok(cached_result.clone());
        }

        // Resolve each required capability
        for capability_id in required_capabilities {
            if !visited.contains(capability_id) {
                self.resolve_single_dependency(
                    capability_id,
                    &mut dependency_chain,
                    &mut resolution_order,
                    &mut unresolved_dependencies,
                    &mut visited,
                    &mut in_progress,
                )
                .await?;
            }
        }

        // Check for circular dependencies
        let circular_detected = self.detect_circular_dependencies(&dependency_chain).await?;

        // Calculate complexity estimate
        let estimated_complexity = self.calculate_complexity(&dependency_chain).await;

        let result = ResolutionResult {
            dependency_chain,
            resolution_order,
            circular_detected,
            unresolved_dependencies,
            estimated_complexity,
            resolution_timestamp: Utc::now(),
        };

        // Cache the result
        self.resolution_cache
            .write()
            .await
            .insert(cache_key, result.clone());

        info!(
            "✅ Dependency resolution complete - {} dependencies resolved",
            result.dependency_chain.len()
        );
        Ok(result)
    }

    /// Resolve a single capability's dependencies recursively
    fn resolve_single_dependency<'a>(
        &'a self,
        capability_id: &'a str,
        dependency_chain: &'a mut Vec<String>,
        resolution_order: &'a mut Vec<String>,
        unresolved_dependencies: &'a mut Vec<String>,
        visited: &'a mut HashSet<String>,
        in_progress: &'a mut HashSet<String>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + 'a>> {
        Box::pin(async move {
            // Check for circular dependency
            if in_progress.contains(capability_id) {
                warn!(
                    "🔄 Circular dependency detected for capability: {}",
                    capability_id
                );
                self.circular_dependencies
                    .write()
                    .await
                    .insert(capability_id.to_string());
                return Ok(());
            }

            // Mark as in progress
            in_progress.insert(capability_id.to_string());

            // Get dependencies for this capability
            let dependencies = self.get_capability_dependencies(capability_id).await?;

            // Resolve each dependency recursively
            for dep in &dependencies {
                if !visited.contains(dep) {
                    self.resolve_single_dependency(
                        dep,
                        dependency_chain,
                        resolution_order,
                        unresolved_dependencies,
                        visited,
                        in_progress,
                    )
                    .await?;
                }
            }

            // Remove from in progress and add to visited
            in_progress.remove(capability_id);
            visited.insert(capability_id.to_string());

            // Add to dependency chain and resolution order
            dependency_chain.push(capability_id.to_string());
            resolution_order.push(capability_id.to_string());

            Ok(())
        })
    }

    /// Get dependencies for a specific capability
    async fn get_capability_dependencies(&self, capability_id: &str) -> BearDogResult<Vec<String>> {
        let dependency_graph = self.dependency_graph.read().await;
        Ok(dependency_graph
            .get(capability_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Detect circular dependencies in the dependency chain
    async fn detect_circular_dependencies(
        &self,
        dependency_chain: &[String],
    ) -> BearDogResult<bool> {
        let circular_deps = self.circular_dependencies.read().await;

        // Check if any capability in the chain is in the circular dependencies set
        let has_circular = dependency_chain
            .iter()
            .any(|cap| circular_deps.contains(cap));

        if has_circular {
            warn!("🔄 Circular dependencies detected in resolution chain");
        }

        Ok(has_circular)
    }

    /// Calculate complexity estimate for dependency resolution
    async fn calculate_complexity(&self, dependency_chain: &[String]) -> u32 {
        let mut complexity = dependency_chain.len() as u32;

        // Add complexity for each dependency relationship
        let dependency_graph = self.dependency_graph.read().await;
        for capability_id in dependency_chain {
            if let Some(deps) = dependency_graph.get(capability_id) {
                complexity += deps.len() as u32;
            }
        }

        complexity
    }

    /// Add a dependency relationship
    pub async fn add_dependency(
        &self,
        capability_id: &str,
        dependency_id: &str,
    ) -> BearDogResult<()> {
        let mut dependency_graph = self.dependency_graph.write().await;

        let dependencies = dependency_graph
            .entry(capability_id.to_string())
            .or_insert_with(Vec::new);
        if !dependencies.contains(&dependency_id.to_string()) {
            dependencies.push(dependency_id.to_string());
            debug!(
                "➕ Added dependency: {} -> {}",
                capability_id, dependency_id
            );
        }

        // Clear cache since dependency graph changed
        self.resolution_cache.write().await.clear();

        Ok(())
    }

    /// Remove a dependency relationship
    pub async fn remove_dependency(
        &self,
        capability_id: &str,
        dependency_id: &str,
    ) -> BearDogResult<()> {
        let mut dependency_graph = self.dependency_graph.write().await;

        if let Some(dependencies) = dependency_graph.get_mut(capability_id) {
            dependencies.retain(|dep| dep != dependency_id);
            debug!(
                "➖ Removed dependency: {} -> {}",
                capability_id, dependency_id
            );
        }

        // Clear cache since dependency graph changed
        self.resolution_cache.write().await.clear();

        Ok(())
    }

    /// Get all dependencies for a capability
    pub async fn get_all_dependencies(&self, capability_id: &str) -> BearDogResult<Vec<String>> {
        let dependency_graph = self.dependency_graph.read().await;
        Ok(dependency_graph
            .get(capability_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Get all capabilities that depend on a given capability
    pub async fn get_dependents(&self, capability_id: &str) -> BearDogResult<Vec<String>> {
        let dependency_graph = self.dependency_graph.read().await;
        let mut dependents = Vec::new();

        for (cap_id, deps) in dependency_graph.iter() {
            if deps.contains(&capability_id.to_string()) {
                dependents.push(cap_id.clone());
            }
        }

        Ok(dependents)
    }

    /// Clear all cached resolution results
    pub async fn clear_cache(&self) -> BearDogResult<()> {
        self.resolution_cache.write().await.clear();
        debug!("🧹 Cleared dependency resolution cache");
        Ok(())
    }

    /// Get statistics about the dependency graph
    pub async fn get_statistics(&self) -> BearDogResult<DependencyStatistics> {
        let dependency_graph = self.dependency_graph.read().await;
        let circular_deps = self.circular_dependencies.read().await;
        let cache = self.resolution_cache.read().await;

        let total_capabilities = dependency_graph.len();
        let total_dependencies = dependency_graph.values().map(|deps| deps.len()).sum();
        let circular_count = circular_deps.len();
        let cache_size = cache.len();

        let average_dependencies = if total_capabilities > 0 {
            total_dependencies as f64 / total_capabilities as f64
        } else {
            0.0
        };

        Ok(DependencyStatistics {
            total_capabilities,
            total_dependencies,
            circular_dependencies: circular_count,
            average_dependencies,
            cache_size,
        })
    }
}

/// Statistics about the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyStatistics {
    /// Total number of capabilities in the graph
    pub total_capabilities: usize,
    /// Total number of dependency relationships
    pub total_dependencies: usize,
    /// Number of capabilities with circular dependencies
    pub circular_dependencies: usize,
    /// Average number of dependencies per capability
    pub average_dependencies: f64,
    /// Size of the resolution cache
    pub cache_size: usize,
}

impl ResolutionResult {
    /// Check if the resolution was successful
    pub fn is_successful(&self) -> bool {
        self.unresolved_dependencies.is_empty() && !self.circular_detected
    }

    /// Get the number of resolved dependencies
    pub fn resolved_count(&self) -> usize {
        self.dependency_chain.len()
    }

    /// Get the number of unresolved dependencies
    pub fn unresolved_count(&self) -> usize {
        self.unresolved_dependencies.len()
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        let total = self.resolved_count() + self.unresolved_count();
        if total > 0 {
            self.resolved_count() as f64 / total as f64
        } else {
            1.0
        }
    }

    /// Check if resolution is complex
    pub fn is_complex(&self) -> bool {
        self.estimated_complexity > 10
    }
}

impl DependencyStatistics {
    /// Check if the dependency graph is healthy
    pub fn is_healthy(&self) -> bool {
        self.circular_dependencies == 0 && self.average_dependencies < 5.0
    }

    /// Get complexity rating
    pub fn complexity_rating(&self) -> &'static str {
        if self.average_dependencies < 2.0 {
            "Simple"
        } else if self.average_dependencies < 5.0 {
            "Moderate"
        } else if self.average_dependencies < 10.0 {
            "Complex"
        } else {
            "Very Complex"
        }
    }
}
