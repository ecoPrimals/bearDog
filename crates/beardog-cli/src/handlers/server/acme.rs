// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use tracing::info;

/// Spawn the ACME renewal daemon as a background tokio task.
///
/// Reads config from `BEARDOG_ACME_DOMAINS`, `BEARDOG_ACME_EMAIL`, etc.
/// The daemon runs `AcmeClient::run_renewal_loop()` which checks cert
/// expiry every 12 hours and renews when within 30 days of expiration.
///
/// # Errors
///
/// Returns an error if `AcmeConfig::from_env()` or `AcmeClient::new()`
/// fails (e.g., missing `BEARDOG_ACME_DOMAINS`).
pub(super) fn spawn_acme_renewal_daemon() -> Result<(), BearDogError> {
    let config =
        beardog_acme::AcmeConfig::from_env().map_err(|e| BearDogError::Initialization {
            message: format!("ACME config: {e}"),
        })?;

    info!(
        domains = ?config.domains,
        renewal_days = config.renewal_days_before_expiry,
        "initializing ACME renewal daemon"
    );

    let mut client =
        beardog_acme::AcmeClient::new(config).map_err(|e| BearDogError::Initialization {
            message: format!("ACME client: {e}"),
        })?;

    tokio::spawn(async move {
        client.run_renewal_loop().await;
    });

    Ok(())
}
