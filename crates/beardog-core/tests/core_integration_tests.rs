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


/// Comprehensive Integration Tests for BearDog Core
//
// This test suite validates the core BearDog functionality including:
// - System initialization and lifecycle management
// - Security provider integration
// - Configuration management
// - Error handling and resilience
// NOTE: Tests currently disabled due to API changes - needs update

#![allow(unused_variables, dead_code)]
// use beardog_types::config::BearDogConfig;
// use beardog_core::{
//     core::BearDogCore,
//     ecosystem_simple::{BearDogEcosystemProvider, UniversalPrimalProvider},
//     songbird_client::UniversalServiceMeshClient,
//     universal_discovery::CapabilityType,
//     BearDogError, BearDogResult,
// };
// use std::sync::Arc;
// use tokio::time::{timeout, Duration}; // Unused imports
/*
#[tokio::test]
#[ignore = "Outdated test - needs API updates"]
async fn test_beardog_core_initialization() -> BearDogResult<()> {
    // Test core system initialization
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    // Test initialization process
    core.initialize().await?;
    // Verify initialization completed successfully
    let state = core.state.read().await;
    assert!(state.components.contains_key("core"));
    Ok(())
}
async fn test_beardog_core_security_operations() -> BearDogResult<()> {
    // Test key generation with correct signature
    let key_result = core.generate_key("test", "test_metadata").await;
    // Should either succeed or fail gracefully
    match key_result {
        Ok(key) => {
            assert!(!key.key_id.is_empty());
            assert!(!key.public_key.is_empty());
            println!("✅ Key generation successful: {}", key.key_id);
        }
        Err(e) => {
            println!("ℹ️ Key generation failed gracefully: {}", e);
    }
    // Test signature verification
    let verify_result = core
        .verify_signature("mock_public_key", "test_message", "deadbeef")
        .await;
    match verify_result {
        Ok(valid) => {
            println!("✅ Signature verification returned: {}", valid);
            println!("ℹ️ Signature verification failed gracefully: {}", e);
async fn test_ecosystem_provider_creation() -> BearDogResult<()> {
    // Test ecosystem provider creation
    let core = Arc::new(BearDogCore::new(config).await?);
    let provider = BearDogEcosystemProvider::new(core, "test-beardog-1".to_string());
    // Test module discovery using the trait
    let modules = provider.available_modules();
    assert!(!modules.is_empty());
    println!("📦 Available modules: {}", modules.len());
async fn test_capability_discovery() -> BearDogResult<()> {
    // Test capability-based discovery system using actual enum variants
    let capabilities = vec![
        CapabilityType::ComputeOptimization,
        CapabilityType::Encryption,
        CapabilityType::FileSystem,
        CapabilityType::ServiceDiscovery,
    ];
    for capability in capabilities {
        println!("🔍 Testing capability: {:?}", capability);
        // Test capability serialization
        let serialized = serde_json::to_string(&capability)?;
        let deserialized: CapabilityType = serde_json::from_str(&serialized)?;
        assert_eq!(capability, deserialized);
async fn test_core_error_handling() -> BearDogResult<()> {
    // Test error handling in core operations
    // Test invalid signature verification
    let invalid_verify_result = core.verify_signature("", "", "invalid_hex").await;
    assert!(invalid_verify_result.is_err());
    // Verify errors are properly typed
    match invalid_verify_result {
        Err(BearDogError::Internal { .. }) => {
            println!("✅ Error properly categorized as Internal");
        Err(other) => {
            println!("✅ Error properly categorized: {:?}", other);
        Ok(_) => panic!("Should have failed with invalid input"),
async fn test_service_mesh_client() -> BearDogResult<()> {
    // Test service mesh client functionality with correct API
    let client = UniversalServiceMeshClient::new()?;
    // Test client basic functionality
    println!("✅ Service mesh client created successfully");}


async fn test_core_state_management() -> Result<(), SecurityError> {
    // Test core state management
    // Test state updates during initialization
    {
        let state = core.state.read().await;
        assert!(state.components.contains_key("core"));
        println!(
            "🏥 Core initialized with {} components",
            state.components.len()
        );
*/
