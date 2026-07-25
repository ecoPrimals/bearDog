// SPDX-License-Identifier: AGPL-3.0-or-later

//! BLAKE3-digest replay cache for enrollment proofs.

use parking_lot::Mutex;
use std::collections::HashMap;

use super::config::{current_unix_timestamp, load_timestamp_window};

/// Maximum entries in the replay cache before forced pruning.
const REPLAY_CACHE_MAX_ENTRIES: usize = 10_000;

/// Tracks seen enrollment proofs to prevent replay attacks.
///
/// Keys are BLAKE3 digests of the full proof message; values are the wall-clock
/// timestamp from the proof (used for expiry). Entries older than the timestamp
/// window are pruned on each `check_and_record` call.
pub struct ReplayCache {
    seen: Mutex<HashMap<[u8; 32], u64>>,
}

impl ReplayCache {
    pub(crate) fn new() -> Self {
        Self {
            seen: Mutex::new(HashMap::new()),
        }
    }

    /// Check whether `digest` has been seen. If not, record it. Returns `true`
    /// if this is a replay (already seen), `false` if fresh.
    pub(crate) fn check_and_record(&self, digest: [u8; 32], proof_timestamp: u64) -> bool {
        let window = load_timestamp_window();
        let now = current_unix_timestamp();
        let mut map = self.seen.lock();

        if map.len() > REPLAY_CACHE_MAX_ENTRIES / 2 {
            let cutoff = now.saturating_sub(window);
            map.retain(|_, ts| *ts > cutoff);
        }

        if map.contains_key(&digest) {
            return true;
        }

        map.insert(digest, proof_timestamp);
        false
    }
}
