

use super::TrustLevel;
use super::{TrustConfig, TrustStore};
use beardog_errors::BearDogError;
use tracing::debug;

#[derive(Debug, Clone)]
}

impl TrustPropagation {

    #[inline]

    #[must_use = "Trust propagation result should be checked"]
/// Propagate Trust operation.
    pub fn propagate_trust(&mut TrustStore,
        from_node: &str,
        to_node: &str,
        trust_level: TrustLevel,
    ) -> Result<(), BearDogError> {
        debug!(
            "📡 Propagating trust: {} -> {} ({:?})",
            from_node, to_node, trust_level
        );

        if !self.config.enable_propagation {
            return Err(BearDogError::validation("Trust propagation is disabled"));
        }

        Ok(())
    }
}
