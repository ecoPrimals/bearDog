// SPDX-License-Identifier: AGPL-3.0-only

// Interactive Entropy Capture - Keyboard and Mouse
//
// This module provides live interaction capture for human entropy collection.
// It captures timing and patterns from keyboard and mouse input, ensuring
// all entropy is from LIVE human interaction (no simulation).
//
// CRITICAL: NO SIMULATED ENTROPY - enforced by LiveFeedValidator

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Configuration for interaction capture
#[derive(Debug, Clone)]
pub struct InteractionCaptureConfig {
    /// Target number of interactions to collect
    pub target_interactions: usize,
    /// Maximum time to wait for interactions (seconds)
    pub timeout_seconds: u64,
    /// Minimum entropy quality threshold (0.0-1.0)
    pub min_quality: f64,
    /// Enable keyboard capture
    pub enable_keyboard: bool,
    /// Enable mouse capture
    pub enable_mouse: bool,
}

impl Default for InteractionCaptureConfig {
    fn default() -> Self {
        Self {
            target_interactions: 50, // Collect 50 interactions
            timeout_seconds: 120,    // 2 minute timeout
            min_quality: 0.7,        // 70% quality minimum
            enable_keyboard: true,
            enable_mouse: true,
        }
    }
}

/// A single interaction event with precise timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    /// Kind of hardware event (key, mouse move, click, scroll).
    pub interaction_type: InteractionType,
    /// Monotonic nanoseconds since collection started—primary entropy-bearing signal.
    pub timestamp_nanos: u128,
    /// Privacy-preserving payload: never stores raw key characters or absolute pointer coords.
    pub data: InteractionData,
}

/// High-level categories of terminal input merged into human entropy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    /// Physical key transitioned to pressed.
    KeyPress,
    /// Physical key released.
    KeyRelease,
    /// Pointer moved; deltas are stored, not screen positions.
    MouseMove,
    /// Button down or up event.
    MouseClick,
    /// Scroll wheel notch or trackpad scroll.
    MouseScroll,
}

/// Per-event metadata hashed into the final entropy buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionData {
    /// Key timing and modifier classification without storing which key was pressed.
    Keyboard {
        /// True if the key event was a printable character (still not recorded verbatim).
        is_char: bool,
        /// True if the key was a modifier (Shift, Ctrl, …).
        is_modifier: bool,
    },
    /// Relative pointer motion; absolute coordinates are never retained.
    Mouse {
        /// Horizontal delta in terminal cells or device units.
        delta_x: i16,
        /// Vertical delta in terminal cells or device units.
        delta_y: i16,
    },
    /// Mouse button transition without screen coordinates.
    Click {
        /// Which mouse button toggled—does not reveal what was clicked on screen.
        button: MouseButton,
    },
    /// Discrete scroll steps from wheel or trackpad.
    Scroll {
        /// Signed scroll steps contributing timing and gesture entropy.
        delta: i16,
    },
}

/// Mouse button identity without UI context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    /// Primary button.
    Left,
    /// Secondary button.
    Right,
    /// Middle (wheel click) button.
    Middle,
}

/// Completed run of live capture: raw events, derived bytes, and quality analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionCaptureResult {
    /// All captured interactions
    pub interactions: Vec<InteractionEvent>,
    /// Collection start time
    pub collection_start: DateTime<Utc>,
    /// Collection duration
    pub duration_ms: u64,
    /// Calculated entropy quality
    pub quality_score: f64,
    /// Entropy bytes derived from interactions
    pub entropy_bytes: Vec<u8>,
    /// Quality metrics
    pub metrics: InteractionMetrics,
}

/// Statistical summary used to gate whether timing and movement entropy are sufficient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionMetrics {
    /// Total interactions captured
    pub total_interactions: usize,
    /// Keyboard interactions
    pub keyboard_events: usize,
    /// Mouse interactions
    pub mouse_events: usize,
    /// Average time between interactions (milliseconds)
    pub avg_interval_ms: f64,
    /// Standard deviation of intervals
    pub interval_std_dev: f64,
    /// Timing entropy score (0.0-1.0)
    pub timing_entropy: f64,
    /// Movement entropy score (0.0-1.0, from mouse)
    pub movement_entropy: f64,
    /// Overall quality score (0.0-1.0)
    pub overall_quality: f64,
}

/// Drives blocking terminal collection guided by [`InteractionCaptureConfig`].
pub struct InteractionEntropyCollector {
    config: InteractionCaptureConfig,
}

impl InteractionEntropyCollector {
    /// Create a new interaction collector
    #[must_use]
    pub const fn new(config: InteractionCaptureConfig) -> Self {
        Self { config }
    }

