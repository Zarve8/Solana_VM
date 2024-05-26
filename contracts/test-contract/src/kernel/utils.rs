use std::fmt::Display;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;

pub fn assert<T: Display + std::cmp::PartialEq>(real: T, expected: T) -> ProgramResult {
    if real != expected {
        msg!("expected: {}, provided: {}", expected, real);
        return Err(ProgramError::Custom(0));
    }
    Ok(())
}
