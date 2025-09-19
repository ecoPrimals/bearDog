

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// The capability value
    pub capability: CapabilityType,

    /// Name of the handler
    pub handler_name: Option<String>,


    pub timestamp: DateTime<Utc>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl VendorErrorContext {

    #[must_use] 
/// New operation.
    /// Creates a new instance
    pub fn new(Uuid, capability: CapabilityType) -> Self {
        Self {
            request_id,
            capability,
            handler_name: None,
            timestamp: Utc::now(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// With Handler operation.
    /// Creates instance with handler
    pub fn with_handler(mut self, handler_name: impl Into<&str>) -> Self {
        self.handler_name = Some(impl Into<&str>, value: impl Into<&str>) -> Self {
        self.metadata.insert(key.into(), value.into());

pub use beardog_errors::BearDogError;