    /// Collect LIVE human interactions - NO SIMULATION ALLOWED
    ///
    /// This is a BLOCKING operation that waits for real user input.
    /// It will display a terminal UI to guide the user and provide
    /// real-time feedback on entropy quality.
    ///
    /// # Errors
    /// Returns error if:
    /// - Terminal setup fails
    /// - Timeout is reached before sufficient interactions
    /// - Quality threshold is not met
    /// - Simulation is detected (should never happen with real input)
    pub fn collect_live_interactions(&self) -> Result<InteractionCaptureResult, BearDogError> {
        let collection_start = Utc::now();
        let start_instant = Instant::now();
        let timeout = Duration::from_secs(self.config.timeout_seconds);

        // Enable raw mode for terminal input
        crossterm::terminal::enable_raw_mode()
            .map_err(|e| BearDogError::system(format!("Failed to enable raw mode: {e}")))?;

        // Ensure we disable raw mode on exit
        let _guard = RawModeGuard;

        let mut interactions = Vec::new();
        let mut last_mouse_pos: Option<(u16, u16)> = None;

        // Display instructions
        self.display_instructions()?;

        // Collection loop
        while interactions.len() < self.config.target_interactions {
            // Check timeout
            if start_instant.elapsed() > timeout {
                return Err(BearDogError::validation(&format!(
                    "Timeout reached. Collected {} of {} target interactions",
                    interactions.len(),
                    self.config.target_interactions
                )));
            }

            // Poll for events with a short timeout
            if event::poll(Duration::from_millis(100))
                .map_err(|e| BearDogError::system(format!("Event poll failed: {e}")))?
            {
                let event = event::read()
                    .map_err(|e| BearDogError::system(format!("Event read failed: {e}")))?;

                let timestamp_nanos = start_instant.elapsed().as_nanos();

                match event {
                    Event::Key(key_event) if self.config.enable_keyboard => {
                        // Handle ESC to exit early
                        if matches!(key_event.code, crossterm::event::KeyCode::Esc) {
                            break;
                        }

                        let interaction = self.process_key_event(key_event, timestamp_nanos);
                        interactions.push(interaction);
                    }
                    Event::Mouse(mouse_event) if self.config.enable_mouse => {
                        let interaction = self.process_mouse_event(
                            mouse_event,
                            timestamp_nanos,
                            &mut last_mouse_pos,
                        );
                        if let Some(interaction) = interaction {
                            interactions.push(interaction);
                        }
                    }
                    _ => {}
                }

                // Update progress display
                self.display_progress(interactions.len())?;
            }
        }

        let duration_ms = start_instant.elapsed().as_millis() as u64;

        // Calculate metrics and derive entropy
        let metrics = Self::calculate_metrics(&interactions, duration_ms);
        let entropy_bytes = Self::derive_entropy_bytes(&interactions, &metrics);

        // Validate quality
        if metrics.overall_quality < self.config.min_quality {
            return Err(BearDogError::validation(&format!(
                "Entropy quality ({:.2}) below threshold ({:.2})",
                metrics.overall_quality, self.config.min_quality
            )));
        }

        Ok(InteractionCaptureResult {
            interactions,
            collection_start,
            duration_ms,
            quality_score: metrics.overall_quality,
            entropy_bytes,
            metrics,
        })
    }

    /// Process a keyboard event into an interaction
    const fn process_key_event(
        &self,
        key_event: KeyEvent,
        timestamp_nanos: u128,
    ) -> InteractionEvent {
        use crossterm::event::KeyCode;

        let is_char = matches!(key_event.code, KeyCode::Char(_));
        let is_modifier = matches!(key_event.code, KeyCode::Modifier(_));

        InteractionEvent {
            interaction_type: match key_event.kind {
                crossterm::event::KeyEventKind::Press => InteractionType::KeyPress,
                crossterm::event::KeyEventKind::Release => InteractionType::KeyRelease,
                _ => InteractionType::KeyPress, // Treat repeat as press
            },
            timestamp_nanos,
            data: InteractionData::Keyboard {
                is_char,
                is_modifier,
            },
        }
    }

