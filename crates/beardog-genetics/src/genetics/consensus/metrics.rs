

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
    /// The successful consensus value
    pub successful_consensus: AtomicU64,
    /// The failed consensus value
    pub failed_consensus: AtomicU64,
    pub average_consensus_time_ms: AtomicU64,
    /// The average participation rate value
    pub average_participation_rate: AtomicU64,
    /// The genetic quality trend value
    pub genetic_quality_trend: AtomicU64,
    /// The trust network health value
    pub trust_network_health: AtomicU64,
    detailed_metrics: Arc<RwLock<DetailedMetrics>>,
}

#[derive(Debug, Clone)]
    participation_rates: Vec<f64>,
    genetic_qualities: Vec<f64>,
}

impl ConsensusMetrics {
/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            total_proposals: AtomicU64::new(0),
            successful_consensus: AtomicU64::new(0),
            failed_consensus: AtomicU64::new(0),
            average_consensus_time_ms: AtomicU64::new(0),
            average_participation_rate: AtomicU64::new(0),
            genetic_quality_trend: AtomicU64::new(0),
            trust_network_health: AtomicU64::new(0),
            detailed_metrics: Arc::new(RwLock::new(DetailedMetrics::default())),
        }
    }

/// Increment Total Proposals operation.
    pub fn increment_total_proposals(&self) {
        self.total_proposals.fetch_add(1, Ordering::Relaxed);
    }

/// Increment Successful Consensus operation.
    pub fn increment_successful_consensus(&self) {
        self.successful_consensus.fetch_add(1, Ordering::Relaxed);
    }

/// Increment Failed Consensus operation.
    pub fn increment_failed_consensus(&self) {
        self.failed_consensus.fetch_add(1, Ordering::Relaxed);
    }

/// Record Consensus Time operation.
    pub fn record_consensus_time(&self, time_ms: u64) {
        let mut detailed = self.detailed_metrics.write();
        detailed.consensus_times.push(time_ms);

        let average = detailed.consensus_times.iter().sum::<u64>() / detailed.consensus_times.len() as u64;
        self.average_consensus_time_ms.store(average, Ordering::Relaxed);
    }

/// Record Participation Rate operation.
    pub fn record_participation_rate(&self, rate: f64) {
        let mut detailed = self.detailed_metrics.write();
        detailed.participation_rates.push(rate);

        let average = detailed.participation_rates.iter().sum::<f64>() / detailed.participation_rates.len() as f64;
        self.average_participation_rate.store((average * 100.0) as u64, Ordering::Relaxed);
    }

/// Record Genetic Quality operation.
    pub fn record_genetic_quality(&self, quality: f64) {
        let mut detailed = self.detailed_metrics.write();
        detailed.genetic_qualities.push(quality);

        let average = detailed.genetic_qualities.iter().sum::<f64>() / detailed.genetic_qualities.len() as f64;
        self.genetic_quality_trend.store((average * 100.0) as u64, Ordering::Relaxed);
    }

/// Get Total Proposals operation.
    /// Gets total_proposals
    /// Gets total_proposals
    pub fn get_total_proposals(&self) -> u64 {
        self.total_proposals.load(Ordering::Relaxed)
    }

/// Get Successful Consensus operation.
    /// Gets successful_consensus
    /// Gets successful_consensus
    pub fn get_successful_consensus(&self) -> u64 {
        self.successful_consensus.load(Ordering::Relaxed)
    }

/// Get Failed Consensus operation.
    /// Gets failed_consensus
    /// Gets failed_consensus
    pub fn get_failed_consensus(&self) -> u64 {
        self.failed_consensus.load(Ordering::Relaxed)
    }

/// Get Success Rate operation.
    /// Gets success_rate
    /// Gets success_rate
    pub fn get_success_rate(&self) -> f64 {
        let total = self.get_total_proposals();
        if total == 0 {
            0.0
        } else {
            self.get_successful_consensus() as f64 / total as f64
        }
    }

/// Get Average Consensus Time Ms operation.
    /// Gets average_consensus_time_ms
    /// Gets average_consensus_time_ms
    pub fn get_average_consensus_time_ms(&self) -> u64 {
        self.average_consensus_time_ms.load(Ordering::Relaxed)
    }

/// Get Average Participation Rate operation.
    /// Gets average_participation_rate
    /// Gets average_participation_rate
    pub fn get_average_participation_rate(&self) -> f64 {
        self.average_participation_rate.load(Ordering::Relaxed) as f64 / 100.0
    }
} 
