// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

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
