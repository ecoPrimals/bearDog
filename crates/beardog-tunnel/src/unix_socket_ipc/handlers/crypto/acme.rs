// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME Phase 2 crypto delegation surface for `songBird`.
//!
//! Provides the JSON-RPC crypto operations that an external ACME client
//! (songBird) needs to delegate to `bearDog` as the trust foundation:
//!
//! - [`handle_ecdsa_p256_generate_signing_keypair`] — persistent ECDSA P-256 signing keypair
//! - [`handle_sign_jws_es256`] — JWS ES256 signing (RFC 7515, raw 64-byte `r||s`)
//! - [`handle_jwk_thumbprint`] — JWK thumbprint (RFC 7638, SHA-256)
//! - [`handle_build_csr`] — PKCS#10 CSR generation (RFC 2986)
//! - [`handle_parse_certificate`] — X.509 certificate metadata extraction
//!
//! All operations are pure Rust (`RustCrypto`), constant-time, and zeroized.

use base64::Engine;
use base64::engine::general_purpose::{STANDARD as BASE64, URL_SAFE_NO_PAD as BASE64URL};
use p256::ecdsa::{
    SigningKey, VerifyingKey,
    signature::Signer,
};
use serde_json::{Value, json};
use sha2::{Sha256, Digest};
use tracing::{debug, info};
use zeroize::Zeroizing;

use crate::unix_socket_ipc::handlers::HandlerError;

/// Generate an ECDSA P-256 signing keypair.
///
/// Unlike `crypto.sign_ecdsa_secp256r1` (ephemeral), this returns the private key
/// so the caller can persist it for ACME account registration or TLS leaf certs.
///
/// # RPC Method: `crypto.ecdsa_p256_generate_signing_keypair`
///
/// # Returns
///
/// ```json
/// {
///   "private_key": "base64_32_byte_scalar",
///   "public_key": "base64_uncompressed_sec1_point",
///   "public_key_jwk": { "kty": "EC", "crv": "P-256", "x": "...", "y": "..." },
///   "algorithm": "ES256",
///   "curve": "P-256"
/// }
/// ```
///
/// # Errors
///
/// Returns an error if key generation fails.
pub async fn handle_ecdsa_p256_generate_signing_keypair(
    _params: Option<&Value>,
) -> Result<Value, HandlerError> {
    info!("Crypto: ecdsa_p256_generate_signing_keypair (persistent ACME key)");

    let signing_key = SigningKey::random(&mut p256::elliptic_curve::rand_core::OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);

    let private_bytes = Zeroizing::new(signing_key.to_bytes());
    let private_b64 = BASE64.encode(*private_bytes);

    let point = verifying_key.to_encoded_point(false);
    let public_b64 = BASE64.encode(point.as_bytes());

    let x_bytes = point.x().ok_or("P-256 key missing x coordinate")?;
    let y_bytes = point.y().ok_or("P-256 key missing y coordinate")?;
    let x_b64url = BASE64URL.encode(x_bytes);
    let y_b64url = BASE64URL.encode(y_bytes);

    debug!(
        "Generated P-256 signing keypair: public_key {} bytes",
        point.as_bytes().len()
    );

    Ok(json!({
        "private_key": private_b64,
        "public_key": public_b64,
        "public_key_jwk": {
            "kty": "EC",
            "crv": "P-256",
            "x": x_b64url,
            "y": y_b64url,
        },
        "algorithm": "ES256",
        "curve": "P-256",
    }))
}

