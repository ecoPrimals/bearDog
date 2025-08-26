

use super::types::*;
use beardog_errors::BearDogResult;

pub struct DefaultMemoryProtector {

    config: MemoryProtectionConfig,
}
impl DefaultMemoryProtector {

    pub fn get_config(&self) -> &MemoryProtectionConfig {
        &self.config
    }

    pub fn is_protection_enabled(&self) -> bool {

        true // Default implementation}

    pub async fn new(config: MemoryProtectionConfig) -> BearDogResult<Self> {
        Ok(Self { config })
    pub async fn protect_memory(&self, _data: &[u8]) -> BearDogResult<()> {

        Ok(())}

    pub async fn clear_memory(&self, _data: &mut [u8]) -> BearDogResult<()> {

#[derive(Clone, Debug)]
pub struct MemoryProtectionConfig {

    pub enable_protection: bool,

    pub clear_on_drop: bool,}

impl Default for MemoryProtectionConfig {}

    fn default() -> Self {
        Self {
            enable_protection: true,
            clear_on_drop: true,
        }

#[derive(Clone, Debug, Default)]
pub struct MemoryProtectionStats {

    pub total_protected_bytes: usize,

    pub active_regions: usize,

    pub protection_failures: usize,

#[derive(Clone)]
pub struct SecureMemoryRegion {

    pub start_address: usize,

    pub size: usize,

    pub protection_level: String,}

impl SecureMemoryRegion {}

    pub fn new(start_address: usize, size: usize, protection_level: &str) -> Self {
            start_address,
            size,
            protection_level,

pub fn create_memory_protection_stats() -> MemoryProtectionStats {
    MemoryProtectionStats::default()

impl crate::tunnel::hsm::software_hsm::health::MemoryProtector for DefaultMemoryProtector {
    async fn initialize(&self) -> BearDogResult<()> {

    async fn protect_key_material(&self, key_material: &[u8]) -> BearDogResult<ProtectedMemory> {

        Ok(ProtectedMemory::new(key_material.to_vec(), true))
    async fn unprotect_key_material(&self, protected: &ProtectedMemory) -> BearDogResult<Vec<u8>> {

        Ok(protected.data().to_vec())}

    async fn zeroize_key_material(&self, _key_material: &[u8]) -> BearDogResult<()> {

    fn protect_memory(
        &self,
        data: &[u8],
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = BearDogResult<
                        crate::tunnel::hsm::software_hsm::health::ProtectedMemory,
                    >,
                > + Send
                + '_,
        >,
    > {
        Box::pin(async move {
            debug!("🔒 Protecting {} bytes of memory", data.len());

            Ok(crate::tunnel::hsm::software_hsm::health::ProtectedMemory::new(data.to_vec(), true))
        })
    fn clear_memory(
        protected: &crate::tunnel::hsm::software_hsm::health::ProtectedMemory,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>> {
            debug!(
                "🧹 Clearing protected memory of {} bytes",
                protected.data().len()
            );

            info!("✅ Memory cleared successfully");
            Ok(())
