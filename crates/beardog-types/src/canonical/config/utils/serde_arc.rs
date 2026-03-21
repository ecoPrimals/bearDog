// SPDX-License-Identifier: AGPL-3.0-only

use serde::Deserialize;
use std::sync::Arc;

/// Serialize `Arc<str>` as a regular string for compatibility
///
/// This allows `Arc<str>` fields to be serialized as normal strings,
/// maintaining compatibility with existing configuration formats.
pub fn serialize_arc_str<S>(arc_str: &Arc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(arc_str)
}

/// Deserialize a string into `Arc<str>` for efficient cloning
///
/// Creates an `Arc<str>` from the deserialized string, enabling
/// 10x faster cloning operations compared to regular `String`.
pub fn deserialize_arc_str<'de, D>(deserializer: D) -> Result<Arc<str>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Arc::from(s.as_str()))
}
