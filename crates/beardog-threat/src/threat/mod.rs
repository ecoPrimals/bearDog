use beardog_errors::BearDogError;
pub mod handlers;
pub mod ml_engine;
pub mod tests;
pub mod types;

pub use handlers::core::ThreatDetectionEngine;
pub use ml_engine::SmartThreatMLEngine;
pub use types::{
    DetectionRule, MitigationStep, SecurityEvent, ThreatDetectionConfig, ThreatEvent,
    ThreatIndicator, ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget, ThreatType,
};

pub struct ThreatAPI;
impl ThreatAPI {
    pub async fn create_default() -> Result<ThreatDetectionEngine, BearDogError> {
        ThreatDetectionEngine::new(ThreatDetectionConfig::default()).await
    }

    pub async fn new_with_ml() -> Result<(ThreatDetectionEngine, SmartThreatMLEngine), BearDogError>
    {
        let threat_engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).await?;
        let ml_engine = SmartThreatMLEngine::new();
        Ok((threat_engine, ml_engine))
    }
}
