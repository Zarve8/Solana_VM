pub use crate::epoch_rewards::EpochRewards;
use crate::{impl_sysvar_get, program_error::ProgramError, sysvar::Sysvar};

crate::declare_sysvar_id!("SysvarEpochRewards1111111111111111111111111", EpochRewards);


impl Sysvar for EpochRewards {
    fn get() -> Result<Self, ProgramError> {
        Ok(EpochRewards {
            total_rewards: 0,
            distributed_rewards: 0,
            distribution_complete_block_height: 0
        })
    }
}