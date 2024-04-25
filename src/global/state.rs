use data_manager::prelude::VMState;
use std::sync::Mutex;
use crate::global::data_manager::GLOBAL_DATA;


lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: Mutex<VMState> = Mutex::new({
        let manager = GLOBAL_DATA.read().expect("Failed to access GLObal DAta on load");
        VMState::load(0, &manager)
    });
}


