use std::collections::{HashMap};
use {
    crate::{
        account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction,
         pubkey::Pubkey,
    },
};
use super_lib::prelude::*;
use crate::custom::solvm::SolVm;
use piper::messenger::child_messenger::Messenger;
use super_lib::prelude::*;
use solana_program::program_error::ProgramError;
use crate::instruction::AccountMeta;
use crate::custom::account_storage::STORAGE;
use crate::custom::global_storage::GLOBAL_VARS;
use crate::msg;


pub fn not_accessible<T>(method: &str) -> T {
    panic!("{} not accessible without Environment provided", method)
}

pub fn superkey_from_pubkey(key: &Pubkey) -> SuperKey {
    SuperKey(key.0)
}

pub fn superaccount_from_accountinfo(info: &AccountInfo) -> SuperAccount {
    SuperAccount {
        lamports: **info.lamports.borrow(),
        data: info.data.borrow().to_vec(),
    }
}

pub fn supermeta_from_accountmeta(meta: &AccountMeta, info: &AccountInfo) -> SuperMeta {
    SuperMeta {
        address: superkey_from_pubkey(&meta.pubkey),
        owner: superkey_from_pubkey(&info.owner),
        executable: info.executable, //Not required
        writable: meta.is_writable,
        is_signer: meta.is_signer,
    }
}


pub struct DefaultSolVm {}

impl DefaultSolVm {
    pub fn new() -> Self {
        return DefaultSolVm {}
    }
}


#[allow(clippy::arithmetic_side_effects)]
impl SolVm for DefaultSolVm {
// +++++++++ Logger +++++++++
    fn sol_log(&self, message: &str) {
        Messenger::send(SuperSysCall::Log {
           message: String::from(message)
        });
    }

    fn sol_log_compute_units(&self) {
        self.sol_log("compute units not available");
    }

    fn sol_remaining_compute_units(&self) -> u64 {
        0
    }

    fn sol_log_64(&self, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) {
        self.sol_log(&format!(
            "{arg1:#x}, {arg2:#x}, {arg3:#x}, {arg4:#x}, {arg5:#x}"
        ));
    }

    fn sol_log_data(&self, fields: &[&[u8]]) {
        not_accessible("sol_get_return_data")
    }
// ++++++++++ CPI ++++++++++
    fn sol_invoke_signed(
        &mut self,
        _instruction: &Instruction,
        _account_infos: &[AccountInfo],
        _signers_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let mut signed: Option<SuperKey> = None;
        let self_id: Pubkey = {GLOBAL_VARS.lock().unwrap().self_id.clone()};
        if(_signers_seeds.len() > 0) {
            let generated_key = Pubkey::create_program_address(
                _signers_seeds[0], &self_id
            );
            signed = match generated_key {
                Ok(key) => Some(SuperKey(key.to_bytes())),
                Err(_) => None //TODO return invalid seeds error
            };
        }
        //TODO return error on account not provided
        let mut accounts: HashMap<SuperKey, &AccountInfo> = _account_infos.iter()
            .map(|info| (
                (superkey_from_pubkey(&info.key), info)
            )).collect();
        let metas: Vec<SuperMeta> = _instruction.accounts.iter()
            .map(|meta| supermeta_from_accountmeta
                (meta,
                 accounts.get(&superkey_from_pubkey(&meta.pubkey)).unwrap())
            )
            .collect();
        //accounts.retain(|_, info| info.is_writable && info.owner.eq(&self_id));
        Messenger::send(SuperSysCall::CPI {
            program_id: superkey_from_pubkey(&_instruction.program_id),
            transfer: SuperTransfer {
                metas,
                accounts: accounts.iter()
                    .filter(|(_, info)| info.is_writable && info.owner.eq(&self_id))
                    .map(|(key, info)| (key.clone(), superaccount_from_accountinfo(*info)))
                    .collect()
            },
            instruction: _instruction.data.clone(),
            accounts: _instruction.accounts.iter()
                .map(|meta| superkey_from_pubkey(&meta.pubkey))
                .collect(),
            signed
        });
        let mut result: SuperSysCall = Messenger::receive();
        match result {
            SuperSysCall::ProgramFinished {mut transfer, error} => {
                match error {
                    Some(err_data) => {
                        Err(ProgramError::from(err_data))
                    }
                    None => {
                        let mut storage = STORAGE.lock().unwrap();
                        for (key, account) in transfer.accounts.iter_mut() {
                            storage.insert(key, account.to_owned());
                            let replacable = accounts.get(key).unwrap();
                            replacable.data.replace(storage.get_static::<SuperKey, SuperAccount>(key).data.as_mut_slice());
                            replacable.lamports.replace(&mut storage.get_static::<SuperKey, SuperAccount>(key).lamports);
                        }
                        Ok(())
                    }
                }
            }
            _ => {
                panic!("Unexpected Cross_Process Syscall")
            }
        }
    }

    fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
        not_accessible("sol_get_return_data")
    }

    fn sol_set_return_data(&self, _data: &[u8]) {
        not_accessible("sol_set_return_data")
    }

    fn sol_get_processed_sibling_instruction(&self, _index: usize) -> Option<Instruction> {
        not_accessible("sol_get_processed_sibling_instruction")
    }

    fn sol_get_clock_sysvar(&self, _var_addr: *mut u8) -> u64 {
        not_accessible("sol_get_clock_sysvar")
    }

    fn sol_get_epoch_schedule_sysvar(&self, _var_addr: *mut u8) -> u64 {
        not_accessible("sol_get_epoch_schedule_sysvar")
    }

    fn sol_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64 {
        not_accessible("sol_get_fees_sysvar")
    }

    fn sol_get_rent_sysvar(&self, _var_addr: *mut u8) -> u64 {
        not_accessible("sol_get_rent_sysvar")
    }

    fn sol_get_epoch_rewards_sysvar(&self, _var_addr: *mut u8) -> u64 {
        not_accessible("sol_get_epoch_rewards_sysvar")
    }

    fn sol_get_last_restart_slot(&self, _var_addr: *mut u8) -> u64 {
        not_accessible("sol_get_last_restart_slot")
    }

    unsafe fn sol_memcpy(&self, dst: *mut u8, src: *const u8, n: usize) {
        not_accessible("sol_memcpy")
    }

    unsafe fn sol_memmove(&self, dst: *mut u8, src: *const u8, n: usize) {
        not_accessible("sol_memmove")
    }

    unsafe fn sol_memcmp(&self, s1: *const u8, s2: *const u8, n: usize, result: *mut i32) {
        not_accessible("sol_memcmp")
    }

    unsafe fn sol_memset(&self, s: *mut u8, c: u8, n: usize) {
        not_accessible("sol_memset")
    }

    fn sol_get_stack_height(&self) -> u64 {
        not_accessible("sol_get_stack_height")
    }
}