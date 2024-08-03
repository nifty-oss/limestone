use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

/// The time-to-live for a slot value.
///
/// If the slot is older than this value, the account derivation is considered
/// invalid.
///
/// This value is set to 150 slots, which is approximately 1 minute 19 seconds, assuming
/// 400ms block times. This is similar to the default value for the `recent_blockhash`
/// validation.
pub const DEFAULT_TTL: u64 = 150;

/// The arguments for the `create_account` function.
pub struct Arguments<'a, 'b> {
    /// Funding account.
    ///
    /// This account must have enough lamports to create the new account, be writable
    /// and signer.
    pub to: &'b AccountInfo<'a>,

    /// Account to be created.
    ///
    /// This account must be writable and signer.
    pub from: &'b AccountInfo<'a>,

    /// Seeds for the account address derivation.
    ///
    /// If `None`, the address of the `from` account will be used. The `seeds` should not
    /// include the `slot` value, since its value will be added to the seeds.
    pub seeds: Option<&'b [&'b [u8]]>,

    /// Slot to derive the new account.
    ///
    /// The slot must be within the TTL range.
    pub slot: u64,

    /// Number of lamports to transfer to the new account.
    pub lamports: u64,

    /// Number of bytes of memory to allocate.
    pub space: u64,

    /// Address of program that will own the new account.
    ///
    /// If `None`, the program referenced by `program_id` will be used as owner.
    pub owner: Option<Pubkey>,
}

/// Creates a new account with the default TTL.
///
/// This function is a wrapper around [`create_account_with_ttl`] with the default TTL
/// value.
pub fn create_account(program_id: &Pubkey, arguments: Arguments) -> ProgramResult {
    create_account_with_ttl(program_id, arguments, DEFAULT_TTL)
}

/// Creates a new account.
///
/// The address of the new account is derived from the `base` account and the `slot`
/// value for the specifued `program_id`.
///
/// The `slot` value must be within the `ttl` (time-to-live) range (`slot <= current slot + ttl`);
/// otherwise, the account derivation is considered invalid and a `ProgramError::InvalidArgument`
/// is returned.
///
/// If the `base` is `None`, the address of the `from` account will be used in the derivation.
///
/// # Errors
///
/// This function returns two errors:
///
/// - `ProgramError::InvalidArgument` if the slot is too old
/// - `ProgramError::InvalidSeeds` if the derived address does not match the `to` account
///   (`to` must match `[base, slot]`).
pub fn create_account_with_ttl(
    program_id: &Pubkey,
    arguments: Arguments,
    ttl: u64,
) -> ProgramResult {
    // Check if the slot is too old.
    //
    // The slot received must be within the TTL range.
    let currrent_slot = Clock::get()?.slot;

    if currrent_slot > arguments.slot + ttl {
        return Err(ProgramError::InvalidArgument);
    }

    let slot_seeds = arguments.slot.to_le_bytes();
    // Seeds has an extra allocation to store the bump value for the signer.
    let mut seeds = if let Some(seeds) = arguments.seeds {
        let mut seeds_with_slot = Vec::with_capacity(seeds.len() + 2);
        seeds_with_slot.extend_from_slice(seeds);
        seeds_with_slot.push(&slot_seeds);
        seeds_with_slot
    } else {
        let mut seeds_with_slot = Vec::with_capacity(3);
        seeds_with_slot.extend_from_slice(&[arguments.from.key.as_ref(), &slot_seeds]);
        seeds_with_slot
    };

    // Derive the address for the new account.
    let (address, bump) = Pubkey::find_program_address(&seeds, program_id);

    if arguments.to.key != &address {
        return Err(ProgramError::InvalidSeeds);
    }

    let signer_bump = [bump];
    seeds.push(&signer_bump);

    // Creates a new account.
    solana_program::program::invoke_signed(
        &solana_program::system_instruction::create_account(
            arguments.from.key,
            arguments.to.key,
            arguments.lamports,
            arguments.space,
            arguments.owner.as_ref().unwrap_or(program_id),
        ),
        &[arguments.from.clone(), arguments.to.clone()],
        &[&seeds],
    )
}
