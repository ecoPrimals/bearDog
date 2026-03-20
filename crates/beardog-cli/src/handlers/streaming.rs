// SPDX-License-Identifier: AGPL-3.0-only

//! Streaming Encryption/Decryption for Large Files
//!
//! Handles large files (100GB+) without loading into memory.
//! Uses chunked processing with progress reporting.

use base64::Engine; // For base64 decoding
use beardog_errors::BearDogError;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

/// Chunk size for streaming (16MB - optimal for most systems)
const CHUNK_SIZE: usize = 16 * 1024 * 1024;

/// Handle streaming encryption
///
/// Encrypts large files in chunks without loading entire file into memory.
pub async fn handle_streaming_encrypt(
    key_id: &str,
    input_path: &str,
    output_path: &str,
) -> Result<(), BearDogError> {
    println!("🔐 BearDog Streaming Encryption");
    println!("================================\n");

    // Validate input file exists
    if !Path::new(input_path).exists() {
        return Err(BearDogError::validation(&format!(
            "Input file not found: {input_path}"
        )));
    }

    // Get file size for progress reporting
    let metadata = std::fs::metadata(input_path)?;
    let file_size = metadata.len();
    println!("📂 Input file: {input_path} ({file_size} bytes)");
    println!("🔑 Key ID: {key_id}");
    println!("💾 Output file: {output_path}\n");

    // Open input and output files
    let mut input_file = File::open(input_path)?;
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    // Write metadata header (version, key_id, chunk_size)
    let header = format!("BEARDOG_STREAM_V1\n{key_id}\n{CHUNK_SIZE}\n");
    output_file.write_all(header.as_bytes())?;

    // Process file in chunks
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_processed = 0u64;
    let mut chunk_index = 0u64;

    println!("⏳ Processing chunks...\n");

    loop {
        // Read chunk
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }

        total_processed += bytes_read as u64;
        let progress = (total_processed as f64 / file_size as f64) * 100.0;

        // Encrypt chunk (using BearDog crypto service)
        let plaintext = &buffer[..bytes_read];
        let encrypted = encrypt_chunk(key_id, plaintext, chunk_index).await?;

        // Write encrypted chunk with length prefix
        let chunk_len = encrypted.len() as u32;
        output_file.write_all(&chunk_len.to_le_bytes())?;
        output_file.write_all(&encrypted)?;

        // Progress reporting
        print!("\r   Chunk {chunk_index}: {progress:.1}% ({total_processed}/{file_size} bytes)");
        std::io::stdout().flush()?;

        chunk_index += 1;
    }

    println!("\n\n✅ Encryption complete!");
    println!("   Total chunks: {chunk_index}");
    println!(
        "   Output size: {} bytes",
        std::fs::metadata(output_path)?.len()
    );

    Ok(())
}

