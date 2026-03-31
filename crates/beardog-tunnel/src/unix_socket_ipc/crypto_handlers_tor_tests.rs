// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use rand::RngCore;

fn json_str<'a>(v: &'a Value, key: &'static str) -> Result<&'a str, BearDogError> {
    v.get(key)
        .and_then(|x| x.as_str())
        .ok_or_else(|| BearDogError::invalid_input("missing JSON string field"))
}

#[tokio::test]
async fn test_ntor_client_init_basic() -> Result<(), BearDogError> {
    let node_id = [0u8; 20];
    let onion_key = [1u8; 32];

    let params = json!({
        "node_id": BASE64.encode(&node_id),
        "node_onion_key": BASE64.encode(&onion_key)
    });

    let result = handle_tor_ntor_client_init(Some(&params)).await?;

    assert!(result.get("ephemeral_public").is_some());
    let ephem_b64 = json_str(&result, "ephemeral_public")?;
    let ephem = BASE64
        .decode(ephem_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))?;
    assert_eq!(ephem.len(), 32);
    assert!(result.get("client_state").is_some());
    Ok(())
}

#[tokio::test]
async fn test_ntor_full_handshake() -> Result<(), BearDogError> {
    let server_secret = StaticSecret::random_from_rng(chacha20poly1305::aead::OsRng);
    let server_public = PublicKey::from(&server_secret);
    let node_id = [42u8; 20];

    let init_params = json!({
        "node_id": BASE64.encode(&node_id),
        "node_onion_key": BASE64.encode(server_public.as_bytes())
    });

    let init_result = handle_tor_ntor_client_init(Some(&init_params)).await?;
    let client_state = json_str(&init_result, "client_state")?;
    let client_public = json_str(&init_result, "ephemeral_public")?;

    let server_params = json!({
        "client_public": client_public,
        "node_id": BASE64.encode(&node_id),
        "onion_secret_key": BASE64.encode(server_secret.as_bytes()),
        "onion_public_key": BASE64.encode(server_public.as_bytes())
    });

    let server_result = handle_tor_ntor_server_respond(Some(&server_params)).await?;
    let server_ephem = json_str(&server_result, "ephemeral_public")?;
    let server_auth = json_str(&server_result, "server_auth")?;

    let finish_params = json!({
        "client_state": client_state,
        "server_public": server_ephem,
        "server_auth": server_auth
    });

    let finish_result = handle_tor_ntor_client_finish(Some(&finish_params)).await?;

    assert_eq!(
        finish_result
            .get("valid")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| BearDogError::invalid_input("missing valid"))?,
        true
    );

    let client_kf = json_str(&finish_result, "forward_key")?;
    let server_kf = json_str(&server_result, "forward_key")?;
    assert_eq!(client_kf, server_kf);

    let client_kb = json_str(&finish_result, "backward_key")?;
    let server_kb = json_str(&server_result, "backward_key")?;
    assert_eq!(client_kb, server_kb);
    Ok(())
}

#[tokio::test]
async fn test_cell_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
    let key = [0xABu8; 32];
    let plaintext = b"Hello, Tor cell encryption!";
    let counter = 0u64;

    let encrypt_params = json!({
        "key": BASE64.encode(&key),
        "counter": counter,
        "data": BASE64.encode(plaintext)
    });

    let encrypt_result = handle_tor_cell_encrypt(Some(&encrypt_params)).await?;
    let ciphertext = json_str(&encrypt_result, "ciphertext")?;

    let decrypt_params = json!({
        "key": BASE64.encode(&key),
        "counter": counter,
        "ciphertext": ciphertext
    });

    let decrypt_result = handle_tor_cell_decrypt(Some(&decrypt_params)).await?;
    let decrypted_b64 = json_str(&decrypt_result, "plaintext")?;
    let decrypted = BASE64
        .decode(decrypted_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))?;

    assert_eq!(decrypted, plaintext);
    Ok(())
}

