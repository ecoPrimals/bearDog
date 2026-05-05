// SPDX-License-Identifier: AGPL-3.0-or-later

#![cfg(test)]

use serde::Serialize;

pub fn assert_serde_json_roundtrip<T>(v: &T)
where
    T: Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug,
{
    let json = serde_json::to_value(v).expect("serialize to serde_json::Value");
    let back: T = serde_json::from_value(json.clone()).expect("deserialize from Value");
    let again = serde_json::to_value(&back).expect("re-serialize after roundtrip");
    assert_eq!(json, again, "serde roundtrip must preserve JSON form");
}
