// SPDX-License-Identifier: AGPL-3.0-only

// Entropy Hierarchy Monitoring
//
// This module provides monitoring and analytics capabilities for entropy hierarchy operations.

use super::types::{EntropyClass, EntropyHierarchyConfig, EntropySeed};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// Rolling mean duration in nanoseconds for observability (seed/mix/validate paths).
#[derive(Debug, Clone, Default)]
struct RollingTiming {
    sum_ns: u128,
    count: u64,
}

impl RollingTiming {
    fn record(&mut self, elapsed: Duration) {
        self.sum_ns += elapsed.as_nanos();
        self.count += 1;
    }

    fn average_ms(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "mean duration in ms; ns sum and count converted to f64 for dashboards"
        )]
        {
            (self.sum_ns as f64) / (self.count as f64) / 1_000_000.0
        }
    }
}

fn average_quality_from_seeds(active_seeds: &HashMap<Uuid, EntropySeed>) -> f64 {
    if active_seeds.is_empty() {
        return 0.0;
    }
    let total_quality: f64 = active_seeds
        .values()
        .map(|seed| match &seed.entropy_class {
            EntropyClass::HumanLivedExperience { quality_score, .. } => *quality_score,
            EntropyClass::HumanSupervisedMachine { quality_score, .. } => *quality_score,
            EntropyClass::StoreBoughtMachine { quality_score, .. } => *quality_score,
        })
        .sum();
    #[expect(
        clippy::cast_precision_loss,
        reason = "mean quality over active seed count"
    )]
    let n = active_seeds.len() as f64;
    total_quality / n
}

/// Tracks aggregate statistics and performs scheduled cleanup of expired seeds.
#[derive(Debug, Clone, Default)]
pub struct EntropyMonitor {
    _config: EntropyMonitoringConfig,
    metrics: PerformanceMetrics,
    seed_creation_timing: RollingTiming,
    entropy_mixing_timing: RollingTiming,
    validation_timing: RollingTiming,
}

impl EntropyMonitor {
    /// Creates a new monitor seeded with the hierarchy's quality threshold.
    #[must_use]
    pub fn new(config: &EntropyHierarchyConfig) -> Self {
        Self {
            _config: EntropyMonitoringConfig {
                enable_detailed_analytics: true,
                cleanup_interval_seconds: 3600,
                max_seed_age_days: 30,
                quality_threshold: config.min_human_quality,
            },
            metrics: PerformanceMetrics::default(),
            seed_creation_timing: RollingTiming::default(),
            entropy_mixing_timing: RollingTiming::default(),
            validation_timing: RollingTiming::default(),
        }
    }

    /// Records wall-clock time for creating a seed (validation + mixing + construction).
    pub fn record_seed_creation_duration(&mut self, elapsed: Duration) {
        self.seed_creation_timing.record(elapsed);
    }

    /// Records time spent in entropy mixing / seed construction inside the mixing engine.
    pub fn record_entropy_mixing_duration(&mut self, elapsed: Duration) {
        self.entropy_mixing_timing.record(elapsed);
    }

    /// Records time spent in entropy validation (quality and age checks).
    pub fn record_validation_duration(&mut self, elapsed: Duration) {
        self.validation_timing.record(elapsed);
    }

    /// Merge counter fields, hierarchy-derived averages, and rolling timings into one snapshot.
    pub fn snapshot_performance_metrics(
        &self,
        active_seeds: &HashMap<Uuid, EntropySeed>,
    ) -> PerformanceMetrics {
        let mut m = self.metrics.clone();
        m.active_seeds_count = active_seeds.len();
        m.total_entropy_generated = active_seeds
            .values()
            .map(|s| s.entropy_data.len())
            .sum::<usize>() as u64;
        m.average_quality_score = average_quality_from_seeds(active_seeds);
        if self.seed_creation_timing.count > 0 {
            m.seed_creation_time_ms = self.seed_creation_timing.average_ms();
        }
        if self.entropy_mixing_timing.count > 0 {
            m.entropy_mixing_time_ms = self.entropy_mixing_timing.average_ms();
        }
        if self.validation_timing.count > 0 {
            m.validation_time_ms = self.validation_timing.average_ms();
        }
        m
    }

