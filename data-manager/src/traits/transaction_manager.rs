use super_lib::prelude::SuperKey;
use crate::prelude::TransactionData;


pub trait TransactionManager {
    fn get_transaction(&self, key: &String) -> Option<TransactionData>;
    fn set_transaction(&self, key: &String, transaction: &TransactionData);
}
