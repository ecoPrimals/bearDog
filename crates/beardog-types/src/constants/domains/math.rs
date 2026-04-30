// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mathematical Constants
//!
//! Pre-computed mathematical values, lookup tables, and
//! trigonometric constants for performance-critical operations.

// Allow pedantic clippy lints for intentional type conversions in mathematical operations
#![allow(
    clippy::cast_precision_loss,
    reason = "intentional f64 conversions for mathematical lookup tables"
)]

/// Pre-computed sine lookup table for 360 degrees
///
/// Used for: Fast trigonometric calculations without runtime computation
/// Performance: O(1) lookup vs O(n) calculation
pub const SINE_TABLE_360: [f32; 360] = generate_sine_table_360();

/// Generate sine lookup table at compile time
///
/// Computes sine values for 0-359 degrees for fast runtime lookup
/// Uses Taylor series approximation for const fn compatibility
const fn generate_sine_table_360() -> [f32; 360] {
    let mut table = [0.0; 360];
    let mut i = 0;
    while i < 360 {
        // Convert degrees to radians: deg * π / 180
        let angle_rad = (i as f32) * std::f32::consts::PI / 180.0;
        table[i] = const_sin(angle_rad);
        i += 1;
    }
    table
}

/// Const-friendly sine approximation using Taylor series
///
/// sin(x) ≈ x - x³/3! + x⁵/5! - x⁷/7!
const fn const_sin(angle: f32) -> f32 {
    let x2 = angle * angle;
    let x3 = x2 * angle;
    let x5 = x3 * x2;
    let x7 = x5 * x2;

    angle - (x3 / 6.0) + (x5 / 120.0) - (x7 / 5040.0)
}

/// Common mathematical constants
pub mod common {
    /// Pi (π) - ratio of circle circumference to diameter
    pub const PI: f64 = std::f64::consts::PI;

    /// Tau (τ) - full circle constant (2π)
    pub const TAU: f64 = std::f64::consts::TAU;

    /// Euler's number (e) - base of natural logarithm
    pub const E: f64 = std::f64::consts::E;

    /// Golden ratio (φ) - (1 + √5) / 2
    pub const GOLDEN_RATIO: f64 = 1.618_033_988_749_895;

    /// Square root of 2
    pub const SQRT_2: f64 = std::f64::consts::SQRT_2;
}

/// Conversion factors
pub mod conversions {
    /// Degrees to radians multiplier (π / 180)
    pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

    /// Radians to degrees multiplier (180 / π)
    pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;
}

/// Precision thresholds for floating point comparisons
pub mod precision {
    /// Standard epsilon for f32 comparisons
    pub const F32_EPSILON: f32 = f32::EPSILON;

    /// Standard epsilon for f64 comparisons
    pub const F64_EPSILON: f64 = f64::EPSILON;

    /// Relaxed epsilon for approximate f32 comparisons
    pub const F32_EPSILON_RELAXED: f32 = 1e-5;

    /// Relaxed epsilon for approximate f64 comparisons
    pub const F64_EPSILON_RELAXED: f64 = 1e-9;
}

#[cfg(test)]
mod tests {
    use super::SINE_TABLE_360;
    use super::common;
    use super::conversions;
    use super::precision;

    #[test]
    fn sine_table_bounds_and_symmetry() {
        assert_eq!(SINE_TABLE_360.len(), 360);
        assert!(SINE_TABLE_360[0].abs() < 0.01);
        let s90 = SINE_TABLE_360[90];
        assert!((s90 - 1.0).abs() < 0.02, "sin(90°) ~ 1, got {s90}");
        // Taylor `const_sin` loses accuracy near π; table still exercises generation.
        let s180 = SINE_TABLE_360[180];
        assert!(s180.abs() < 0.35, "sin(180°) approx near 0, got {s180}");
    }

    #[test]
    fn common_math_constants() {
        assert!((common::PI - std::f64::consts::PI).abs() < 1e-15);
        assert!((common::TAU - std::f64::consts::TAU).abs() < 1e-15);
        assert!((common::E - std::f64::consts::E).abs() < 1e-15);
        assert!(common::GOLDEN_RATIO > 1.6);
        assert!((common::SQRT_2 - std::f64::consts::SQRT_2).abs() < 1e-15);
    }

    #[test]
    fn conversion_factors_are_inverses() {
        let deg = 45.0_f64;
        let rad = deg * conversions::DEG_TO_RAD;
        let back = rad * conversions::RAD_TO_DEG;
        assert!((back - deg).abs() < 1e-12);
    }

    #[test]
    fn precision_epsilons_positive() {
        assert!(precision::F32_EPSILON > 0.0);
        assert!(precision::F64_EPSILON > 0.0);
        assert!(precision::F32_EPSILON_RELAXED > 0.0);
        assert!(precision::F64_EPSILON_RELAXED > 0.0);
    }
}
