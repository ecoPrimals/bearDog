// SPDX-License-Identifier: AGPL-3.0-only

//! Structured deployment errors via [`BearDogError`](beardog_errors::BearDogError).
//!
//! Prefer consistent message prefixes (e.g. `NDK Error:`, `Build Failed:`) when calling
//! [`BearDogError::system`](beardog_errors::BearDogError::system) so CLI output and monitoring
//! stay uniform across `beardog-deploy` and related binaries.
