// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests for the BTSP handshake enforcement module.

#[cfg(test)]
mod tests {
    use crate::btsp_handshake::crypto::{
        compute_challenge_hmac, derive_handshake_key, derive_session_keys,
        generate_ephemeral_keypair, x25519_shared_secret,
    };
    use crate::btsp_handshake::framing::{read_frame, write_frame};
    use crate::btsp_handshake::session::{BtspCipher, BtspSession};
    use crate::btsp_handshake::types::{
        BTSP_HANDSHAKE_VERSION, ChallengeResponse, ClientHello, HandshakeComplete, ServerHello,
    };
    use crate::btsp_handshake::{FamilySeed, perform_server_handshake};
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use tokio::io::duplex;

    /// Simulate the client side of the BTSP handshake for testing.
    async fn client_handshake<S>(stream: &mut S, family_seed: &[u8]) -> BtspSession
    where
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        let handshake_key = derive_handshake_key(family_seed).expect("hk");

        // Step 1: Send ClientHello
        let (client_secret, client_pub) = generate_ephemeral_keypair();
        let hello = ClientHello {
            version: BTSP_HANDSHAKE_VERSION,
            client_ephemeral_pub: BASE64.encode(client_pub.as_bytes()),
        };
        let json = serde_json::to_vec(&hello).expect("ser");
        write_frame(stream, &json).await.expect("write hello");

        // Step 2: Read ServerHello
        let server_hello_bytes = read_frame(stream).await.expect("read server hello");
        let server_hello: ServerHello =
            serde_json::from_slice(&server_hello_bytes).expect("parse server hello");
        let server_pub_bytes = BASE64
            .decode(&server_hello.server_ephemeral_pub)
            .expect("dec");
        let challenge = BASE64
            .decode(&server_hello.challenge)
            .expect("dec challenge");

        // Step 3: Send ChallengeResponse
        let hmac = compute_challenge_hmac(
            &handshake_key,
            &challenge,
            client_pub.as_bytes(),
            &server_pub_bytes,
        )
        .expect("hmac");

        let resp = ChallengeResponse {
            response: BASE64.encode(hmac),
            preferred_cipher: "chacha20_poly1305".into(),
        };
        let resp_json = serde_json::to_vec(&resp).expect("ser resp");
        write_frame(stream, &resp_json).await.expect("write resp");

        // Step 4: Read HandshakeComplete
        let complete_bytes = read_frame(stream).await.expect("read complete");
        let complete: HandshakeComplete =
            serde_json::from_slice(&complete_bytes).expect("parse complete");

        // Derive session keys (client side: encrypt = client_to_server, decrypt = server_to_client)
        let server_pub_arr: [u8; 32] = server_pub_bytes.try_into().expect("len");
        let their_pub = x25519_dalek::PublicKey::from(server_pub_arr);
        let shared = x25519_shared_secret(&client_secret, &their_pub);
        let keys = derive_session_keys(&shared, complete.session_id.as_bytes()).expect("sk");

