#![allow(unused_variables, dead_code)]
use beardog_errors::BearDogError;

/*
#[tokio::test]
#[ignore = "Outdated test - needs API updates"]
async fn test_beardog_core_initialization() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    core.initialize().await?;

    let state = core.state.read().await;
    assert!(state.components.contains_key("core"));
    Ok(())
}
async fn test_beardog_core_security_operations() -> Result<(), BearDogError> {

    let key_result = core.generate_key("test", "test_metadata").await;

    match key_result {
        Ok(key) => {
            assert!(!key.key_id.is_empty());
            assert!(!key.public_key.is_empty());
            println!("✅ Key generation successful: {}", key.key_id);
        }
        Err(e) => {
            println!("ℹ️ Key generation failed gracefully: {}", e);
    }

    let verify_result = core
        .verify_signature("mock_public_key", "test_message", "deadbeef")
        .await;
    match verify_result {
        Ok(valid) => {
            println!("✅ Signature verification returned: {}", valid);
            println!("ℹ️ Signature verification failed gracefully: {}", e);
async fn test_ecosystem_provider_creation() -> Result<(), BearDogError> {

    let core = Arc::new(BearDogCore::new(config).await?);
    let provider = BearDogEcosystemProvider::new(core, "test-beardog-1".to_string());

    let modules = provider.available_modules();
    assert!(!modules.is_empty());
    println!("📦 Available modules: {}", modules.len());
async fn test_capability_discovery() -> Result<(), BearDogError> {

    let capabilities = vec![
        CapabilityType::ComputeOptimization,
        CapabilityType::Encryption,
        CapabilityType::FileSystem,
        CapabilityType::ServiceDiscovery,
    ];
    for capability in capabilities {
        println!("🔍 Testing capability: {:?}", capability);

        let serialized = serde_json::to_vec(&capability)?;
        let deserialized: CapabilityType = serde_json::from_str(&serialized)?;
        assert_eq!(capability, deserialized);
async fn test_core_error_handling() -> Result<(), BearDogError> {

    let invalid_verify_result = core.verify_signature("", "", "invalid_hex").await;
    assert!(invalid_verify_result.is_err());

    match invalid_verify_result {
        Err(BearDogError::Internal { .. }) => {
            println!("✅ Error properly categorized as Internal");
        Err(other) => {
            println!("✅ Error properly categorized: {:?}", other);
        Ok(_) => panic!("Should have failed with invalid input"),
async fn test_service_mesh_client() -> Result<(), BearDogError> {

    let client = UniversalServiceMeshClient::new()?;

    println!("✅ Service mesh client created successfully");}

async fn test_core_state_management() -> Result<(), SecurityError> {

    {
        let state = core.state.read().await;
        assert!(state.components.contains_key("core"));
        println!(
            "🏥 Core initialized with {} components",
            state.components.len()
        );
*/
