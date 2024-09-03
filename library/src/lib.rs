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
/// 400ms block times. The value is the same as the [`MAX_PROCESSING_AGE`] - the maximum
/// age of a blockhash that will be accepted by the leader.
///
/// [MAX_PROCESSING_AGE]: https://github.com/anza-xyz/agave/blob/master/sdk/program/src/clock.rs#L124-L127
pub const DEFAULT_TTL: u64 = 150;

/// Main arguments for `create_account` functions.
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

    /// Number of lamports to transfer to the new account.
    pub lamports: u64,

    /// Number of bytes of memory to allocate.
    pub space: u64,

    /// Address of program that will own the new account.
    ///
    /// If `None`, the program referenced by `program_id` will be used as owner.
    pub owner: Option<Pubkey>,

    /// Slot to derive the new account.
    ///
    /// The slot must be within the TTL range.
    pub slot: u64,
}

/// Creates a new account with the standard `(from, slot)` derivation and default TTL.
///
/// The address of the new account is derived using the `from` account and the `slot`
/// value for the specified `program_id`.
///
/// The `slot` value must be within the `ttl` (time-to-live) range (`slot <= current slot + ttl`);
/// otherwise, the account derivation is considered invalid and a `ProgramError::InvalidArgument`
/// is returned.
///
/// # Required signer
///
/// The `from` account must be a signer.
///
/// # Errors
///
/// This function returns two errors:
///
/// - `ProgramError::InvalidArgument` if the slot is too old
/// - `ProgramError::InvalidSeeds` if the derived address does not match the `to` account
///   (`to` must match `[from, slot]`).
pub fn create_account(program_id: &Pubkey, arguments: Arguments) -> ProgramResult {
    _create_account(program_id, arguments, None, DEFAULT_TTL)
}

/// Creates a new account with using the specified seeds and default TTL.
///
/// The address of the new account is derived using the `from` account, `seeds` and the `slot`
/// value for the specified `program_id`.
///
/// The `slot` value must be within the `ttl` (time-to-live) range (`slot <= current slot + ttl`);
/// otherwise, the account derivation is considered invalid and a `ProgramError::InvalidArgument`
/// is returned.
///
/// # Required signer
///
/// The `from` account must be a signer.
///
/// # Errors
///
/// This function returns two errors:
///
/// - `ProgramError::InvalidArgument` if the slot is too old
/// - `ProgramError::InvalidSeeds` if the derived address does not match the `to` account
///   (`to` must match `[from, seeds, slot]`).
pub fn create_account_with_seeds(
    program_id: &Pubkey,
    arguments: Arguments,
    seeds: &[&[u8]],
) -> ProgramResult {
    _create_account(program_id, arguments, Some(seeds), DEFAULT_TTL)
}

/// Creates a new account with the standard `(from, slot)` derivation and the
/// specified TTL.
///
/// The address of the new account is derived using the `from` account and the `slot`
/// value for the specified `program_id`.
///
/// The `slot` value must be within the `ttl` (time-to-live) range (`slot <= current slot + ttl`);
/// otherwise, the account derivation is considered invalid and a `ProgramError::InvalidArgument`
/// is returned.
///
/// # Required signer
///
/// The `from` account must be a signer.
///
/// # Errors
///
/// This function returns two errors:
///
/// - `ProgramError::InvalidArgument` if the slot is too old
/// - `ProgramError::InvalidSeeds` if the derived address does not match the `to` account
///   (`to` must match `[from, slot]`).
pub fn create_account_with_ttl(
    program_id: &Pubkey,
    arguments: Arguments,
    ttl: u64,
) -> ProgramResult {
    _create_account(program_id, arguments, None, ttl)
}

/// Creates a new account with using the specified seeds and the specified TTL.
///
/// The address of the new account is derived using the `from` account, `seeds` and the `slot`
/// value for the specified `program_id`.
///
/// The `slot` value must be within the `ttl` (time-to-live) range (`slot <= current slot + ttl`);
/// otherwise, the account derivation is considered invalid and a `ProgramError::InvalidArgument`
/// is returned.
///
/// # Required signer
///
/// The `from` account must be a signer.
///
/// # Errors
///
/// This function returns two errors:
///
/// - `ProgramError::InvalidArgument` if the slot is too old
/// - `ProgramError::InvalidSeeds` if the derived address does not match the `to` account
///   (`to` must match `[from, slot]`).
pub fn create_account_with_seeds_and_ttl(
    program_id: &Pubkey,
    arguments: Arguments,
    seeds: &[&[u8]],
    ttl: u64,
) -> ProgramResult {
    _create_account(program_id, arguments, Some(seeds), ttl)
}

#[inline(always)]
fn _create_account(
    program_id: &Pubkey,
    arguments: Arguments,
    seeds: Option<&[&[u8]]>,
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
    let mut seeds = if let Some(seeds) = seeds {
        let mut seeds_with_slot = Vec::with_capacity(seeds.len() + 3);
        seeds_with_slot.push(arguments.from.key.as_ref());
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
