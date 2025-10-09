use super::primal_types::{
    PrimalCapability, PrimalError, PrimalHealth, PrimalMetadata, PrimalRequest, PrimalResponse,
    UniversalIntegrationConfig,
};

#[allow(async_fn_in_trait)]
pub trait EcoPrimal: Send + Sync {
    fn metadata(&self) -> &PrimalMetadata;

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

    fn health_check(&self) -> PrimalHealth;

    /// Shuts down the primal service
    ///
    /// # Errors
    /// Returns `PrimalError` if shutdown fails or cleanup encounters errors
    fn shutdown(&self) -> Result<(), PrimalError>;
}
