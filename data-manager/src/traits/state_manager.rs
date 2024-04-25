use super_lib::prelude::VMID;
use crate::types::vm_state::VMState;


pub trait StateManager {
    fn get_state(&self, id: VMID) -> Option<VMState>;
    fn set_state(&self, state: &VMState);
}
