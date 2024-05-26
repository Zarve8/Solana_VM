use std::collections::{HashMap, HashSet};
use solana_program::program_error::ProgramError;
use super_lib::prelude::*;


#[derive(Debug)]
pub struct ExecutorState {
    indexed: Vec<SuperKey>,
    pub accounts: HashMap<SuperKey, SuperAccount>,
    pub metas: HashMap<SuperKey, SuperMeta>,
    pub initial_signers: HashSet<SuperKey>
}


impl ExecutorState {
    pub fn new(mut state: &SuperState) -> Self {
        let mut accounts = HashMap::with_capacity(state.accounts.len());
        let mut metas = HashMap::with_capacity(state.accounts.len());
        let mut indexed = Vec::with_capacity(state.accounts.len());
        for (meta, account) in state.accounts.iter() {
            indexed.push(meta.address.clone());
            accounts.insert(meta.address.clone(), account.to_owned());
            metas.insert(meta.address.clone(), meta.to_owned());
        }
        Self {
            initial_signers: state.list_signers(),
            indexed,
            accounts,
            metas
        }
    }

    pub fn get_by_index(&self, index: usize) -> &SuperKey {
        &self.indexed[index]
    }

    pub fn construct_transfer_from_instr(&self, instr: &SuperInstruction, signers: &HashSet<SuperKey>) -> SuperTransfer {
        let mut accounts = HashMap::new();
        let mut metas = Vec::new();
        for index in instr.accounts.iter() {
            let address = self.get_by_index(*index);
            accounts.insert(address.clone(), self.accounts.get(address).unwrap().clone());
            let mut meta = self.metas.get(address).unwrap().clone();
            if signers.contains(address) {
                meta.is_signer = true;
            }
            metas.push(meta);
        }
        SuperTransfer {
            accounts,
            metas
        }
    }

    pub fn consume_transfer(&mut self, transfer: &SuperTransfer) {
        for (key, account) in transfer.accounts.iter() {
            let mut self_account = self.accounts.get_mut(key).unwrap();
            self_account.data = account.data.clone();
            // self_account.lamports = account.lamports;
        }
    }

    pub fn consume_transfer_only_writable(&mut self, transfer: &SuperTransfer, program_id: &SuperKey) {
        for meta in transfer.metas.iter() {
            if meta.writable && meta.owner.eq(program_id) {
                let transferred_account = transfer.accounts.get(&meta.address).unwrap();
                let mut self_account = self.accounts.get_mut(&meta.address).unwrap();
                self_account.data = transferred_account.data.clone();
                // self_account.lamports = transferred_account.lamports;
            }
        }
    }

    pub fn consume_transfer_with_meta(&mut self, transfer: &SuperTransfer) {
        for meta in transfer.metas.iter() {
            let mut self_meta = self.metas.get_mut(&meta.address).unwrap();
            let transferred_account = transfer.accounts.get(&meta.address).unwrap();
            let mut self_account = self.accounts.get_mut(&meta.address).unwrap();
            self_account.data = transferred_account.data.clone();
            self_account.lamports = transferred_account.lamports;
            self_meta.owner = meta.owner.clone();
        }
    }

    pub fn construct_transfer(&mut self, addresses: &Vec<SuperKey>, signers: &HashSet<SuperKey>) -> SuperTransfer {
        let mut accounts = HashMap::new();
        let mut metas = Vec::new();
        for address in addresses.iter() {
            accounts.insert(address.clone(), self.accounts.get(address).unwrap().clone());
            let mut meta = self.metas.get(address).unwrap().clone();
            if signers.contains(address) {
                meta.is_signer = true;
            }
            metas.push(meta);
        }
        SuperTransfer {
            accounts,
            metas
        }
    }
}
