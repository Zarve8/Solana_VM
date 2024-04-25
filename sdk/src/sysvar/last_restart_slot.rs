pub use crate::last_restart_slot::LastRestartSlot;
use crate::{impl_sysvar_get, program_error::ProgramError, sysvar::Sysvar};

crate::declare_sysvar_id!(
    "SysvarLastRestartS1ot1111111111111111111111",
    LastRestartSlot
);

#[cfg(feature = "solvm")]
impl Sysvar for LastRestartSlot {
    fn get() -> Result<Self, ProgramError> {
        Ok(LastRestartSlot {
            last_restart_slot: 0
        })
    }
}