    /// Removes seeds whose `expires_at` is in the past.
    pub fn cleanup_expired_seeds(&mut self, active_seeds: &mut HashMap<Uuid, EntropySeed>) {
        let now = Utc::now();
        let mut expired_seeds = Vec::new();

        for (seed_id, seed) in active_seeds.iter() {
            if let Some(expires_at) = seed.metadata.expires_at
                && now > expires_at
            {
                expired_seeds.push(*seed_id);
            }
        }

        for seed_id in expired_seeds {
            active_seeds.remove(&seed_id);
        }
    }

    /// Aggregates seed-class counts across all active seeds.
    #[must_use]
    pub fn get_statistics(
        &self,
        active_seeds: &HashMap<Uuid, EntropySeed>,
    ) -> EntropyHierarchyStats {
        let mut human_entropy_seeds = 0;
        let mut human_supervised_seeds = 0;
        let mut machine_entropy_seeds = 0;

        for seed in active_seeds.values() {
            match &seed.entropy_class {
                EntropyClass::HumanLivedExperience { .. } => human_entropy_seeds += 1,
                EntropyClass::HumanSupervisedMachine { .. } => human_supervised_seeds += 1,
                EntropyClass::StoreBoughtMachine { .. } => machine_entropy_seeds += 1,
            }
        }

        EntropyHierarchyStats {
            total_seeds: active_seeds.len(),
            human_entropy_seeds,
            human_supervised_seeds,
            machine_entropy_seeds,
            event_seeds: 0,
            self_sovereign_seeds: 0,
        }
    }

    /// Computes quality/age histograms over the active seed set.
    #[must_use]
    pub fn get_analytics(&self, active_seeds: &HashMap<Uuid, EntropySeed>) -> EntropyAnalytics {
        let stats = self.get_statistics(active_seeds);
        let mut quality_scores = Vec::new();

        for seed in active_seeds.values() {
            let quality_score = match &seed.entropy_class {
                EntropyClass::HumanLivedExperience { quality_score, .. } => *quality_score,
                EntropyClass::HumanSupervisedMachine { quality_score, .. } => *quality_score,
                EntropyClass::StoreBoughtMachine { quality_score, .. } => *quality_score,
            };
            quality_scores.push(quality_score);
        }

        let average_quality = if quality_scores.is_empty() {
            0.0
        } else {
            #[expect(
                clippy::cast_precision_loss,
                reason = "sample count as f64 for mean quality"
            )]
            let n = quality_scores.len() as f64;
            quality_scores.iter().sum::<f64>() / n
        };

        EntropyAnalytics {
            stats,
            average_quality,
            quality_distribution: self.calculate_quality_distribution(&quality_scores),
            age_distribution: self.calculate_age_distribution(active_seeds),
        }
    }

    /// Derives an overall health assessment from the most recent quality score.
    #[must_use]
    pub fn get_health_status(&self) -> EntropyHealthStatus {
        let quality_score = if self.metrics.average_quality_score > 0.0 {
            self.metrics.average_quality_score
        } else {
            0.0
        };
        EntropyHealthStatus {
            overall_health: "Healthy".to_string(),
            quality_score,
            performance_metrics: self.snapshot_performance_metrics(&HashMap::new()),
            last_updated: Utc::now(),
        }
    }

    /// Bins quality scores into `excellent / good / acceptable / poor` buckets.
    pub(crate) fn calculate_quality_distribution(
        &self,
        quality_scores: &[f64],
    ) -> HashMap<String, u32> {
        let mut distribution = HashMap::new();

        for &score in quality_scores {
            let category = if score >= 0.9 {
                "excellent"
            } else if score >= 0.8 {
                "good"
            } else if score >= 0.7 {
                "acceptable"
            } else {
                "poor"
            };

            *distribution.entry(category.to_string()).or_insert(0) += 1;
        }

        distribution
    }

    /// Buckets active seeds by age into `very_new / new / recent / old`.
    pub(crate) fn calculate_age_distribution(
        &self,
        active_seeds: &HashMap<Uuid, EntropySeed>,
    ) -> HashMap<String, u32> {
        let mut distribution = HashMap::new();
        let now = Utc::now();

        for seed in active_seeds.values() {
            let age_hours = (now - seed.metadata.created_at).num_hours();
            let category = if age_hours < 1 {
                "very_new"
            } else if age_hours < 24 {
                "new"
            } else if age_hours < 168 {
                "recent"
            } else {
                "old"
            };

            *distribution.entry(category.to_string()).or_insert(0) += 1;
        }

        distribution
    }

    /// Overwrites the stored counters with externally-computed values.
    pub fn update_metrics(&mut self, metrics: PerformanceMetrics) {
        self.metrics = metrics;
    }

    /// Last-recorded performance counters mirrored from the hierarchy manager.
    #[must_use]
    pub const fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }
}

