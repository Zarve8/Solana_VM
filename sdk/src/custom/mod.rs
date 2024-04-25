#[cfg(all(not(target_os = "solana"), feature = "solvm"))]
pub mod solvm;

#[cfg(all(not(target_os = "solana"), feature = "solvm"))]
pub mod definitions;

#[cfg(all(not(target_os = "solana"), feature = "solvm"))]
pub mod default;
pub mod account_storage;
pub mod global_storage;


#[cfg(all(not(target_os = "solana"), feature = "solvm"))]
pub use definitions::*;
