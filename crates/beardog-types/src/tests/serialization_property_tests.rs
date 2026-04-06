// SPDX-License-Identifier: AGPL-3.0-or-later

//! Property-based `serde_json` roundtrips and error formatting (proptest).

#![cfg(test)]

use crate::BearDogError;
use crate::SecuritySettings;
use crate::UnifiedBearDogConfig;
use crate::{KeyId, RegistrationId, ServiceInstanceId};
use proptest::prelude::*;
use serde_json::Value;

fn roundtrip_json<T>(v: &T) -> Result<T, serde_json::Error>
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(v)?;
    serde_json::from_str(&json)
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 64,
        ..ProptestConfig::default()
    })]

    /// KeyId roundtrips through serde_json for arbitrary UTF-8 strings.
    #[test]
    fn key_id_json_roundtrip(s in prop::string::string_regex(".{0,256}").unwrap()) {
        let id = KeyId::new(s);
        let back: KeyId = roundtrip_json(&id).expect("KeyId roundtrip");
        prop_assert_eq!(back, id);
    }

    /// RegistrationId roundtrips through serde_json.
    #[test]
    fn registration_id_json_roundtrip(s in prop::string::string_regex(".{0,256}").unwrap()) {
        let id = RegistrationId::new(s);
        let back: RegistrationId = roundtrip_json(&id).expect("RegistrationId roundtrip");
        prop_assert_eq!(back, id);
    }

    /// ServiceInstanceId roundtrips through serde_json.
    #[test]
    fn service_instance_id_json_roundtrip(s in prop::string::string_regex(".{0,256}").unwrap()) {
        let id = ServiceInstanceId::new(s);
        let back: ServiceInstanceId = roundtrip_json(&id).expect("ServiceInstanceId roundtrip");
        prop_assert_eq!(back, id);
    }

    /// Unified default config: decode after roundtrip matches semantically (`features` uses `HashMap`, so
    /// raw JSON key order may differ between serializations).
    #[test]
    fn unified_bear_dog_config_default_roundtrip(_ in prop::num::u8::ANY) {
        let config = UnifiedBearDogConfig::default();
        let json = serde_json::to_string(&config).expect("to_string");
        let back: UnifiedBearDogConfig = serde_json::from_str(&json).expect("from_str");
        let json2 = serde_json::to_string(&back).expect("to_string 2");
        let v1: Value = serde_json::from_str(&json).expect("value 1");
        let v2: Value = serde_json::from_str(&json2).expect("value 2");
        prop_assert_eq!(v1, v2);
    }

    /// Security settings: arbitrary field values roundtrip through JSON.
    #[test]
    fn security_settings_json_roundtrip(
        session_timeout_seconds in any::<u64>(),
        max_login_attempts in any::<u32>(),
        enable_mfa in any::<bool>(),
        hash_rounds in any::<u32>(),
        audit_retention_days in any::<u32>(),
    ) {
        let settings = SecuritySettings {
            session_timeout_seconds,
            max_login_attempts,
            enable_mfa,
            hash_rounds,
            audit_retention_days,
        };
        let json = serde_json::to_string(&settings).expect("to_string");
        let back: SecuritySettings = serde_json::from_str(&json).expect("from_str");
        let json2 = serde_json::to_string(&back).expect("to_string 2");
        prop_assert_eq!(json, json2);
    }

    /// BearDogError: Display matches `to_string`, Debug is non-empty; serde_json roundtrip for Security.
    #[test]
    fn bear_dog_error_display_debug_and_security_json_roundtrip(
        msg in prop::string::string_regex(".{1,512}").unwrap(),
    ) {
        let err = BearDogError::security(msg);
        let disp = err.to_string();
        prop_assert!(!disp.is_empty());
        prop_assert_eq!(disp, format!("{err}"));
        let dbg = format!("{err:?}");
        prop_assert!(!dbg.is_empty());

        let back: BearDogError = roundtrip_json(&err).expect("BearDogError roundtrip");
        prop_assert_eq!(back, err);
    }
}
