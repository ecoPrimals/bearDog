// SPDX-License-Identifier: AGPL-3.0-or-later

//! Length-prefixed frame I/O for BTSP connections.
//!
//! All BTSP frames (handshake messages and post-handshake JSON-RPC) use:
//!
//! ```text
//! ┌──────────┬──────────────────────────────────────────┐
//! │ Length(4) │ Payload (Length bytes)                    │
//! └──────────┴──────────────────────────────────────────┘
//! ```
//!
//! Length is 4-byte big-endian. Maximum frame: 16 MiB (`0x0100_0000`).

use beardog_errors::BearDogError;
use beardog_types::constants::domains::timeouts::NETWORK_READ_TIMEOUT;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::Duration;

/// Maximum BTSP frame payload: 16 MiB.
const MAX_FRAME_SIZE: u32 = 0x0100_0000;

/// Read timeout for a single BTSP frame.
const FRAME_READ_TIMEOUT: Duration = NETWORK_READ_TIMEOUT;

/// Read one length-prefixed frame from an async stream.
///
/// # Errors
///
/// Returns an error on timeout, I/O failure, or if the advertised length
/// exceeds `MAX_FRAME_SIZE`.
pub async fn read_frame<S: tokio::io::AsyncRead + Unpin>(
    stream: &mut S,
) -> Result<Vec<u8>, BearDogError> {
    let mut len_buf = [0u8; 4];

    tokio::time::timeout(FRAME_READ_TIMEOUT, stream.read_exact(&mut len_buf))
        .await
        .map_err(|_| BearDogError::system("BTSP frame read timed out".to_string()))?
        .map_err(|e| BearDogError::system(format!("BTSP frame length read failed: {e}")))?;

    let len = u32::from_be_bytes(len_buf);
    if len > MAX_FRAME_SIZE {
        return Err(BearDogError::system(format!(
            "BTSP frame too large: {len} bytes (max {MAX_FRAME_SIZE})"
        )));
    }

    let len = len as usize;
    let mut payload = vec![0u8; len];

    tokio::time::timeout(FRAME_READ_TIMEOUT, stream.read_exact(&mut payload))
        .await
        .map_err(|_| BearDogError::system("BTSP frame payload read timed out".to_string()))?
        .map_err(|e| BearDogError::system(format!("BTSP frame payload read failed: {e}")))?;

    Ok(payload)
}

/// Write one length-prefixed frame to an async stream.
///
/// # Errors
///
/// Returns an error if the payload exceeds `MAX_FRAME_SIZE` or on I/O failure.
pub async fn write_frame<S: tokio::io::AsyncWrite + Unpin>(
    stream: &mut S,
    payload: &[u8],
) -> Result<(), BearDogError> {
    let len: u32 = payload
        .len()
        .try_into()
        .map_err(|_| BearDogError::system("BTSP frame payload exceeds u32::MAX".to_string()))?;

    if len > MAX_FRAME_SIZE {
        return Err(BearDogError::system(format!(
            "BTSP frame too large: {len} bytes (max {MAX_FRAME_SIZE})"
        )));
    }

    stream
        .write_all(&len.to_be_bytes())
        .await
        .map_err(|e| BearDogError::system(format!("BTSP frame length write failed: {e}")))?;
    stream
        .write_all(payload)
        .await
        .map_err(|e| BearDogError::system(format!("BTSP frame payload write failed: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    #[tokio::test]
    async fn roundtrip_frame() {
        let (mut client, mut server) = duplex(4096);
        let payload = b"hello btsp frame";

        write_frame(&mut client, payload).await.expect("write");
        let got = read_frame(&mut server).await.expect("read");
        assert_eq!(got, payload);
    }

    #[tokio::test]
    async fn empty_frame() {
        let (mut c, mut s) = duplex(4096);
        write_frame(&mut c, b"").await.expect("write empty");
        let got = read_frame(&mut s).await.expect("read empty");
        assert!(got.is_empty());
    }

    #[tokio::test]
    async fn rejects_oversized_frame_length() {
        let (mut c, mut s) = duplex(4096);
        let bad_len: u32 = MAX_FRAME_SIZE + 1;
        c.write_all(&bad_len.to_be_bytes())
            .await
            .expect("write len");
        let result = read_frame(&mut s).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn rejects_oversized_write() {
        let (mut c, _s) = duplex(4096);
        let big = vec![0u8; (MAX_FRAME_SIZE + 1) as usize];
        let result = write_frame(&mut c, &big).await;
        assert!(result.is_err());
    }
}