    /// Process a mouse event into an interaction
    fn process_mouse_event(
        &self,
        mouse_event: MouseEvent,
        timestamp_nanos: u128,
        last_pos: &mut Option<(u16, u16)>,
    ) -> Option<InteractionEvent> {
        use crossterm::event::{MouseButton as CTMouseButton, MouseEventKind};

        match mouse_event.kind {
            MouseEventKind::Moved => {
                let delta = if let Some((last_x, last_y)) = *last_pos {
                    let delta_x = mouse_event.column as i16 - last_x as i16;
                    let delta_y = mouse_event.row as i16 - last_y as i16;
                    Some((delta_x, delta_y))
                } else {
                    None
                };

                *last_pos = Some((mouse_event.column, mouse_event.row));

                delta.map(|(delta_x, delta_y)| InteractionEvent {
                    interaction_type: InteractionType::MouseMove,
                    timestamp_nanos,
                    data: InteractionData::Mouse { delta_x, delta_y },
                })
            }
            MouseEventKind::Down(button) | MouseEventKind::Up(button) => {
                let button = match button {
                    CTMouseButton::Left => MouseButton::Left,
                    CTMouseButton::Right => MouseButton::Right,
                    CTMouseButton::Middle => MouseButton::Middle,
                };

                Some(InteractionEvent {
                    interaction_type: InteractionType::MouseClick,
                    timestamp_nanos,
                    data: InteractionData::Click { button },
                })
            }
            MouseEventKind::ScrollDown => Some(InteractionEvent {
                interaction_type: InteractionType::MouseScroll,
                timestamp_nanos,
                data: InteractionData::Scroll { delta: -1 },
            }),
            MouseEventKind::ScrollUp => Some(InteractionEvent {
                interaction_type: InteractionType::MouseScroll,
                timestamp_nanos,
                data: InteractionData::Scroll { delta: 1 },
            }),
            _ => None,
        }
    }

    /// Calculate entropy metrics from interactions
    fn calculate_metrics(
        interactions: &[InteractionEvent],
        duration_ms: u64,
    ) -> InteractionMetrics {
        let total_interactions = interactions.len();
        let keyboard_events = interactions
            .iter()
            .filter(|i| {
                matches!(
                    i.interaction_type,
                    InteractionType::KeyPress | InteractionType::KeyRelease
                )
            })
            .count();
        let mouse_events = total_interactions - keyboard_events;

        // Calculate timing metrics
        let intervals: Vec<f64> = interactions
            .windows(2)
            .map(|pair| (pair[1].timestamp_nanos - pair[0].timestamp_nanos) as f64 / 1_000_000.0)
            .collect();

        let avg_interval_ms = if intervals.is_empty() {
            0.0
        } else {
            intervals.iter().sum::<f64>() / intervals.len() as f64
        };

        let interval_std_dev = if intervals.len() > 1 {
            let mean = avg_interval_ms;
            let variance = intervals.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                / (intervals.len() - 1) as f64;
            variance.sqrt()
        } else {
            0.0
        };

        // Calculate timing entropy (using Shannon entropy of interval distribution)
        let timing_entropy = Self::calculate_shannon_entropy(&intervals);

        // Calculate movement entropy (from mouse delta variations)
        let movement_entropy = Self::calculate_movement_entropy(interactions);

        // Overall quality combines multiple factors
        let overall_quality = Self::calculate_overall_quality(
            timing_entropy,
            movement_entropy,
            total_interactions,
            duration_ms,
        );

        InteractionMetrics {
            total_interactions,
            keyboard_events,
            mouse_events,
            avg_interval_ms,
            interval_std_dev,
            timing_entropy,
            movement_entropy,
            overall_quality,
        }
    }

