use beardog_adapters::universal::UniversalAdapter;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityRequest;

#[cfg(test)]
mod storage_capability_integration_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_storage_adapter_initialization() -> Result<(), BearDogError> {
        let adapter = UniversalAdapter::new("storage_capability")?;

        assert!(adapter.is_initialized());
        assert_eq!(adapter.adapter_name(), "storage_capability");

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_capability_discovery() -> Result<(), BearDogError> {
        let adapter = UniversalAdapter::new("storage_capability")?;

        let capabilities = adapter.discover_capabilities()?;
        assert!(!capabilities.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_adapter_communication() -> Result<(), BearDogError> {
        let adapter = UniversalAdapter::new("storage_capability")?;

        let request = CapabilityRequest::new("test_capability", serde_json::json!({}));
        let response = adapter.execute_capability(request)?;

        assert!(response.is_success());

        Ok(())
    }
}
