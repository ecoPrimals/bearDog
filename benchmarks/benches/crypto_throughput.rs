// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tower Atomic — Crypto Throughput Benchmark
//!
//! Measures raw encrypt/decrypt throughput for the crypto primitives bearDog
//! provides to the Tower Atomic stack. songBird uses these numbers for the
//! parity assessment against WireGuard.
//!
//! Run: `cargo bench --bench crypto_throughput -p benchmarks`

#![allow(missing_docs, clippy::all, clippy::unwrap_used, clippy::expect_used)]

use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit,
    aead::{Aead, AeadCore, OsRng},
};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use ed25519_dalek::{Signer, Verifier};
use std::time::Duration;

const PAYLOAD_SIZES: &[(&str, usize)] = &[
    ("64B", 64),
    ("256B", 256),
    ("1KB", 1024),
    ("4KB", 4096),
    ("16KB", 16_384),
    ("64KB", 65_536),
    ("256KB", 262_144),
    ("1MB", 1_048_576),
];

// ---------------------------------------------------------------------------
// ChaCha20-Poly1305 encrypt/decrypt (Tower Atomic session cipher)
// ---------------------------------------------------------------------------

fn bench_chacha20_poly1305_encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("chacha20poly1305_encrypt");

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    for &(label, size) in PAYLOAD_SIZES {
        let plaintext = vec![0xABu8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(label), &plaintext, |b, pt| {
            b.iter(|| {
                let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
                let ct = cipher.encrypt(&nonce, pt.as_ref()).expect("encrypt");
                black_box(ct);
            });
        });
    }
    group.finish();
}

fn bench_chacha20_poly1305_decrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("chacha20poly1305_decrypt");

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    for &(label, size) in PAYLOAD_SIZES {
        let plaintext = vec![0xCDu8; size];
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .expect("pre-encrypt");

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(label),
            &(&nonce, &ciphertext),
            |b, &(n, ct)| {
                b.iter(|| {
                    let pt = cipher.decrypt(n, ct.as_ref()).expect("decrypt");
                    black_box(pt);
                });
            },
        );
    }
    group.finish();
}

fn bench_chacha20_poly1305_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("chacha20poly1305_roundtrip");

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    for &(label, size) in PAYLOAD_SIZES {
        let plaintext = vec![0xEFu8; size];
        group.throughput(Throughput::Bytes(size as u64 * 2));

        group.bench_with_input(BenchmarkId::from_parameter(label), &plaintext, |b, pt| {
            b.iter(|| {
                let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
                let ct = cipher.encrypt(&nonce, pt.as_ref()).expect("encrypt");
                let recovered = cipher.decrypt(&nonce, ct.as_ref()).expect("decrypt");
                black_box(recovered);
            });
        });
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// AES-256-GCM encrypt/decrypt (alternative AEAD for comparison)
// ---------------------------------------------------------------------------

fn bench_aes256gcm_encrypt(c: &mut Criterion) {
    use aes_gcm::{Aes256Gcm, KeyInit as _, aead::Aead as _};

    let mut group = c.benchmark_group("aes256gcm_encrypt");
    let key = aes_gcm::Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    for &(label, size) in PAYLOAD_SIZES {
        let plaintext = vec![0x42u8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(label), &plaintext, |b, pt| {
            b.iter(|| {
                let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng);
                let ct = cipher.encrypt(&nonce, pt.as_ref()).expect("encrypt");
                black_box(ct);
            });
        });
    }
    group.finish();
}

fn bench_aes256gcm_decrypt(c: &mut Criterion) {
    use aes_gcm::{Aes256Gcm, KeyInit as _, aead::Aead as _};

    let mut group = c.benchmark_group("aes256gcm_decrypt");
    let key = aes_gcm::Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    for &(label, size) in PAYLOAD_SIZES {
        let plaintext = vec![0x99u8; size];
        let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .expect("pre-encrypt");

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(label),
            &(&nonce, &ciphertext),
            |b, &(n, ct)| {
                b.iter(|| {
                    let pt = cipher.decrypt(n, ct.as_ref()).expect("decrypt");
                    black_box(pt);
                });
            },
        );
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// Ed25519 sign/verify (identity + enrollment signatures)
// ---------------------------------------------------------------------------

fn bench_ed25519_sign(c: &mut Criterion) {
    let mut group = c.benchmark_group("ed25519_sign");

    let mut rng_bytes = [0u8; 32];
    rand::Fill::fill(&mut rng_bytes, &mut rand::rng());
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&rng_bytes);

    for &(label, size) in &PAYLOAD_SIZES[..5] {
        let message = vec![0x11u8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(label), &message, |b, msg| {
            b.iter(|| {
                let sig = signing_key.sign(msg);
                black_box(sig);
            });
        });
    }
    group.finish();
}