/// Sign data with ECDSA P-256 in JWS ES256 format (raw 64-byte `r||s`).
///
/// ACME JWS (RFC 7515) requires raw `(r || s)` concatenation, not ASN.1 DER.
/// This is the format songBird needs for ACME request signing.
///
/// # RPC Method: `crypto.sign_jws_es256`
///
/// # Parameters
///
/// ```json
/// {
///   "signing_input": "base64_encoded_bytes_to_sign",
///   "private_key": "base64_encoded_32_byte_scalar"
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "signature": "base64url_encoded_64_byte_r_s",
///   "algorithm": "ES256"
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the private key is invalid or signing fails.
pub async fn handle_sign_jws_es256(
    params: Option<&Value>,
) -> Result<Value, HandlerError> {
    info!("Crypto: sign_jws_es256 (ACME JWS signing)");

    let params = params.ok_or("Missing parameters for JWS ES256 signing")?;

    let input_b64 = params
        .get("signing_input")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'signing_input' parameter")?;

    let key_b64 = params
        .get("private_key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'private_key' parameter")?;

    let input_bytes = BASE64
        .decode(input_b64)
        .map_err(|e| format!("Invalid base64 signing_input: {e}"))?;

    let key_bytes = Zeroizing::new(
        BASE64
            .decode(key_b64)
            .map_err(|e| format!("Invalid base64 private_key: {e}"))?,
    );

    let signing_key = SigningKey::from_bytes(key_bytes.as_slice().into())
        .map_err(|e| format!("Invalid P-256 private key: {e}"))?;

    let signature: p256::ecdsa::Signature = signing_key
        .try_sign(&input_bytes)
        .map_err(|e| format!("ES256 signing failed: {e}"))?;

    let sig_b64url = BASE64URL.encode(signature.to_bytes());

    debug!("JWS ES256: signed {} bytes → 64-byte signature", input_bytes.len());

    Ok(json!({
        "signature": sig_b64url,
        "algorithm": "ES256",
    }))
}

/// Compute a JWK thumbprint (RFC 7638) using SHA-256.
///
/// Used for ACME HTTP-01 challenge `keyAuthorization = token || '.' || thumbprint`.
///
/// # RPC Method: `crypto.jwk_thumbprint`
///
/// # Parameters
///
/// ```json
/// {
///   "public_key_jwk": { "kty": "EC", "crv": "P-256", "x": "...", "y": "..." }
/// }
/// ```
///
/// Or provide the raw public key and let bearDog build the JWK:
///
/// ```json
/// {
///   "public_key": "base64_uncompressed_sec1_point"
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "thumbprint": "base64url_sha256_thumbprint"
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the JWK is malformed or the public key is invalid.
pub async fn handle_jwk_thumbprint(
    params: Option<&Value>,
) -> Result<Value, HandlerError> {
    info!("Crypto: jwk_thumbprint (RFC 7638)");

    let params = params.ok_or("Missing parameters for JWK thumbprint")?;

    let (x_b64url, y_b64url) = if let Some(jwk) = params.get("public_key_jwk") {
        let x = jwk
            .get("x")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'x' in JWK")?;
        let y = jwk
            .get("y")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'y' in JWK")?;
        (x.to_string(), y.to_string())
    } else if let Some(pk_b64) = params.get("public_key").and_then(|v| v.as_str()) {
        let pk_bytes = BASE64
            .decode(pk_b64)
            .map_err(|e| format!("Invalid base64 public_key: {e}"))?;
        let vk = VerifyingKey::from_sec1_bytes(&pk_bytes)
            .map_err(|e| format!("Invalid P-256 public key: {e}"))?;
        let point = vk.to_encoded_point(false);
        let x = point.x().ok_or("Missing x coordinate")?;
        let y = point.y().ok_or("Missing y coordinate")?;
        (BASE64URL.encode(x), BASE64URL.encode(y))
    } else {
        return Err("Provide 'public_key_jwk' or 'public_key'".into());
    };

    // RFC 7638: lexicographically sorted, no whitespace
    let canonical = format!(
        r#"{{"crv":"P-256","kty":"EC","x":"{x_b64url}","y":"{y_b64url}"}}"#
    );

    let hash = Sha256::digest(canonical.as_bytes());
    let thumbprint = BASE64URL.encode(hash);

    debug!("JWK thumbprint computed: {}", &thumbprint[..8]);

    Ok(json!({
        "thumbprint": thumbprint,
    }))
}

