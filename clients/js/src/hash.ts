import { assertDigestCapabilityIsAvailable } from '@solana/assertions';

/**
 * Generates a SHA-256 hash from a set of bytes.
 */
export async function hash(
  input: Uint8Array | Uint8Array[]
): Promise<Uint8Array> {
  // Ensure the digest capability is available.
  assertDigestCapabilityIsAvailable();
  const hash = new Uint8Array(
    await crypto.subtle.digest(
      'SHA-256',
      Array.isArray(input) ? mergeBytes(input) : input
    )
  );
  return hash;
}

/**
 * Concatenates an array of `Uint8Array`s into a single `Uint8Array`.
 */
const mergeBytes = (bytesArr: Uint8Array[]): Uint8Array => {
  const totalLength = bytesArr.reduce((total, arr) => total + arr.length, 0);
  const result = new Uint8Array(totalLength);
  let offset = 0;
  bytesArr.forEach((arr) => {
    result.set(arr, offset);
    offset += arr.length;
  });
  return result;
};
