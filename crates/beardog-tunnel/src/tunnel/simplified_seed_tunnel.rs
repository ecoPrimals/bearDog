// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::*;

pub struct SimplifiedSeedTunnel;

impl SimplifiedSeedTunnel {
    /// New operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
}
