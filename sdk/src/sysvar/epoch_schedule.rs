pub use crate::epoch_schedule::EpochSchedule;
use crate::{impl_sysvar_get, program_error::ProgramError, sysvar::Sysvar};

crate::declare_sysvar_id!("SysvarEpochSchedu1e111111111111111111111111", EpochSchedule);

impl Sysvar for EpochSchedule {
    fn get() -> Result<Self, ProgramError> {
        Ok(EpochSchedule {
            slots_per_epoch: 0,
            leader_schedule_slot_offset: 0,
            warmup: false,
            first_normal_epoch: 0,
            first_normal_slot: 0
        })
    }
}