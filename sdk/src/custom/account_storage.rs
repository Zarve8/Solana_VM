use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::sync::Mutex;
use super_lib::prelude::*;
use crate::msg;


lazy_static::lazy_static! {
    pub static ref STORAGE: Mutex<AccountStorage> = Mutex::new(AccountStorage::new::<SuperKey, SuperAccount>());
}


// Never use outside of module
// Will lead to memory leaks
pub struct AccountStorage {
    pub pointer: u64,
    pub capacity: usize
}


impl AccountStorage {
    pub fn new<K: Sized + Clone + Hash, V: Sized + Clone>() -> Self {
        let mut map: HashMap<K, V> = HashMap::new();
        let mut itered: Vec<(K, V)> = map.iter_mut().map(|(key, value)| (key.to_owned(), value.to_owned())).collect();
        let mut pointer = itered.as_mut_ptr() as *mut _ as *mut u8;
        AccountStorage {
            pointer: pointer as u64,
            capacity: 0
        }
    }

    fn load<K: Sized + Clone + Hash + Eq, V: Sized + Clone>(&mut self) -> HashMap<K, V> {
        unsafe {
            let mut PTR: *mut u8 = self.pointer as *mut u8;
            let mut vector: Vec<(K, V)> = Vec::from( std::slice::from_raw_parts(PTR as *mut (K, V), self.capacity));
            //TODO optimise: drops value
            vector.iter_mut().map(|(key, value)| (key.to_owned(), value.to_owned())).collect()
        }
    }

    fn save<K: Sized + Clone + Hash, V: Sized + Clone>(&mut self, mut map: HashMap<K, V>) {
        //TODO optimise: drops value
        let mut itered: Vec<(K, V)> = map.iter_mut().map(|(key, value)| (key.to_owned(), value.to_owned())).collect();
        self.capacity = itered.len();
        let mut pointer = itered.as_mut_ptr() as *mut _ as *mut u8;
        self.pointer = pointer as u64;
        mem::forget(itered);
    }

    pub fn insert<K: Sized + Clone + Hash + Eq, V: Sized + Clone>(&mut self, key: &K, value: V) {
        unsafe {
            let mut map: HashMap<K, V> = self.load::<K, V>();
            if map.contains_key(key) {
                let old_value = map.remove(key).unwrap();
                drop(old_value);
            }
            map.insert(key.clone(), value);
            self.save(map);
        }
    }

    pub fn get_static<K: Sized + Clone + Hash + Eq, V: Sized + Clone>(&mut self, key: &K) -> &'static mut V {
        unsafe {
            let mut map: HashMap<K, V> = self.load::<K, V>();
            let value: *mut V = map.get_mut(key).unwrap();
            mem::forget(map);
            value.as_mut().unwrap()
        }
    }
}