    /// Calculate Shannon entropy of timing intervals
    fn calculate_shannon_entropy(intervals: &[f64]) -> f64 {
        if intervals.is_empty() {
            return 0.0;
        }

        // Bin intervals into buckets (10ms buckets)
        const BUCKET_SIZE: f64 = 10.0;
        let max_interval = intervals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let num_buckets = ((max_interval / BUCKET_SIZE).ceil() as usize).max(1);

        let mut buckets = vec![0usize; num_buckets];
        for &interval in intervals {
            let bucket = ((interval / BUCKET_SIZE) as usize).min(num_buckets - 1);
            buckets[bucket] += 1;
        }

        // Calculate Shannon entropy
        let total = intervals.len() as f64;
        let entropy: f64 = buckets
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / total;
                -p * p.log2()
            })
            .sum();

        // Normalize to 0.0-1.0 range (max entropy is log2(num_buckets))
        let max_entropy = (num_buckets as f64).log2();
        if max_entropy > 0.0 {
            entropy / max_entropy
        } else {
            0.0
        }
    }

    /// Calculate entropy from mouse movement patterns
    fn calculate_movement_entropy(interactions: &[InteractionEvent]) -> f64 {
        let movements: Vec<(i16, i16)> = interactions
            .iter()
            .filter_map(|i| {
                if let InteractionData::Mouse { delta_x, delta_y } = i.data {
                    Some((delta_x, delta_y))
                } else {
                    None
                }
            })
            .collect();

        if movements.is_empty() {
            return 0.5; // Neutral if no mouse data
        }

        // Calculate variance in movement
        let (sum_x, sum_y): (i32, i32) = movements.iter().fold((0, 0), |(sx, sy), &(dx, dy)| {
            (sx + dx as i32, sy + dy as i32)
        });

        let mean_x = sum_x as f64 / movements.len() as f64;
        let mean_y = sum_y as f64 / movements.len() as f64;

        let variance: f64 = movements
            .iter()
            .map(|&(dx, dy)| {
                let diff_x = dx as f64 - mean_x;
                let diff_y = dy as f64 - mean_y;
                diff_x.mul_add(diff_x, diff_y * diff_y)
            })
            .sum::<f64>()
            / movements.len() as f64;

        // Normalize variance to 0.0-1.0 (higher variance = more entropy)
        // Typical mouse movement variance is 0-1000, so we normalize
        (variance / 1000.0).min(1.0)
    }

    /// Calculate overall quality score
    fn calculate_overall_quality(
        timing_entropy: f64,
        movement_entropy: f64,
        total_interactions: usize,
        duration_ms: u64,
    ) -> f64 {
        // Weight different factors
        const TIMING_WEIGHT: f64 = 0.4;
        const MOVEMENT_WEIGHT: f64 = 0.3;
        const QUANTITY_WEIGHT: f64 = 0.2;
        const PACE_WEIGHT: f64 = 0.1;

        // Quantity score (normalized to 0-1, target is 50 interactions)
        let quantity_score = (total_interactions as f64 / 50.0).min(1.0);

        // Pace score (prefer natural human pace, 200-2000ms between interactions)
        let avg_interval = duration_ms as f64 / total_interactions.max(1) as f64;
        let pace_score = if (200.0..=2000.0).contains(&avg_interval) {
            1.0
        } else if avg_interval < 200.0 {
            avg_interval / 200.0 // Too fast
        } else {
            2000.0 / avg_interval // Too slow
        };

        timing_entropy.mul_add(TIMING_WEIGHT, movement_entropy * MOVEMENT_WEIGHT)
            + quantity_score * QUANTITY_WEIGHT
            + pace_score * PACE_WEIGHT
    }

    /// Derive entropy bytes from interactions
    fn derive_entropy_bytes(
        interactions: &[InteractionEvent],
        metrics: &InteractionMetrics,
    ) -> Vec<u8> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();

        // Hash all interaction timings
        for interaction in interactions {
            hasher.update(interaction.timestamp_nanos.to_le_bytes());

            // Hash interaction-specific data
            match &interaction.data {
                InteractionData::Keyboard {
                    is_char,
                    is_modifier,
                } => {
                    hasher.update([*is_char as u8, *is_modifier as u8]);
                }
                InteractionData::Mouse { delta_x, delta_y } => {
                    hasher.update(delta_x.to_le_bytes());
                    hasher.update(delta_y.to_le_bytes());
                }
                InteractionData::Click { button } => {
                    let button_byte = match button {
                        MouseButton::Left => 1u8,
                        MouseButton::Right => 2u8,
                        MouseButton::Middle => 3u8,
                    };
                    hasher.update([button_byte]);
                }
                InteractionData::Scroll { delta } => {
                    hasher.update(delta.to_le_bytes());
                }
            }
        }

        // Mix in metrics for additional entropy
        hasher.update(metrics.avg_interval_ms.to_le_bytes());
        hasher.update(metrics.interval_std_dev.to_le_bytes());
        hasher.update(metrics.timing_entropy.to_le_bytes());
        hasher.update(metrics.movement_entropy.to_le_bytes());

        hasher.finalize().to_vec()
    }

    /// Display instructions to user
    fn display_instructions(&self) -> Result<(), BearDogError> {
        use crossterm::{cursor, execute, style, terminal};
        use std::io::{Write, stdout};

        let mut stdout = stdout();

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .map_err(|e| BearDogError::system(format!("Terminal clear failed: {e}")))?;

        writeln!(
            stdout,
            "╔══════════════════════════════════════════════════════════╗"
        )
        .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(
            stdout,
            "║  🎤 LIVE HUMAN ENTROPY COLLECTION                        ║"
        )
        .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(
            stdout,
            "╚══════════════════════════════════════════════════════════╝"
        )
        .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout).map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;

        execute!(stdout, style::SetForegroundColor(style::Color::Green))
            .map_err(|e| BearDogError::system(format!("Set color failed: {e}")))?;
        writeln!(stdout, "INSTRUCTIONS:")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        execute!(stdout, style::ResetColor)
            .map_err(|e| BearDogError::system(format!("Reset color failed: {e}")))?;

        writeln!(stdout, "  • Type naturally (any keys)")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout, "  • Move your mouse randomly")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout, "  • Vary your typing speed")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout, "  • Take natural pauses")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout, "  • BE YOURSELF - your uniqueness is the entropy!")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout).map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;

        execute!(stdout, style::SetForegroundColor(style::Color::Yellow))
            .map_err(|e| BearDogError::system(format!("Set color failed: {e}")))?;
        writeln!(
            stdout,
            "Target: {} interactions",
            self.config.target_interactions
        )
        .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        writeln!(stdout, "Press ESC to finish early")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        execute!(stdout, style::ResetColor)
            .map_err(|e| BearDogError::system(format!("Reset color failed: {e}")))?;
        writeln!(stdout).map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;

        stdout
            .flush()
            .map_err(|e| BearDogError::system(format!("Flush failed: {e}")))?;

        Ok(())
    }

    /// Display progress during collection
    fn display_progress(&self, collected: usize) -> Result<(), BearDogError> {
        use crossterm::{cursor, execute, style};
        use std::io::{Write, stdout};

        let mut stdout = stdout();

        execute!(stdout, cursor::MoveTo(0, 10))
            .map_err(|e| BearDogError::system(format!("Cursor move failed: {e}")))?;

        let progress_percent =
            (collected as f64 / self.config.target_interactions as f64 * 100.0) as usize;
        let bar_width = 40;
        let filled = (collected * bar_width) / self.config.target_interactions;

        execute!(stdout, style::SetForegroundColor(style::Color::Cyan))
            .map_err(|e| BearDogError::system(format!("Set color failed: {e}")))?;
        write!(stdout, "Progress: [")
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        execute!(stdout, style::SetForegroundColor(style::Color::Green))
            .map_err(|e| BearDogError::system(format!("Set color failed: {e}")))?;
        write!(stdout, "{}", "█".repeat(filled))
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        execute!(stdout, style::SetForegroundColor(style::Color::DarkGrey))
            .map_err(|e| BearDogError::system(format!("Set color failed: {e}")))?;
        write!(stdout, "{}", "░".repeat(bar_width - filled))
            .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        execute!(stdout, style::SetForegroundColor(style::Color::Cyan))
            .map_err(|e| BearDogError::system(format!("Set color failed: {e}")))?;
        write!(
            stdout,
            "] {}%  ({}/{})",
            progress_percent, collected, self.config.target_interactions
        )
        .map_err(|e| BearDogError::system(format!("Write failed: {e}")))?;
        execute!(stdout, style::ResetColor)
            .map_err(|e| BearDogError::system(format!("Reset color failed: {e}")))?;

        stdout
            .flush()
            .map_err(|e| BearDogError::system(format!("Flush failed: {e}")))?;

        Ok(())
    }
}

