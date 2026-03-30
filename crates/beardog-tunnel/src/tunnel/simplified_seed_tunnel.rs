// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::*;

pub struct SimplifiedSeedTunnel;

impl SimplifiedSeedTunnel {
    /// Creates a new instance
    ///
    /// # Errors
    ///
    /// Returns an error if the tunnel cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
}