/// Entropy hierarchy statistics
#[derive(Debug, Clone)]
pub struct EntropyHierarchyStats {
    /// Total seed count across all classes.
    pub total_seeds: usize,
    /// Count of `HumanLivedExperience` seeds.
    pub human_entropy_seeds: u32,
    /// Count of `HumanSupervisedMachine` seeds.
    pub human_supervised_seeds: u32,
    /// Count of `StoreBoughtMachine` seeds.
    pub machine_entropy_seeds: u32,
    /// Count of event-based seeds (reserved).
    pub event_seeds: u32,
    /// Count of self-sovereign seeds (reserved).
    pub self_sovereign_seeds: u32,
}

/// Derived analytics: averages and histograms over active seeds.
#[derive(Debug, Clone)]
pub struct EntropyAnalytics {
    /// Seed-class breakdown.
    pub stats: EntropyHierarchyStats,
    /// Mean quality score across all active seeds.
    pub average_quality: f64,
    /// Quality-bucket histogram.
    pub quality_distribution: HashMap<String, u32>,
    /// Age-bucket histogram.
    pub age_distribution: HashMap<String, u32>,
}

/// Human-readable health rollup plus the numeric metrics it was derived from.
#[derive(Debug, Clone)]
pub struct EntropyHealthStatus {
    /// Overall health label (e.g. "Healthy").
    pub overall_health: String,
    /// Current quality score used for the assessment.
    pub quality_score: f64,
    /// Underlying counters backing the health summary.
    pub performance_metrics: PerformanceMetrics,
    /// Timestamp of this health snapshot.
    pub last_updated: DateTime<Utc>,
}

/// Counters and rolling timings (nanosecond-backed averages) for dashboards and adaptive tuning.
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Number of currently active seeds.
    pub active_seeds_count: usize,
    /// Cumulative entropy bytes produced.
    pub total_entropy_generated: u64,
    /// Mean quality score across active seeds.
    pub average_quality_score: f64,
    /// Rolling average time to create a seed (milliseconds).
    pub seed_creation_time_ms: f64,
    /// Rolling average mixing time (milliseconds).
    pub entropy_mixing_time_ms: f64,
    /// Rolling average validation time (milliseconds).
    pub validation_time_ms: f64,
    /// Lifetime operation count.
    pub total_operations: u64,
    /// Lifetime successful operations.
    pub successful_operations: u64,
    /// Lifetime failed operations.
    pub failed_operations: u64,
}

/// Monitoring configuration
#[derive(Debug, Clone, Default)]
pub struct EntropyMonitoringConfig {
    /// Whether detailed analytics collection is enabled.
    pub enable_detailed_analytics: bool,
    /// Interval between automatic cleanup sweeps (seconds).
    pub cleanup_interval_seconds: u64,
    /// Maximum seed age before forced expiration (days).
    pub max_seed_age_days: u32,
    /// Minimum quality threshold for alerting.
    pub quality_threshold: f64,
}

#[cfg(test)]
#[path = "monitoring_tests.rs"]
mod tests;
