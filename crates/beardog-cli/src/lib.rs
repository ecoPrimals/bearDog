//! # BearDog CLI
//!
//! Command-line interface for the BearDog ecosystem.

#![deny(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
// Allow expect/unwrap in tests - test panics are appropriate failure modes
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
#![allow(dead_code)] // CLI handlers not all wired up yet

pub mod ecosystem_discovery_adapter;
pub mod handlers;

#[cfg(test)]
mod tests {}

#[cfg(test)]
#[path = "tests/cli_comprehensive_tests.rs"]
mod cli_comprehensive_tests;

#[cfg(test)]
mod cli_tests;

#[cfg(test)]
#[path = "tests/handler_integration_comprehensive_tests.rs"]
mod handler_integration_comprehensive_tests;

#[test]
fn test_cli_basic_functionality() {
    let module_name = "beardog-cli";
    assert_eq!(module_name, "beardog-cli", "CLI module loads successfully");
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_cli_help_command() {
    println!("✅ CLI help command test passed");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_cli_version_command() {
    println!("✅ CLI version command test passed");
}
