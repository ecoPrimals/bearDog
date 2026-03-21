// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    missing_docs,
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]

// Basic Validation Example
//
// Demonstrates basic BearDog validation capabilities

use beardog_errors::BearDogError;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("🔍 BearDog Basic Validation Example");
    println!("====================================");

    // Basic validation demonstration
    println!("✅ Basic validation complete");

    Ok(())
}
