

use beardog_errors::BearDogResult;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;
use crate::universal::vendor_adapter::{UniversalVendorRequest, UniversalVendorResponse};

pub struct VendorMetricsCollector {
    collector_id: Uuid,
    metrics: Arc<VendorMetrics>,
}

#[derive(Default)]
struct VendorMetrics {
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    total_response_time_ms: AtomicU64,
impl VendorMetricsCollector {

    pub async fn new() -> BearDogResult<Self> {
        let collector_id = Uuid::new_v4();
        tracing::info!("📊 Creating Vendor Metrics Collector: {}", collector_id);
        Ok(Self {
            collector_id,
            metrics: Arc::new(VendorMetrics::default()),
        })
    }

    pub async fn record_request(&self, _request: &UniversalVendorRequest) {
        self.metrics.total_requests.fetch_add(1, Ordering::Relaxed);
        tracing::debug!("📈 Recorded vendor request");

    pub async fn record_success(
        &self,
        _request: &UniversalVendorRequest,
        _response: &UniversalVendorResponse,
    ) {
        self.metrics
            .successful_requests
            .fetch_add(1, Ordering::Relaxed);
        tracing::debug!("✅ Recorded successful vendor response");

    pub async fn record_failure(
        error: &beardog_errors::BearDogError,
        self.metrics.failed_requests.fetch_add(1, Ordering::Relaxed);
        tracing::warn!("❌ Recorded vendor failure: {}", error);

    pub async fn record_response_time(&self, start_time: Instant) {
        let elapsed_ms = start_time.elapsed().as_millis().min(u64::MAX as u128) as u64;
            .total_response_time_ms
            .fetch_add(elapsed_ms, Ordering::Relaxed);

    pub async fn get_summary(&self) -> BearDogResult<MetricsSummary> {
        let total = self.metrics.total_requests.load(Ordering::Relaxed);
        let successful = self.metrics.successful_requests.load(Ordering::Relaxed);
        let failed = self.metrics.failed_requests.load(Ordering::Relaxed);
        let total_time = self.metrics.total_response_time_ms.load(Ordering::Relaxed);
        let average_response_time_ms = if total > 0 {
            total_time as f64 / total as f64
        } else {
            0.0
        };
        Ok(MetricsSummary {
            collector_id: self.collector_id,
            total_requests: total,
            successful_requests: successful,
            failed_requests: failed,
            average_response_time_ms,

#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub collector_id: Uuid,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
