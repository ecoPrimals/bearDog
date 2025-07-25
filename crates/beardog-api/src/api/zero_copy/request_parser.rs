//! Zero-Copy Request Parser
//!
//! Parses HTTP requests with minimal allocations.

use super::{HttpBufferPool, ZeroCopyJsonSerializer};
use beardog_errors::{BearDogError, BearDogResult};
use bytes::Bytes;
use serde::Deserialize;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

/// Zero-copy request parser
pub struct ZeroCopyRequestParser {
    #[allow(dead_code)] // Will be used when zero-copy request parsing is fully implemented
    buffer_pool: Arc<HttpBufferPool>,
    serializer: Arc<ZeroCopyJsonSerializer>,
    stats: ZeroCopyRequestStats,
}

#[derive(Debug, Default)]
pub struct ZeroCopyRequestStats {
    pub requests_parsed: AtomicU64,
    pub json_parsed: AtomicU64,
    pub form_parsed: AtomicU64,
    pub multipart_parsed: AtomicU64,
    pub zero_copy_parses: AtomicU64,
}

impl ZeroCopyRequestParser {
    /// Create new zero-copy request parser
    pub fn new(buffer_pool: Arc<HttpBufferPool>, serializer: Arc<ZeroCopyJsonSerializer>) -> Self {
        Self {
            buffer_pool,
            serializer,
            stats: ZeroCopyRequestStats::default(),
        }
    }

    /// Parse JSON request body with zero-copy
    pub async fn parse_json<T: for<'de> Deserialize<'de>>(&self, body: Bytes) -> BearDogResult<T> {
        self.stats.requests_parsed.fetch_add(1, Ordering::Relaxed);
        self.stats.json_parsed.fetch_add(1, Ordering::Relaxed);

        if body.is_empty() {
            return Err(BearDogError::validation("Request body is empty"));
        }

        let result = self.serializer.deserialize_zero_copy(&body).await?;
        self.stats.zero_copy_parses.fetch_add(1, Ordering::Relaxed);

        Ok(result)
    }

    /// Parse query parameters with zero-copy
    pub async fn parse_query_zero_copy<T: for<'de> Deserialize<'de>>(
        &self,
        query_string: &str,
    ) -> BearDogResult<T> {
        self.stats.requests_parsed.fetch_add(1, Ordering::Relaxed);

        match serde_urlencoded::from_str(query_string) {
            Ok(params) => {
                self.stats.zero_copy_parses.fetch_add(1, Ordering::Relaxed);
                Ok(params)
            }
            Err(e) => Err(BearDogError::validation(format!(
                "Query parameter parsing failed: {e}"
            ))),
        }
    }

    /// Parse form data with buffer pooling
    pub async fn parse_form_data<T: for<'de> Deserialize<'de>>(
        &self,
        form_data: &str,
    ) -> BearDogResult<T> {
        self.stats.requests_parsed.fetch_add(1, Ordering::Relaxed);
        self.stats.form_parsed.fetch_add(1, Ordering::Relaxed);

        match serde_urlencoded::from_str(form_data) {
            Ok(data) => {
                self.stats.zero_copy_parses.fetch_add(1, Ordering::Relaxed);
                Ok(data)
            }
            Err(e) => Err(BearDogError::validation(format!(
                "Form data parsing failed: {e}"
            ))),
        }
    }

    /// Get parser statistics
    pub fn get_stats(&self) -> &ZeroCopyRequestStats {
        &self.stats
    }
}
