// SPDX-License-Identifier: AGPL-3.0-only

//! Pure CTAP2 CBOR encode/decode — no HID, no async, unit-testable.

use beardog_errors::BearDogError;
use ciborium::Value as CborValue;
use std::collections::BTreeMap;

/// CTAP2 command IDs (FIDO2 CTAP2 spec).
pub const CTAP2_MAKE_CREDENTIAL: u8 = 0x01;
/// CTAP2 `authenticatorGetAssertion`.
pub const CTAP2_GET_ASSERTION: u8 = 0x02;
/// CTAP2 `authenticatorGetInfo`.
pub const CTAP2_GET_INFO: u8 = 0x04;

/// Successful CTAP2 status byte.
pub const CTAP2_OK: u8 = 0x00;

/// Parsed `authenticatorMakeCredential` response (CTAP2 layer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeCredentialResponse {
    /// Credential ID from attested credential data.
    pub credential_id: Vec<u8>,
    /// COSE-encoded public key bytes.
    pub raw_cose_public_key: Vec<u8>,
    /// Raw CBOR `attStmt` map bytes (for attestation verification).
    pub attestation_statement: Vec<u8>,
}

/// Parsed `authenticatorGetAssertion` response (CTAP2 layer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAssertionResponse {
    /// Signature over `authenticatorData || clientDataHash`.
    pub signature: Vec<u8>,
    /// Raw authenticator data bytes.
    pub authenticator_data: Vec<u8>,
}

/// Build a CTAP2 `authenticatorMakeCredential` message body: `[cmd][CBOR map]`.
///
/// # Errors
///
/// Returns an error if CBOR encoding fails (should not occur for bounded inputs).
pub fn build_make_credential(
    rp_id: &str,
    user_id: &[u8],
    user_name: &str,
    client_data_hash: &[u8],
    alg: i64,
    pin_uv_auth: Option<(&[u8], u64)>,
) -> Result<Vec<u8>, BearDogError> {
    let mut map: Vec<(CborValue, CborValue)> = Vec::new();

    // 1: clientDataHash
    map.push((
        CborValue::Integer(1.into()),
        CborValue::Bytes(client_data_hash.to_vec()),
    ));

    // 2: rp
    let rp = CborValue::Map(vec![
        (
            CborValue::Text("id".to_string()),
            CborValue::Text(rp_id.to_string()),
        ),
        (
            CborValue::Text("name".to_string()),
            CborValue::Text(rp_id.to_string()),
        ),
    ]);
    map.push((CborValue::Integer(2.into()), rp));

    // 3: user
    let user = CborValue::Map(vec![
        (
            CborValue::Text("id".to_string()),
            CborValue::Bytes(user_id.to_vec()),
        ),
        (
            CborValue::Text("name".to_string()),
            CborValue::Text(user_name.to_string()),
        ),
        (
            CborValue::Text("displayName".to_string()),
            CborValue::Text(user_name.to_string()),
        ),
    ]);
    map.push((CborValue::Integer(3.into()), user));

    // 4: pubKeyCredParams
    let param = CborValue::Map(vec![
        (
            CborValue::Text("type".to_string()),
            CborValue::Text("public-key".to_string()),
        ),
        (
            CborValue::Text("alg".to_string()),
            CborValue::Integer(alg.into()),
        ),
    ]);
    map.push((CborValue::Integer(4.into()), CborValue::Array(vec![param])));

    // 7: options (resident key + user presence)
    let opts = CborValue::Map(vec![
        (CborValue::Text("rk".to_string()), CborValue::Bool(true)),
        (CborValue::Text("up".to_string()), CborValue::Bool(true)),
    ]);
    map.push((CborValue::Integer(7.into()), opts));

    if let Some((param, protocol)) = pin_uv_auth {
        map.push((
            CborValue::Integer(8.into()),
            CborValue::Bytes(param.to_vec()),
        ));
        map.push((
            CborValue::Integer(9.into()),
            CborValue::Integer(protocol.into()),
        ));
    }

    let cbor_map = CborValue::Map(map);
    let mut body = Vec::new();
    ciborium::into_writer(&cbor_map, &mut body)
        .map_err(|e| BearDogError::system(format!("MakeCredential CBOR encode: {e}")))?;

    let mut out = Vec::with_capacity(1 + body.len());
    out.push(CTAP2_MAKE_CREDENTIAL);
    out.extend(body);
    Ok(out)
}

