// SPDX-License-Identifier: AGPL-3.0-or-later

//! Identity verification handlers for the `BearDog` ecosystem.
//!
//! Each [`VerificationMethod`] maps to a handler implementing [`IdentityVerificationHandler`].
//! Production deployments register real implementations; stubs return [`BearDogError::not_yet_available`].

use super::types::{VerificationContext, VerificationMethod, VerificationResult};
use beardog_errors::BearDogError;

/// Channel-specific handler that evaluates a [`VerificationContext`] and returns a [`VerificationResult`].
pub trait IdentityVerificationHandler: Send + Sync {
    /// Verification channel this handler satisfies.
    fn method(&self) -> VerificationMethod;

    /// Perform identity verification for the supplied context.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when verification cannot be performed or the handler is unimplemented.
    fn verify(&self, context: &VerificationContext) -> Result<VerificationResult, BearDogError>;
}

/// Routes verification requests to registered channel handlers.
#[derive(Default)]
pub struct VerificationHandlerRegistry {
    handlers: Vec<Box<dyn IdentityVerificationHandler>>,
}

impl VerificationHandlerRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a handler; later registrations for the same [`VerificationMethod`] take precedence.
    pub fn register(&mut self, handler: Box<dyn IdentityVerificationHandler>) {
        self.handlers.push(handler);
    }

    /// Dispatches verification to the handler matching `method`.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError::not_yet_available`] when no handler is registered for `method`.
    pub fn verify(
        &self,
        method: &VerificationMethod,
        context: &VerificationContext,
    ) -> Result<VerificationResult, BearDogError> {
        self.handlers
            .iter()
            .rev()
            .find(|handler| {
                std::mem::discriminant(&handler.method()) == std::mem::discriminant(method)
            })
            .map_or_else(
                || {
                    Err(BearDogError::not_yet_available(format!(
                        "no verification handler registered for {method:?}"
                    )))
                },
                |handler| handler.verify(context),
            )
    }
}

/// Placeholder handler that reports its channel is not yet implemented.
pub struct StubVerificationHandler {
    method: VerificationMethod,
}

impl StubVerificationHandler {
    /// Creates a stub for the given verification channel.
    #[must_use]
    pub fn new(method: VerificationMethod) -> Self {
        Self { method }
    }
}

impl IdentityVerificationHandler for StubVerificationHandler {
    fn method(&self) -> VerificationMethod {
        self.method.clone()
    }

    fn verify(&self, _context: &VerificationContext) -> Result<VerificationResult, BearDogError> {
        Err(BearDogError::not_yet_available(format!(
            "{:?} verification is not yet implemented",
            self.method
        )))
    }
}

/// Registry pre-populated with stub handlers for every [`VerificationMethod`].
#[must_use]
pub fn default_stub_registry() -> VerificationHandlerRegistry {
    let mut registry = VerificationHandlerRegistry::new();
    for method in [
        VerificationMethod::Cryptographic,
        VerificationMethod::Biometric,
        VerificationMethod::Genetic,
        VerificationMethod::Behavioral,
        VerificationMethod::MultiFactorAuth,
        VerificationMethod::ZeroKnowledgeProof,
    ] {
        registry.register(Box::new(StubVerificationHandler::new(method)));
    }
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn empty_context() -> VerificationContext {
        VerificationContext {
            genetics: None,
            required_permission: None,
            context_data: HashMap::new(),
        }
    }

    #[test]
    fn stub_handler_returns_not_yet_available() {
        let handler = StubVerificationHandler::new(VerificationMethod::Biometric);
        let result = handler.verify(&empty_context());

        assert!(result.is_err());
        let err = result.expect_err("stub should fail");
        assert!(
            err.to_string().contains("not yet"),
            "expected not_yet_available, got: {err}"
        );
    }

    #[test]
    fn registry_routes_to_matching_handler() {
        let registry = default_stub_registry();
        let result = registry.verify(&VerificationMethod::Genetic, &empty_context());

        assert!(result.is_err());
        assert!(
            result
                .expect_err("stub registry should fail")
                .to_string()
                .contains("Genetic"),
        );
    }

    #[test]
    fn registry_reports_missing_handler() {
        let registry = VerificationHandlerRegistry::new();
        let result = registry.verify(&VerificationMethod::Cryptographic, &empty_context());

        assert!(result.is_err());
        assert!(
            result
                .expect_err("missing handler should fail")
                .to_string()
                .contains("no verification handler registered"),
        );
    }
}