#[tokio::test]
async fn test_tor_kdf() -> Result<(), BearDogError> {
    let key_seed = [0x42u8; 32];

    let params = json!({
        "key_seed": BASE64.encode(&key_seed),
        "key_count": 4,
        "key_length": 20
    });

    let result = handle_tor_kdf(Some(&params)).await?;

    let keys = result
        .get("keys")
        .and_then(|v| v.as_array())
        .ok_or_else(|| BearDogError::invalid_input("missing keys array"))?;
    assert_eq!(keys.len(), 4);

    for key in keys {
        let s = key
            .as_str()
            .ok_or_else(|| BearDogError::invalid_input("key entry not a string"))?;
        let key_bytes = BASE64
            .decode(s)
            .map_err(|e| BearDogError::invalid_input(&format!("base64: {e}")))?;
        assert_eq!(key_bytes.len(), 20);
    }
    Ok(())
}

#[test]
fn test_constant_time_compare() {
    let a = [1, 2, 3, 4];
    let b = [1, 2, 3, 4];
    let c = [1, 2, 3, 5];

    assert!(constant_time_compare(&a, &b));
    assert!(!constant_time_compare(&a, &c));
    assert!(!constant_time_compare(&a, &[1, 2, 3]));
}

#[tokio::test]
async fn ntor_client_init_missing_params() {
    let e = handle_tor_ntor_client_init(None).await.unwrap_err();
    assert!(e.to_string().contains("Missing") || e.to_string().contains("parameters"));
}

#[tokio::test]
async fn ntor_client_init_bad_node_id_len() {
    let params = json!({
        "node_id": BASE64.encode([0u8; 19]),
        "node_onion_key": BASE64.encode([1u8; 32]),
    });
    let e = handle_tor_ntor_client_init(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("20") || e.to_string().contains("node_id"));
}

#[tokio::test]
async fn ntor_client_init_bad_onion_key_len() {
    let params = json!({
        "node_id": BASE64.encode([0u8; 20]),
        "node_onion_key": BASE64.encode([1u8; 31]),
    });
    let e = handle_tor_ntor_client_init(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("32") || e.to_string().contains("onion"));
}

#[tokio::test]
async fn ntor_client_init_invalid_node_id_base64() {
    let params = json!({
        "node_id": "@@@",
        "node_onion_key": BASE64.encode([1u8; 32]),
    });
    let e = handle_tor_ntor_client_init(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("base64") || e.to_string().contains("node_id"));
}

#[tokio::test]
async fn ntor_client_finish_bad_client_state_len() {
    let params = json!({
        "client_state": BASE64.encode([0u8; 10]),
        "server_public": BASE64.encode([0u8; 32]),
        "server_auth": BASE64.encode([0u8; 32]),
    });
    let e = handle_tor_ntor_client_finish(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("client_state") || e.to_string().contains("length"));
}

#[tokio::test]
async fn ntor_client_finish_auth_mismatch_returns_invalid() {
    let node_id = [7u8; 20];
    let onion_key = [8u8; 32];
    let init = handle_tor_ntor_client_init(Some(&json!({
        "node_id": BASE64.encode(&node_id),
        "node_onion_key": BASE64.encode(&onion_key),
    })))
    .await
    .unwrap();

    let mut bad_auth = vec![0u8; 32];
    rand::rng().fill_bytes(&mut bad_auth);
    let params = json!({
        "client_state": json_str(&init, "client_state").unwrap(),
        "server_public": BASE64.encode([9u8; 32]),
        "server_auth": BASE64.encode(&bad_auth),
    });
    let out = handle_tor_ntor_client_finish(Some(&params)).await.unwrap();
    assert_eq!(out.get("valid").and_then(|v| v.as_bool()), Some(false));
}

