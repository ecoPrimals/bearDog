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


/// Zero-Copy JSON Serialization
///
/// Provides JSON serialization with buffer pooling for minimal allocations.

use super::HttpBufferPool;
use beardog_errors::{BearDogError, BearDogResult};
use bytes::{BufMut, Bytes};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
/// Zero-copy JSON serializer with buffer pooling
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
    /// Create new zero-copy JSON serializer}


    pub fn new(buffer_pool: Arc<HttpBufferPool>) -> Self {
        Self {
            buffer_pool,
            stats: ZeroCopySerializerStats::default(),
        }
    }
    /// Serialize to JSON with zero-copy optimization
    pub async fn serialize_zero_copy<T: Serialize>(&self, value: &T) -> BearDogResult<Bytes> {
        self.stats.serializations.fetch_add(1, Ordering::Relaxed);
        // Estimate size and get appropriate buffer
        let mut buffer = self.buffer_pool.get_medium_buffer().await;
        // Serialize directly into buffer
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
    /// Deserialize from JSON with zero-copy optimization
    pub async fn deserialize_zero_copy<T: for<'de> Deserialize<'de>>(
        &self,
        data: &[u8],
    ) -> BearDogResult<T> {
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
    /// Batch serialize multiple items}


    pub async fn batch_serialize<T: Serialize>(&self, items: &[T]) -> BearDogResult<Vec<Bytes>> {
        let mut results = Vec::with_capacity(items.len());
        for item in items {
            let serialized = self.serialize_zero_copy(item).await?;
            results.push(serialized);
        Ok(results)
    /// Get serializer statistics
    pub fn get_stats(&self) -> &ZeroCopySerializerStats {
        &self.stats
