// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key generation helpers for the tarpc crypto server.

use crate::tarpc_types::{CryptoResult, KeyPair};

pub(super) async fn generate_ed25519() -> CryptoResult<KeyPair> {
    use ed25519_dalek::SigningKey;
    use rand::RngCore;

    let mut secret_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    Ok(KeyPair {
        public_key: verifying_key.as_bytes().to_vec(),
        private_key: signing_key.to_bytes().to_vec(),
    })
}

pub(super) async fn generate_x25519_ephemeral() -> CryptoResult<KeyPair> {
    use rand::RngCore;
    use x25519_dalek::{PublicKey, StaticSecret};

    let mut secret_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut secret_bytes);

    let static_secret = StaticSecret::from(secret_bytes);
    let public_key = PublicKey::from(&static_secret);

    Ok(KeyPair {
        public_key: public_key.as_bytes().to_vec(),
        private_key: static_secret.as_bytes().to_vec(),
    })
}

pub(super) async fn generate_ecdh_p256() -> CryptoResult<KeyPair> {
    use p256::SecretKey;
    use p256::elliptic_curve::rand_core::OsRng;

    let mut rng = OsRng;
    let secret_key = SecretKey::random(&mut rng);
    let public_key = secret_key.public_key();

    Ok(KeyPair {
        public_key: public_key.to_sec1_bytes().to_vec(),
        private_key: secret_key.to_bytes().to_vec(),
    })
}

pub(super) async fn generate_ecdh_p384() -> CryptoResult<KeyPair> {
    use p384::SecretKey;
    use p384::elliptic_curve::rand_core::OsRng;

    let mut rng = OsRng;
    let secret = SecretKey::random(&mut rng);
    let public = secret.public_key();

    Ok(KeyPair {
        public_key: public.to_sec1_bytes().to_vec(),
        private_key: secret.to_bytes().to_vec(),
    })
}
