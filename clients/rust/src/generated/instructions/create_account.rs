//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct CreateAccount {
    /// Funding account
    pub from: solana_program::pubkey::Pubkey,
    /// New account
    pub to: solana_program::pubkey::Pubkey,
    /// Base public key for the account derivation
    pub base: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateAccount {
    pub fn instruction(
        &self,
        args: CreateAccountInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateAccountInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.from, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.to, false,
        ));
        if let Some(base) = self.base {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                base, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LIMESTONE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateAccountInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LIMESTONE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateAccountInstructionData {
    discriminator: u8,
}

impl CreateAccountInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 0 }
    }
}

impl Default for CreateAccountInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAccountInstructionArgs {
    pub slot: u64,
    pub lamports: u64,
    pub space: u64,
    pub owner: Pubkey,
}

/// Instruction builder for `CreateAccount`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` from
///   1. `[writable]` to
///   2. `[optional]` base
///   3. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct CreateAccountBuilder {
    from: Option<solana_program::pubkey::Pubkey>,
    to: Option<solana_program::pubkey::Pubkey>,
    base: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    slot: Option<u64>,
    lamports: Option<u64>,
    space: Option<u64>,
    owner: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Funding account
    #[inline(always)]
    pub fn from(&mut self, from: solana_program::pubkey::Pubkey) -> &mut Self {
        self.from = Some(from);
        self
    }
    /// New account
    #[inline(always)]
    pub fn to(&mut self, to: solana_program::pubkey::Pubkey) -> &mut Self {
        self.to = Some(to);
        self
    }
    /// `[optional account]`
    /// Base public key for the account derivation
    #[inline(always)]
    pub fn base(&mut self, base: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.base = base;
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn slot(&mut self, slot: u64) -> &mut Self {
        self.slot = Some(slot);
        self
    }
    #[inline(always)]
    pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.lamports = Some(lamports);
        self
    }
    #[inline(always)]
    pub fn space(&mut self, space: u64) -> &mut Self {
        self.space = Some(space);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CreateAccount {
            from: self.from.expect("from is not set"),
            to: self.to.expect("to is not set"),
            base: self.base,
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateAccountInstructionArgs {
            slot: self.slot.clone().expect("slot is not set"),
            lamports: self.lamports.clone().expect("lamports is not set"),
            space: self.space.clone().expect("space is not set"),
            owner: self.owner.clone().expect("owner is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_account` CPI accounts.
pub struct CreateAccountCpiAccounts<'a, 'b> {
    /// Funding account
    pub from: &'b solana_program::account_info::AccountInfo<'a>,
    /// New account
    pub to: &'b solana_program::account_info::AccountInfo<'a>,
    /// Base public key for the account derivation
    pub base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_account` CPI instruction.
pub struct CreateAccountCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Funding account
    pub from: &'b solana_program::account_info::AccountInfo<'a>,
    /// New account
    pub to: &'b solana_program::account_info::AccountInfo<'a>,
    /// Base public key for the account derivation
    pub base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateAccountInstructionArgs,
}

impl<'a, 'b> CreateAccountCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateAccountCpiAccounts<'a, 'b>,
        args: CreateAccountInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            from: accounts.from,
            to: accounts.to,
            base: accounts.base,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.from.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.to.key,
            false,
        ));
        if let Some(base) = self.base {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *base.key, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LIMESTONE_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateAccountInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LIMESTONE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.from.clone());
        account_infos.push(self.to.clone());
        if let Some(base) = self.base {
            account_infos.push(base.clone());
        }
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CreateAccount` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` from
///   1. `[writable]` to
///   2. `[optional]` base
///   3. `[]` system_program
#[derive(Clone, Debug)]
pub struct CreateAccountCpiBuilder<'a, 'b> {
    instruction: Box<CreateAccountCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateAccountCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateAccountCpiBuilderInstruction {
            __program: program,
            from: None,
            to: None,
            base: None,
            system_program: None,
            slot: None,
            lamports: None,
            space: None,
            owner: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Funding account
    #[inline(always)]
    pub fn from(&mut self, from: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.from = Some(from);
        self
    }
    /// New account
    #[inline(always)]
    pub fn to(&mut self, to: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.to = Some(to);
        self
    }
    /// `[optional account]`
    /// Base public key for the account derivation
    #[inline(always)]
    pub fn base(
        &mut self,
        base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.base = base;
        self
    }
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn slot(&mut self, slot: u64) -> &mut Self {
        self.instruction.slot = Some(slot);
        self
    }
    #[inline(always)]
    pub fn lamports(&mut self, lamports: u64) -> &mut Self {
        self.instruction.lamports = Some(lamports);
        self
    }
    #[inline(always)]
    pub fn space(&mut self, space: u64) -> &mut Self {
        self.instruction.space = Some(space);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: Pubkey) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CreateAccountInstructionArgs {
            slot: self.instruction.slot.clone().expect("slot is not set"),
            lamports: self
                .instruction
                .lamports
                .clone()
                .expect("lamports is not set"),
            space: self.instruction.space.clone().expect("space is not set"),
            owner: self.instruction.owner.clone().expect("owner is not set"),
        };
        let instruction = CreateAccountCpi {
            __program: self.instruction.__program,

            from: self.instruction.from.expect("from is not set"),

            to: self.instruction.to.expect("to is not set"),

            base: self.instruction.base,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateAccountCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    from: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    to: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    slot: Option<u64>,
    lamports: Option<u64>,
    space: Option<u64>,
    owner: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
