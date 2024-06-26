use std::sync::{Arc, RwLock};
use super_lib::prelude::{SuperKey, SuperVars, VMID};
use crate::prelude::{AccountManager, AccountMeta, DataManager, StateManager};
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
        Self::new_with_id(0)
    }

    pub fn new_with_id(id: VMID) -> Self {
        VMState {
            id,
            lamports_per_signature: 5000,
            vars: SuperVars::default(),
            blockhash: String::from("EkSnNWid2cvwEVnVx9aBqawnmiCNiDgp3gUdkDPTKN1N")
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

    pub fn initialize(&self, manager: &DataManager) {
        manager.set_account_meta(&SuperKey::faucet().to_string(), &AccountMeta {
            address: SuperKey::faucet(),
            owner: SuperKey::system_program(),
            executable: false,
            lamports: 1000000000000000000,
        });
        println!("Initialized VM State: {}", self.id);
    }
}