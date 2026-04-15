// SPDX-License-Identifier: AGPL-3.0-or-later

//! Serialize tests that mutate [`beardog_errors::process_env`] for the same keys.

/// Guards `ECOPRIMALS_GENOME_TARGETS` overlay mutations across concurrent unit tests.
pub static ECOPRIMALS_GENOME_TARGETS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
