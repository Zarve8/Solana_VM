use std::collections::HashMap;
use std::sync::Mutex;
use solana_program::pubkey::Pubkey;


lazy_static::lazy_static! {
    pub static ref GLOBAL: Mutex<ForceAccountUpdate> = Mutex::new(ForceAccountUpdate::new());
}

pub struct ForceAccountUpdate {
    pub datas: HashMap<Pubkey, Vec<u8>>,
    pub owners: HashMap<Pubkey, Pubkey>
}

impl ForceAccountUpdate {
    pub fn new() -> Self {
        Self {
            datas: HashMap::new(),
            owners: HashMap::new()
        }
    }

    pub fn push_data(key: &Pubkey, data: Vec<u8>) {
        let mut global = GLOBAL.lock().unwrap();
        global.datas.insert(key.clone(), data);
    }

    pub fn push_owner(key: &Pubkey, owner: Pubkey) {
        let mut global = GLOBAL.lock().unwrap();
        global.owners.insert(key.clone(), owner.clone());
    }

    pub fn pack() -> (HashMap<Pubkey, Vec<u8>>, HashMap<Pubkey, Pubkey>) {
        let global = GLOBAL.lock().unwrap();
        (global.datas.clone(), global.owners.clone())
    }
}