use solana_program::custom::global_storage::GLOBAL_VARS;
pub use crate::clock::Clock;
use crate::{impl_sysvar_get, program_error::ProgramError, sysvar::Sysvar};

crate::declare_sysvar_id!("SysvarC1ock11111111111111111111111111111111", Clock);

impl Sysvar for Clock {
    fn get() -> Result<Self, ProgramError> {
        let globals = GLOBAL_VARS.lock().unwrap();
        Ok(Clock {
            slot: globals.slot,
            epoch_start_timestamp: 0,
            epoch: 0,
            leader_schedule_epoch: 0,
            unix_timestamp: globals.timestamp as i64
        })
    }
}
