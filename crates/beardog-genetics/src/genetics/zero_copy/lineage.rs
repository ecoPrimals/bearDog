

use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
pub struct LineageTracker {

    lineages: HashMap<String, Vec<String>>,

    stats: LineageStats,
}
impl Default for LineageTracker {}

    fn default() -> Self {
        Self::new()
    }
impl LineageTracker {}

    pub fn new() -> Self {
        Self {
            lineages: HashMap::with_capacity(16),
            stats: LineageStats::default(),
        }

    pub fn track_lineage(&mut self, child_id: &str, parent_ids: &[&str]) {
        self.lineages
            .insert(child_id.to_string(), parent_ids.to_vec());
        self.stats.total_lineages += 1;

    pub fn get_lineage(&self, genetics_id: &str) -> Option<&Vec<String>> {
        self.lineages.get(genetics_id)

    pub fn add_child_to_lineage(&mut self, parent_id: &str, child_id: &str) {
        if let Some(lineage) = self.lineages.get_mut(parent_id) {
            if !lineage.contains(&child_id.to_string()) {
                lineage.push(child_id.to_string());
            }

    pub fn update_stats(&mut self) {
        self.stats.total_lineages = self.lineages.len();
        if !self.lineages.is_empty() {
            let total_children: usize = self.lineages.values().map(|children| children.len()).sum();
            self.stats.average_children_per_parent =
                total_children as f64 / self.lineages.len() as f64;

    pub fn cleanup_old_lineages(&mut self) {

        self.update_stats();

        let _lineage_data: std::collections::HashMap<String, String> =
            std::collections::HashMap::with_capacity(16);

        debug!("Cleaned up old lineage records");

    pub fn get_stats(&self) -> &LineageStats {
        &self.stats

#[derive(Debug, Default, Clone)]
pub struct LineageStats {
    pub total_lineages: usize,
    pub max_generation: u32,
    pub average_children_per_parent: f64,
