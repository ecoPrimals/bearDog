// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genetic entropy mixing (BLAKE3) for the tarpc crypto server.

use crate::tarpc_types::{CryptoResult, EntropyMixRequest, MixedEntropy};

pub(super) async fn genetic_mix_entropy(request: EntropyMixRequest) -> CryptoResult<MixedEntropy> {
    // Use BLAKE3 keyed hash for entropy mixing
    let mut hasher = blake3::Hasher::new();
    hasher.update(request.context.as_bytes());

    for source in &request.sources {
        hasher.update(source);
    }

    let hash = hasher.finalize();

    Ok(MixedEntropy {
        entropy: hash.as_bytes().to_vec(),
    })
}
