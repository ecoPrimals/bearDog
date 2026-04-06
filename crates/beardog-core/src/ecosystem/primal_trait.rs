// SPDX-License-Identifier: AGPL-3.0-or-later

use super::primal_types::{
    PrimalCapability, PrimalError, PrimalHealth, PrimalMetadata, PrimalRequest, PrimalResponse,
    UniversalIntegrationConfig,
};

/// Core trait for primal ecosystem services
///
/// Defines the standard interface that all primals must implement to participate
/// in the ecosystem, enabling universal discovery, integration, and management.
#[allow(
    async_fn_in_trait,
    reason = "Async primal trait methods for ecosystem integration"
)]
pub trait EcoPrimal: Send + Sync {
    /// Returns metadata describing this primal's identity and characteristics
    fn metadata(&self) -> &PrimalMetadata;

    /// Returns the list of capabilities this primal provides
    fn capabilities(&self) -> Vec<PrimalCapability>;

    /// Initializes the primal service component
    ///
    /// # Errors
    /// Returns `PrimalError` if initialization fails due to invalid configuration or resource unavailability
    fn initialize(&self, config: &UniversalIntegrationConfig) -> Result<(), PrimalError>;

    /// Handles incoming primal request
    ///
    /// # Errors
    /// Returns `PrimalError` if the request cannot be processed or validation fails
    fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError>;

    /// Performs health check and returns current operational status
    fn health_check(&self) -> PrimalHealth;

    /// Shuts down the primal service
    ///
    /// # Errors
    /// Returns `PrimalError` if shutdown fails or cleanup encounters errors
    fn shutdown(&self) -> Result<(), PrimalError>;
}
