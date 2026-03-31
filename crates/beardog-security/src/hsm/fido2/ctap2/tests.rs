// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use std::collections::BTreeMap;

#[test]
fn test_ctap2_status_conversion() {
    assert_eq!(Ctap2Status::from_byte(0x00), Ctap2Status::Success);
    assert_eq!(Ctap2Status::from_byte(0x01), Ctap2Status::InvalidCommand);
    assert!(Ctap2Status::Success.is_success());
    assert!(!Ctap2Status::InvalidCommand.is_success());
}

#[test]
fn test_ctap2_command_codes() {
    assert_eq!(Ctap2Command::GetInfo as u8, 0x04);
    assert_eq!(Ctap2Command::MakeCredential as u8, 0x01);
    assert_eq!(Ctap2Command::GetAssertion as u8, 0x02);
    assert_eq!(Ctap2Command::CredentialManagement as u8, 0x0A);
}

#[test]
fn test_ctap2_status_from_byte_maps_and_other() {
    assert_eq!(Ctap2Status::from_byte(0x03), Ctap2Status::Other);
    assert_eq!(Ctap2Status::from_byte(0xFE), Ctap2Status::Other);
    assert_eq!(Ctap2Status::from_byte(0x02), Ctap2Status::InvalidParameter);
    assert_eq!(Ctap2Status::from_byte(0x12), Ctap2Status::CborError);
    assert_eq!(Ctap2Status::from_byte(0x36), Ctap2Status::PinRequired);
}

#[test]
fn test_ctap2_status_error_messages() {
    assert_eq!(Ctap2Status::Success.to_error_message(), "Success");
    assert_eq!(
        Ctap2Status::InvalidCommand.to_error_message(),
        "Invalid command"
    );
    assert_eq!(
        Ctap2Status::UserActionPending.to_error_message(),
        "User action required (touch button)"
    );
    assert_eq!(Ctap2Status::Other.to_error_message(), "Unknown error");
}

#[test]
fn test_ctap_hid_command_as_u8() {
    assert_eq!(CtapHidCommand::Msg.as_u8(), 0x83);
    assert_eq!(CtapHidCommand::Cbor.as_u8(), 0x90);
    assert_eq!(CtapHidCommand::Init.as_u8(), 0x86);
    assert_eq!(CtapHidCommand::Ping.as_u8(), 0x81);
    assert_eq!(CtapHidCommand::Cancel.as_u8(), 0x91);
    assert_eq!(CtapHidCommand::Error.as_u8(), 0xBF);
    assert_eq!(CtapHidCommand::Keepalive.as_u8(), 0xBB);
}

#[test]
fn test_ctap2_device_info_debug() {
    let info = Ctap2DeviceInfo {
        versions: vec!["FIDO_2_0".into()],
        extensions: vec![],
        aaguid: vec![1, 2, 3],
        options: BTreeMap::from([("rk".into(), true)]),
        max_msg_size: Some(1024),
        pin_protocols: Some(vec![1]),
        max_credential_count_in_list: None,
        max_credential_id_length: None,
        transports: None,
        algorithms: None,
        max_serialized_large_blob_array: None,
        force_pin_change: None,
        min_pin_length: None,
        firmware_version: None,
        max_cred_blob_length: None,
        max_rpids_for_set_min_pin_length: None,
        preferred_platform_uv_attempts: None,
        uv_modality: None,
        certifications: None,
        remaining_discoverable_credentials: None,
        vendor_prototype_config_commands: None,
    };
    let s = format!("{info:?}");
    assert!(s.contains("Ctap2DeviceInfo") || s.contains("FIDO"));
}

#[cfg(all(test, feature = "fido2"))]
mod fido2_hid_tests {
    use super::super::*;
    use async_trait::async_trait;
    use beardog_hid::{HidDevice, HidDeviceInfo, ProductId, VendorId};
    use ciborium::Value as CborValue;

    struct ScriptedHid {
        info: HidDeviceInfo,
        init_nonce: [u8; 8],
        reads: u32,
    }

    impl ScriptedHid {
        fn new() -> Self {
            Self {
                info: HidDeviceInfo {
                    vendor_id: VendorId(0),
                    product_id: ProductId(0),
                    manufacturer: String::new(),
                    product: String::new(),
                    serial: String::new(),
                    path: "/mock".to_string(),
                },
                init_nonce: [0u8; 8],
                reads: 0,
            }
        }

        fn fill_init_response(&self, buf: &mut [u8]) -> usize {
            buf.fill(0);
            buf[0..4].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
            buf[4] = CtapHidCommand::Init as u8;
            buf[7..15].copy_from_slice(&self.init_nonce);
            buf[15..19].copy_from_slice(&0xCAFE_BABEu32.to_be_bytes());
            64
        }

        fn fill_ctap_ok_cbor(&self, buf: &mut [u8], cid: u32, cbor_body: &[u8]) -> usize {
            let inner_len = 1 + cbor_body.len();
            buf[0..4].copy_from_slice(&cid.to_be_bytes());
            buf[4] = CtapHidCommand::Msg as u8;
            buf[5] = ((inner_len >> 8) & 0xFF) as u8;
            buf[6] = (inner_len & 0xFF) as u8;
            buf[7] = 0x00;
            buf[8..8 + cbor_body.len()].copy_from_slice(cbor_body);
            8 + cbor_body.len()
        }
    }

    #[async_trait]
    impl HidDevice for ScriptedHid {
        async fn write(&mut self, report: &[u8]) -> Result<usize, BearDogError> {
            if report.len() >= 15 && report[4] == CtapHidCommand::Init as u8 {
                self.init_nonce.copy_from_slice(&report[7..15]);
            }
            Ok(report.len())
        }

        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, BearDogError> {
            self.reads += 1;
            if self.reads == 1 {
                return Ok(self.fill_init_response(buf));
            }
            let map = CborValue::Map(vec![(
                CborValue::Integer(1i64.into()),
                CborValue::Array(vec![CborValue::Text("FIDO_2_0".to_string())]),
            )]);
            let mut body = Vec::new();
            ciborium::into_writer(&map, &mut body).unwrap();
            let n = self.fill_ctap_ok_cbor(buf, 0xCAFE_BABE, &body);
            Ok(n)
        }

        fn info(&self) -> &HidDeviceInfo {
            &self.info
        }
    }

    #[tokio::test]
    async fn ctaphid_init_success() {
        let mut dev: Box<dyn HidDevice> = Box::new(ScriptedHid::new());
        let cid = ctaphid_init(&mut dev).await.expect("init");
        assert_eq!(cid, 0xCAFE_BABE);
    }

    #[tokio::test]
    async fn send_ctap2_command_success_after_init() {
        let mut dev: Box<dyn HidDevice> = Box::new(ScriptedHid::new());
        let cid = ctaphid_init(&mut dev).await.unwrap();
        let payload = send_ctap2_command(&mut dev, cid, Ctap2Command::GetInfo, &[])
            .await
            .expect("getinfo payload");
        assert!(!payload.is_empty());
    }

    #[tokio::test]
    async fn ctap2_get_info_parses_versions() {
        let mut dev: Box<dyn HidDevice> = Box::new(ScriptedHid::new());
        let info = ctap2_get_info(&mut dev).await.expect("getinfo");
        assert!(info.versions.iter().any(|v| v.contains("FIDO")));
    }
}