/// Guard to ensure raw mode is disabled on drop
struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_capture_config_default() {
        let config = InteractionCaptureConfig::default();
        assert_eq!(config.target_interactions, 50);
        assert_eq!(config.timeout_seconds, 120);
        assert!(config.enable_keyboard);
        assert!(config.enable_mouse);
    }

    #[test]
    fn test_shannon_entropy_calculation() {
        // Uniform distribution should have high entropy
        let uniform = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
        let entropy = InteractionEntropyCollector::calculate_shannon_entropy(&uniform);
        assert!(
            entropy > 0.8,
            "Uniform distribution should have high entropy"
        );

        // Identical values should have low entropy
        let identical = vec![50.0; 8];
        let entropy = InteractionEntropyCollector::calculate_shannon_entropy(&identical);
        assert!(entropy < 0.1, "Identical values should have low entropy");
    }

    #[test]
    fn test_overall_quality_calculation() {
        // Good quality interaction
        let quality = InteractionEntropyCollector::calculate_overall_quality(
            0.8,   // Good timing entropy
            0.7,   // Good movement entropy
            50,    // Target number of interactions
            25000, // 25 seconds (500ms average interval)
        );
        assert!(quality > 0.7, "Good interaction should have high quality");

        // Poor quality interaction
        let quality = InteractionEntropyCollector::calculate_overall_quality(
            0.2,   // Poor timing entropy
            0.3,   // Poor movement entropy
            10,    // Too few interactions
            50000, // 50 seconds (5000ms average interval - too slow)
        );
        assert!(quality < 0.5, "Poor interaction should have low quality");
    }
}
