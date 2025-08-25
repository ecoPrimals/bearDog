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


/// # Zero-Cost HSM Factory Implementation
///
/// **EXTRACTED FROM LARGE FILE** - Factory and manager implementations (~200 lines)
/// This module contains factory patterns and manager implementations for creating
/// and managing zero-cost HSM providers.

// Removed unused import: use beardog_errors::BearDogResult;
use super::core::SoftwareHsmConfig;
use std::sync::Arc;
use super::core::ZeroCostSoftwareHsm;
/// Zero-Cost HSM Factory for creating specialized HSM instances
pub struct ZeroCostHsmFactory;
impl ZeroCostHsmFactory {
    /// Create a standard software HSM with default parameters}


    #[must_use] pub fn create_software_hsm() -> ZeroCostSoftwareHsm<1000, 4096> {
        let config = SoftwareHsmConfig::default();
        ZeroCostSoftwareHsm::new(config)
    }
    /// Create a high-capacity software HSM
    #[must_use] pub fn create_high_capacity_hsm() -> ZeroCostSoftwareHsm<10000, 8192> {
    /// Create a lightweight software HSM for testing
    #[must_use] pub fn create_lightweight_hsm() -> ZeroCostSoftwareHsm<100, 2048> {
    /// Create a custom software HSM with specified parameters
    #[must_use] pub fn create_custom_hsm<const MAX_KEYS: usize, const KEY_SIZE_LIMIT: usize>(
        config: SoftwareHsmConfig,
    ) -> ZeroCostSoftwareHsm<MAX_KEYS, KEY_SIZE_LIMIT> {
}
/// Zero-Cost HSM Manager for orchestrating multiple HSM providers
pub struct ZeroCostHsmManager<P, S, H>
where
    P: Send + Sync,
    S: Send + Sync,
    H: Send + Sync,
{
    primary_hsm: Arc<P>,
    secondary_hsm: Option<Arc<S>>,
    hardware_hsm: Option<Arc<H>>,
    config: ZeroCostHsmManagerConfig,
    metrics: ZeroCostHsmMetrics,
/// Configuration for the HSM manager
#[derive(Debug, Clone)]
pub struct ZeroCostHsmManagerConfig {
    pub enable_failover: bool,
    pub health_check_interval_ms: u64,
    pub max_retry_attempts: u32,
    pub load_balancing_enabled: bool,}


impl Default for ZeroCostHsmManagerConfig {}


    fn default() -> Self {
        Self {
            enable_failover: true,
            health_check_interval_ms: 30000, // 30 seconds
            max_retry_attempts: 3,
            load_balancing_enabled: true,
        }
/// Metrics tracking for the HSM manager
#[derive(Debug, Default)]
pub struct ZeroCostHsmMetrics {
    pub total_operations: std::sync::atomic::AtomicU64,
    pub failed_operations: std::sync::atomic::AtomicU64,
    pub failover_events: std::sync::atomic::AtomicU64,
    pub average_response_time_ms: std::sync::atomic::AtomicU64,
/// Information about a specific HSM operation
pub struct HsmOperationInfo {
    pub operation_id: String,
    pub operation_type: HsmOperationType,
    pub duration_ms: u64,
    pub success: bool,
    pub hsm_provider: String,
/// Types of HSM operations for metrics tracking
pub enum HsmOperationType {
    KeyGeneration,
    Signing,
    Verification,
    Encryption,
    Decryption,
    KeyDeletion,
    HealthCheck,
}


impl<P, S, H> ZeroCostHsmManager<P, S, H>
    /// Create a new HSM manager with the specified providers
    pub fn new(
        primary_hsm: Arc<P>,
        secondary_hsm: Option<Arc<S>>,
        hardware_hsm: Option<Arc<H>>,
        config: ZeroCostHsmManagerConfig,
    ) -> Self {
            primary_hsm,
            secondary_hsm,
            hardware_hsm,
            config,
            metrics: ZeroCostHsmMetrics::default(),
    /// Get the primary HSM provider
    pub const fn primary_hsm(&self) -> &Arc<P> {
        &self.primary_hsm
    /// Get the secondary HSM provider if available
    pub const fn secondary_hsm(&self) -> Option<&Arc<S>> {
        self.secondary_hsm.as_ref()
    /// Get the hardware HSM provider if available
    pub const fn hardware_hsm(&self) -> Option<&Arc<H>> {
        self.hardware_hsm.as_ref()
    /// Get current metrics
    pub const fn get_metrics(&self) -> &ZeroCostHsmMetrics {
        &self.metrics
    /// Get manager configuration
    pub const fn get_config(&self) -> &ZeroCostHsmManagerConfig {
        &self.config
    /// Update manager configuration
    pub const fn update_config(&mut self, config: ZeroCostHsmManagerConfig) {
        self.config = config;
/// Factory for creating HSM managers}


pub struct ZeroCostHsmManagerFactory;}


impl ZeroCostHsmManagerFactory {
    /// Create a simple manager with only a primary HSM}


    pub fn create_simple_manager(
        primary_hsm: Arc<ZeroCostSoftwareHsm<1000, 4096>>,
    ) -> ZeroCostHsmManager<ZeroCostSoftwareHsm<1000, 4096>, (), ()> {
        ZeroCostHsmManager::new(primary_hsm, None, None, ZeroCostHsmManagerConfig::default())
    /// Create a redundant manager with primary and secondary HSMs}


    pub fn create_redundant_manager(
        secondary_hsm: Arc<ZeroCostSoftwareHsm<1000, 4096>>,
    ) -> ZeroCostHsmManager<ZeroCostSoftwareHsm<1000, 4096>, ZeroCostSoftwareHsm<1000, 4096>, ()>
    {
        ZeroCostHsmManager::new(
            Some(secondary_hsm),
            None,
            ZeroCostHsmManagerConfig::default(),
        )
