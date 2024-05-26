pub mod transfer;
pub mod assign;
pub mod set_account_owner;
pub mod can_data_be_resized;
pub mod can_data_be_changed;
pub mod set_data_length;
pub mod allocate;
pub mod create_account;
pub mod allocate_and_assign;

use solana_program::account_info::AccountInfo;
use solana_program::instruction::InstructionError;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use solana_program::system_instruction::SystemInstruction;
use crate::limited_deserialize::limited_deserialize;
use crate::processor::create_account::create_account;
use crate::processor::transfer::{transfer, transfer_with_seeds};


pub struct Processor {}

impl Processor {
    pub fn process_instruction<'g>(_program_id: &Pubkey, accounts: &'g [AccountInfo<'g>], instruction_data: &[u8]) -> Result<(), InstructionError> {
        let instruction = limited_deserialize::<SystemInstruction>(instruction_data)?;
        let accounts_iter = &mut accounts.iter();
        match instruction {
            SystemInstruction::Transfer {lamports} => transfer(accounts_iter, lamports),
            SystemInstruction::TransferWithSeed {lamports, from_seed, from_owner } => transfer_with_seeds(accounts_iter, lamports, from_seed, from_owner),
            SystemInstruction::CreateAccount {space, lamports, owner} => create_account(accounts_iter, lamports, space, &owner),
            SystemInstruction::CreateAccountWithSeed {lamports, space, owner, base, seed} => create_account(accounts_iter, lamports, space, &owner),
            _ => {
                msg!("System Instruction not Implemented");
                Ok(())
            }
        }
    }
}