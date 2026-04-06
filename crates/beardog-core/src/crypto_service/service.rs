// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core crypto service implementation
//!
//! Contains the main `BearDogCryptoService` struct and initialization logic.

use super::config::{CryptoServiceConfig, CryptoServiceState};
use beardog_errors::BearDogError;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// Result type alias for crypto service operations
pub type Result<T> = std::result::Result<T, BearDogError>;

/// Main crypto service implementation
///
/// Provides protocol-agnostic cryptographic operations that can be
/// exposed via HTTP, JSON-RPC, tarpc, or any other protocol.
pub struct BearDogCryptoService {
    /// Service configuration
    pub(super) config: Arc<CryptoServiceConfig>,
    /// Runtime state
    pub(super) state: Arc<CryptoServiceState>,
}

impl BearDogCryptoService {
    /// Create a new crypto service
    ///
    /// # Arguments
    ///
    /// * `config` - Service configuration
    ///
    /// # Errors
    ///
    /// Returns an error if service initialization fails
    pub fn new(config: CryptoServiceConfig) -> Result<Self> {
        Ok(Self {
            config: Arc::new(config),
            state: Arc::new(CryptoServiceState::new()),
        })
    }

    /// Increment operation counter and return previous value
    pub(super) fn increment_operation_count(&self) -> u64 {
        self.state.operation_count.fetch_add(1, Ordering::Relaxed)
    }

    /// Validate data size against configuration limits
    ///
    /// # Errors
    ///
    /// Returns error if data exceeds `max_data_size`
    pub(super) fn validate_data_size(&self, data: &[u8]) -> Result<()> {
        if data.len() > self.config.max_data_size {
            return Err(BearDogError::business(format!(
                "Data size {} exceeds maximum {}",
                data.len(),
                self.config.max_data_size
            )));
        }
        Ok(())
    }
}