/// Handle streaming decryption
///
/// Decrypts large files in chunks without loading entire file into memory.
pub async fn handle_streaming_decrypt(
    input_path: &str,
    output_path: &str,
) -> Result<(), BearDogError> {
    println!("🔓 BearDog Streaming Decryption");
    println!("================================\n");

    // Validate input file exists
    if !Path::new(input_path).exists() {
        return Err(BearDogError::validation(&format!(
            "Input file not found: {input_path}"
        )));
    }

    // Get file size for progress reporting
    let metadata = std::fs::metadata(input_path)?;
    let file_size = metadata.len();
    println!("📂 Input file: {input_path} ({file_size} bytes)");

    // Open input and output files
    let mut input_file = File::open(input_path)?;
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    // Read and validate header - Use manual reading to avoid BufReader consuming extra bytes
    // Read header lines manually to know exact position
    let mut header_bytes = Vec::new();
    let mut byte_buffer = [0u8; 1];

    // Read until we have 3 newlines (version, key_id, chunk_size)
    let mut newline_count = 0;
    while newline_count < 3 {
        input_file.read_exact(&mut byte_buffer)?;
        header_bytes.push(byte_buffer[0]);
        if byte_buffer[0] == b'\n' {
            newline_count += 1;
        }
    }

    // Parse header from bytes
    let header_str = String::from_utf8(header_bytes)
        .map_err(|_| BearDogError::validation("Invalid UTF-8 in header"))?;

    let mut lines = header_str.lines();

    // Validate version
    let version_line = lines
        .next()
        .ok_or_else(|| BearDogError::validation("Missing version line"))?;
    if !version_line.starts_with("BEARDOG_STREAM_V1") {
        return Err(BearDogError::validation(
            "Invalid file format (not a BearDog streaming encrypted file)",
        ));
    }

    // Read key_id
    let key_id = lines
        .next()
        .ok_or_else(|| BearDogError::validation("Missing key_id line"))?
        .trim()
        .to_string();
    println!("🔑 Key ID: {key_id}");

    // Read chunk_size
    let _chunk_size: usize = lines
        .next()
        .ok_or_else(|| BearDogError::validation("Missing chunk_size line"))?
        .trim()
        .parse()
        .map_err(|_| BearDogError::validation("Invalid chunk size in header"))?;

    println!("💾 Output file: {output_path}\n");
    println!("⏳ Processing chunks...\n");

    // Process chunks - now input_file is at the correct position
    let mut total_processed = 0u64;
    let mut chunk_index = 0u64;

    loop {
        // Read chunk length prefix
        let mut len_buffer = [0u8; 4];
        match input_file.read_exact(&mut len_buffer) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break, // EOF
            Err(e) => return Err(e.into()),
        }

        let chunk_len = u32::from_le_bytes(len_buffer) as usize;

        // Read encrypted chunk
        let mut encrypted_chunk = vec![0u8; chunk_len];
        input_file.read_exact(&mut encrypted_chunk)?;

        // Decrypt chunk
        let decrypted = decrypt_chunk(&key_id, &encrypted_chunk, chunk_index).await?;

        // Write decrypted chunk
        output_file.write_all(&decrypted)?;

        total_processed += decrypted.len() as u64;
        let progress = (total_processed as f64 / file_size as f64) * 100.0;

        // Progress reporting
        print!("\r   Chunk {chunk_index}: {progress:.1}% ({total_processed} bytes)");
        std::io::stdout().flush()?;

        chunk_index += 1;
    }

    println!("\n\n✅ Decryption complete!");
    println!("   Total chunks: {chunk_index}");
    println!(
        "   Output size: {} bytes",
        std::fs::metadata(output_path)?.len()
    );

    Ok(())
}

/// Encrypt a single chunk
async fn encrypt_chunk(
    key_id: &str,
    plaintext: &[u8],
    chunk_index: u64,
) -> Result<Vec<u8>, BearDogError> {
    // Load key from key store
    let key = super::key_store::load_key(key_id)?;

    // Derive chunk-specific nonce (deterministic based on chunk index)
    let mut nonce = [0u8; 12];
    nonce[..8].copy_from_slice(&chunk_index.to_le_bytes());

    // Use ChaCha20-Poly1305 for streaming (better for large data)
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit},
    };

    // Decode key material
    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(&key.key_material_b64)
        .map_err(|e| BearDogError::validation(&format!("Invalid key material: {e}")))?;

    if key_bytes.len() != 32 {
        return Err(BearDogError::validation(
            "Key must be 32 bytes for ChaCha20",
        ));
    }

    let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes)
        .map_err(|e| BearDogError::validation(&format!("Invalid key: {e}")))?;

    let nonce_obj = Nonce::from(nonce);

    // Encrypt chunk
    let ciphertext = cipher
        .encrypt(&nonce_obj, plaintext)
        .map_err(|e| BearDogError::validation(&format!("Encryption failed: {e}")))?;

    // Return: nonce (12 bytes) + ciphertext (with tag)
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt a single chunk
async fn decrypt_chunk(
    key_id: &str,
    encrypted: &[u8],
    _chunk_index: u64,
) -> Result<Vec<u8>, BearDogError> {
    // Load key from key store
    let key = super::key_store::load_key(key_id)?;

    // Extract nonce (first 12 bytes)
    if encrypted.len() < 12 {
        return Err(BearDogError::validation(
            "Invalid encrypted chunk (too short)",
        ));
    }

    let nonce = &encrypted[..12];
    let ciphertext = &encrypted[12..];

    // Use ChaCha20-Poly1305
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit},
    };

    // Decode key material
    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(&key.key_material_b64)
        .map_err(|e| BearDogError::validation(&format!("Invalid key material: {e}")))?;

    if key_bytes.len() != 32 {
        return Err(BearDogError::validation(
            "Key must be 32 bytes for ChaCha20",
        ));
    }

    let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes)
        .map_err(|e| BearDogError::validation(&format!("Invalid key: {e}")))?;

    let nonce_obj = Nonce::from_slice(nonce);

    // Decrypt chunk
    let plaintext = cipher
        .decrypt(nonce_obj, ciphertext)
        .map_err(|e| BearDogError::validation(&format!("Decryption failed: {e}")))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chunk_encrypt_decrypt() {
        // This would require a test key in the key store
        // For now, just test that the functions exist and have correct signatures
        assert_eq!(CHUNK_SIZE, 16 * 1024 * 1024);
    }
}
