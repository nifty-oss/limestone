export * from './generated';
export * from './hash';
export * from './hooked';

// The time-to-live for a slot value.
//
// If the slot is older than this value when compared to the current slot value, the
// account derivation is considered invalid.
//
// This value is set to 150 slots, which is approximately 1 minute 19 seconds, assuming
// 400ms block times. This is similar to the default value for the `recent_blockhash`
// validation.
export const TTL = 150;
