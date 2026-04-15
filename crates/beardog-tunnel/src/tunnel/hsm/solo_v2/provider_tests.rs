// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_solo_v2_provider_creation() {
    let device_info = SoloV2DeviceInfo {
        device_id: "test-device".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: true,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };

    let config = SoloV2Config::default();
    let provider = SoloV2Provider::new(device_info, config);
    assert!(provider.is_ok());
}

#[test]
fn test_solo_v2_provider_info() {
    let device_info = SoloV2DeviceInfo {
        device_id: "test-device".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: true,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };

    let config = SoloV2Config::default();
    let provider = SoloV2Provider::new(device_info, config)
        .expect("connected test Solo V2 device should construct");

    let info = provider.get_provider_info();
    assert_eq!(info.provider_type, ProviderType::UsbToken);
    assert!(info.name.contains("Solo V2"));
}

#[tokio::test]
async fn test_solo_v2_pin_management() {
    let device_info = SoloV2DeviceInfo {
        device_id: "test-device".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: true,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };

    let config = SoloV2Config::default();
    let provider = SoloV2Provider::new(device_info, config)
        .expect("connected test Solo V2 device should construct");

    // Test PIN setting
    let result = provider.set_pin("123456".to_string()).await;
    assert!(result.is_ok());
}

#[test]
fn solo_v2_new_fails_when_disconnected() {
    let device_info = SoloV2DeviceInfo {
        device_id: "gone".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: false,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };
    let err = match SoloV2Provider::new(device_info, SoloV2Config::default()) {
        Ok(_) => panic!("expected disconnected device to fail"),
        Err(e) => e,
    };
    let msg = format!("{err}");
    assert!(msg.contains("not connected") || msg.contains("gone"));
}

#[tokio::test]
async fn solo_v2_sign_with_device_errors_on_unknown_key() {
    let device_info = SoloV2DeviceInfo {
        device_id: "test-device".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: true,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };
    let provider = SoloV2Provider::new(device_info, SoloV2Config::default())
        .expect("connected test Solo V2 device should construct");
    let err = provider
        .sign_with_device("missing-key", b"data")
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("not found") || msg.contains("Key"));
}

#[test]
fn solo_v2_universal_trait_directs_to_async_methods() {
    use crate::universal_hsm::traits::UniversalHsmProvider;

    let device_info = SoloV2DeviceInfo {
        device_id: "test-device".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: true,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };
    let provider = SoloV2Provider::new(device_info, SoloV2Config::default())
        .expect("connected test Solo V2 device should construct");
    assert!(
        provider
            .generate_key(crate::tunnel::hsm::types::KeyType::Ed25519)
            .is_err()
    );
    assert!(provider.sign("k", b"d").is_err());
    assert!(provider.verify("k", b"d", b"s").is_err());
    assert!(!provider.get_capabilities().is_empty());
}

#[cfg(not(feature = "ctap2"))]
#[tokio::test]
async fn solo_v2_generate_key_without_ctap2_returns_not_implemented() {
    let device_info = SoloV2DeviceInfo {
        device_id: "test-device".to_string(),
        product_name: "Solo V2 Test".to_string(),
        firmware_version: "1.0.0".to_string(),
        is_connected: true,
        vendor_id: 0x20a0,
        product_id: 0x42b2,
    };
    let provider = SoloV2Provider::new(device_info, SoloV2Config::default())
        .expect("connected test Solo V2 device should construct");
    let err = provider
        .generate_key_on_device(KeyType::Ed25519, "kid".to_string())
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("CTAP2") || msg.contains("not implemented") || msg.contains("ctap2"),
        "{msg}"
    );
}

#[cfg(feature = "ctap2")]
mod ctap2_mock_tests {
    use super::*;
    use crate::tunnel::hsm::solo_v2::ctap2_protocol::{
        CTAP2_GET_ASSERTION, CTAP2_MAKE_CREDENTIAL, CTAP2_OK, parse_get_assertion_response,
        parse_make_credential_response,
    };
    use crate::tunnel::hsm::solo_v2::transport::Ctap2Transport;
    use async_trait::async_trait;
    use ciborium::Value as CborValue;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockCtap2Transport {
        make_cred_response: Vec<u8>,
        get_assertion_response: Vec<u8>,
    }

