use solana_program::keccak::{hashv, HASH_BYTES};

/// Returns the bytes resulting from hashing of the seeds' bytes.
///
/// The has is computed as the keccak256 hash of the seeds' bytes.
pub fn compute(seeds: &[&[u8]]) -> [u8; HASH_BYTES] {
    hashv(seeds).0
}
