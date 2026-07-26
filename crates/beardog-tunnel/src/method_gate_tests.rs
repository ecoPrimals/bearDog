// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for [`crate::method_gate`] — classification, gate enforcement,
//! auth handlers, and dispatch routing.

mod method_gate_test_helpers;

#[path = "method_gate_classification_tests.rs"]
mod classification_tests;

#[path = "method_gate_enforcement_tests.rs"]
mod enforcement_tests;

#[path = "method_gate_auth_tests.rs"]
mod auth_tests;

#[path = "method_gate_dispatch_tests.rs"]
mod dispatch_tests;