fn bench_ed25519_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("ed25519_verify");

    let mut rng_bytes = [0u8; 32];
    rand::Fill::fill(&mut rng_bytes, &mut rand::rng());
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&rng_bytes);
    let verifying_key = signing_key.verifying_key();

    for &(label, size) in &PAYLOAD_SIZES[..5] {
        let message = vec![0x22u8; size];
        let signature = signing_key.sign(&message);

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(label),
            &(&message, &signature),
            |b, &(msg, sig)| {
                b.iter(|| {
                    let _ = black_box(verifying_key.verify(msg, sig));
                });
            },
        );
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// X25519 key exchange (BTSP handshake DH)
// ---------------------------------------------------------------------------

fn bench_x25519_key_exchange(c: &mut Criterion) {
    let mut group = c.benchmark_group("x25519_key_exchange");

    group.bench_function("diffie_hellman", |b| {
        b.iter(|| {
            let alice_secret = x25519_dalek::EphemeralSecret::random_from_rng(OsRng);
            let alice_public = x25519_dalek::PublicKey::from(&alice_secret);

            let bob_secret = x25519_dalek::EphemeralSecret::random_from_rng(OsRng);
            let bob_public = x25519_dalek::PublicKey::from(&bob_secret);

            let alice_shared = alice_secret.diffie_hellman(&bob_public);
            let _bob_shared = bob_secret.diffie_hellman(&alice_public);

            black_box(alice_shared);
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// HKDF-SHA256 key derivation (session key export)
// ---------------------------------------------------------------------------

fn bench_hkdf_derive(c: &mut Criterion) {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let mut group = c.benchmark_group("hkdf_sha256_derive");

    let ikm = [0xABu8; 32];
    let salt = b"beardog-session-v1";

    let key_counts = [1u32, 2, 4, 8];

    for count in key_counts {
        group.bench_with_input(BenchmarkId::new("keys", count), &count, |b, &n| {
            b.iter(|| {
                let hk = Hkdf::<Sha256>::new(Some(salt), &ikm);
                for i in 0..n {
                    let info = format!("session-key-{i}");
                    let mut okm = [0u8; 32];
                    hk.expand(info.as_bytes(), &mut okm).expect("expand");
                    black_box(okm);
                }
            });
        });
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// HMAC-SHA256 (enrollment proof computation)
// ---------------------------------------------------------------------------

fn bench_hmac_sha256(c: &mut Criterion) {
    use hmac::Mac;

    let mut group = c.benchmark_group("hmac_sha256");
    let key = [0x33u8; 32];

    for &(label, size) in &PAYLOAD_SIZES[..6] {
        let data = vec![0x44u8; size];
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::from_parameter(label), &data, |b, d| {
            b.iter(|| {
                let mut mac = <hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(&key).expect("key");
                mac.update(d);
                let result = mac.finalize().into_bytes();
                black_box(result);
            });
        });
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// Criterion configuration
// ---------------------------------------------------------------------------

criterion_group! {
    name = crypto_throughput;
    config = {
        let c = Criterion::default()
            .measurement_time(Duration::from_secs(5))
            .warm_up_time(Duration::from_secs(2))
            .sample_size(100);
        #[cfg(feature = "profiling")]
        let c = c.with_profiler(pprof::criterion::PProfProfiler::new(100, pprof::criterion::Output::Flamegraph(None)));
        c
    };
    targets =
        bench_chacha20_poly1305_encrypt,
        bench_chacha20_poly1305_decrypt,
        bench_chacha20_poly1305_roundtrip,
        bench_aes256gcm_encrypt,
        bench_aes256gcm_decrypt,
        bench_ed25519_sign,
        bench_ed25519_verify,
        bench_x25519_key_exchange,
        bench_hkdf_derive,
        bench_hmac_sha256,
}

criterion_main!(crypto_throughput);