/// Build a PKCS#10 Certificate Signing Request (RFC 2986).
///
/// Generates a new ECDSA P-256 key and CSR for the given domains.
/// songBird uses this to request certificates from ACME CAs.
///
/// # RPC Method: `x509.build_csr`
///
/// # Parameters
///
/// ```json
/// {
///   "domains": ["example.com", "www.example.com"],
///   "common_name": "example.com"
/// }
/// ```
///
/// Optionally provide an existing private key:
///
/// ```json
/// {
///   "domains": ["example.com"],
///   "common_name": "example.com",
///   "private_key": "base64_encoded_32_byte_scalar"
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "csr_der": "base64_encoded_der_csr",
///   "private_key": "base64_encoded_private_key",
///   "public_key": "base64_encoded_public_key",
///   "algorithm": "ECDSA-P256-SHA256",
///   "domains": ["example.com", "www.example.com"]
/// }
/// ```
///
/// # Errors
///
/// Returns an error if CSR construction fails or the private key is invalid.
pub async fn handle_build_csr(
    params: Option<&Value>,
) -> Result<Value, HandlerError> {
    info!("X509: build_csr (PKCS#10 for ACME)");

    let params = params.ok_or("Missing parameters for CSR generation")?;

    let domains = params
        .get("domains")
        .and_then(|v| v.as_array())
        .ok_or("Missing 'domains' array parameter")?;

    if domains.is_empty() {
        return Err("'domains' array must not be empty".into());
    }

    let domain_strings: Vec<String> = domains
        .iter()
        .map(|v| {
            v.as_str()
                .map(String::from)
                .ok_or_else(|| "Each domain must be a string".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let cn = params
        .get("common_name")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| domain_strings[0].as_str());

    let signing_key = if let Some(key_b64) = params.get("private_key").and_then(|v| v.as_str()) {
        let key_bytes = Zeroizing::new(
            BASE64
                .decode(key_b64)
                .map_err(|e| format!("Invalid base64 private_key: {e}"))?,
        );
        SigningKey::from_bytes(key_bytes.as_slice().into())
            .map_err(|e| format!("Invalid P-256 private key: {e}"))?
    } else {
        SigningKey::random(&mut p256::elliptic_curve::rand_core::OsRng)
    };

    let verifying_key = VerifyingKey::from(&signing_key);

    let csr_der = build_pkcs10_csr(cn, &domain_strings, &signing_key)?;

    let private_bytes = Zeroizing::new(signing_key.to_bytes());
    let point = verifying_key.to_encoded_point(false);

    debug!(
        "Built CSR: CN={}, SANs={}, DER={} bytes",
        cn,
        domain_strings.len(),
        csr_der.len()
    );

    Ok(json!({
        "csr_der": BASE64.encode(&csr_der),
        "private_key": BASE64.encode(*private_bytes),
        "public_key": BASE64.encode(point.as_bytes()),
        "algorithm": "ECDSA-P256-SHA256",
        "domains": domain_strings,
    }))
}

fn build_pkcs10_csr(
    cn: &str,
    domains: &[String],
    signing_key: &SigningKey,
) -> Result<Vec<u8>, HandlerError> {
    use x509_cert::builder::{Builder, RequestBuilder};
    use x509_cert::der::Encode;
    use x509_cert::der::asn1::Ia5String;
    use x509_cert::name::Name;
    use x509_cert::ext::pkix::SubjectAltName;
    use x509_cert::ext::pkix::name::GeneralName;

    let subject: Name = format!("CN={cn}")
        .parse()
        .map_err(|e| format!("Invalid CN '{cn}': {e}"))?;

    let mut builder = RequestBuilder::new(subject, signing_key)
        .map_err(|e| format!("CSR builder init failed: {e}"))?;

    let mut sans = Vec::new();
    for d in domains {
        let ia5 = Ia5String::new(d)
            .map_err(|e| format!("Invalid domain '{d}': {e}"))?;
        sans.push(GeneralName::DnsName(ia5));
    }

    if !sans.is_empty() {
        let san_ext = SubjectAltName(sans);
        builder
            .add_extension(&san_ext)
            .map_err(|e| format!("Failed to add SAN extension: {e}"))?;
    }

    let csr = builder
        .build::<p256::ecdsa::DerSignature>()
        .map_err(|e| format!("CSR signing failed: {e}"))?;

    let der_bytes = csr
        .to_der()
        .map_err(|e| format!("CSR DER encoding failed: {e}"))?;

    Ok(der_bytes)
}

/// Parse an X.509 certificate and extract metadata.
///
/// Returns structured metadata without full chain verification — use
/// `tls.verify_certificate` for validation. This method is for renewal
/// monitoring and certificate inventory.
///
/// # RPC Method: `x509.parse_certificate`
///
/// # Parameters
///
/// ```json
/// {
///   "certificate": "base64_encoded_der_certificate"
/// }
/// ```
///
/// Or PEM:
///
/// ```json
/// {
///   "certificate_pem": "-----BEGIN CERTIFICATE-----\n..."
/// }
/// ```
///
/// # Returns
///
/// ```json
/// {
///   "subject": "CN=example.com",
///   "issuer": "CN=Let's Encrypt Authority X3",
///   "not_before": 1706140800,
///   "not_after": 1714003200,
///   "serial_number": "03:a1:...",
///   "san_entries": ["example.com", "www.example.com"],
///   "signature_algorithm": "ecdsa-with-SHA256",
///   "version": 3,
///   "is_ca": false
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the certificate cannot be parsed.
#[cfg(feature = "tls-x509")]
pub async fn handle_parse_certificate(
    params: Option<&Value>,
) -> Result<Value, HandlerError> {
    use x509_parser::prelude::FromDer;
    use x509_parser::certificate::X509Certificate;
    use x509_parser::extensions::{GeneralName, ParsedExtension};
    use x509_parser::oid_registry;

    info!("X509: parse_certificate (metadata extraction)");

    let params = params.ok_or("Missing parameters for certificate parsing")?;

    let cert_der = if let Some(b64) = params.get("certificate").and_then(|v| v.as_str()) {
        BASE64
            .decode(b64)
            .map_err(|e| format!("Invalid base64 certificate: {e}"))?
    } else if let Some(pem) = params.get("certificate_pem").and_then(|v| v.as_str()) {
        extract_der_from_pem(pem)?
    } else {
        return Err("Provide 'certificate' (base64 DER) or 'certificate_pem'".into());
    };

    let (_, cert) = X509Certificate::from_der(&cert_der)
        .map_err(|e| format!("Failed to parse X.509 certificate: {e}"))?;

    let not_before = cert.validity().not_before.timestamp();
    let not_after = cert.validity().not_after.timestamp();
    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();
    let serial = format!("{}", cert.serial);
    let version = cert.version().0 + 1; // X.509 version is 0-indexed
    let sig_alg = cert.signature_algorithm.algorithm.to_string();

    let mut san_entries = Vec::new();
    if let Ok(Some(san_ext)) =
        cert.get_extension_unique(&oid_registry::OID_X509_EXT_SUBJECT_ALT_NAME)
        && let ParsedExtension::SubjectAlternativeName(san) = san_ext.parsed_extension() {
            for name in &san.general_names {
                if let GeneralName::DNSName(dns) = name {
                    san_entries.push((*dns).to_string());
                }
            }
        }

    let is_ca = cert
        .get_extension_unique(&oid_registry::OID_X509_EXT_BASIC_CONSTRAINTS)
        .ok()
        .flatten()
        .is_some_and(|bc_ext| matches!(bc_ext.parsed_extension(), ParsedExtension::BasicConstraints(bc) if bc.ca));

    debug!(
        "Parsed cert: subject={}, expires={}, SANs={}",
        subject, not_after, san_entries.len()
    );

    Ok(json!({
        "subject": subject,
        "issuer": issuer,
        "not_before": not_before,
        "not_after": not_after,
        "serial_number": serial,
        "san_entries": san_entries,
        "signature_algorithm": sig_alg,
        "version": version,
        "is_ca": is_ca,
    }))
}

#[cfg(feature = "tls-x509")]
fn extract_der_from_pem(pem_str: &str) -> Result<Vec<u8>, HandlerError> {
    let b64_content: String = pem_str
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect();
    BASE64
        .decode(b64_content.trim())
        .map_err(|e| format!("Invalid PEM content: {e}").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn generate_signing_keypair_returns_jwk() {
        let result = handle_ecdsa_p256_generate_signing_keypair(None)
            .await
            .expect("keygen");
        assert_eq!(result["algorithm"], "ES256");
        assert_eq!(result["curve"], "P-256");
        assert!(result["private_key"].as_str().is_some());
        assert!(result["public_key"].as_str().is_some());
        let jwk = &result["public_key_jwk"];
        assert_eq!(jwk["kty"], "EC");
        assert_eq!(jwk["crv"], "P-256");
        assert!(jwk["x"].as_str().is_some());
        assert!(jwk["y"].as_str().is_some());
    }

    #[tokio::test]
    async fn sign_jws_es256_roundtrip() {
        let keypair = handle_ecdsa_p256_generate_signing_keypair(None)
            .await
            .expect("keygen");
        let priv_key = keypair["private_key"].as_str().unwrap();

        let signing_input = BASE64.encode(b"test.payload");
        let params = json!({
            "signing_input": signing_input,
            "private_key": priv_key,
        });
        let result = handle_sign_jws_es256(Some(&params))
            .await
            .expect("sign");
        assert_eq!(result["algorithm"], "ES256");
        let sig_b64url = result["signature"].as_str().unwrap();
        let sig_bytes = BASE64URL.decode(sig_b64url).expect("decode sig");
        assert_eq!(sig_bytes.len(), 64, "JWS ES256 must be 64 bytes (r||s)");
    }

    #[tokio::test]
    async fn sign_jws_es256_rejects_missing_params() {
        assert!(handle_sign_jws_es256(None).await.is_err());
    }

    #[tokio::test]
    async fn sign_jws_es256_rejects_bad_key() {
        let params = json!({
            "signing_input": BASE64.encode(b"data"),
            "private_key": "not-valid-base64!!!",
        });
        assert!(handle_sign_jws_es256(Some(&params)).await.is_err());
    }

    #[tokio::test]
    async fn jwk_thumbprint_from_jwk() {
        let keypair = handle_ecdsa_p256_generate_signing_keypair(None)
            .await
            .expect("keygen");
        let jwk = keypair["public_key_jwk"].clone();
        let params = json!({ "public_key_jwk": jwk });
        let result = handle_jwk_thumbprint(Some(&params))
            .await
            .expect("thumbprint");
        let tp = result["thumbprint"].as_str().unwrap();
        assert!(!tp.is_empty());
        let tp_bytes = BASE64URL.decode(tp).expect("decode");
        assert_eq!(tp_bytes.len(), 32, "SHA-256 thumbprint is 32 bytes");
    }

    #[tokio::test]
    async fn jwk_thumbprint_from_public_key() {
        let keypair = handle_ecdsa_p256_generate_signing_keypair(None)
            .await
            .expect("keygen");
        let pk = keypair["public_key"].as_str().unwrap();
        let params = json!({ "public_key": pk });
        let result = handle_jwk_thumbprint(Some(&params))
            .await
            .expect("thumbprint");
        assert!(result["thumbprint"].as_str().is_some());
    }

    #[tokio::test]
    async fn jwk_thumbprint_deterministic() {
        let keypair = handle_ecdsa_p256_generate_signing_keypair(None)
            .await
            .expect("keygen");
        let jwk = keypair["public_key_jwk"].clone();
        let pk = keypair["public_key"].as_str().unwrap();

        let tp1 = handle_jwk_thumbprint(Some(&json!({ "public_key_jwk": jwk })))
            .await
            .expect("tp1")["thumbprint"]
            .as_str()
            .unwrap()
            .to_string();
        let tp2 = handle_jwk_thumbprint(Some(&json!({ "public_key": pk })))
            .await
            .expect("tp2")["thumbprint"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(tp1, tp2, "JWK and raw public key must produce same thumbprint");
    }

    #[tokio::test]
    async fn build_csr_basic() {
        let params = json!({
            "domains": ["example.com", "www.example.com"],
            "common_name": "example.com",
        });
        let result = handle_build_csr(Some(&params)).await.expect("csr");
        assert_eq!(result["algorithm"], "ECDSA-P256-SHA256");
        assert!(result["csr_der"].as_str().is_some());
        assert!(result["private_key"].as_str().is_some());
        assert!(result["public_key"].as_str().is_some());
        let domains = result["domains"].as_array().unwrap();
        assert_eq!(domains.len(), 2);
    }

    #[tokio::test]
    async fn build_csr_with_existing_key() {
        let keypair = handle_ecdsa_p256_generate_signing_keypair(None)
            .await
            .expect("keygen");
        let priv_key = keypair["private_key"].as_str().unwrap();
        let params = json!({
            "domains": ["test.example.com"],
            "private_key": priv_key,
        });
        let result = handle_build_csr(Some(&params)).await.expect("csr");
        assert_eq!(
            result["private_key"].as_str().unwrap(),
            priv_key,
            "should use provided key"
        );
    }

    #[tokio::test]
    async fn build_csr_rejects_empty_domains() {
        let params = json!({ "domains": [] });
        assert!(handle_build_csr(Some(&params)).await.is_err());
    }

    #[tokio::test]
    async fn build_csr_rejects_missing_params() {
        assert!(handle_build_csr(None).await.is_err());
    }

    #[cfg(feature = "tls-x509")]
    mod x509_tests {
        use super::*;

        const TEST_CERT_DER_B64: &str = "MIIDNDCCAhygAwIBAgIUf+M7I0CH2ZID6J0thpRUase1uBEwDQYJKoZIhvcNAQELBQAwGzEZMBcGA1UEAwwQdGVzdC5leGFtcGxlLmNvbTAeFw0yNjAzMjcxOTQxMTNaFw0yODA2MjkxOTQxMTNaMBsxGTAXBgNVBAMMEHRlc3QuZXhhbXBsZS5jb20wggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCTewKgJskFWRc0UyPWlK971NNatMnxcXRmHSgSoy6zoQSroHa8q0iwoXqP4iy5x5FXsIHy48LoYEjFaZurWQYdgKgu+PVScYes5fjGY1y9ekiJu8w1d3w3ACPjJdJgL/imcVE5Bt23Eipe/o96wcCle7zCq5Od3KOJmJQtrjVdty8WaKL9AJc+pbARiE+85uyIgist/7vw3Crz7rd9i2R9F452GyaeEVR7ujcEzuctu0+/qJf/I6nSKzPQlh9KUeb4iseczwCE2pCyDIOJ2bMf4FLo8edHl9NYhucin90MuQQ9vgpQuWHxbvCLYMN2m17PPa51JtLV/et4gkkDe/zNAgMBAAGjcDBuMB0GA1UdDgQWBBT0usnXENpMnE3jfAbxup+2BHSC/zAfBgNVHSMEGDAWgBT0usnXENpMnE3jfAbxup+2BHSC/zAPBgNVHRMBAf8EBTADAQH/MBsGA1UdEQQUMBKCEHRlc3QuZXhhbXBsZS5jb20wDQYJKoZIhvcNAQELBQADggEBAFMVy0sug1e8xq3j1UibW9+WuX5wZ1v9AoXRBiaPyWeAPa1994r/4Qz9w2M0+U4os9EqF/zSOi7S0rLot6b6igbhNk8WrLoleQXVB/BTYAZY7gqxaclZVC1CpjJEDoWFMPN7QQvqUiddLrlPd+t75KxDGQgK5YrA+h93UJY3/auWEdY95+pwPaOgQFnw+Ng+VJIa490GPrQR1LH5pfinu0q2sY+acSFhbU7TsXzXihESBtrJ9YkTbEImtuk8aVOeENreLLPEbfNHLIsGwM88Fre5ci8TUtwA86QTbQiyAz0GeJjsWCOWRYEQ53vxJdTJCXKgzVngfNN5qfyfzTNrC14=";

        #[tokio::test]
        async fn parse_certificate_from_der() {
            let params = json!({ "certificate": TEST_CERT_DER_B64 });
            let result = handle_parse_certificate(Some(&params))
                .await
                .expect("parse");
            assert!(result["subject"].as_str().unwrap().contains("test.example.com"));
            assert!(result["issuer"].as_str().is_some());
            assert!(result["not_before"].as_i64().is_some());
            assert!(result["not_after"].as_i64().is_some());
            assert!(result["serial_number"].as_str().is_some());
            let sans = result["san_entries"].as_array().unwrap();
            assert!(sans.iter().any(|s| s.as_str() == Some("test.example.com")));
            assert!(result["is_ca"].as_bool() == Some(true));
        }

        #[tokio::test]
        async fn parse_certificate_rejects_missing_params() {
            assert!(handle_parse_certificate(None).await.is_err());
        }

        #[tokio::test]
        async fn parse_certificate_rejects_bad_der() {
            let params = json!({ "certificate": BASE64.encode([1, 2, 3, 4]) });
            assert!(handle_parse_certificate(Some(&params)).await.is_err());
        }
    }
}
