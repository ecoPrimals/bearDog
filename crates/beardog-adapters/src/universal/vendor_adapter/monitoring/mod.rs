

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;
use crate::universal::vendor_adapter::{UniversalVendorRequest, UniversalVendorResponse};

pub struct VendorMetricsCollector {
    collector_id: Uuid,
    metrics: Arc<VendorMetrics>,
}

#[derive(Debug, Clone)]
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    total_response_time_ms: AtomicU64,
impl VendorMetricsCollector {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        let collector_id = Uuid::new_v4();
        tracing::info!("📊 Creating Vendor Metrics Collector: {}", collector_id);
        Ok(Self {
            collector_id,
            metrics: Arc::new(VendorMetrics::default()),
        })
    }

/// Record Request operation.
    pub fn record_request(&self, _request: &UniversalVendorRequest) {
        self.metrics.total_requests.fetch_add(1, Ordering::Relaxed);
        tracing::debug!("📈 Recorded vendor request");

/// Record Success operation.
    pub fn record_success(&UniversalVendorRequest,
        _response: &UniversalVendorResponse,
    ) {
        self.metrics
            .successful_requests
            .fetch_add(1, Ordering::Relaxed);
        tracing::debug!("✅ Recorded successful vendor response");

/// Record Failure operation.
    pub fn record_failure(&beardog_errors::BearDogError,
        self.metrics.failed_requests.fetch_add(1, Ordering::Relaxed);
        tracing::warn!("❌ Recorded vendor failure: {}", error);

/// Record Response Time operation.
    pub fn record_response_time(&self, start_time: Instant) {
        let elapsed_ms = start_time.elapsed().as_millis().min(u64::MAX as u128) as u64;
            .total_response_time_ms
            .fetch_add(elapsed_ms, Ordering::Relaxed);

/// Get Summary operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets summary
    /// Gets summary
    pub fn get_summary(&self) -> Result<MetricsSummary, BearDogError> {
        let total = self.metrics.total_requests.load(Ordering::Relaxed);
        let successful = self.metrics.successful_requests.load(Ordering::Relaxed);
        let failed = self.metrics.failed_requests.load(Ordering::Relaxed);
        let total_time = self.metrics.total_response_time_ms.load(Ordering::Relaxed);
        let average_response_time_ms = if total > 0 {
            total_time as f64 / total as f64
        } else {
            0.0
        };
        Ok(self.collector_id,
            total_requests: total,
            successful_requests: successful,
            failed_requests: failed,
            average_response_time_ms,

#[derive(Debug, Clone)]
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of successful_requests
    pub successful_requests: u64,
    /// Number of failed_requests
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
