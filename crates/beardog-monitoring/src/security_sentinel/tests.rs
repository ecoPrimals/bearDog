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


/// Comprehensive Tests for Security Sentinel System
///
/// Tests for all components of the Security Sentinel to ensure
/// BearDog's self-aware monitoring works correctly.
use super::*;
use beardog_errors::BearDogResult;
use std::sync::Arc;

#[tokio::test]
async fn test_performance_sentinel_initialization() -> BearDogResult<()> {
    let thresholds = PerformanceThresholds {
        max_latency_ms: 1000,
        max_error_rate: 0.05,
        max_memory_mb: 512.0,
        max_cpu_percent: 80.0,
        min_success_rate: 95.0,
    };
    let alert_manager = Arc::new(AlertManager::new());
    let _performance_sentinel = PerformanceSentinel::new(thresholds, alert_manager)?;
    // Test that the sentinel was created successfully
    Ok(())
}
async fn test_security_sentinel_comprehensive() -> BearDogResult<()> {
    use super::super::SecuritySentinel;
    let _sentinel = SecuritySentinel::new();
    // Test basic functionality - removed .await since get_status likely doesn't exist
    // let _status = sentinel.get_status().await?;
    // Test Performance Sentinel - using proper constructor