/// Build a CTAP2 `authenticatorGetAssertion` message body.
///
/// # Errors
///
/// Returns an error if CBOR encoding fails (should not occur for bounded inputs).
pub fn build_get_assertion(
    rp_id: &str,
    client_data_hash: &[u8],
    allow_credentials: &[&[u8]],
    pin_uv_auth: Option<(&[u8], u64)>,
) -> Result<Vec<u8>, BearDogError> {
    let mut map: Vec<(CborValue, CborValue)> = vec![
        (
            CborValue::Integer(1.into()),
            CborValue::Text(rp_id.to_string()),
        ),
        (
            CborValue::Integer(2.into()),
            CborValue::Bytes(client_data_hash.to_vec()),
        ),
    ];

    if !allow_credentials.is_empty() {
        let allow: Vec<CborValue> = allow_credentials
            .iter()
            .map(|id| {
                CborValue::Map(vec![
                    (
                        CborValue::Text("type".to_string()),
                        CborValue::Text("public-key".to_string()),
                    ),
                    (
                        CborValue::Text("id".to_string()),
                        CborValue::Bytes((*id).to_vec()),
                    ),
                ])
            })
            .collect();
        map.push((CborValue::Integer(3.into()), CborValue::Array(allow)));
    }

    let opts = CborValue::Map(vec![(
        CborValue::Text("up".to_string()),
        CborValue::Bool(true),
    )]);
    map.push((CborValue::Integer(5.into()), opts));

    if let Some((param, protocol)) = pin_uv_auth {
        map.push((
            CborValue::Integer(6.into()),
            CborValue::Bytes(param.to_vec()),
        ));
        map.push((
            CborValue::Integer(7.into()),
            CborValue::Integer(protocol.into()),
        ));
    }

    let cbor_map = CborValue::Map(map);
    let mut body = Vec::new();
    ciborium::into_writer(&cbor_map, &mut body)
        .map_err(|e| BearDogError::system(format!("GetAssertion CBOR encode: {e}")))?;

    let mut out = Vec::with_capacity(1 + body.len());
    out.push(CTAP2_GET_ASSERTION);
    out.extend(body);
    Ok(out)
}

/// Parse CTAP2 `authenticatorMakeCredential` response: `[status][CBOR]`.
///
/// # Errors
///
/// Returns an error if the response is empty, the status byte is non-success, CBOR is invalid,
/// or required map entries are missing or malformed.
pub fn parse_make_credential_response(
    response: &[u8],
) -> Result<MakeCredentialResponse, BearDogError> {
    if response.is_empty() {
        return Err(BearDogError::system("empty CTAP2 response".to_string()));
    }
    let status = response[0];
    if status != CTAP2_OK {
        return Err(BearDogError::system(format!(
            "CTAP2 MakeCredential failed: status 0x{status:02x}"
        )));
    }
    if response.len() < 2 {
        return Err(BearDogError::system(
            "CTAP2 MakeCredential: missing CBOR body".to_string(),
        ));
    }

    let cbor_value: CborValue = ciborium::from_reader(&response[1..])
        .map_err(|e| BearDogError::system(format!("MakeCredential CBOR: {e}")))?;

    let CborValue::Map(entries) = cbor_value else {
        return Err(BearDogError::system(
            "MakeCredential response not a CBOR map".to_string(),
        ));
    };

    let map: BTreeMap<i128, CborValue> = entries
        .into_iter()
        .filter_map(|(k, v)| {
            if let CborValue::Integer(i) = k {
                let n: i128 = i.into();
                Some((n, v))
            } else {
                None
            }
        })
        .collect();

    let auth_data = map.get(&2).and_then(|v| {
        if let CborValue::Bytes(b) = v {
            Some(b.clone())
        } else {
            None
        }
    });

    let auth_data = auth_data.ok_or_else(|| {
        BearDogError::system("MakeCredential response missing authData (key 2)".to_string())
    })?;

    let (credential_id, raw_cose_public_key) = parse_attested_credential_data(&auth_data)?;

    let att_stmt = map.get(&3).ok_or_else(|| {
        BearDogError::system("MakeCredential response missing attStmt (key 3)".to_string())
    })?;

    let mut attestation_statement = Vec::new();
    ciborium::into_writer(att_stmt, &mut attestation_statement)
        .map_err(|e| BearDogError::system(format!("encode attStmt: {e}")))?;

    Ok(MakeCredentialResponse {
        credential_id,
        raw_cose_public_key,
        attestation_statement,
    })
}

