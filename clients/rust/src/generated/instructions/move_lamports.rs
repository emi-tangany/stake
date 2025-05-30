//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
#[derive(Debug)]
pub struct MoveLamports {
    /// Active or inactive source stake account
    pub source_stake: solana_program::pubkey::Pubkey,
    /// Mergeable destination stake account
    pub destination_stake: solana_program::pubkey::Pubkey,
    /// Stake authority
    pub stake_authority: solana_program::pubkey::Pubkey,
}

impl MoveLamports {
    pub fn instruction(
        &self,
        args: MoveLamportsInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: MoveLamportsInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.source_stake,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.destination_stake,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&MoveLamportsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveLamportsInstructionData {
    discriminator: u32,
}

impl MoveLamportsInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 17 }
    }
}

impl Default for MoveLamportsInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveLamportsInstructionArgs {
    pub args: u64,
}

/// Instruction builder for `MoveLamports`.
///
/// ### Accounts:
///
///   0. `[writable]` source_stake
///   1. `[writable]` destination_stake
///   2. `[signer]` stake_authority
#[derive(Clone, Debug, Default)]
pub struct MoveLamportsBuilder {
    source_stake: Option<solana_program::pubkey::Pubkey>,
    destination_stake: Option<solana_program::pubkey::Pubkey>,
    stake_authority: Option<solana_program::pubkey::Pubkey>,
    args: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MoveLamportsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Active or inactive source stake account
    #[inline(always)]
    pub fn source_stake(&mut self, source_stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.source_stake = Some(source_stake);
        self
    }
    /// Mergeable destination stake account
    #[inline(always)]
    pub fn destination_stake(
        &mut self,
        destination_stake: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.destination_stake = Some(destination_stake);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_authority = Some(stake_authority);
        self
    }
    #[inline(always)]
    pub fn args(&mut self, args: u64) -> &mut Self {
        self.args = Some(args);
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
        let accounts = MoveLamports {
            source_stake: self.source_stake.expect("source_stake is not set"),
            destination_stake: self
                .destination_stake
                .expect("destination_stake is not set"),
            stake_authority: self.stake_authority.expect("stake_authority is not set"),
        };
        let args = MoveLamportsInstructionArgs {
            args: self.args.clone().expect("args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `move_lamports` CPI accounts.
pub struct MoveLamportsCpiAccounts<'a, 'b> {
    /// Active or inactive source stake account
    pub source_stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Mergeable destination stake account
    pub destination_stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `move_lamports` CPI instruction.
pub struct MoveLamportsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Active or inactive source stake account
    pub source_stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Mergeable destination stake account
    pub destination_stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: MoveLamportsInstructionArgs,
}

impl<'a, 'b> MoveLamportsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MoveLamportsCpiAccounts<'a, 'b>,
        args: MoveLamportsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            source_stake: accounts.source_stake,
            destination_stake: accounts.destination_stake,
            stake_authority: accounts.stake_authority,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.source_stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.destination_stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&MoveLamportsInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.source_stake.clone());
        account_infos.push(self.destination_stake.clone());
        account_infos.push(self.stake_authority.clone());
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

/// Instruction builder for `MoveLamports` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` source_stake
///   1. `[writable]` destination_stake
///   2. `[signer]` stake_authority
#[derive(Clone, Debug)]
pub struct MoveLamportsCpiBuilder<'a, 'b> {
    instruction: Box<MoveLamportsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MoveLamportsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MoveLamportsCpiBuilderInstruction {
            __program: program,
            source_stake: None,
            destination_stake: None,
            stake_authority: None,
            args: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Active or inactive source stake account
    #[inline(always)]
    pub fn source_stake(
        &mut self,
        source_stake: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.source_stake = Some(source_stake);
        self
    }
    /// Mergeable destination stake account
    #[inline(always)]
    pub fn destination_stake(
        &mut self,
        destination_stake: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.destination_stake = Some(destination_stake);
        self
    }
    /// Stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_authority = Some(stake_authority);
        self
    }
    #[inline(always)]
    pub fn args(&mut self, args: u64) -> &mut Self {
        self.instruction.args = Some(args);
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
        let args = MoveLamportsInstructionArgs {
            args: self.instruction.args.clone().expect("args is not set"),
        };
        let instruction = MoveLamportsCpi {
            __program: self.instruction.__program,

            source_stake: self
                .instruction
                .source_stake
                .expect("source_stake is not set"),

            destination_stake: self
                .instruction
                .destination_stake
                .expect("destination_stake is not set"),

            stake_authority: self
                .instruction
                .stake_authority
                .expect("stake_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct MoveLamportsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    source_stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    destination_stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    args: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
