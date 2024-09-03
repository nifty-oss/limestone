use arrayref::array_ref;
use shank::{ShankContext, ShankInstruction};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

/// Instructions supported by the Limestone program.
#[derive(Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    /// Create a new account.
    #[account(0, writable, signer, name="from", desc="Funding account")]
    #[account(1, writable, name="to", desc="New account (pda of `[from, slot number]`)")]
    #[account(2, optional, name="seed", desc="Additional seed for the account derivation")]
    #[account(3, name="system_program", desc="The system program")]
    CreateAccount {
        /// Slot number for the account derivation.
        slot: u64,

        /// Number of lamports to transfer to the new account
        lamports: u64,

        /// Number of bytes of memory to allocate
        space: u64,

        /// Address of program that will own the new account
        owner: Pubkey,
    },
}

impl Instruction {
    /// Unpacks a byte buffer into a [Instruction](enum.Instruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            // 0 - CreateAccount
            //   *  8 - slot (u64)
            //   *  8 - lamports (u64)
            //   *  8 - space (u64)
            //   * 32 - owner (Pubkey)
            Some((&0, rest)) if rest.len() == 56 => {
                let slot = u64::from_le_bytes(*array_ref![rest, 0, 8]);
                let lamports = u64::from_le_bytes(*array_ref![rest, 8, 8]);
                let space = u64::from_le_bytes(*array_ref![rest, 16, 8]);
                let owner = Pubkey::from(*array_ref![rest, 24, 32]);

                Ok(Instruction::CreateAccount {
                    slot,
                    lamports,
                    space,
                    owner,
                })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
