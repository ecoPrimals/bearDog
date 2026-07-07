// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_acme::{create_hot_reload_pair, AcmeClient, AcmeConfig, HotReloadAcceptor};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{error, info};

/// Supervisor result containing the TLS acceptor for the gateway.
pub(super) struct AcmeGateway {
    pub acceptor: HotReloadAcceptor,
}

/// Start the full ACME subsystem: HTTP-01 solver + renewal daemon + hot-reload.
///
/// Returns a `HotReloadAcceptor` that the :443 listener should use.
pub(super) async fn start_acme_gateway() -> Result<AcmeGateway, BearDogError> {
    let config = AcmeConfig::from_env().map_err(|e| BearDogError::Initialization {
        message: format!("ACME config: {e}"),
    })?;

    info!(
        domains = ?config.domains,
        challenge_port = config.challenge_port,
        "starting ACME gateway (HTTP-01 solver + renewal + TLS hot-reload)"
    );

    let mut client = AcmeClient::new(config.clone()).map_err(|e| BearDogError::Initialization {
        message: format!("ACME client: {e}"),
    })?;

    // 1. Spawn HTTP-01 challenge solver on port 80 (or configured port)
    let solver = client.solver().clone();
    let challenge_port = config.challenge_port;
    tokio::spawn(async move {
        if let Err(e) = solver.serve(challenge_port).await {
            error!(error = %e, "ACME HTTP-01 solver failed");
        }
    });

    // 2. Bootstrap TLS: load existing cert or issue new one
    let primary_domain = config.domains.first().ok_or_else(|| BearDogError::Initialization {
        message: "BEARDOG_ACME_DOMAINS is empty".to_string(),
    })?;

    let store = client.store().clone();
    let (acceptor, controller) =
        if let Some(cert) = store.load_cert(primary_domain).await.map_err(|e| {
            BearDogError::Initialization {
                message: format!("load cert: {e}"),
            }
        })? {
            info!(domain = %primary_domain, "loaded existing ACME certificate");
            create_hot_reload_pair(&cert.fullchain_pem, &cert.privkey_pem).map_err(|e| {
                BearDogError::Initialization {
                    message: format!("TLS bootstrap: {e}"),
                }
            })?
        } else {
            info!(
                domain = %primary_domain,
                "no existing cert — issuing initial certificate"
            );
            client
                .issue_certificate()
                .await
                .map_err(|e| BearDogError::Initialization {
                    message: format!("initial cert issuance: {e}"),
                })?;
            let cert = store
                .load_cert(primary_domain)
                .await
                .map_err(|e| BearDogError::Initialization {
                    message: format!("load new cert: {e}"),
                })?
                .ok_or_else(|| BearDogError::Initialization {
                    message: "cert not found after issuance".to_string(),
                })?;
            create_hot_reload_pair(&cert.fullchain_pem, &cert.privkey_pem).map_err(|e| {
                BearDogError::Initialization {
                    message: format!("TLS bootstrap: {e}"),
                }
            })?
        };

    // 3. Wire hot-reload controller into client and spawn renewal daemon
    let controller = Arc::new(controller);
    client.set_reload_controller(Arc::clone(&controller));
    tokio::spawn(async move {
        client.run_renewal_loop().await;
    });

    Ok(AcmeGateway { acceptor })
}
