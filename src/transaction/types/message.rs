use std::collections::HashSet;
use data_manager::prelude::AccountData;
use super_lib::prelude::{SuperAccount, SuperInstruction, SuperKey, SuperMeta, SuperState, SuperTransaction};
use crate::global::data_manager::GLOBAL_DATA;
use crate::global::state::GLOBAL_STATE;
use crate::transaction::types::compiled_instruction::CompiledInstruction;
use crate::transaction::types::hash::Hash;
use crate::transaction::types::message_header::MessageHeader;
use data_manager::prelude::AccountManager;


pub struct Message {
    pub header: MessageHeader,
    pub account_keys: Vec<SuperKey>,
    pub recent_blockhash: Hash,
    pub signers: HashSet<SuperKey>,
    pub writables: HashSet<SuperKey>,
    pub instructions: Vec<CompiledInstruction>,
    pub payer: usize
}


impl Message {
    pub fn tx_fee(&self) -> u64 {
        let state = GLOBAL_STATE.lock().expect("Failed to read Global State");
        state.lamports_per_signature * (self.header.num_required_signatures as u64)
    }

    pub fn compile_message(&self) -> SuperTransaction {
        let state = GLOBAL_STATE.lock().expect("Failed to read Global State");
        let manager = GLOBAL_DATA.read().expect("Failed to read Global DaTA");
        let mut state = SuperState {
            id: state.id,
            vars: state.vars.clone(),
            accounts: self.account_keys.iter()
                .map(|key| {
                    let (meta, lamports) = match manager.get_account_meta_by_key(key) {
                        None =>  {
                            (SuperMeta {
                                address: key.clone(),
                                owner: SuperKey::system_program(),
                                is_signer: self.signers.contains(key),
                                writable: self.writables.contains(key),
                                executable: false
                            }, 0)
                        },
                        Some(meta) => {
                            (SuperMeta {
                                address: meta.address,
                                owner: meta.owner,
                                is_signer: self.signers.contains(key),
                                writable: self.writables.contains(key),
                                executable: meta.executable
                            }, meta.lamports)
                        }
                    };
                    let data = match manager.get_account_data_by_key(key) {
                        None => Vec::new(),
                        Some(data) => data.bytes
                    };
                    (meta, SuperAccount{lamports, data})
                })
                .collect(),
        };
        SuperTransaction {
            instructions: self.instructions.iter()
                .map(|instr| SuperInstruction {
                    program: instr.program_id_index as usize,
                    accounts: instr.accounts.iter().map(|index| *index as usize).collect(),
                    data: instr.data.clone(),
                })
                .collect(),
            state,
            payer: self.payer
        }
    }
}