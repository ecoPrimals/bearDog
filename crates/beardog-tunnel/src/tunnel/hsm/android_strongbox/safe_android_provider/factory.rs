// SPDX-License-Identifier: AGPL-3.0-or-later

//! Factory for creating the best available Android HSM provider.

use super::{
    SafeHardwareProvider, SafeMobileHardwareProvider, SoftwareFallback, StrongBoxAvailable,
    TeeAvailable,
};
use beardog_errors::BearDogError;
use tracing::info;

/// Factory for creating Android providers
pub struct SafeAndroidProviderFactory;

impl SafeAndroidProviderFactory {
    /// Creates the best available provider
    ///
    /// # Errors
    ///
    /// Returns an error if no provider can be created
    pub fn create_best_provider() -> Result<Box<dyn SafeHardwareProvider>, BearDogError> {
        info!("🏭 Creating best available Android provider");

        // Try StrongBox first
        if let Some(strongbox_provider) =
            SafeMobileHardwareProvider::<StrongBoxAvailable>::detect_strongbox()?
        {
            info!("🛡️ Using StrongBox provider (highest security)");
            return Ok(Box::new(strongbox_provider));
        }

        // Try TEE second
        if let Some(tee_provider) = SafeMobileHardwareProvider::<TeeAvailable>::detect_tee()? {
            info!("🔐 Using TEE provider (hardware security)");
            return Ok(Box::new(tee_provider));
        }

        // Fallback to software
        info!("💻 Using software provider (fallback)");
        let software_provider =
            SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback)?;
        Ok(Box::new(software_provider))
    }
}
