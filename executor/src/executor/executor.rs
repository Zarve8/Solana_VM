use super_lib::prelude::*;
use crate::executor::state::ExecutorState;
use crate::executor::stack::ExecutorStack;
use crate::system_program::utils::SYSTEM_PROGRAM;


pub struct Executor {
    id: VMID,
    vars: SuperVars,
    state: ExecutorState,
    stack: ExecutorStack,
    reported: SuperReporter,
}

impl Executor {
    pub fn new(state: SuperState) -> Self {
        Self {
            state: ExecutorState::new(&state),
            id: state.id,
            vars: state.vars,
            reported: SuperReporter::new(),
            stack: ExecutorStack::default()
        }
    }

    pub fn execute_instruction(&mut self, instr: &SuperInstruction) -> Option<u64>{
        self.stack = ExecutorStack::new(self.id, self.state.get_by_index(instr.program), self.state.initial_signers.clone());
        self.stack.send(SuperSysCall::StartProgram {
            program_id: self.state.get_by_index(instr.program).clone(),
            transfer: self.state.construct_transfer_from_instr(&instr, self.stack.get_last_signers()),
            instruction: instr.data.clone(),
            vars: self.vars.clone()
        });
        self.reported.log(format!("Program {} invoke [1]", self.stack.get_last_id().to_string()));
        while true {
            let call = {
                self.stack.get_last_program().receive::<SuperSysCall>()
            };
            let result = self.process_call(call);
            match result {
                Ok(next) => {
                    if !next {
                        return None;
                    }
                }
                Err(err_data) => {
                    return Some(err_data);
                }
            }
        }
        None // Never reached
    }

    fn process_call(&mut self, call: SuperSysCall) -> Result<bool, u64>{
        match call {
            SuperSysCall::StartProgram { .. } => {panic!("Start Call not allowed from Program");}
            SuperSysCall::CPI { program_id, transfer, instruction, accounts, signed } => {
                let caller_id = self.stack.get_last_id();
                self.state.consume_transfer_only_writable(&transfer, caller_id);
                self.reported.log(format!("Program {} invoke [{}]", program_id.to_string(), self.stack.depth() + 1));
                self.reported.inner_instructions.push(SuperInnerInstruction {
                    program: program_id.clone(),
                    accounts: accounts.clone(),
                    data: instruction.clone(),
                    stack_height: self.stack.depth() + 1,
                });
                drop(transfer);
                self.process_cpi(program_id, instruction, accounts, signed);
                Ok(true)
            }
            SuperSysCall::Log { message } => {
                self.reported.log(format!("Program Logged: {}", message));
                Ok(true)
            }
            SuperSysCall::ProgramFinished { error, transfer} => {
                let called_id = self.stack.pop_program();
                if self.stack.depth() == 0 {
                    self.process_instruction_finish(&called_id, error, transfer)
                }
                else {
                    self.process_cpi_finish(&called_id, error, transfer);
                    Ok(true)
                }
            }
        }
    }

    fn process_cpi(&mut self, program_id: SuperKey, instruction: Vec<u8>, accounts: Vec<SuperKey>, signed: Option<SuperKey>)  {
        self.stack.push_program(&program_id, signed);
        self.stack.send(SuperSysCall::StartProgram {
            program_id,
            transfer: self.state.construct_transfer(&accounts, self.stack.get_last_signers()),
            instruction,
            vars: self.vars.clone()
        });
    }

    fn process_cpi_finish(&mut self, called_id: &SuperKey, error: Option<u64>, transfer: SuperTransfer) {
        match error {
            None => {
                if called_id.eq(&SYSTEM_PROGRAM) {
                    self.state.consume_transfer_with_meta(&transfer);
                }
                else {
                    self.state.consume_transfer(&transfer);
                }
                self.reported.log(String::from("Program consumed: 0 of 0 compute units"));
                self.reported.log(format!("Program returned {} success", called_id.to_string()));
                self.stack.send(SuperSysCall::ProgramFinished {
                    error: None,
                    transfer,
                });
            }
            Some(err_data) => {
                self.reported.log(format!("Program returned error: {}", crate::system_program::utils::format_program_error(err_data)));
                // self.stack.pop_program();
                self.stack.send(SuperSysCall::ProgramFinished {
                    error: Some(err_data),
                    transfer: SuperTransfer::empty(),
                });
            }
        }
    }

    fn process_instruction_finish(&mut self, called_id: &SuperKey, error: Option<u64>, transfer: SuperTransfer) -> Result<bool, u64> {
        match error {
            None => {
                if called_id.eq(&SYSTEM_PROGRAM) {
                    self.state.consume_transfer_with_meta(&transfer);
                }
                else {
                    self.state.consume_transfer(&transfer);
                }
                self.reported.log(String::from("Program consumed: 0 of 0 compute units"));
                self.reported.log(format!("Program returned {} success", called_id.to_string()));
                Ok(false)
            }
            Some(err_data) => {
                self.reported.log(format!("Program {} failed: {}", called_id.to_string(), crate::system_program::utils::format_program_error(err_data)));
                Err(err_data)
            }
        }
    }

    pub fn pack_reporter(&mut self) -> SuperReporter {
        {
            let mut reporter = &mut self.reported;
            for (key, meta) in self.state.metas.iter() {
                if meta.writable {
                    reporter.changed_accounts.push(
                        (meta.clone(), self.state.accounts.get(key).unwrap().clone())
                    );
                }
            }
        }
        self.reported.clone()
    }
}
