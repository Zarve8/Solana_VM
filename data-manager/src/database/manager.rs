use redb::{Database, ReadableTable, TableDefinition};
use borsh::{BorshDeserialize, BorshSerialize};
use super_lib::prelude::{SuperKey, VMID};
use crate::database::tables::{ACCOUNT_DATA_TABLE, ACCOUNT_META_TABLE, STATE_TABLE, TRANSACTION_TABLE};
use crate::prelude::{AccountData, AccountManager, AccountMeta, TransactionData, TransactionManager};
use crate::traits::state_manager::StateManager;
use crate::types::vm_state::VMState;


pub struct DataManager {
    db: Database
}

impl DataManager {
    pub fn new(path: &str) -> Self {
        let db = Database::create(path).expect("Failed to create database");
        let write_txn = db.begin_write().expect("Failed to write tx");
        {
            write_txn.open_table(ACCOUNT_META_TABLE).expect("Failed to open table");
            write_txn.open_table(ACCOUNT_DATA_TABLE).expect("Failed to open table");
            write_txn.open_table(TRANSACTION_TABLE).expect("Failed to open table");
            write_txn.open_table(STATE_TABLE).expect("Failed to open table");
        }
        write_txn.commit().expect("Failed to commit tx");
        Self { db }
    }

    fn set<D: BorshSerialize,>(&self, table: TableDefinition<&str, Vec<u8>>, key: &str, data: &D) {
        let mut buf = Vec::new();
        data.serialize(&mut buf).expect("Failed to serialize data for database");
        let write_txn = self.db.begin_write().expect("Failed to write tx");
        {
            let mut table = write_txn.open_table(table).expect("Failed to open table");
            table.insert(key, buf).expect("Failed to insert value");
        }
        write_txn.commit().expect("Failed to commit tx");
    }

    fn get<T: BorshDeserialize>(&self, table: TableDefinition<&str, Vec<u8>>, key: &str) -> Option<T> {
        let read_txn = self.db.begin_read().expect("Failed to read database");
        let table = read_txn.open_table(table).expect("Failed to open table");
        let buf = match table.get(key) {
            Ok(data) => {
                match data {
                    Some(data) => {
                        data.value()
                    },
                    None => {return None}
                }
            }
            Err(_) => {
                return None;
            }
        };
        Some(T::deserialize(&mut buf.as_slice()).expect("Failed to deserialize data"))
    }
}

impl AccountManager for DataManager {
    fn get_account_meta(&self, key: &str) -> Option<AccountMeta> {
        self.get(ACCOUNT_META_TABLE, key)
    }

    fn set_account_meta(&self, key: &str, meta: &AccountMeta) {
        self.set(ACCOUNT_META_TABLE, key, meta);
    }

    fn get_account_data(&self, key: &str) -> Option<AccountData> {
        self.get(ACCOUNT_DATA_TABLE, key)
    }

    fn set_account_data(&self, key: &str, data: &AccountData) {
        self.set(ACCOUNT_DATA_TABLE, key, data);
    }

    fn get_account_meta_by_key(&self, key: &SuperKey) -> Option<AccountMeta> {
        self.get(ACCOUNT_META_TABLE, &key.to_string())
    }

    fn set_account_meta_by_key(&self, key: &SuperKey, meta: &AccountMeta) {
        self.set(ACCOUNT_META_TABLE, &key.to_string(), meta);
    }

    fn get_account_data_by_key(&self, key: &SuperKey) -> Option<AccountData> {
        self.get(ACCOUNT_DATA_TABLE, &key.to_string())
    }

    fn set_account_data_by_key(&self, key: &SuperKey, data: &AccountData) {
        self.set(ACCOUNT_DATA_TABLE, &key.to_string(), data);
    }
}

impl TransactionManager for DataManager {
    fn get_transaction(&self, key: &String) -> Option<TransactionData> {
        self.get(TRANSACTION_TABLE, key)
    }

    fn set_transaction(&self, key: &String, transaction: &TransactionData) {
        self.set(TRANSACTION_TABLE, key, transaction);
    }
}

impl StateManager for DataManager {
    fn get_state(&self, id: VMID) -> Option<VMState> {
        self.get(STATE_TABLE, &id.to_string())
    }

    fn set_state(&self, state: &VMState) {
        self.set(STATE_TABLE, &state.id.to_string(), state);
    }
}
