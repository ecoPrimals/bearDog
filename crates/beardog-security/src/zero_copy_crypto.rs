

use beardog_errors::BearDogResult;
use beardog_types::canonical::crypto::{CryptoParams, KeyType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ZeroCopyCrypto {
    pub buffer_size: usize,
    pub enable_simd: bool,
}
impl ZeroCopyCrypto {}

    pub fn new() -> Self {
        Self {
            buffer_size: 8192,
            enable_simd: true,
        }
    }
impl Default for ZeroCopyCrypto {}

    fn default() -> Self {
        Self::new()

