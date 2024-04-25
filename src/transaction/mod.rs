use std::collections::VecDeque;
use super_lib::prelude::{SuperInstruction, SuperKey, SuperReporter, SuperTransaction};
use crate::transaction::types::message::Message;
use std::future::Future;
use std::process::{Command, Stdio};
use futures::channel::oneshot;
use data_manager::prelude::{TransactionData};
use crate::global::data_manager::GLOBAL_DATA;
use crate::global::state::GLOBAL_STATE;
use data_manager::prelude::TransactionManager;
use std::time::{SystemTime, UNIX_EPOCH};
use sha256::{digest, try_digest};
use piper::messenger::spawner_messenger::Messenger as SpawnerMessenger;

pub mod types;


pub type TransactionOutput = Result<String, SuperReporter>;

pub struct TransactionStack {
    txs: VecDeque<Message>,
    resolvers: VecDeque<oneshot::Sender<TransactionOutput>>,
    is_active: bool,
}

impl TransactionStack {
    pub fn new() -> Self {
        TransactionStack {
            txs: VecDeque::new(),
            resolvers: VecDeque::new(),
            is_active: false
        }
    }

    fn invoke_self(&mut self) {
        println!("Invoked");
        if self.is_active {
            println!("Invoking Self Twice");
            return;
        }
        self.is_active = true;
        let mut msg = self.txs.pop_back();
        let mut resolver = self.resolvers.pop_back();
        while msg.is_some() {
            let res = Self::process_tx(msg.unwrap(), {
                let state = GLOBAL_STATE.lock().expect("Failed to read Global State");
                state.blockhash.clone()
            });
            println!("Execution Result Accured");
            resolver.unwrap().send(res);
            println!("Execution Result Sent");
            msg = self.txs.pop_back();
            resolver = self.resolvers.pop_back();
        }
        self.is_active = false;
        println!("Queue is empty");
    }

    pub fn push(&mut self, msg: Message) -> oneshot::Receiver<TransactionOutput>{
        let (sender, receiver) = oneshot::channel::<TransactionOutput>();
        self.txs.push_front(msg);
        self.resolvers.push_front(sender);
        self.invoke_self();
        if !self.is_active {
            self.invoke_self();
        }
        println!("Awaiting Response from Oneshot Channel");
        receiver
    }

    fn process_tx(msg: Message, recent_hash: String) -> TransactionOutput {
        let tx = msg.compile_message(); //TODO add check
        match Self::invoke_executor(&tx) {
            None => {
                println!("Failed Execution with unknown Error");
                Err(SuperReporter::from_internal_failure())
            }
            Some(reporter) => {
                match reporter.error {
                    None => {
                        println!("Report: {:?}", reporter);
                        let time = Self::get_time();
                        let tx_data = TransactionData::from_reporter(
                            &tx,
                            reporter,
                            vec![], //TODO add when signed
                            time,
                            recent_hash.clone()
                        );
                        let block_hash = Self::next_hash(&recent_hash);
                        {
                            let manager = GLOBAL_DATA.read().expect("Failed to access Global Data");
                            manager.set_transaction(&block_hash, &tx_data);
                        }
                        {
                            let mut state = GLOBAL_STATE.lock().expect("Failed to read Global State");
                            state.blockhash = block_hash.clone();
                            state.vars.slot += 1;
                            state.vars.timestamp = time;
                            state.save(&GLOBAL_DATA);
                        }
                        Ok(block_hash)
                    }
                    Some(err_data) => {
                        println!("Failed Execution with data: {}", err_data);
                        Err(reporter)
                    }
                }
            }
        }
    }

    fn invoke_executor(tx: &SuperTransaction) -> Option<SuperReporter> {
        let result = std::panic::catch_unwind(|| {
            let CHILD_EXEC: &str = "/home/gregory/Desktop/Rust/super-vm/target/debug/executor";
            let child = Command::new(CHILD_EXEC)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            let mut messenger = SpawnerMessenger::new(child, ());
            messenger.send::<SuperTransaction>(tx.clone());
            messenger.receive::<SuperReporter>()
        });
        match result {
            Ok(report) => Some(report),
            Err(_) => None
        }
    }

    fn next_hash(hash: &String) -> String {
        digest(hash.clone())
    }

    fn get_time() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }
}
