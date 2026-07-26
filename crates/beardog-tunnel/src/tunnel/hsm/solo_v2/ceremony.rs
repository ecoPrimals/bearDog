// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ceremony tap sequence types and timing entropy analysis.

/// A single tap in a ceremony sequence.
pub struct CeremonyTap {
    /// Zero-based tap index within the ceremony.
    pub index: usize,
    /// Fresh OS-RNG challenge sent for this tap (Tier 1).
    pub challenge: Vec<u8>,
    /// Signature from the authenticator containing hardware RNG nonce (Tier 2).
    pub signature: Vec<u8>,
    /// Transport-layer timing metadata (Tier 3 human temporal entropy).
    pub timing: super::hid_transport::TapTimingEntropy,
}

/// Result of a completed tap-sequence ceremony.
pub struct CeremonyResult {
    /// All taps collected during the ceremony.
    pub taps: Vec<CeremonyTap>,
    /// How many taps were originally requested.
    pub taps_requested: usize,
    /// Total wall-clock duration of the ceremony in milliseconds.
    pub total_duration_ms: u64,
    /// Caller-supplied purpose label (e.g. `"loam_seed"`, `"entropy_harvest"`).
    pub purpose: String,
}

impl CeremonyResult {
    /// Number of taps actually completed.
    #[must_use]
    pub fn taps_completed(&self) -> usize {
        self.taps.len()
    }

    /// Inter-tap intervals in nanoseconds.
    #[must_use]
    pub fn inter_tap_intervals_ns(&self) -> Vec<u64> {
        self.taps
            .windows(2)
            .map(|w| {
                w[1].timing
                    .response_received_ns
                    .saturating_sub(w[0].timing.response_received_ns)
            })
            .collect()
    }

    /// Inter-tap intervals in milliseconds (for display).
    #[must_use]
    pub fn inter_tap_intervals_ms(&self) -> Vec<u64> {
        self.inter_tap_intervals_ns()
            .iter()
            .map(|ns| ns / 1_000_000)
            .collect()
    }

    /// Mean human reaction time in milliseconds (keepalive/command to response).
    #[must_use]
    pub fn mean_reaction_ms(&self) -> u64 {
        if self.taps.is_empty() {
            return 0;
        }
        let sum: u64 = self.taps.iter().map(|t| t.timing.reaction_ns()).sum();
        sum / (self.taps.len() as u64) / 1_000_000
    }

    /// Standard deviation of reaction times in milliseconds (jitter = entropy quality).
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "timing arithmetic with known ranges"
    )]
    pub fn reaction_jitter_ms(&self) -> u64 {
        if self.taps.len() < 2 {
            return 0;
        }
        let mean_ns = {
            let sum: u64 = self.taps.iter().map(|t| t.timing.reaction_ns()).sum();
            sum / self.taps.len() as u64
        };
        let variance: u64 = self
            .taps
            .iter()
            .map(|t| {
                let diff = i128::from(t.timing.reaction_ns()) - i128::from(mean_ns);
                (diff * diff) as u64
            })
            .sum::<u64>()
            / (self.taps.len() as u64 - 1);
        let std_dev_ns = (variance as f64).sqrt() as u64;
        std_dev_ns / 1_000_000
    }

    /// Rough estimate of timing entropy bits (log2 of inter-tap jitter range).
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "log2 of microsecond range safely fits in u32"
    )]
    pub fn timing_entropy_bits_estimate(&self) -> u32 {
        let intervals = self.inter_tap_intervals_ns();
        if intervals.len() < 2 {
            return 0;
        }
        let min = intervals.iter().copied().min().unwrap_or(0);
        let max = intervals.iter().copied().max().unwrap_or(0);
        let range_us = (max.saturating_sub(min)) / 1_000;
        if range_us == 0 {
            return 0;
        }
        (range_us as f64).log2() as u32
    }
}
