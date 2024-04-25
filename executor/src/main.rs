use piper::prelude::ChildMessenger;
use solana_program::program_error::ProgramError;
use super_lib::prelude::{SuperReporter, SuperTransaction};

mod system_program;
mod executor;
mod tests;


pub mod prelude {
    use super_lib::prelude::*;
    use crate::executor::executor::Executor;

    pub fn execute(mut tx: SuperTransaction) -> SuperReporter {
        let mut executor = Executor::new(tx.state);
        for instr in tx.instructions.iter() {
            match executor.execute_instruction(instr) {
                None => {},
                Some(err_data) => {
                    let mut report = executor.pack_reporter();
                    report.error = Some(err_data);
                    return report;
                }
            };
        }
        executor.pack_reporter()
    }
}


pub fn take_fees(tx: &mut SuperTransaction) -> Result<(), (ProgramError, String)> {
    if tx.state.accounts.len() < tx.payer {
        return Err((ProgramError::InvalidArgument, format!("Payer account not priveded")));
    }
    let fees: u64 = 5000;
    let (payer_meta, payer_acc) = &mut tx.state.accounts[tx.payer];
    if payer_acc.lamports < fees {
        return Err((ProgramError::InsufficientFunds, format!("Failed to pay fees")));
    }
    if payer_meta.writable == false {
        return Err((ProgramError::InvalidArgument, format!("Payer account is not writable")));
    }
    if payer_meta.executable == true {
        return Err((ProgramError::InvalidArgument, format!("Payer account cannot be executable")));
    }
    if payer_meta.is_signer == false {
        return Err((ProgramError::MissingRequiredSignature, format!("Payer account is a signer")));
    }
    payer_acc.lamports -= fees;
    Ok(())
}

pub fn preprocess(tx: &mut SuperTransaction) -> Result<(), (ProgramError, String)> {
    take_fees(tx)
}


#[cfg(not(test))]
fn main() {
    let mut tx: SuperTransaction = ChildMessenger::receive();
    match preprocess(&mut tx) {
        Ok(_) => {},
        Err((error, msg)) => {
            let mut reporter = SuperReporter::new();
            reporter.error = Some(error.into());
            reporter.log(msg);
            ChildMessenger::send(reporter);
            return;
        }
    }
    let reporter = prelude::execute(tx);
    ChildMessenger::send(reporter);
}