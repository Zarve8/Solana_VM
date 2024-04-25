pub use crate::rent::Rent;
use crate::{impl_sysvar_get, program_error::ProgramError, sysvar::Sysvar};
use crate::custom::global_storage::GLOBAL_VARS;

crate::declare_sysvar_id!("SysvarRent111111111111111111111111111111111", Rent);


impl Sysvar for Rent {
    fn get() -> Result<Self, ProgramError> {
        let globals = GLOBAL_VARS.lock().unwrap();
        Ok(Rent {
            lamports_per_byte_year: globals.lamports_per_byte_year,
            exemption_threshold: 2.0,
            burn_percent: 0
        })
    }
}