    impl MockCtap2Transport {
        fn with_success_responses() -> Self {
            let mut auth_data = vec![0u8; 32];
            auth_data.push(0x41);
            auth_data.extend([0u8; 4]);
            auth_data.extend([0u8; 16]);
            auth_data.push(0);
            auth_data.push(4);
            auth_data.extend_from_slice(&[1, 2, 3, 4]);
            auth_data.extend_from_slice(&[0xa1, 0x01, 0x18, 0x2b]);

            let mc = CborValue::Map(vec![
                (
                    CborValue::Integer(1.into()),
                    CborValue::Text("packed".to_string()),
                ),
                (CborValue::Integer(2.into()), CborValue::Bytes(auth_data)),
                (CborValue::Integer(3.into()), CborValue::Map(vec![])),
            ]);
            let mut mc_body = Vec::new();
            ciborium::into_writer(&mc, &mut mc_body).unwrap();
            let mut make_cred = vec![CTAP2_OK];
            make_cred.extend(mc_body);

            let ga = CborValue::Map(vec![
                (
                    CborValue::Integer(2.into()),
                    CborValue::Bytes(vec![0xcc; 37]),
                ),
                (
                    CborValue::Integer(3.into()),
                    CborValue::Bytes(vec![0xdd; 64]),
                ),
            ]);
            let mut ga_body = Vec::new();
            ciborium::into_writer(&ga, &mut ga_body).unwrap();
            let mut get_assert = vec![CTAP2_OK];
            get_assert.extend(ga_body);

            Self {
                make_cred_response: make_cred,
                get_assertion_response: get_assert,
            }
        }
    }

    #[async_trait]
    impl Ctap2Transport for MockCtap2Transport {
        async fn send_receive(
            &mut self,
            command: &[u8],
        ) -> Result<Vec<u8>, beardog_errors::BearDogError> {
            match command.first() {
                Some(&x) if x == CTAP2_MAKE_CREDENTIAL => Ok(self.make_cred_response.clone()),
                Some(&x) if x == CTAP2_GET_ASSERTION => Ok(self.get_assertion_response.clone()),
                _ => Err(BearDogError::system(
                    "mock: unknown CTAP command".to_string(),
                )),
            }
        }
    }

    fn sample_device() -> SoloV2DeviceInfo {
        SoloV2DeviceInfo {
            device_id: "test-device".to_string(),
            product_name: "Solo V2 Test".to_string(),
            firmware_version: "1.0.0".to_string(),
            is_connected: true,
            vendor_id: 0x1209,
            product_id: 0xbeee,
        }
    }

    #[tokio::test]
    async fn mock_transport_make_credential_roundtrip_parse() {
        let mock = MockCtap2Transport::with_success_responses();
        let cmd = crate::tunnel::hsm::solo_v2::build_make_credential(
            "rp.example",
            b"u1",
            "u",
            &[0u8; 32],
            -8,
            None,
        )
        .expect("build");
        assert_eq!(cmd[0], CTAP2_MAKE_CREDENTIAL);
        let resp = mock.make_cred_response.clone();
        let p = parse_make_credential_response(&resp).expect("parse");
        assert_eq!(p.credential_id, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn mock_transport_get_assertion_roundtrip_parse() {
        let mock = MockCtap2Transport::with_success_responses();
        let cmd = crate::tunnel::hsm::solo_v2::build_get_assertion(
            "rp.example",
            &[1u8; 32],
            &[&b"cid"[..]],
            None,
        )
        .expect("build");
        assert_eq!(cmd[0], CTAP2_GET_ASSERTION);
        let p = parse_get_assertion_response(&mock.get_assertion_response).expect("parse");
        assert_eq!(p.signature, vec![0xdd; 64]);
    }

    #[tokio::test]
    async fn provider_generate_key_with_mock_transport_succeeds() {
        let transport = Arc::new(Mutex::new(MockCtap2Transport::with_success_responses()));
        let provider = SoloV2Provider::with_ctap2_transport(
            sample_device(),
            SoloV2Config::default(),
            transport,
        )
        .expect("provider with mock");

        let handle = provider
            .generate_key_on_device(KeyType::Ed25519, "kid-1".to_string())
            .await
            .expect("generate with mock transport");

        assert_eq!(handle.credential_id, vec![1, 2, 3, 4]);
        assert!(!handle.public_key.is_empty());
    }

    #[tokio::test]
    async fn provider_sign_with_mock_transport_succeeds() {
        let transport = Arc::new(Mutex::new(MockCtap2Transport::with_success_responses()));
        let provider = SoloV2Provider::with_ctap2_transport(
            sample_device(),
            SoloV2Config::default(),
            transport,
        )
        .expect("provider with mock");

        provider
            .generate_key_on_device(KeyType::Ed25519, "kid-1".to_string())
            .await
            .expect("generate");

        let sig = provider
            .sign_with_device("kid-1", b"hello")
            .await
            .expect("sign");
        assert_eq!(sig, vec![0xdd; 64]);
    }
}