#[tokio::test]
async fn ntor_server_respond_missing_client_public() {
    let params = json!({
        "node_id": BASE64.encode([0u8; 20]),
        "onion_secret_key": BASE64.encode([1u8; 32]),
        "onion_public_key": BASE64.encode([2u8; 32]),
    });
    let e = handle_tor_ntor_server_respond(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("client_public"));
}

#[tokio::test]
async fn ntor_server_respond_client_public_wrong_length() {
    let params = json!({
        "client_public": BASE64.encode([0u8; 31]),
        "node_id": BASE64.encode([0u8; 20]),
        "onion_secret_key": BASE64.encode([1u8; 32]),
        "onion_public_key": BASE64.encode([2u8; 32]),
    });
    let e = handle_tor_ntor_server_respond(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("32"));
}

#[tokio::test]
async fn tor_cell_encrypt_missing_counter() {
    let params = json!({
        "key": BASE64.encode([0u8; 32]),
        "data": BASE64.encode(b"hi"),
    });
    let e = handle_tor_cell_encrypt(Some(&params)).await.unwrap_err();
    assert!(e.to_string().contains("counter"));
}

#[tokio::test]
async fn tor_cell_decrypt_missing_ciphertext() {
    let params = json!({
        "key": BASE64.encode([0u8; 32]),
        "counter": 0u64,
    });
    let e = handle_tor_cell_decrypt(Some(&params)).await.unwrap_err();
    assert!(e.to_string().contains("ciphertext"));
}

#[tokio::test]
async fn tor_kdf_defaults_and_lengths() {
    let params = json!({
        "key_seed": BASE64.encode([0xEEu8; 32]),
    });
    let out = handle_tor_kdf(Some(&params)).await.unwrap();
    let keys = out.get("keys").and_then(|v| v.as_array()).unwrap();
    assert_eq!(keys.len(), 4);
    for k in keys {
        let raw = BASE64.decode(k.as_str().unwrap()).unwrap();
        assert_eq!(raw.len(), 20);
    }
}

#[tokio::test]
async fn tor_kdf_custom_count_and_length() {
    let params = json!({
        "key_seed": BASE64.encode([0xDDu8; 32]),
        "key_count": 2,
        "key_length": 16,
    });
    let out = handle_tor_kdf(Some(&params)).await.unwrap();
    let keys = out.get("keys").and_then(|v| v.as_array()).unwrap();
    assert_eq!(keys.len(), 2);
    let raw = BASE64.decode(keys[0].as_str().unwrap()).unwrap();
    assert_eq!(raw.len(), 16);
}

#[tokio::test]
async fn ntor_client_init_missing_node_id() {
    let params = json!({
        "node_onion_key": BASE64.encode([1u8; 32]),
    });
    let e = handle_tor_ntor_client_init(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("node_id"));
}

#[tokio::test]
async fn ntor_client_init_missing_onion_key() {
    let params = json!({
        "node_id": BASE64.encode([0u8; 20]),
    });
    let e = handle_tor_ntor_client_init(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("onion") || e.to_string().contains("node_onion"));
}

#[tokio::test]
async fn ntor_client_finish_missing_params() {
    let e = handle_tor_ntor_client_finish(None).await.unwrap_err();
    assert!(e.to_string().contains("Missing") || e.to_string().contains("parameters"));
}

#[tokio::test]
async fn ntor_client_finish_missing_server_public() {
    let init = handle_tor_ntor_client_init(Some(&json!({
        "node_id": BASE64.encode([1u8; 20]),
        "node_onion_key": BASE64.encode([2u8; 32]),
    })))
    .await
    .unwrap();
    let params = json!({
        "client_state": json_str(&init, "client_state").unwrap(),
        "server_auth": BASE64.encode([0u8; 32]),
    });
    let e = handle_tor_ntor_client_finish(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("server_public"));
}

