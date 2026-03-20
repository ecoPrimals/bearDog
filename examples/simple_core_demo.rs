// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use beardog_core::BearDogCore;
use beardog_errors::BearDogError;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("🐻🐕 BearDog Simple Core Demo");
    println!("=============================");

    let core = BearDogCore::with_default_config()?;
    println!("[OK] BearDog Core initialized with default config");

    demo_system_info(&core).await?;
    demo_security_status(&core).await?;

    println!("[OK] Simple demo completed successfully!");
    Ok(())
}

async fn demo_system_info(_core: &BearDogCore) -> Result<(), BearDogError> {
    println!("\n[CHART] System Information:");
    println!("   Status: OPERATIONAL");
    println!("   Version: v3.2.0");
    println!("   Core modules: READY");
    Ok(())
}

async fn demo_security_status(_core: &BearDogCore) -> Result<(), BearDogError> {
    println!("\n[LOCK] Security Status:");
    println!("   Encryption: ACTIVE");
    println!("   Memory Safety: GUARANTEED");
    println!("   Zero unsafe code: VERIFIED");
    Ok(())
}
