// SPDX-License-Identifier: AGPL-3.0-only

//! Android Keystore JNI bridge.
//!
//! On `target_os = "android"` this module calls through JNI to the
//! Android Keystore API, requesting StrongBox-backed keys when available.
//!
//! On all other platforms the public functions return
//! [`BearDogError::not_implemented`] so that the software fallback is used.

use beardog_errors::BearDogError;

// ── Android JNI bridge (compiled only on Android) ──────────────────────

#[cfg(target_os = "android")]
mod platform {
    use beardog_errors::BearDogError;
    use jni::objects::{JObject, JString, JValue};
    use jni::JNIEnv;
    use tracing::{debug, error, info};

    /// Obtain a `KeyStore` Java object pointing at `"AndroidKeyStore"`.
    fn open_keystore(env: &mut JNIEnv<'_>) -> Result<JObject<'_>, BearDogError> {
        let ks_class = env
            .find_class("java/security/KeyStore")
            .map_err(|e| BearDogError::system(format!("KeyStore class not found: {e}")))?;

        let provider: JString<'_> = env
            .new_string("AndroidKeyStore")
            .map_err(|e| BearDogError::system(format!("JNI string: {e}")))?;

        let ks = env
            .call_static_method(
                ks_class,
                "getInstance",
                "(Ljava/lang/String;)Ljava/security/KeyStore;",
                &[JValue::Object(&provider)],
            )
            .and_then(|v| v.l())
            .map_err(|e| BearDogError::system(format!("KeyStore.getInstance: {e}")))?;

        env.call_method(&ks, "load", "(Ljava/security/KeyStore$LoadStoreParameter;)V", &[JValue::Object(&JObject::null())])
            .map_err(|e| BearDogError::system(format!("KeyStore.load: {e}")))?;

        Ok(ks)
    }

    pub fn generate_strongbox_key(
        env: &mut JNIEnv<'_>,
        key_alias: &str,
        algorithm: &str,
        _key_size: u32,
        require_strongbox: bool,
    ) -> Result<(), BearDogError> {
        info!("JNI: generating key alias={key_alias} algo={algorithm} strongbox={require_strongbox}");
        let _ks = open_keystore(env)?;
        // Phase 2: wire KeyGenParameterSpec.Builder with setIsStrongBoxBacked(true)
        Err(BearDogError::not_implemented(
            "StrongBox JNI key generation: parameter builder wiring pending",
        ))
    }

    pub fn delete_key(env: &mut JNIEnv<'_>, key_alias: &str) -> Result<(), BearDogError> {
        let ks = open_keystore(env)?;
        let alias: JString<'_> = env
            .new_string(key_alias)
            .map_err(|e| BearDogError::system(format!("JNI string: {e}")))?;
        env.call_method(
            &ks,
            "deleteEntry",
            "(Ljava/lang/String;)V",
            &[JValue::Object(&alias)],
        )
        .map_err(|e| BearDogError::system(format!("KeyStore.deleteEntry: {e}")))?;
        debug!("JNI: deleted key alias={key_alias}");
        Ok(())
    }

    pub fn key_exists(env: &mut JNIEnv<'_>, key_alias: &str) -> Result<bool, BearDogError> {
        let ks = open_keystore(env)?;
        let alias: JString<'_> = env
            .new_string(key_alias)
            .map_err(|e| BearDogError::system(format!("JNI string: {e}")))?;
        let exists = env
            .call_method(
                &ks,
                "containsAlias",
                "(Ljava/lang/String;)Z",
                &[JValue::Object(&alias)],
            )
            .and_then(|v| v.z())
            .map_err(|e| BearDogError::system(format!("KeyStore.containsAlias: {e}")))?;
        Ok(exists)
    }

    pub fn is_strongbox_available(env: &mut JNIEnv<'_>) -> bool {
        env.find_class("android/security/keystore/StrongBoxUnavailableException")
            .is_ok()
    }
}

// ── Stub bridge (non-Android) ──────────────────────────────────────────

#[cfg(not(target_os = "android"))]
mod platform {
    use beardog_errors::BearDogError;

    pub fn generate_strongbox_key(
        _key_alias: &str,
        _algorithm: &str,
        _key_size: u32,
        _require_strongbox: bool,
    ) -> Result<(), BearDogError> {
        Err(BearDogError::not_implemented(
            "StrongBox key generation requires target_os = android",
        ))
    }

    pub fn delete_key(_key_alias: &str) -> Result<(), BearDogError> {
        Err(BearDogError::not_implemented(
            "StrongBox key deletion requires target_os = android",
        ))
    }

    pub fn key_exists(_key_alias: &str) -> Result<bool, BearDogError> {
        Ok(false)
    }

    pub fn is_strongbox_available() -> bool {
        std::env::var("ANDROID_STRONGBOX_AVAILABLE")
            .map(|v| v == "1" || v == "true")
            .unwrap_or(false)
    }
}

pub use platform::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strongbox_not_available_on_host() {
        assert!(!is_strongbox_available());
    }

    #[test]
    fn generate_returns_not_implemented_on_host() {
        let err = generate_strongbox_key("test", "AES", 256, true).unwrap_err();
        assert!(err.to_string().contains("android"));
    }

    #[test]
    fn delete_returns_not_implemented_on_host() {
        let err = delete_key("test").unwrap_err();
        assert!(err.to_string().contains("android"));
    }

    #[test]
    fn key_exists_returns_false_on_host() {
        assert!(!key_exists("test").unwrap());
    }
}
