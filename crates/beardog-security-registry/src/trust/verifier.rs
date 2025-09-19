

use super::TrustConfig;
use super::TrustLevel;
use beardog_errors::BearDogError;
use tracing::debug;

#[derive(Debug, Clone)]
}

impl TrustVerifier {

    #[inline]

    #[must_use = "Trust verification result should be checked"]
/// Verify Trust Establishment operation.
    pub fn verify_trust_establishment(&str,
        to_node: &str,
        trust_level: TrustLevel,
    ) -> Result<(), BearDogError> {
        debug!(
            "🔍 Verifying trust establishment: {} -> {} ({:?})",
            from_node, to_node, trust_level
        );

        if trust_level > self.config.max_trust_level {
            return Err(BearDogError::validation(
                "Trust level exceeds maximum allowed",
            ));
        }

        Ok(())
    }

    #[inline]
        #[allow(clippy::unnecessary_wraps)]

    #[must_use = "Trust maintenance verification result should be checked"]
/// Verify Trust Maintenance operation.
    pub fn verify_trust_maintenance(&str,
        to_node: &str,
        trust_level: TrustLevel,
    ) -> Result<(), BearDogError> {
        debug!(
            "🔄 Verifying trust maintenance: {} -> {} ({:?})",
            from_node, to_node, trust_level
        );

        if from_node.is_empty() || to_node.is_empty() {
            return Err(BearDogError::validation("Invalid node identifiers"));
        }

        Ok(())
    }
}
