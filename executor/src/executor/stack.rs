use piper::messenger::spawner_messenger::Messenger as SpawnerMessenger;
use std::process::{Command, Stdio};
use std::collections::HashSet;
use super_lib::prelude::*;
use program_manager::prelude::get_program_path;


pub type ChildProcess = SpawnerMessenger<SuperKey>;

pub const CHILD_EXEC: &str = "/home/gregory/Desktop/Rust/super-vm/target/debug/spawner";


pub struct ExecutorStack {
    id: VMID,
    processes: Vec<ChildProcess>,
    signers_stack: Vec<HashSet<SuperKey>>
}

impl ExecutorStack {
    pub fn default() -> Self {
        Self {
            id: 0,
            processes: Vec::new(),
            signers_stack: Vec::new()
        }
    }

    pub fn new(id: VMID, program_id: &SuperKey, signers: HashSet<SuperKey>) -> Self {
        let child = Command::new(CHILD_EXEC)
            .arg(get_program_path(id, program_id))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        Self {
            id,
            processes: Vec::from([SpawnerMessenger::new(child, program_id.clone())]),
            signers_stack: Vec::from([signers])
        }
    }

    pub fn depth(&self) -> usize {
        self.processes.len()
    }

    pub fn push_program(&mut self, program_id: &SuperKey, signed: Option<SuperKey>) {
        let mut signers = self.signers_stack.last().unwrap().clone();
        match signed {
            None => {},
            Some(signer) => { signers.insert(signer); }
        };
        let child = Command::new(CHILD_EXEC)
            .arg(get_program_path(self.id, program_id))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        self.processes.push(SpawnerMessenger::new(child, program_id.clone()));
        self.signers_stack.push(signers);
    }

    pub fn pop_program(&mut self) -> SuperKey {
        self.signers_stack.pop();
        let mut process = self.processes.pop().unwrap();
        process.kill();
        process.data
    }

    pub fn get_last_id<'a>(&'a self) -> &'a SuperKey {
        &self.processes.last().unwrap().data
    }

    pub fn get_last_program<'a>(&'a mut self) -> &'a mut ChildProcess {
        self.processes.last_mut().unwrap()
    }

    pub fn get_last_signers<'a>(&'a self) -> &'a HashSet<SuperKey> {
        self.signers_stack.last().unwrap()
    }

    pub fn send(&mut self, data: SuperSysCall) {
        let mut process = self.get_last_program();
        process.send::<SuperSysCall>(data);
    }
}
