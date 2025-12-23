use beardog_errors::BearDogError;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_functionality() -> Result<(), BearDogError> {
        Ok(())
    }

    #[tokio::test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    async fn test_error_handling() -> Result<(), BearDogError> {
        Ok(())
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_configuration() -> Result<(), BearDogError> {
        Ok(())
    }
}
