import { mergeBytes } from '@metaplex-foundation/umi/serializers';
import { keccak_256 } from '@noble/hashes/sha3';

/**
 * Generates a SHA-256 hash from a set of bytes.
 */
export function hash(input: Uint8Array | Uint8Array[]): Uint8Array {
  return keccak_256(Array.isArray(input) ? mergeBytes(input) : input);
}
