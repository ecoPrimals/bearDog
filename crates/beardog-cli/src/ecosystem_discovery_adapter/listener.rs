// SPDX-License-Identifier: AGPL-3.0-only

//! Ecosystem listener lifecycle and discovery triggering.

use super::EcosystemDiscoveryAdapter;
use beardog_core::zero_knowledge_bootstrap::ecosystem_listener::EcosystemListener;
use beardog_errors::BearDogError;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

impl EcosystemDiscoveryAdapter {
    /// Initialize and start the ecosystem listener
    ///
    /// # Errors
    /// Returns an error if listener initialization or startup fails
    async fn ensure_listener_started(&self) -> Result<(), BearDogError> {
        let mut listener_guard = self.listener.write().await;

        // If listener already exists, it's running
        if listener_guard.is_some() {
            return Ok(());
        }

        info!("🎧 Starting EcosystemListener for discovery...");

        // Create new listener
        let mut listener = EcosystemListener::from_env(
            self.config.clone(),
            Arc::clone(&self.discovered_primals),
            Arc::clone(&self.discovered_capabilities),
        )?;

        // Start listening (non-blocking background tasks)
        listener.start_listening()?;

        *listener_guard = Some(listener);

        info!("✅ EcosystemListener started successfully");
        Ok(())
    }

    /// Trigger discovery process with timeout
    ///
    /// This initiates discovery and waits briefly for initial results.
    /// Uses modern idiomatic async/await patterns.
    ///
    /// # Errors
    /// Returns an error if listener startup fails (not if no primals found)
    pub async fn trigger_discovery(&self) -> Result<(), BearDogError> {
        info!("🔍 Triggering ecosystem discovery...");
        info!(
            "   Methods: Unix socket directory scan (biomeos), mDNS, environment, HTTP discovery"
        );

        // Start listener if not already running
        self.ensure_listener_started().await?;

        // Wait for initial discovery with proper timeout
        // Modern approach: Use tokio::select with timeout instead of sleep + check
        let discovery_timeout = Duration::from_secs(2);

        // Poll for results with timeout (no arbitrary sleep)
        let start = tokio::time::Instant::now();
        let mut interval = tokio::time::interval(Duration::from_millis(50));

        while start.elapsed() < discovery_timeout {
            interval.tick().await;

            let primal_count = self.discovered_primals.read().await.len();
            let capability_count = self.discovered_capabilities.read().await.len();

            // If we've discovered anything, we can return early
            if primal_count > 0 || capability_count > 0 {
                info!(
                    "✅ Discovered {} primals, {} capabilities",
                    primal_count, capability_count
                );
                break;
            }
        }

        // Final check after timeout
        let primal_count = self.discovered_primals.read().await.len();
        let capability_count = self.discovered_capabilities.read().await.len();

        if primal_count == 0 {
            warn!("📋 No primals discovered yet (discovery is ongoing)");
            warn!("   This is normal - discovery continues in background");
        } else {
            info!(
                "✅ Discovered {} primal(s) with {} capability type(s)",
                primal_count, capability_count
            );
        }

        Ok(())
    }
}
