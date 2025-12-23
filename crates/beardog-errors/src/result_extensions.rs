use crate::BearDogError;
use std::fmt::Display;

pub trait ResultValidationExt<T> {
    /// Creates instance with validation context
    fn with_validation_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with internal context
    fn with_internal_context(self, context: &str) -> Result<T, BearDogError>;
    /// Creates instance with operation context
    fn with_operation_context(self, operation: &str) -> Result<T, BearDogError>;
    /// Creates instance with component context
    fn with_component_context(self, component: &str) -> Result<T, BearDogError>;
    /// Creates instance with detailed context
    fn with_detailed_context<F>(self, context_fn: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultValidationExt<T> for Result<T, E>
where
    E: Display + Send + Sync + 'static,
{
    /// Creates instance with validation context
    fn with_validation_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::validation(format!("{context}: {e}")))
    }

    /// Creates instance with internal context
    fn with_internal_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::internal(format!("{context}: {e}")))
    }

    /// Creates instance with operation context
    fn with_operation_context(self, operation: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::internal(format!("Operation '{operation}' failed: {e}")))
    }

    /// Creates instance with component context
    fn with_component_context(self, component: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::internal(format!("Component '{component}' error: {e}")))
    }

    /// Creates instance with detailed context
    fn with_detailed_context<F>(self, context_fn: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| BearDogError::internal(format!("{}: {e}", context_fn())))
    }
}

pub trait OptionValidationExt<T> {
    fn ok_or_validation_error(self, message: &str) -> Result<T, BearDogError>;
    fn ok_or_not_found(self, item_type: &str) -> Result<T, BearDogError>;
    fn ok_or_missing_required(self, field_name: &str) -> Result<T, BearDogError>;
}

impl<T> OptionValidationExt<T> for Option<T> {
    fn ok_or_validation_error(self, message: &str) -> Result<T, BearDogError> {
        self.ok_or_else(|| BearDogError::validation(message.to_string()))
    }


    fn ok_or_not_found(self, item_type: &str) -> Result<T, BearDogError> {
        self.ok_or_else(|| BearDogError::internal(format!("{item_type} not found")))
    }


    fn ok_or_missing_required(self, field_name: &str) -> Result<T, BearDogError> {
        self.ok_or_else(|| {
            BearDogError::validation(format!("Required field '{field_name}' is missing"))
        })
    }
}

pub trait ErrorChainExt {
    fn chain_with_context(self, context: &str) -> BearDogError;
    fn chain_with_lazy_context<F>(self, context_fn: F) -> BearDogError
    where
        F: FnOnce() -> String;
}

impl<E> ErrorChainExt for E
where
    E: Display + Send + Sync + 'static,
{
    fn chain_with_context(self, context: &str) -> BearDogError {
        BearDogError::internal(format!("{context}: {self}"))
    }


    fn chain_with_lazy_context<F>(self, context_fn: F) -> BearDogError
    where
        F: FnOnce() -> String,
    {
        BearDogError::internal(format!("{}: {self}", context_fn()))
    }
}
