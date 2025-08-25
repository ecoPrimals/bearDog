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


//! HSM Manager Tests
//!
//! Tests for HSM manager functionality including tier selection and failover

use super::HsmTestHarness;
use beardog::{BearDogError, BearDogResult};

/// Test HSM manager functionality
pub async fn test_hsm_manager(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("🏗️ Testing HSM Manager Functionality");

    // Test manager initialization and configuration
    test_manager_initialization(harness).await?;
    
    // Test tier selection logic
    test_tier_selection(harness).await?;
    
    // Test failover mechanisms
    test_failover_mechanisms(harness).await?;
    
    println!("✅ HSM Manager tests completed");
    Ok(())
}

async fn test_manager_initialization(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🚀 Testing manager initialization");
    
    let start_time = std::time::Instant::now();
    
    // Test manager status
    let status = harness.hsm_manager.get_status().await?;
    assert!(status.is_healthy(), "HSM Manager should be healthy");
    
    // Test available providers
    let providers = harness.hsm_manager.list_providers().await?;
    assert!(providers.len() >= 2, "Should have at least 2 providers (StrongBox + Software)");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "manager", true);
    
    println!("    ✅ Manager initialization tests passed");
    Ok(())
}

async fn test_tier_selection(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🎯 Testing tier selection logic");
    
    let start_time = std::time::Instant::now();
    
    // Test automatic tier selection
    let selected_tier = harness.hsm_manager.select_optimal_tier().await?;
    println!("    Selected tier: {:?}", selected_tier);
    
    // Test manual tier preference
    harness.hsm_manager.set_tier_preference(selected_tier).await?;
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "manager", true);
    
    println!("    ✅ Tier selection tests passed");
    Ok(())
}

async fn test_failover_mechanisms(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔄 Testing failover mechanisms");
    
    let start_time = std::time::Instant::now();
    
    // Test failover configuration
    let failover_config = harness.hsm_manager.get_failover_config().await?;
    assert!(failover_config.enabled, "Failover should be enabled");
    
    // Test simulated failover
    let failover_result = harness.hsm_manager.test_failover().await?;
    assert!(failover_result.successful, "Failover test should succeed");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "manager", true);
    
    println!("    ✅ Failover mechanism tests passed");
    Ok(())
}

#[tokio::test]
async fn test_hsm_manager_standalone() -> BearDogResult<()> {
    let mut harness = super::HsmTestHarness::new().await?;
    test_hsm_manager(&mut harness).await
} 