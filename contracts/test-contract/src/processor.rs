use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::instruction::Instruction;
use borsh::BorshDeserialize;
use crate::test_instructions::account_create::create_account;
use crate::test_instructions::account_write::write_account;
use crate::test_instructions::check_signature::check_signature;
use crate::test_instructions::message::message;
use crate::test_instructions::program_call::program_call;
use crate::test_instructions::program_signed_call::program_signed_call;
use crate::test_instructions::sysvars_read::read_sysvars;
use crate::test_instructions::throw_error::throw_error;
use crate::test_instructions::transfer_sol::transfer_sol;


pub struct Processor {}


impl Processor {
    pub fn process_instruction<'g>(_program_id: &Pubkey, accounts: &'g [AccountInfo<'g>], instruction_data: &[u8],) -> ProgramResult {
        let instruction = Instruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let accounts_iter = &mut accounts.iter();
        match instruction {
            Instruction::Message => message(accounts_iter, _program_id),
            Instruction::WriteAccount => write_account(accounts_iter, _program_id),
            Instruction::CallProgram => program_call(accounts_iter, _program_id),
            Instruction::CallProgramSigned => program_signed_call(accounts_iter, _program_id),
            Instruction::CheckSignature => check_signature(accounts_iter, _program_id),
            Instruction::ReadSysvars => read_sysvars(accounts_iter, _program_id),
            Instruction::TransferSol {amount} => transfer_sol(accounts_iter, _program_id, amount as u64),
            Instruction::CreateAccount {bytes} => create_account(accounts_iter, _program_id, bytes),
            Instruction::ThrowError => throw_error(accounts_iter, _program_id),
            _ => {msg!("Instruction not implemented"); Ok(())}
        }
    }
}