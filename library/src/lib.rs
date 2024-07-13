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

    /// Base account address to derive the new account.
    ///
    /// If `None`, the address of the `from` account will be used.
    pub base: Option<Pubkey>,

    /// Slot to derive the new account.
    ///
    /// The slot must be within the TTL range.
    pub slot: u64,

    /// Number of lamports to transfer to the new account.
    pub lamports: u64,

    /// Number of bytes of memory to allocate.
    pub space: u64,

    /// Address of program that will own the new account.
    pub owner: Pubkey,
}

/// Creates a new account with the default TTL.
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

    let base = arguments.base.unwrap_or(*arguments.from.key);

    // Derive the address for the new account.
    let (address, bump) =
        Pubkey::find_program_address(&[base.as_ref(), &arguments.slot.to_le_bytes()], program_id);

    if arguments.to.key != &address {
        return Err(ProgramError::InvalidSeeds);
    }

    let signer_seeds = &[base.as_ref(), &arguments.slot.to_le_bytes(), &[bump]];

    // Creates a new account.
    solana_program::program::invoke_signed(
        &solana_program::system_instruction::create_account(
            arguments.from.key,
            arguments.to.key,
            arguments.lamports,
            arguments.space,
            &arguments.owner,
        ),
        &[arguments.from.clone(), arguments.to.clone()],
        &[signer_seeds],
    )
}
