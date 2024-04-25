use {
    crate::{
        account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction,
        pubkey::Pubkey,
    },
    std::sync::{Arc, RwLock},
};
use crate::custom::default::DefaultSolVm;
use std::sync::Mutex;


lazy_static::lazy_static! {
    //pub static ref SOLVM: Mutex<Arc<RwLock<Box<dyn SolVm>>>> = Mutex::new(Arc::new(RwLock::new(Box::new(DefaultSolVm::new()))));
    pub static ref SOLVM: Mutex<Box<dyn SolVm>> = Mutex::new((Box::new(DefaultSolVm::new())));
}

pub trait SolVm: Sync + Send {
    fn sol_log(&self, message: &str);
    fn sol_log_compute_units(&self);
    fn sol_remaining_compute_units(&self) -> u64;
    fn sol_log_64(&self, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64);
    fn sol_log_data(&self, fields: &[&[u8]]);
    fn sol_invoke_signed(
        &mut self,
        _instruction: &Instruction,
        _account_infos: &[AccountInfo],
        _signers_seeds: &[&[&[u8]]],
    ) -> ProgramResult;
    fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)>;
    fn sol_set_return_data(&self, _data: &[u8]);
    fn sol_get_processed_sibling_instruction(&self, _index: usize) -> Option<Instruction>;
    fn sol_get_clock_sysvar(&self, _var_addr: *mut u8) -> u64;
    fn sol_get_epoch_schedule_sysvar(&self, _var_addr: *mut u8) -> u64;
    fn sol_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64;
    fn sol_get_rent_sysvar(&self, _var_addr: *mut u8) -> u64;
    fn sol_get_epoch_rewards_sysvar(&self, _var_addr: *mut u8) -> u64;
    fn sol_get_last_restart_slot(&self, _var_addr: *mut u8) -> u64;
    unsafe fn sol_memcpy(&self, dst: *mut u8, src: *const u8, n: usize);
    unsafe fn sol_memmove(&self, dst: *mut u8, src: *const u8, n: usize);
    unsafe fn sol_memcmp(&self, s1: *const u8, s2: *const u8, n: usize, result: *mut i32);
    unsafe fn sol_memset(&self, s: *mut u8, c: u8, n: usize);
    fn sol_get_stack_height(&self) -> u64;
}
