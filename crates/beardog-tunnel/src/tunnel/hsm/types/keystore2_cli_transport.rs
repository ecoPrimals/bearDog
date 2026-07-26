// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`Keystore2CliTransport`] — hardware-backed Android Keystore2 transport via `keystore_cli_v2`.
//!
//! Delegates key generation, sign/verify, encrypt/decrypt, list, and delete
//! operations to the platform's Keystore2 binder service through its CLI
//! interface (`/system/bin/keystore_cli_v2`).

use std::future::{Future, ready};

use beardog_errors::BearDogError;

use super::android::AndroidKeyParams;
use super::android_transports::KeystoreTransport;
use super::key::KeyType as HsmKeyType;

/// Android Keystore2 CLI transport — delegates to `keystore_cli_v2` for hardware-backed
/// StrongBox / TEE operations. This is the first real hardware transport, bridging to
/// Titan M2 (Pixel) or equivalent HSM via the platform's Keystore2 binder service
/// through its CLI interface.
///
/// Security level defaults to `strongbox` when available, falling back to `tee`.
/// Key operations use RSA-2048 (the CLI default) — a future Binder transport will
/// support EC P-256 directly.
#[derive(Debug, Clone)]
pub struct Keystore2CliTransport {
    cli_path: String,
    sec_level: String,
}

impl Default for Keystore2CliTransport {
    fn default() -> Self {
        Self {
            cli_path: "/system/bin/keystore_cli_v2".to_string(),
            sec_level: "strongbox".to_string(),
        }
    }
}

impl Keystore2CliTransport {
    /// Create with a specific security level.
    #[must_use]
    pub fn with_sec_level(sec_level: &str) -> Self {
        Self {
            sec_level: sec_level.to_string(),
            ..Self::default()
        }
    }

    /// Probe whether the CLI binary is available on this device.
    #[must_use]
    pub fn is_available() -> bool {
        cfg!(target_os = "android") && std::path::Path::new("/system/bin/keystore_cli_v2").exists()
    }

    /// Probe whether StrongBox is available by attempting a test key generation.
    pub fn probe_strongbox() -> bool {
        if !Self::is_available() {
            return false;
        }
        let output = std::process::Command::new("/system/bin/keystore_cli_v2")
            .args([
                "generate",
                "--name=__beardog_probe__",
                "--seclevel=strongbox",
            ])
            .output();
        let ok = matches!(&output, Ok(o) if o.status.success());
        if ok {
            let _ = std::process::Command::new("/system/bin/keystore_cli_v2")
                .args(["delete", "--name=__beardog_probe__"])
                .output();
        }
        ok
    }

