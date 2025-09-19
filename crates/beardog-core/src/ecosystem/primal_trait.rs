use super::primal_types::*;

#[allow(async_fn_in_trait)]
pub trait EcoPrimal: Send + Sync {
    fn metadata(&self) -> &PrimalMetadata;

    fn capabilities(&self) -> Vec<PrimalCapability>;

    /// Initializes componentialize
    fn initialize(&self, config: &UniversalIntegrationConfig) -> Result<(), PrimalError>;

    /// Handles request
    fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError>;

    fn health_check(&self) -> PrimalHealth;

    fn shutdown(&self) -> Result<(), PrimalError>;
}
