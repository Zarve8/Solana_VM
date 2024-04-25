mod account;

#[cfg(any(feature = "virtual", feature = "executor"))]
mod execution;

#[cfg(any(feature = "super", feature = "executor"))]
mod transaction;
#[cfg(any(feature = "super", feature = "executor"))]
mod vm_id;


pub mod prelude {
    pub use crate::account::data::SuperData;
    pub use crate::account::account::SuperAccount;
    pub use crate::account::key::SuperKey;
    pub use crate::transaction::vars::SuperVars;
    pub use crate::account::meta::SuperMeta;

    #[cfg(any(feature = "virtual", feature = "executor"))]
    pub use crate::execution::transfer::SuperTransfer;
    #[cfg(any(feature = "virtual", feature = "executor"))]
    pub use crate::execution::syscall::SuperSysCall;

    #[cfg(any(feature = "super", feature = "executor"))]
    pub use crate::transaction::transaction::SuperTransaction;
    #[cfg(any(feature = "super", feature = "executor"))]
    pub use crate::transaction::reporter::SuperReporter;
    #[cfg(any(feature = "super", feature = "executor"))]
    pub use crate::transaction::inner_instruction::SuperInnerInstruction;
    #[cfg(any(feature = "super", feature = "executor"))]
    pub use crate::transaction::state::SuperState;
    #[cfg(any(feature = "super", feature = "executor"))]
    pub use crate::transaction::instruction::SuperInstruction;
    #[cfg(any(feature = "super", feature = "executor"))]
    pub use crate::vm_id::VMID;
}
