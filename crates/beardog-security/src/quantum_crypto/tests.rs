// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for quantum crypto module

use super::*;

#[cfg(test)]
mod quantum_crypto_tests {
    use super::*;

    #[test]
    fn test_security_levels() {
        assert!(SecurityLevel::Level1 < SecurityLevel::Level3);
        assert!(SecurityLevel::Level3 < SecurityLevel::Level5);
        assert_eq!(SecurityLevel::default(), SecurityLevel::Level3);
    }

    #[test]
    fn test_algorithm_defaults() {
        assert_eq!(KemAlgorithm::default(), KemAlgorithm::Kyber768);
        assert_eq!(
            SignatureAlgorithm::default(),
            SignatureAlgorithm::Dilithium3
        );
    }

    #[test]
    fn test_quantum_private_key() {
        let key = QuantumPrivateKey::new(vec![1, 2, 3, 4], "test-algo");
        assert_eq!(key.algorithm(), "test-algo");
        assert_eq!(key.len(), 4);
        assert!(!key.is_empty());

        // Debug should redact key data
        let debug = format!("{key:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("1, 2, 3, 4"));
    }

    #[test]
    fn test_kyber_engine() {
        let engine = KyberEngine::new(SecurityLevel::Level3).unwrap();

        let keypair = engine.generate_keypair().unwrap();
        assert!(!keypair.public_key.is_empty());
        assert!(keypair.private_key.is_some());
        assert_eq!(keypair.algorithm, KemAlgorithm::Kyber768);

        let exchange = engine.encapsulate(&keypair.public_key).unwrap();
        assert!(!exchange.shared_secret.is_empty());
        assert!(!exchange.ciphertext.is_empty());

        let secret = engine
            .decapsulate(
                &exchange.ciphertext,
                keypair.private_key.as_ref().unwrap().key_data(),
            )
            .unwrap();
        assert!(!secret.is_empty());
    }

    #[test]
    fn test_dilithium_engine() {
        let engine = DilithiumEngine::new(SecurityLevel::Level3).unwrap();

        let keypair = engine.generate_keypair().unwrap();
        assert!(!keypair.public_key.is_empty());
        assert!(keypair.private_key.is_some());

        let sig = engine
            .sign(
                keypair.private_key.as_ref().unwrap().key_data(),
                b"test message",
            )
            .unwrap();
        assert!(!sig.signature.is_empty());

        let valid = engine
            .verify(&keypair.public_key, b"test message", &sig.signature)
            .unwrap();
        assert!(valid);
    }

    #[test]
    fn test_sphincs_engine() {
        let engine = SphincsEngine::new(SecurityLevel::Level5).unwrap();

        let keypair = engine.generate_keypair().unwrap();
        assert_eq!(keypair.algorithm, SignatureAlgorithm::SphincsPlus);

        let sig = engine
            .sign(
                keypair.private_key.as_ref().unwrap().key_data(),
                b"test message",
            )
            .unwrap();
        // SPHINCS+ signatures are large
        assert!(sig.signature.len() > 10_000);
    }

    #[test]
    fn test_quantum_crypto_engine() {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level3).unwrap();

        // KEM operations
        let kem = engine.generate_kem_keypair().unwrap();
        let exchange = engine.encapsulate(&kem.public_key).unwrap();
        let _ = engine
            .decapsulate(
                &exchange.ciphertext,
                kem.private_key.as_ref().unwrap().key_data(),
            )
            .unwrap();

        // Signature operations
        let sig_keypair = engine.generate_signature_keypair().unwrap();
        let sig = engine.sign(&sig_keypair, b"message").unwrap();
        let valid = engine
            .verify(&sig_keypair, b"message", &sig.signature)
            .unwrap();
        assert!(valid);

        // Metrics
        assert!(engine.operations_count() >= 5);
        assert!(engine.is_hybrid_mode());
    }

    #[test]
    fn test_engine_with_sphincs() {
        let engine = QuantumCryptoEngine::new(SecurityLevel::Level5).unwrap();

        let keypair = engine
            .generate_signature_keypair_with(SignatureAlgorithm::SphincsPlus)
            .unwrap();
        assert_eq!(keypair.algorithm, SignatureAlgorithm::SphincsPlus);
    }

    #[test]
    fn test_security_level_selection() {
        // Level 1-2 -> Kyber512
        let engine = KyberEngine::new(SecurityLevel::Level1).unwrap();
        let kp = engine.generate_keypair().unwrap();
        assert_eq!(kp.algorithm, KemAlgorithm::Kyber512);

        // Level 3 -> Kyber768
        let engine = KyberEngine::new(SecurityLevel::Level3).unwrap();
        let kp = engine.generate_keypair().unwrap();
        assert_eq!(kp.algorithm, KemAlgorithm::Kyber768);

        // Level 4-5 -> Kyber1024
        let engine = KyberEngine::new(SecurityLevel::Level5).unwrap();
        let kp = engine.generate_keypair().unwrap();
        assert_eq!(kp.algorithm, KemAlgorithm::Kyber1024);
    }
}
