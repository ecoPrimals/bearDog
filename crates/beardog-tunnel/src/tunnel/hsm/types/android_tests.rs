// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[tokio::test]
async fn rejects_empty_key_id_on_generate() {
    let ks = AndroidKeystore::with_stub_transport(AndroidHsmConfig::default()).expect("ks");
    let mut p = AndroidKeyParams::new();
    p.set_key_size(256);
    let err = ks.generate_key("", &p).await.expect_err("empty id");
    let msg = err.to_string();
    assert!(
        msg.contains("empty") || msg.contains("Empty"),
        "unexpected message: {msg}"
    );
}

#[tokio::test]
async fn stub_roundtrip_sign_verify_list() {
    let ks = AndroidKeystore::with_stub_transport(AndroidHsmConfig::default()).expect("ks");
    let mut p = AndroidKeyParams::new();
    p.set_purposes(vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify]);
    ks.generate_key("my-key", &p).await.expect("gen");
    let sig = ks.sign("my-key", b"hello").await.expect("sign");
    assert!(ks.verify("my-key", b"hello", &sig).await.expect("verify"));
    assert!(!ks.verify("my-key", b"other", &sig).await.expect("verify2"));
    let keys = ks.list_keys().await.expect("list");
    assert_eq!(keys.len(), 1);
    assert!(ks.key_exists("my-key").await.expect("exists"));
}

#[tokio::test]
async fn stub_health_uses_deterministic_metrics() {
    let m = AndroidHealthMonitor::with_transport(Arc::new(HealthMetricsTransportBackend::Stub(
        StubHealthMetricsTransport::default(),
    )));
    let s = m.get_health_status().await.expect("health");
    assert!(s.is_healthy);
    assert_eq!(s.performance_metrics.operations_per_second, 42.0);
}
