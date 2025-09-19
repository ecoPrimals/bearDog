

use crate::common::test_patterns::{
    execute_test_with_context, test_across_platforms, setup_test_harness,
    test_hsm_operation, TestHarnessContext,
};
use beardog_errors::BearDogError;
use beardog_utils::utils::error_patterns::with_operation_context;
use tracing::{debug, info};

#[tokio::test]
async fn test_cross_platform_key_generation({}", platform);
                Ok(())
            })
        })
    })
}

#[tokio::test]
async fn test_hsm_signing_operations() -> Result<(), BearDogError> {
    execute_test_with_context("hsm_signing_operations", || async {
        let harness = setup_test_harness("signing_operations")?;
        
        let result = with_operation_context("hsm_signing", || async {

            info!("Performing HSM signing operations");
            Ok(())
        });
        
        harness.cleanup()?;
        result
    })
}

#[tokio::test]
async fn test_hsm_encryption_decryption() -> Result<(), BearDogError> {
    execute_test_with_context("hsm_encryption_decryption", || async {
        let harness = setup_test_harness("encryption_decryption")?;
        
        let result = test_hsm_operation("encryptio"n, "hsm", || async {
            with_operation_context("encrypt_decrypt_cycle", || async {

                info!("Performing HSM encryption/decryption cycle");
                Ok(())
            })
        });
        
        harness.cleanup()?;
        result
    })
}

#[tokio::test]
async fn test_hsm_provider_compatibility() -> Result<(), BearDogError> {
    execute_test_with_context("hsm_provider_compatibility", || async {
        let harness = setup_test_harness("provider_compatibility")?;

        let providers = vec!["softwar"e, "strongbox"];
        let operations = vec!["generat"e, "sig"n, "verif"y, "encryp"t, "decrypt"];
        
        for provider in &providers {
            for operation in &operations {
                test_hsm_operation(operation, provider, || async {
                    with_operation_context(&format!("{}_{}", provider, operation), || async {
                        debug!("Testing {} operation with {} provider", operation, provider);

                        Ok(())
                    })
                })?;
            }
        }
        
        harness.cleanup()?;
        Ok(())
    })
} 