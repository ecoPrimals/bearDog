

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub use beardog_errors::BearDogResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorErrorContext {

    pub request_id: Uuid,

    pub capability: CapabilityType,

    pub handler_name: Option<String>,

    pub timestamp: DateTime<Utc>,

    pub metadata: HashMap<String, String>,
}
impl VendorErrorContext {

    #[must_use] 
    pub fn new(request_id: Uuid, capability: CapabilityType) -> Self {
        Self {
            request_id,
            capability,
            handler_name: None,
            timestamp: Utc::now(),
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn with_handler(mut self, handler_name: impl Into<&str>) -> Self {
        self.handler_name = Some(handler_name.into());
        self

    pub fn with_metadata(mut self, key: impl Into<&str>, value: impl Into<&str>) -> Self {
        self.metadata.insert(key.into(), value.into());

pub use beardog_errors::{BearDogError, BearDogResult};