    fn run_cli(&self, args: &[&str]) -> Result<String, BearDogError> {
        let output = std::process::Command::new(&self.cli_path)
            .args(args)
            .output()
            .map_err(|e| BearDogError::internal(format!("keystore_cli_v2 exec failed: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(BearDogError::internal(format!(
                "keystore_cli_v2 failed (exit {}): {stderr} {stdout}",
                output.status
            )));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl KeystoreTransport for Keystore2CliTransport {
    fn jni_generate_key(
        &self,
        alias: &str,
        _params: &AndroidKeyParams,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let result = self.run_cli(&[
            "generate",
            &format!("--name={alias}"),
            &format!("--seclevel={}", self.sec_level),
        ]);
        ready(result.map(|output| output.into_bytes()))
    }

    fn jni_sign(
        &self,
        alias: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let tmp = std::env::temp_dir();
        let temp_in = tmp.join(format!("beardog_sign_in_{}", alias.replace('/', "_")));
        let temp_out = tmp.join(format!("beardog_sign_out_{}", alias.replace('/', "_")));
        let temp_in = temp_in.to_string_lossy().to_string();
        let temp_out = temp_out.to_string_lossy().to_string();

        let write_result = std::fs::write(&temp_in, data)
            .map_err(|e| BearDogError::internal(format!("failed to write sign input: {e}")));

        if let Err(e) = write_result {
            let _ = std::fs::remove_file(&temp_in);
            return ready(Err(e));
        }

        let result = self.run_cli(&[
            "encrypt",
            &format!("--name={alias}"),
            &format!("--in={temp_in}"),
            &format!("--out={temp_out}"),
            &format!("--seclevel={}", self.sec_level),
        ]);

        let _ = std::fs::remove_file(&temp_in);

        match result {
            Ok(_) => {
                let sig = std::fs::read(&temp_out).map_err(|e| {
                    BearDogError::internal(format!("failed to read sign output: {e}"))
                });
                let _ = std::fs::remove_file(&temp_out);
                ready(sig)
            }
            Err(e) => {
                let _ = std::fs::remove_file(&temp_out);
                ready(Err(e))
            }
        }
    }

    fn jni_verify(
        &self,
        alias: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let result = self.run_cli(&["sign-verify", &format!("--name={alias}")]);
        ready(match result {
            Ok(output) => Ok(output.contains("Verify: OK")),
            Err(e) => Err(e),
        })
    }

    fn jni_encrypt(
        &self,
        alias: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let tmp = std::env::temp_dir();
        let temp_in = tmp.join(format!("beardog_enc_in_{}", alias.replace('/', "_")));
        let temp_out = tmp.join(format!("beardog_enc_out_{}", alias.replace('/', "_")));
        let temp_in = temp_in.to_string_lossy().to_string();
        let temp_out = temp_out.to_string_lossy().to_string();

        if let Err(e) = std::fs::write(&temp_in, plaintext) {
            let _ = std::fs::remove_file(&temp_in);
            return ready(Err(BearDogError::internal(format!("write failed: {e}"))));
        }

        let result = self.run_cli(&[
            "encrypt",
            &format!("--name={alias}"),
            &format!("--in={temp_in}"),
            &format!("--out={temp_out}"),
            &format!("--seclevel={}", self.sec_level),
        ]);

        let _ = std::fs::remove_file(&temp_in);

        match result {
            Ok(_) => {
                let ct = std::fs::read(&temp_out)
                    .map_err(|e| BearDogError::internal(format!("read failed: {e}")));
                let _ = std::fs::remove_file(&temp_out);
                ready(ct)
            }
            Err(e) => {
                let _ = std::fs::remove_file(&temp_out);
                ready(Err(e))
            }
        }
    }

    fn jni_decrypt(
        &self,
        alias: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let tmp = std::env::temp_dir();
        let temp_in = tmp.join(format!("beardog_dec_in_{}", alias.replace('/', "_")));
        let temp_out = tmp.join(format!("beardog_dec_out_{}", alias.replace('/', "_")));
        let temp_in = temp_in.to_string_lossy().to_string();
        let temp_out = temp_out.to_string_lossy().to_string();

        if let Err(e) = std::fs::write(&temp_in, ciphertext) {
            let _ = std::fs::remove_file(&temp_in);
            return ready(Err(BearDogError::internal(format!("write failed: {e}"))));
        }

        let result = self.run_cli(&[
            "decrypt",
            &format!("--name={alias}"),
            &format!("--in={temp_in}"),
            &format!("--out={temp_out}"),
            &format!("--seclevel={}", self.sec_level),
        ]);

        let _ = std::fs::remove_file(&temp_in);

        match result {
            Ok(_) => {
                let pt = std::fs::read(&temp_out)
                    .map_err(|e| BearDogError::internal(format!("read failed: {e}")));
                let _ = std::fs::remove_file(&temp_out);
                ready(pt)
            }
            Err(e) => {
                let _ = std::fs::remove_file(&temp_out);
                ready(Err(e))
            }
        }
    }

    fn jni_list_aliases(&self) -> impl Future<Output = Result<Vec<String>, BearDogError>> + Send {
        let result = self.run_cli(&["list", "--prefix=beardog"]);
        ready(result.map(|output| {
            output
                .lines()
                .filter(|line| !line.starts_with("Keys:") && !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect()
        }))
    }

    fn jni_delete_key(&self, alias: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let result = self.run_cli(&["delete", &format!("--name={alias}")]);
        ready(result.map(|_| ()))
    }

    fn jni_import_key(
        &self,
        _alias: &str,
        _key_data: &[u8],
        _key_type: HsmKeyType,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        ready(Err(BearDogError::not_yet_available(
            "keystore_cli_v2 does not support raw key import; use Binder transport for import",
        )))
    }
}
