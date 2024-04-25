use std::hash::Hash;
use std::sync::Mutex;
use super_lib::prelude::*;
use crate::pubkey::Pubkey;


lazy_static::lazy_static! {
    pub static ref GLOBAL_VARS: Mutex<GlobalVars> = Mutex::new(GlobalVars::new());
}


pub struct GlobalVars {
    pub self_id: Pubkey,
    pub slot: u64,
    pub timestamp: u64,
    pub lamports_per_byte_year: u64,
}


impl GlobalVars {
    pub fn new() -> Self {
        GlobalVars {
            self_id: Pubkey::from([0; 32]),
            slot: 0,
            timestamp: 0,
            lamports_per_byte_year: 0
        }
    }

    pub fn consume_vars(&mut self, vars: &Self) {
        self.self_id = vars.self_id.clone();
        self.slot = vars.slot;
        self.timestamp = vars.timestamp;
        self.lamports_per_byte_year = vars.lamports_per_byte_year;
    }

    pub fn from(vars: &SuperVars, program_id: &SuperKey) -> Self {
        Self {
            self_id: Pubkey::from(program_id.0.clone()),
            slot: vars.slot,
            timestamp: vars.timestamp,
            lamports_per_byte_year: vars.lamports_per_byte_year
        }
    }
}