#[tokio::test]
async fn ntor_client_finish_server_public_wrong_len() {
    let init = handle_tor_ntor_client_init(Some(&json!({
        "node_id": BASE64.encode([3u8; 20]),
        "node_onion_key": BASE64.encode([4u8; 32]),
    })))
    .await
    .unwrap();
    let params = json!({
        "client_state": json_str(&init, "client_state").unwrap(),
        "server_public": BASE64.encode([0u8; 31]),
        "server_auth": BASE64.encode([0u8; 32]),
    });
    let e = handle_tor_ntor_client_finish(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("32") || e.to_string().contains("server_public"));
}

#[tokio::test]
async fn ntor_client_finish_server_auth_wrong_len() {
    let init = handle_tor_ntor_client_init(Some(&json!({
        "node_id": BASE64.encode([5u8; 20]),
        "node_onion_key": BASE64.encode([6u8; 32]),
    })))
    .await
    .unwrap();
    let params = json!({
        "client_state": json_str(&init, "client_state").unwrap(),
        "server_public": BASE64.encode([7u8; 32]),
        "server_auth": BASE64.encode([0u8; 16]),
    });
    let e = handle_tor_ntor_client_finish(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("server_auth") || e.to_string().contains("32"));
}

#[tokio::test]
async fn ntor_server_respond_missing_node_id() {
    let params = json!({
        "client_public": BASE64.encode([0u8; 32]),
        "onion_secret_key": BASE64.encode([1u8; 32]),
        "onion_public_key": BASE64.encode([2u8; 32]),
    });
    let e = handle_tor_ntor_server_respond(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("node_id"));
}

#[tokio::test]
async fn ntor_server_respond_missing_onion_secret() {
    let params = json!({
        "client_public": BASE64.encode([0u8; 32]),
        "node_id": BASE64.encode([0u8; 20]),
        "onion_public_key": BASE64.encode([2u8; 32]),
    });
    let e = handle_tor_ntor_server_respond(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("onion_secret"));
}

#[tokio::test]
async fn ntor_server_respond_node_id_wrong_len() {
    let params = json!({
        "client_public": BASE64.encode([0u8; 32]),
        "node_id": BASE64.encode([0u8; 19]),
        "onion_secret_key": BASE64.encode([1u8; 32]),
        "onion_public_key": BASE64.encode([2u8; 32]),
    });
    let e = handle_tor_ntor_server_respond(Some(&params))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("20") || e.to_string().contains("node_id"));
}

#[tokio::test]
async fn tor_cell_encrypt_missing_key() {
    let params = json!({
        "counter": 0u64,
        "data": BASE64.encode(b"x"),
    });
    let e = handle_tor_cell_encrypt(Some(&params)).await.unwrap_err();
    assert!(e.to_string().contains("key"));
}

#[tokio::test]
async fn tor_cell_encrypt_key_wrong_len() {
    let params = json!({
        "key": BASE64.encode([0u8; 16]),
        "counter": 0u64,
        "data": BASE64.encode(b"x"),
    });
    let e = handle_tor_cell_encrypt(Some(&params)).await.unwrap_err();
    assert!(e.to_string().contains("32") || e.to_string().contains("key"));
}

#[tokio::test]
async fn tor_cell_decrypt_missing_key() {
    let params = json!({
        "counter": 0u64,
        "ciphertext": BASE64.encode(b"x"),
    });
    let e = handle_tor_cell_decrypt(Some(&params)).await.unwrap_err();
    assert!(e.to_string().contains("key"));
}

#[tokio::test]
async fn tor_kdf_missing_key_seed() {
    let e = handle_tor_kdf(Some(&json!({ "key_count": 2 })))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("key_seed"));
}

#[tokio::test]
async fn tor_kdf_invalid_key_seed_b64() {
    let e = handle_tor_kdf(Some(&json!({ "key_seed": "@@@" })))
        .await
        .unwrap_err();
    assert!(e.to_string().contains("base64") || e.to_string().contains("key_seed"));
}
