#![allow(deprecated)]

use {
    crate::{
        fee_calculator::FeeCalculator, impl_sysvar_get, program_error::ProgramError, sysvar::Sysvar,
    },
    solana_sdk_macro::CloneZeroed,
};
use solana_program::custom::global_storage::GLOBAL_VARS;

crate::declare_deprecated_sysvar_id!("SysvarFees111111111111111111111111111111111", Fees);

/// Transaction fees.
#[deprecated(
    since = "1.9.0",
    note = "Please do not use, will no longer be available in the future"
)]
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, CloneZeroed, Default, PartialEq, Eq)]
pub struct Fees {
    pub fee_calculator: FeeCalculator,
}

impl Fees {
    pub fn new(fee_calculator: &FeeCalculator) -> Self {
        #[allow(deprecated)]
        Self {
            fee_calculator: *fee_calculator,
        }
    }
}


impl Sysvar for Fees {
    fn get() -> Result<Self, ProgramError> {
        Ok(Fees{
            fee_calculator: FeeCalculator {
                lamports_per_signature: 5000,
            }
        })
    }
}
