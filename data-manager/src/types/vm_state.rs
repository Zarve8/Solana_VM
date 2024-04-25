use std::sync::{Arc, RwLock};
use super_lib::prelude::{SuperVars, VMID};
use crate::prelude::{DataManager, StateManager};
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct VMState {
    pub id: VMID,
    pub lamports_per_signature: u64,
    pub vars: SuperVars,
    pub blockhash: String
}

impl VMState {
    pub fn new() -> Self {
        VMState {
            id: 0,
            lamports_per_signature: 5000,
            vars: SuperVars::default(),
            blockhash: String::new()
        }
    }

    pub fn load(id: VMID, manager: &DataManager) -> Self {
        let state = manager.get_state(id);
        match state {
            None => {Self::new()}
            Some(state) => state
        }
    }

    pub fn save(&self, data_manager: &Arc<RwLock<DataManager>>) {
        let manager = data_manager.read().expect("Failed to access Global Data");
        manager.set_state(&self);
        println!("Saved VM State");
    }
}