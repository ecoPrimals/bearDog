// SPDX-License-Identifier: AGPL-3.0-or-later

//! Global counter for concurrent HTTP requests handled by the integration API server.
//!
//! The heartbeat service reports this value as [`crate::upa_client::LoadMetrics::active_connections`].
//! It tracks in-flight requests (each accepted request increments until the response completes).

use std::sync::atomic::{AtomicUsize, Ordering};

static ACTIVE_CONNECTIONS: AtomicUsize = AtomicUsize::new(0);

/// Returns the number of HTTP requests currently being processed by the API server.
#[must_use]
pub fn active_connection_count() -> usize {
    ACTIVE_CONNECTIONS.load(Ordering::Relaxed)
}

/// Decrements the counter when dropped (paired with [`ActiveConnectionGuard::new`]).
pub struct ActiveConnectionGuard;

impl ActiveConnectionGuard {
    /// Registers one active connection and returns a guard that decrements on drop.
    #[must_use]
    #[expect(
        clippy::new_without_default,
        reason = "Default would not increment the counter; use new() at request entry"
    )]
    pub fn new() -> Self {
        ACTIVE_CONNECTIONS.fetch_add(1, Ordering::Relaxed);
        Self
    }
}

impl Drop for ActiveConnectionGuard {
    fn drop(&mut self) {
        ACTIVE_CONNECTIONS.fetch_sub(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_increments_and_decrements() {
        let start = active_connection_count();
        {
            let _g = ActiveConnectionGuard::new();
            assert_eq!(active_connection_count(), start + 1);
        }
        assert_eq!(active_connection_count(), start);
    }
}