        let cipher = BtspCipher::from_wire_name(&complete.cipher).expect("cipher");
        BtspSession::new_server(
            complete.session_id,
            cipher,
            keys.client_to_server,
            keys.server_to_client,
        )
    }

    #[tokio::test]
    async fn full_handshake_roundtrip() {
        let seed = b"test-family-seed-for-handshake!!";
        let family_seed = FamilySeed::new(seed.to_vec());

        let (mut client_stream, mut server_stream) = duplex(16384);

        let server_handle = tokio::spawn(async move {
            perform_server_handshake(&mut server_stream, &family_seed)
                .await
                .expect("server handshake")
        });

        let client_handle =
            tokio::spawn(async move { client_handshake(&mut client_stream, seed).await });

        let (server_session, client_session) = tokio::join!(server_handle, client_handle);
        let mut server_session = server_session.expect("join server");
        let mut client_session = client_session.expect("join client");

        assert_eq!(server_session.session_id, client_session.session_id);
        assert_eq!(server_session.cipher, client_session.cipher);

        // Test encrypted communication in both directions
        let msg = b"hello from server";
        let encrypted = server_session.encrypt_frame(msg).expect("enc");
        let decrypted = client_session.decrypt_frame(&encrypted).expect("dec");
        assert_eq!(decrypted, msg);

        let msg2 = b"hello from client";
        let encrypted2 = client_session.encrypt_frame(msg2).expect("enc2");
        let decrypted2 = server_session.decrypt_frame(&encrypted2).expect("dec2");
        assert_eq!(decrypted2, msg2);
    }

    #[tokio::test]
    async fn handshake_rejects_wrong_family_seed() {
        let server_seed = FamilySeed::new(b"server-family-seed-correct!!!!!".to_vec());
        let wrong_seed = b"wrong-family-seed-incorrect!!!!";

        let (mut client_stream, mut server_stream) = duplex(16384);

        let server_handle = tokio::spawn(async move {
            perform_server_handshake(&mut server_stream, &server_seed).await
        });

        let client_handle = tokio::spawn(async move {
            let handshake_key = derive_handshake_key(wrong_seed).expect("hk");
            let (_, client_pub) = generate_ephemeral_keypair();

            let hello = ClientHello {
                version: BTSP_HANDSHAKE_VERSION,
                client_ephemeral_pub: BASE64.encode(client_pub.as_bytes()),
            };
            write_frame(&mut client_stream, &serde_json::to_vec(&hello).unwrap())
                .await
                .unwrap();

            let server_hello_bytes = read_frame(&mut client_stream).await.unwrap();
            let server_hello: ServerHello = serde_json::from_slice(&server_hello_bytes).unwrap();
            let server_pub_bytes = BASE64.decode(&server_hello.server_ephemeral_pub).unwrap();
            let challenge = BASE64.decode(&server_hello.challenge).unwrap();

            let bad_hmac = compute_challenge_hmac(
                &handshake_key,
                &challenge,
                client_pub.as_bytes(),
                &server_pub_bytes,
            )
            .unwrap();

            let resp = ChallengeResponse {
                response: BASE64.encode(bad_hmac),
                preferred_cipher: "chacha20_poly1305".into(),
            };
            write_frame(&mut client_stream, &serde_json::to_vec(&resp).unwrap())
                .await
                .unwrap();

            read_frame(&mut client_stream).await
        });

        let (server_result, _client_result) = tokio::join!(server_handle, client_handle);
        let server_result = server_result.expect("join");

        assert!(
            server_result.is_err(),
            "Handshake should fail with wrong seed"
        );
    }

    #[tokio::test]
    async fn handshake_rejects_wrong_version() {
        let seed = FamilySeed::new(b"version-test-seed!!!!!!!!!!!!!!!".to_vec());

        let (mut client_stream, mut server_stream) = duplex(16384);

        let server_handle =
            tokio::spawn(async move { perform_server_handshake(&mut server_stream, &seed).await });

        let client_handle = tokio::spawn(async move {
            let (_, client_pub) = generate_ephemeral_keypair();
            let hello = ClientHello {
                version: 99,
                client_ephemeral_pub: BASE64.encode(client_pub.as_bytes()),
            };
            write_frame(&mut client_stream, &serde_json::to_vec(&hello).unwrap())
                .await
                .unwrap();

            read_frame(&mut client_stream).await
        });

        let (server_result, client_result) = tokio::join!(server_handle, client_handle);
        assert!(server_result.expect("join").is_err());

        // Client should receive a handshake_error frame
        let error_bytes = client_result.expect("join").expect("read error frame");
        let error: crate::btsp_handshake::HandshakeError =
            serde_json::from_slice(&error_bytes).expect("parse error");
        assert_eq!(error.reason, "unsupported_version");
    }

    #[tokio::test]
    async fn security_mode_development_skips_handshake() {
        let mode = crate::btsp_handshake::BtspSecurityMode::Development;
        assert!(!mode.is_production());
    }

    #[tokio::test]
    async fn security_mode_production_requires_seed() {
        let seed = FamilySeed::new(b"prod-seed".to_vec());
        let mode = crate::btsp_handshake::BtspSecurityMode::Production { family_seed: seed };
        assert!(mode.is_production());
    }

    #[test]
    #[serial_test::serial]
    fn resolve_security_mode_dev_when_no_family_id() {
        beardog_errors::process_env::remove_var("FAMILY_ID");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_ID");
        beardog_errors::process_env::remove_var("BIOMEOS_INSECURE");

        let mode = crate::btsp_handshake::resolve_security_mode().expect("resolve");
        assert!(!mode.is_production());
    }

    #[test]
    #[serial_test::serial]
    fn resolve_security_mode_rejects_insecure_with_family() {
        beardog_errors::process_env::set_var("FAMILY_ID", "production-family");
        beardog_errors::process_env::set_var("BIOMEOS_INSECURE", "1");
        beardog_errors::process_env::set_var("FAMILY_SEED", "some-seed-value");

        let result = crate::btsp_handshake::resolve_security_mode();

        beardog_errors::process_env::remove_var("FAMILY_ID");
        beardog_errors::process_env::remove_var("BIOMEOS_INSECURE");
        beardog_errors::process_env::remove_var("FAMILY_SEED");

        assert!(
            result.is_err(),
            "Should reject FAMILY_ID + BIOMEOS_INSECURE=1"
        );
    }

    #[test]
    fn cipher_suite_negotiation_roundtrip() {
        let tests = [
            ("chacha20_poly1305", BtspCipher::ChaCha20Poly1305),
            ("chacha20", BtspCipher::ChaCha20Poly1305),
            ("hmac_plain", BtspCipher::HmacPlain),
            ("null", BtspCipher::Null),
        ];
        for (name, expected) in tests {
            let cipher = BtspCipher::from_wire_name(name).expect(name);
            assert_eq!(cipher, expected);
        }
        assert!(BtspCipher::from_wire_name("unknown_cipher").is_err());
    }

    #[test]
    fn session_multiple_messages_with_nonce_progression() {
        let key_s2c = [0x11; 32];
        let key_c2s = [0x22; 32];
        let mut server = BtspSession::new_server(
            "test-sid".into(),
            BtspCipher::ChaCha20Poly1305,
            key_s2c,
            key_c2s,
        );
        let mut client = BtspSession::new_server(
            "test-sid".into(),
            BtspCipher::ChaCha20Poly1305,
            key_c2s,
            key_s2c,
        );

        for i in 0..100 {
            let msg = format!("message number {i}");
            let enc = server.encrypt_frame(msg.as_bytes()).expect("enc");
            let dec = client.decrypt_frame(&enc).expect("dec");
            assert_eq!(String::from_utf8(dec).unwrap(), msg);
        }
    }

    /// BTSP-E2E-01: Full handshake over real TCP sockets.
    ///
    /// Validates the complete BTSP handshake path that grapheneGate TCP-only
    /// deployments use: length-prefixed framing over a real TCP connection,
    /// not an in-memory duplex. After handshake, encrypted JSON-RPC roundtrip.
    #[tokio::test]
    async fn btsp_e2e_tcp_handshake_and_encrypted_jsonrpc() {
        use tokio::net::{TcpListener, TcpStream};

        let seed = b"e2e-tcp-family-seed-32bytes!!!!";
        let family_seed = FamilySeed::new(seed.to_vec());

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local addr");

        let server_seed = family_seed.clone();
        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let (mut rd, mut wr) = tokio::io::split(stream);
            let mut combined = tokio::io::join(&mut rd, &mut wr);
            perform_server_handshake(&mut combined, &server_seed)
                .await
                .expect("server handshake over TCP")
        });

        let client_handle = tokio::spawn(async move {
            let stream = TcpStream::connect(addr).await.expect("connect");
            let (mut rd, mut wr) = tokio::io::split(stream);
            let mut combined = tokio::io::join(&mut rd, &mut wr);
            client_handshake(&mut combined, seed).await
        });

        let (server_session, client_session) = tokio::join!(server_handle, client_handle);
        let mut server_session = server_session.expect("join server");
        let mut client_session = client_session.expect("join client");

        assert_eq!(server_session.session_id, client_session.session_id);
        assert_eq!(server_session.cipher, client_session.cipher);

        // Simulate encrypted JSON-RPC: client sends `health.liveness` request
        let jsonrpc_request =
            br#"{"jsonrpc":"2.0","method":"health.liveness","id":1}"#;
        let encrypted_req = client_session
            .encrypt_frame(jsonrpc_request)
            .expect("encrypt request");
        let decrypted_req = server_session
            .decrypt_frame(&encrypted_req)
            .expect("decrypt request");
        assert_eq!(decrypted_req, jsonrpc_request);

        // Server responds with encrypted JSON-RPC response
        let jsonrpc_response =
            br#"{"jsonrpc":"2.0","result":{"status":"ok","primal":"beardog"},"id":1}"#;
        let encrypted_resp = server_session
            .encrypt_frame(jsonrpc_response)
            .expect("encrypt response");
        let decrypted_resp = client_session
            .decrypt_frame(&encrypted_resp)
            .expect("decrypt response");
        assert_eq!(decrypted_resp, jsonrpc_response);
    }
}
