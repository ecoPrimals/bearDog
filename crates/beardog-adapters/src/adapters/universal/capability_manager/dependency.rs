

use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use beardog_errors::BearDogResult;

pub struct DependencyResolver {

    pub dependency_graph: Arc<RwLock<HashMap<String, Vec<String>>>>,

    pub circular_dependencies: Arc<RwLock<HashSet<String>>>,

    pub resolution_cache: Arc<RwLock<HashMap<String, ResolutionResult>>>,
}

#[derive(Debug, Clone)]
pub struct ResolutionResult {

    pub dependency_chain: Vec<String>,

    pub resolution_order: Vec<String>,

    pub circular_detected: bool,

    pub unresolved_dependencies: Vec<String>,

    pub estimated_complexity: u32,

    pub resolution_timestamp: DateTime<Utc>,}

impl DependencyResolver {

    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            dependency_graph: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            circular_dependencies: Arc::new(RwLock::new(HashSet::new())),
            resolution_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }

    pub async fn resolve_dependencies(
        &self,
        required_capabilities: &[&str],
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

        let cache_key = required_capabilities.join("+");
        if let Some(cached_result) = self.resolution_cache.read().await.get(&cache_key) {
            debug!("📋 Using cached resolution result");
            return Ok(cached_result.clone());
        }

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

        let circular_detected = self.detect_circular_dependencies(&dependency_chain).await?;

        let estimated_complexity = self.calculate_complexity(&dependency_chain).await;
        let result = ResolutionResult {
            dependency_chain,
            resolution_order,
            circular_detected,
            unresolved_dependencies,
            estimated_complexity,
            resolution_timestamp: Utc::now(),
        };

        self.resolution_cache
            .write()
            .await
            .insert(cache_key, result.clone());
            "✅ Dependency resolution complete - {} dependencies resolved",
            result.dependency_chain.len()
        Ok(result)

    fn resolve_single_dependency<'a>(
        &'a self,
        capability_id: &'a str,
        dependency_chain: &'a mut Vec<String>,
        resolution_order: &'a mut Vec<String>,
        _unresolved_dependencies: &'a mut Vec<String>,
        visited: &'a mut HashSet<String>,
        in_progress: &'a mut HashSet<String>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + 'a>> {
        Box::pin(async move {

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

            in_progress.insert(capability_id.to_string());

            let dependencies = self.get_capability_dependencies(capability_id).await?;

            for dep in &dependencies {
                if !visited.contains(dep) {
                    self.resolve_single_dependency(
                        dep,
                        dependency_chain,
                        resolution_order,
                        _unresolved_dependencies,
                        visited,
                        in_progress,
                    )
                    .await?;
                }

            in_progress.remove(capability_id);
            visited.insert(capability_id.to_string());

            dependency_chain.push(capability_id.to_string());
            resolution_order.push(capability_id.to_string());
            Ok(())

    async fn get_capability_dependencies(&self, capability_id: &str) -> BearDogResult<Vec<String>> {
        let dependency_graph = self.dependency_graph.read().await;
        Ok(dependency_graph
            .get(capability_id)
            .cloned()
            .unwrap_or_default())

    async fn detect_circular_dependencies(
        dependency_chain: &[&str],
    ) -> BearDogResult<bool> {
        let circular_deps = self.circular_dependencies.read().await;

        let has_circular = dependency_chain
            .iter()
            .any(|cap| circular_deps.contains(cap));
        if has_circular {
            warn!("🔄 Circular dependencies detected in resolution chain");
        Ok(has_circular)

    async fn calculate_complexity(&self, dependency_chain: &[&str]) -> u32 {
        let mut complexity = dependency_chain.len() as u32;

        for capability_id in dependency_chain {
            if let Some(deps) = dependency_graph.get(capability_id) {
                complexity += deps.len() as u32;
        complexity

    pub async fn add_dependency(
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

        self.resolution_cache.write().await.clear();
        Ok(())

    pub async fn remove_dependency(
        if let Some(dependencies) = dependency_graph.get_mut(capability_id) {
            dependencies.retain(|dep| dep != dependency_id);
                "➖ Removed dependency: {} -> {}",

    pub async fn get_all_dependencies(&self, capability_id: &str) -> BearDogResult<Vec<String>> {

    pub async fn get_dependents(&self, capability_id: &str) -> BearDogResult<Vec<String>> {
        let mut dependents = Vec::new();
        for (cap_id, deps) in dependency_graph.iter() {
            if deps.contains(&capability_id.to_string()) {
                dependents.push(cap_id.clone());
        Ok(dependents)

    pub async fn clear_cache(&self) -> BearDogResult<()> {
        debug!("🧹 Cleared dependency resolution cache");

    pub async fn get_statistics(&self) -> BearDogResult<DependencyStatistics> {
        let cache = self.resolution_cache.read().await;
        let total_capabilities = dependency_graph.len();
        let total_dependencies = dependency_graph.values().map(|deps| deps.len()).sum();
        let circular_count = circular_deps.len();
        let cache_size = cache.len();
        let average_dependencies = if total_capabilities > 0 {
            total_dependencies as f64 / total_capabilities as f64
        } else {
            0.0
        Ok(DependencyStatistics {
            total_capabilities,
            total_dependencies,
            circular_dependencies: circular_count,
            average_dependencies,
            cache_size,

pub struct DependencyStatistics {

    pub total_capabilities: usize,

    pub total_dependencies: usize,

    pub circular_dependencies: usize,

    pub average_dependencies: f64,

    pub cache_size: usize,}

impl ResolutionResult {

    pub fn is_successful(&self) -> bool {
        self.unresolved_dependencies.is_empty() && !self.circular_detected

    pub fn resolved_count(&self) -> usize {
        self.dependency_chain.len()

    pub fn unresolved_count(&self) -> usize {
        self.unresolved_dependencies.len()

    pub fn success_rate(&self) -> f64 {
        let total = self.resolved_count() + self.unresolved_count();
        if total > 0 {
            self.resolved_count() as f64 / total as f64
            1.0

    pub fn is_complex(&self) -> bool {
        self.estimated_complexity > 10
impl DependencyStatistics {

    pub fn is_healthy(&self) -> bool {
        self.circular_dependencies == 0 && self.average_dependencies < 5.0

    pub fn complexity_rating(&self) -> &'static str {
        if self.average_dependencies < 2.0 {
            "Simple"
        } else if self.average_dependencies < 5.0 {
            "Moderate"
        } else if self.average_dependencies < 10.0 {
            "Complex"
            "Very Complex"
