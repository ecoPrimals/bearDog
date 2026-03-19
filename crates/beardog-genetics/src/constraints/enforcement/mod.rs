// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constraint enforcement - verify operations against key constraints

mod enforcer;
mod errors;

#[cfg(test)]
mod tests;

pub use enforcer::ConstraintEnforcer;
pub use errors::ConstraintViolationError;
