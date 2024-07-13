use solana_program::pubkey::Pubkey;

pub fn find_pda(base: &Pubkey, slot: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[base.as_ref(), &slot.to_le_bytes()], &crate::ID)
}
