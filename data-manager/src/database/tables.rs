use redb::TableDefinition;
use super_lib::prelude::VMID;

pub const ACCOUNT_DATA_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("account_data");
pub const ACCOUNT_META_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("account_meta");
pub const BLOCK_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("block");
pub const TRANSACTION_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("transaction");
pub const STATE_TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("state");