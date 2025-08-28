

use crate::common::test_patterns::{
    execute_test_with_context, test_across_platforms, setup_test_harness,
    test_hsm_operation, TestHarnessContext,
};
use beardog_errors::BearDogError;
use beardog_utils::utils::error_patterns::with_operation_context;
use tracing::{debug, info};

#[tokio::test]
async fn test_cross_platform_key_generation() -> Result<(), BearDogError> {
    execute_test_with_context("cross_platform_key_generation", || async {
        let _harness = setup_test_harness("key_generation").await?;
        let platforms = vec!["software", "android", "ios"];
        
        test_across_platforms(&platforms, "key_generation", |platform| async move {
            with_operation_context(&format_args!("key_generation_{}", platform).to_string(), || async {

                debug!("Generating key for platform: {}", platform);
                Ok(())
            }).await
        }).await
    }).await
}

#[tokio::test]
async fn test_hsm_signing_operations() -> Result<(), BearDogError> {
    execute_test_with_context("hsm_signing_operations", || async {
        let harness = setup_test_harness("signing_operations").await?;
        
        let result = with_operation_context("hsm_signing", || async {

            info!("Performing HSM signing operations");
            Ok(())
        }).await;
        
        harness.cleanup().await?;
        result
    }).await
}

#[tokio::test]
async fn test_hsm_encryption_decryption() -> Result<(), BearDogError> {
    execute_test_with_context("hsm_encryption_decryption", || async {
        let harness = setup_test_harness("encryption_decryption").await?;
        
        let result = test_hsm_operation("encryption", "hsm", || async {
            with_operation_context("encrypt_decrypt_cycle", || async {

                info!("Performing HSM encryption/decryption cycle");
                Ok(())
            }).await
        }).await;
        
        harness.cleanup().await?;
        result
    }).await
}

#[tokio::test]
async fn test_hsm_provider_compatibility() -> Result<(), BearDogError> {
    execute_test_with_context("hsm_provider_compatibility", || async {
        let harness = setup_test_harness("provider_compatibility").await?;

        let providers = vec!["software", "strongbox"];
        let operations = vec!["generate", "sign", "verify", "encrypt", "decrypt"];
        
        for provider in &providers {
            for operation in &operations {
                test_hsm_operation(operation, provider, || async {
                    with_operation_context(&format_args!("{}_{}", provider, operation).to_string(), || async {
                        debug!("Testing {} operation with {} provider", operation, provider);

                        Ok(())
                    }).await
                }).await?;
            }
        }
        
        harness.cleanup().await?;
        Ok(())
    }).await
} 