/// Parse CTAP2 `authenticatorGetAssertion` response.
///
/// # Errors
///
/// Returns an error if the response is empty, the status byte is non-success, CBOR is invalid,
/// or required map entries are missing or malformed.
pub fn parse_get_assertion_response(response: &[u8]) -> Result<GetAssertionResponse, BearDogError> {
    if response.is_empty() {
        return Err(BearDogError::system("empty CTAP2 response".to_string()));
    }
    let status = response[0];
    if status != CTAP2_OK {
        return Err(BearDogError::system(format!(
            "CTAP2 GetAssertion failed: status 0x{status:02x}"
        )));
    }
    if response.len() < 2 {
        return Err(BearDogError::system(
            "CTAP2 GetAssertion: missing CBOR body".to_string(),
        ));
    }

    let cbor_value: CborValue = ciborium::from_reader(&response[1..])
        .map_err(|e| BearDogError::system(format!("GetAssertion CBOR: {e}")))?;

    let CborValue::Map(entries) = cbor_value else {
        return Err(BearDogError::system(
            "GetAssertion response not a CBOR map".to_string(),
        ));
    };

    let map: BTreeMap<i128, CborValue> = entries
        .into_iter()
        .filter_map(|(k, v)| {
            if let CborValue::Integer(i) = k {
                let n: i128 = i.into();
                Some((n, v))
            } else {
                None
            }
        })
        .collect();

    let signature = map.get(&3).and_then(|v| {
        if let CborValue::Bytes(b) = v {
            Some(b.clone())
        } else {
            None
        }
    });
    let signature = signature.ok_or_else(|| {
        BearDogError::system("GetAssertion response missing signature (key 3)".to_string())
    })?;

    let authenticator_data = map.get(&2).and_then(|v| {
        if let CborValue::Bytes(b) = v {
            Some(b.clone())
        } else {
            None
        }
    });
    let authenticator_data = authenticator_data.ok_or_else(|| {
        BearDogError::system("GetAssertion response missing authData (key 2)".to_string())
    })?;

    Ok(GetAssertionResponse {
        signature,
        authenticator_data,
    })
}

