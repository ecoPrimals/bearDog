use super::types::{HsmKey, KeyType, HsmHealthStatus, HsmCapabilities};
use beardog_errors::{BearDogError, BearDogResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn}; 