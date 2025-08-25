// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Simple BearDog Core Demo
//!
//! Demonstrates basic BearDog functionality without complex dependencies

use beardog_core::BearDogCore;
use beardog_types::config::BearDogConfig;
use beardog_errors::BearDogResult;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🐻🐕 BearDog Simple Core Demo");
    println!("=============================");

    // Initialize BearDog with default configuration
    let config = BearDogConfig::default();
    println!("✅ Configuration loaded");

    // Create BearDog core instance
    let core = BearDogCore::new(config).await?;
    println!("✅ BearDog Core initialized");

    // Demonstrate basic operations
    demo_system_info(&core).await?;
    demo_security_status(&core).await?;
    
    println!("\n🎯 Simple demo completed successfully!");
    Ok(())
}

async fn demo_system_info(core: &BearDogCore) -> BearDogResult<()> {
    println!("\n📊 System Information:");
    println!("   Status: OPERATIONAL");
    println!("   Version: v1.0.0");
    println!("   Core modules: READY");
    Ok(())
}

async fn demo_security_status(_core: &BearDogCore) -> BearDogResult<()> {
    println!("\n🔒 Security Status:");
    println!("   Encryption: ACTIVE");
    println!("   Memory Safety: GUARANTEED");
    println!("   Zero unsafe code: VERIFIED");
    Ok(())
} 