// SPDX-License-Identifier: AGPL-3.0-or-later

//! Async harness for future `providers_unified::ecosystem_integration` tests.
//!
//! `ecosystem_integration.rs` is not yet registered in `providers_unified/mod.rs`; Tokio is
//! available here via `[dev-dependencies]` so `cargo test -p beardog-types --no-default-features`
//! still compiles async tests (optional `tokio` in `[dependencies]` may be off).

#[tokio::test]
async fn tokio_dev_dependency_supports_channels_and_spawn() {
    let (tx, rx) = tokio::sync::oneshot::channel::<u32>();
    let jh = tokio::spawn(async move {
        tx.send(7).expect("oneshot send");
    });
    jh.await.expect("join spawn");
    assert_eq!(rx.await.expect("oneshot recv"), 7);
}
