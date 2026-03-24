// SPDX-License-Identifier: AGPL-3.0-only

//! Deep coverage for [`crate::result_extensions`] traits.

use crate::BearDogError;
use crate::result_extensions::{ErrorChainExt, OptionValidationExt, ResultValidationExt};

#[test]
fn with_validation_context_maps_io_err() {
    let r: Result<(), std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
    let out = r.with_validation_context("ctx");
    assert!(out.is_err());
    let e = out.expect_err("err");
    let s = e.to_string();
    assert!(s.contains("ctx"));
    assert!(s.contains("missing"));
}

#[test]
fn with_internal_context_maps_err() {
    let r: Result<i32, std::num::ParseIntError> = "x".parse();
    let out = r.map(|_| ()).with_internal_context("inner");
    assert!(out.is_err());
    assert!(out.expect_err("e").to_string().contains("inner"));
}

#[test]
fn with_operation_context_includes_operation_name() {
    let r: Result<(), std::io::Error> = Err(std::io::Error::other("io"));
    let out = r.with_operation_context("open");
    let s = out.expect_err("e").to_string();
    assert!(s.contains("open"));
    assert!(s.contains("failed"));
}

#[test]
fn with_component_context_includes_component() {
    let r: Result<(), std::io::Error> = Err(std::io::Error::other("bad"));
    let out = r.with_component_context("db");
    assert!(out.expect_err("e").to_string().contains("db"));
}

#[test]
fn with_detailed_context_invokes_closure_on_err() {
    let r: Result<(), std::io::Error> = Err(std::io::Error::other("x"));
    let out = r.with_detailed_context(|| "lazy detail".to_string());
    assert!(out.expect_err("e").to_string().contains("lazy detail"));
}

#[test]
fn with_detailed_context_ok_branch_skips_closure() {
    let r: Result<i32, std::io::Error> = Ok(7);
    let out = r.with_detailed_context(|| panic!("should not run"));
    assert_eq!(out.expect("ok"), 7);
}

#[test]
fn option_ok_or_validation_error_some() {
    let r = Some(3).ok_or_validation_error("missing");
    assert_eq!(r.expect("ok"), 3);
}

#[test]
fn option_ok_or_validation_error_none() {
    let r: Option<i32> = None;
    let out = r.ok_or_validation_error("field required");
    assert!(out.is_err());
    assert!(out.expect_err("e").to_string().contains("field required"));
}

#[test]
fn option_ok_or_not_found() {
    let r: Option<i32> = None;
    let out = r.ok_or_not_found("item");
    assert!(out.expect_err("e").to_string().contains("not found"));
}

#[test]
fn option_ok_or_missing_required() {
    let r: Option<&str> = None;
    let out = r.ok_or_missing_required("token");
    assert!(out.expect_err("e").to_string().contains("token"));
}

#[test]
fn error_chain_with_context() {
    let e = std::io::Error::new(std::io::ErrorKind::Other, "root");
    let b: BearDogError = e.chain_with_context("wrapped");
    assert!(b.to_string().contains("wrapped"));
}

#[test]
fn error_chain_with_lazy_context() {
    let e = std::io::Error::new(std::io::ErrorKind::Other, "root");
    let b: BearDogError = e.chain_with_lazy_context(|| "lazy".to_string());
    assert!(b.to_string().contains("lazy"));
}
