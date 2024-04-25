use std::sync::{Arc, RwLock};
use data_manager::prelude::*;


lazy_static::lazy_static! {
    pub static ref GLOBAL_DATA: Arc<RwLock<DataManager>> = Arc::new(RwLock::new(DataManager::new("db.redb")));
}