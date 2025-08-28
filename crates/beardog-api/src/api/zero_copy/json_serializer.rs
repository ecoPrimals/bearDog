

use super::HttpBufferPool;
use beardog_errors::BearDogError;
use bytes::{BufMut, Bytes};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

pub struct ZeroCopyJsonSerializer {
    buffer_pool: Arc<HttpBufferPool>,
    stats: ZeroCopySerializerStats,
}
#[derive(Debug, Default)]
pub struct ZeroCopySerializerStats {
    pub serializations: AtomicU64,
    pub deserializations: AtomicU64,
    pub buffer_reuses: AtomicU64,
    pub zero_copy_operations: AtomicU64,
    pub bytes_processed: AtomicU64,}

impl ZeroCopyJsonSerializer {

    pub fn new(buffer_pool: Arc<HttpBufferPool>) -> Self {
        Self {
            buffer_pool,
            stats: ZeroCopySerializerStats::default(),
        }
    }

    pub async fn serialize_zero_copy<T: Serialize>(&self, value: &T) -> Result<Bytes, BearDogError> {
        self.stats.serializations.fetch_add(1, Ordering::Relaxed);

        let mut buffer = self.buffer_pool.get_medium_buffer().await;

        match serde_json::to_writer((&mut buffer).writer(), value) {
            Ok(()) => {
                self.stats.buffer_reuses.fetch_add(1, Ordering::Relaxed);
                self.stats
                    .bytes_processed
                    .fetch_add(buffer.len() as u64, Ordering::Relaxed);
                Ok(buffer.freeze())
            }
            Err(e) => Err(BearDogError::Serialization {
                message: format!("JSON serialization failed: {e}"),
            }),

    pub async fn deserialize_zero_copy<T: for<'de> Deserialize<'de>>(
        &self,
        data: &[u8],
    ) -> Result<T, BearDogError> {
        self.stats.deserializations.fetch_add(1, Ordering::Relaxed);
        self.stats
            .bytes_processed
            .fetch_add(data.len() as u64, Ordering::Relaxed);
        match serde_json::from_slice(data) {
            Ok(value) => {
                    .zero_copy_operations
                    .fetch_add(1, Ordering::Relaxed);
                Ok(value)
            Err(e) => Err(BearDogError::internal(format!("JSON deserialization failed: {e)"),

    pub async fn batch_serialize<T: Serialize>(&self, items: &[T]) -> Result<Vec<Bytes>, BearDogError>> {
        let mut results = Vec::with_capacity(items.len());
        for item in items {
            let serialized = self.serialize_zero_copy(item).await?;
            results.push(serialized);
        Ok(results)

    pub fn get_stats(&self) -> &ZeroCopySerializerStats {
        &self.stats
