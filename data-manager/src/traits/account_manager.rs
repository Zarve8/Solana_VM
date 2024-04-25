use super_lib::prelude::SuperKey;
use crate::prelude::{AccountData, AccountMeta};


pub trait AccountManager: Sync + Send {
    fn get_account_meta(&self, key: &str) -> Option<AccountMeta>;
    fn set_account_meta(&self, key: &str, meta: &AccountMeta);
    fn get_account_data(&self, key: &str) -> Option<AccountData>;
    fn set_account_data(&self, key: &str, data: &AccountData);
    fn get_account_meta_by_key(&self, key: &SuperKey) -> Option<AccountMeta>;
    fn set_account_meta_by_key(&self, key: &SuperKey, meta: &AccountMeta);
    fn get_account_data_by_key(&self, key: &SuperKey) -> Option<AccountData>;
    fn set_account_data_by_key(&self, key: &SuperKey, data: &AccountData);
}
