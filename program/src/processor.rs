use limestone::Arguments;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::{
    accounts::{Context, CreateAccountAccounts},
    Instruction,
};

/// Process instructions for the program.
pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(instruction_data)?;

    match instruction {
        Instruction::CreateAccount {
            slot,
            lamports,
            space,
            owner,
        } => {
            msg!("Instruction: CreateAccount");
            create_account(
                program_id,
                CreateAccountAccounts::context(accounts)?,
                slot,
                lamports,
                space,
                owner,
            )
        }
    }
}

/// Create a new account.
///
/// The account creation logic is implemented in the `limestone` crate.
#[inline(always)]
fn create_account(
    program_id: &Pubkey,
    ctx: Context<CreateAccountAccounts>,
    slot: u64,
    lamports: u64,
    space: u64,
    owner: Pubkey,
) -> ProgramResult {
    let seed = if let Some(seed) = &ctx.accounts.seed {
        seed.key.as_ref()
    } else {
        &[]
    };

    limestone::create_account_with_seeds(
        program_id,
        Arguments {
            to: ctx.accounts.to,
            from: ctx.accounts.from,
            slot,
            lamports,
            space,
            owner: Some(owner),
        },
        &[seed],
    )
}
