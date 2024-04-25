use {
    crate::{
        account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction,
        pubkey::Pubkey,
    },
};


macro_rules! define_stubcall {
	(fn $name:ident($($arg:ident: $typ:ty),*) -> $ret:ty) => {
		pub fn $name($($arg: $typ),*) -> $ret {
            unsafe {
                $crate::custom::solvm::SOLVM.lock().unwrap().$name($($arg), *)
            }
        }
	};
	(fn $name:ident($($arg:ident: $typ:ty),*)) => {
		define_stubcall!(fn $name($($arg: $typ),*) -> ());
	}
}


define_stubcall!(fn sol_log(message: &str));
define_stubcall!(fn sol_log_64(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64));
define_stubcall!(fn sol_log_compute_units());
define_stubcall!(fn sol_remaining_compute_units() -> u64);
define_stubcall!(fn sol_invoke_signed(instruction: &Instruction,account_infos: &[AccountInfo],signers_seeds: &[&[&[u8]]]) -> ProgramResult );
define_stubcall!(fn sol_get_clock_sysvar(var_addr: *mut u8) -> u64);
define_stubcall!(fn sol_get_epoch_schedule_sysvar(var_addr: *mut u8) -> u64);
define_stubcall!(fn sol_get_fees_sysvar(var_addr: *mut u8) -> u64);
define_stubcall!(fn sol_get_rent_sysvar(var_addr: *mut u8) -> u64);
define_stubcall!(fn sol_get_last_restart_slot(var_addr: *mut u8) -> u64);
define_stubcall!(fn sol_memcpy(dst: *mut u8, src: *const u8, n: usize));
define_stubcall!(fn sol_memmove(dst: *mut u8, src: *const u8, n: usize));
define_stubcall!(fn sol_memcmp(s1: *const u8, s2: *const u8, n: usize, result: *mut i32));
define_stubcall!(fn sol_memset(s: *mut u8, c: u8, n: usize));
define_stubcall!(fn sol_get_return_data() -> Option<(Pubkey, Vec<u8>)>);
define_stubcall!(fn sol_set_return_data(data: &[u8]));
define_stubcall!(fn sol_log_data(data: &[&[u8]]));
define_stubcall!(fn sol_get_processed_sibling_instruction(index: usize) -> Option<Instruction>);
define_stubcall!(fn sol_get_stack_height() -> u64);
define_stubcall!(fn sol_get_epoch_rewards_sysvar(var_addr: *mut u8) -> u64);