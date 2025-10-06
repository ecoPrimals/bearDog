use beardog_errors::BearDogError;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_module_integration() -> Result<(), BearDogError> {
        Ok(())
    }
}
