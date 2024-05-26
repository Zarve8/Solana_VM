use solana_program::instruction::InstructionError;
use solana_program::account_info::{AccountInfo};
use solana_program::pubkey::Pubkey;
use crate::processor::allocate::allocate;
use crate::processor::assign::assign;


pub fn allocate_and_assign<'a>(account: &AccountInfo<'a>, space: u64, owner: &Pubkey) -> Result<(), InstructionError> {
    allocate(account, space)?;
    assign(account, owner)
}
