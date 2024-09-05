use solana_program::pubkey::Pubkey;

/// Return the PDA and bump seed for the given `from` and `slot` derivation.
pub fn find_pda(from: &Pubkey, slot: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[from.as_ref(), &slot.to_le_bytes()], &crate::ID)
}

/// Return the PDA and bump seed for the given `from`, `slot`, and `base` derivation.
pub fn find_pda_with_seed(from: &Pubkey, slot: u64, base: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[from.as_ref(), &slot.to_le_bytes(), base.as_ref()],
        &crate::ID,
    )
}
