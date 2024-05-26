use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;
use libloading::{Library, library_filename, Symbol};
use piper::{
    messenger::child_messenger::Messenger
};
use solana_program::{pubkey::Pubkey, account_info::AccountInfo, msg};
use super_lib::prelude::*;
use solana_program::custom::global_storage::{GlobalVars};


pub type ContractEntrypoint = fn(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8], vars: &GlobalVars) -> u64;
pub type SystemEntrypoint = fn(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8], vars: &GlobalVars) -> Result<(HashMap<Pubkey, Vec<u8>>, HashMap<Pubkey, Pubkey>), u64>;

pub fn pack_account_info<'a>(meta: &'a SuperMeta, data: Rc<RefCell<&'a mut [u8]>>, lamports: Rc<RefCell<&'a mut u64>>) -> AccountInfo<'a> {
    AccountInfo {
        key: meta.address.as_any(),
        lamports,
        data,
        owner: meta.owner.as_any(),
        rent_epoch: 0,
        is_signer: meta.is_signer,
        is_writable: meta.writable,
        executable: meta.executable
    }
}


fn call_contract(name: String, program_id: &SuperKey, accounts: &[AccountInfo], instruction_data: &[u8], vars: &GlobalVars) -> Option<u64> {
    unsafe {
        let lib = Library::new(library_filename(name)).unwrap();
        let mut entrypoint: Symbol<ContractEntrypoint> = lib.get(b"entrypoint").unwrap();
        let result = entrypoint(
            program_id.as_any(),
            accounts,
            instruction_data,
            vars
        );
        match result {
            0 => None,
            _ => Some(result)
        }
    }
}

fn call_system(name: String, program_id: &SuperKey, accounts: &[AccountInfo], instruction_data: &[u8], vars: &GlobalVars) -> Result<(HashMap<Pubkey, Vec<u8>>, HashMap<Pubkey, Pubkey>), u64>{
    unsafe {
        let lib = Library::new(library_filename(name)).unwrap();
        let mut entrypoint: Symbol<SystemEntrypoint> = lib.get(b"entrypoint").unwrap();
        entrypoint(
            program_id.as_any(),
            accounts,
            instruction_data,
            vars
        )
    }
}


fn main() {
    let args: Vec<_> = env::args().collect();
    unsafe {
        while true {
            let mut start: SuperSysCall = Messenger::receive();
            match start {
                SuperSysCall::StartProgram { program_id, mut transfer, instruction, vars } => {
                    let mut infos = Vec::with_capacity(transfer.accounts.len());
                    let mut datas: HashMap<SuperKey, Rc<RefCell<&mut [u8]>>> = HashMap::new();
                    let mut lamports: HashMap<SuperKey, Rc<RefCell<&mut u64>>> = HashMap::new();
                    for (key, account) in transfer.accounts.iter_mut() {
                        datas.insert(
                            key.clone(),
                            Rc::new(RefCell::new(account.data.as_mut()))
                        );
                        lamports.insert(
                            key.clone(),
                            Rc::new(RefCell::new(&mut account.lamports))
                        );
                    }
                    for meta in transfer.metas.iter() {
                        infos.push(
                            pack_account_info(meta,
                                              Rc::clone(datas.get(&meta.address).unwrap()),
                                              Rc::clone(lamports.get(&meta.address).unwrap())
                            )
                        );
                    }
                    if program_id.eq(&SuperKey::system_program()) {
                        let result = call_system(args[1].clone(),
                                                  &program_id,
                                                  infos.as_mut(),
                                                  instruction.as_slice(),
                                                  &GlobalVars::from(&vars, &program_id));
                        match result {
                            Ok(update) => {
                                transfer.force_update(
                                    update.0.iter()
                                        .map(|(key, data)| (SuperKey(key.to_bytes()), data))
                                        .collect(),
                                    update.1.iter()
                                        .map(|(key, owner)| (SuperKey(key.to_bytes()), SuperKey(owner.to_bytes())))
                                        .collect()
                                );
                                Messenger::send(SuperSysCall::ProgramFinished {
                                    transfer,
                                    error: None
                                });
                            }
                            Err(code) => {
                                transfer.filter(&program_id);
                                Messenger::send(SuperSysCall::ProgramFinished {
                                    transfer,
                                    error: Some(code)
                                });
                            }
                        }
                    }
                    else {
                        let error = call_contract(args[1].clone(),
                                      &program_id,
                                      infos.as_mut(),
                                      instruction.as_slice(),
                                      &GlobalVars::from(&vars, &program_id));
                        transfer.filter(&program_id);
                        Messenger::send(SuperSysCall::ProgramFinished {
                            transfer,
                            error
                        });
                    }
                }
                _ => {
                    panic!("Invalid Call on Program Start")
                }
            };
        }
    };
}