/// `WebAuthn` attested credential data: variable tail of `authData` when AT flag is set.
fn parse_attested_credential_data(auth_data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), BearDogError> {
    const MIN_PREFIX: usize = 32 + 1 + 4;
    if auth_data.len() < MIN_PREFIX {
        return Err(BearDogError::system(format!(
            "authData too short: {} bytes",
            auth_data.len()
        )));
    }
    let flags = auth_data[32];
    const AT_FLAG: u8 = 0x40;
    if (flags & AT_FLAG) == 0 {
        return Err(BearDogError::system(
            "authData missing AT flag — no attested credential data".to_string(),
        ));
    }

    let mut offset = MIN_PREFIX;
    if auth_data.len() < offset + 16 + 2 {
        return Err(BearDogError::system(
            "authData truncated before AAGUID / credential id length".to_string(),
        ));
    }
    offset += 16; // AAGUID

    let cred_id_len = u16::from_be_bytes([auth_data[offset], auth_data[offset + 1]]) as usize;
    offset += 2;

    if auth_data.len() < offset + cred_id_len {
        return Err(BearDogError::system(
            "authData truncated in credential id".to_string(),
        ));
    }
    let credential_id = auth_data[offset..offset + cred_id_len].to_vec();
    offset += cred_id_len;

    let cose_key_bytes = &auth_data[offset..];
    if cose_key_bytes.is_empty() {
        return Err(BearDogError::system(
            "authData missing COSE public key".to_string(),
        ));
    }

    Ok((credential_id, cose_key_bytes.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_make_credential_starts_with_command_and_non_empty_cbor() {
        let cmd = build_make_credential("example.com", b"user-1", "User", &[7u8; 32], -7, None)
            .expect("build");
        assert_eq!(cmd[0], CTAP2_MAKE_CREDENTIAL);
        assert!(cmd.len() > 8);
    }

    #[test]
    fn build_get_assertion_includes_rp_and_hash() {
        let cmd = build_get_assertion("rp.test", &[9u8; 32], &[&b"cid"[..]], None).expect("build");
        assert_eq!(cmd[0], CTAP2_GET_ASSERTION);
        assert!(cmd.len() > 4);
    }

    #[test]
    fn parse_make_credential_roundtrip_synthetic() {
        // Minimal valid authData: rpIdHash(32) + flags(AT|UP) + signCount + aaguid + credIdLen + id + fake COSE
        let mut auth_data = vec![0u8; 32];
        auth_data.push(0x41); // AT + UP
        auth_data.extend([0u8; 4]); // signCount
        auth_data.extend([0u8; 16]); // aaguid
        auth_data.push(0);
        auth_data.push(4); // cred id len 4
        auth_data.extend_from_slice(&[1, 2, 3, 4]); // credential id
        auth_data.extend_from_slice(&[0xa1, 0x01, 0x18, 0x2b]); // minimal CBOR map

        let att_stmt = CborValue::Map(vec![]);

        let response_val = CborValue::Map(vec![
            (
                CborValue::Integer(1.into()),
                CborValue::Text("packed".to_string()),
            ),
            (CborValue::Integer(2.into()), CborValue::Bytes(auth_data)),
            (CborValue::Integer(3.into()), att_stmt),
        ]);
        let mut cbor = Vec::new();
        ciborium::into_writer(&response_val, &mut cbor).unwrap();

        let mut response = vec![CTAP2_OK];
        response.extend(cbor);

        let parsed = parse_make_credential_response(&response).expect("parse");
        assert_eq!(parsed.credential_id, vec![1, 2, 3, 4]);
        assert!(!parsed.raw_cose_public_key.is_empty());
    }

    #[test]
    fn parse_get_assertion_roundtrip_synthetic() {
        let map = CborValue::Map(vec![
            (
                CborValue::Integer(2.into()),
                CborValue::Bytes(vec![0xaa; 37]),
            ),
            (
                CborValue::Integer(3.into()),
                CborValue::Bytes(vec![0xbb; 64]),
            ),
        ]);
        let mut cbor = Vec::new();
        ciborium::into_writer(&map, &mut cbor).unwrap();
        let mut response = vec![CTAP2_OK];
        response.extend(cbor);

        let p = parse_get_assertion_response(&response).expect("parse");
        assert_eq!(p.signature, vec![0xbb; 64]);
        assert_eq!(p.authenticator_data, vec![0xaa; 37]);
    }

    #[test]
    fn parse_make_credential_errors_on_status() {
        let err = parse_make_credential_response(&[0x2e]).unwrap_err();
        let s = format!("{err}");
        assert!(s.contains("0x2e") || s.contains("failed"));
    }
}
