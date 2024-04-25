use crate::transaction::TransactionStack;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref GLOBAL_EXECUTOR: Mutex<TransactionStack> = Mutex::new(TransactionStack::new());
}