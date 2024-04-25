#[cfg(test)]
mod test {
    use super_lib::prelude::*;
    use crate::prelude::execute;


    const SYSTEM_PROGRAM: SuperKey = SuperKey([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    const PROGRAM_ID: SuperKey = SuperKey([216,137,15,246,105,235,18,242,111,141,103,128,249,105,115,18,16,140,15,252,183,174,191,35,208,224,18,83,70,53,156,163]);
    const ACCOUNT1: SuperKey = SuperKey([1,136,15,246,105,235,18,242,111,141,103,128,249,105,115,18,16,140,15,252,183,174,191,35,208,224,18,83,70,53,156,163]);
    const ACCOUNT2: SuperKey = SuperKey([2,136,15,246,105,235,18,242,111,141,103,128,249,105,115,18,16,140,15,252,183,174,191,35,208,224,18,83,70,53,156,163]);
    const ACCOUNT3: SuperKey = SuperKey([3,136,15,246,105,235,18,242,111,141,103,128,249,105,115,18,16,140,15,252,183,174,191,35,208,224,18,83,70,53,156,163]);
    const ACCOUNT4: SuperKey = SuperKey([4,136,15,246,105,235,18,242,111,141,103,128,249,105,115,18,16,140,15,252,183,174,191,35,208,224,18,83,70,53,156,163]);
    const ACCOUNT5: SuperKey = SuperKey([89, 132, 222, 201, 51, 95, 29, 209, 65, 125, 79, 151, 231, 105, 163, 147, 149, 145, 248, 160, 172, 60, 255, 124, 95, 202, 110, 228, 235, 203, 95, 27]);
    //  FaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa
    /*
        6yk2s4VtX1i7xN7R15WHGtU4AoMtStSBY98H87LtirW
        At9gMnanMWRXeV5vWLxBCdhKBN8bPzLmQT1nKRLcXWr
        EnZKrWfgC18wLc4S1cQ58NvaBvuJM6FMGkuHWjLLLBC
        JgxyMEka2VrM2j2wWsqy489qCVg1JC9w94nni3L48qY
        72SoBX41P6JbUS47asuvYaRrbofX4gEtM2sA8D1x8hVU
     */

    #[test]
    fn message() {
        let state = SuperState {
            id: 0,
            vars: SuperVars::default(),
            accounts: Vec::from([
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                SuperAccount{lamports: 1000000, data: Vec::new()})
            ])
        };
        let instr = SuperInstruction {
            program: 0,
            accounts: Vec::new(),
            data: Vec::from([0]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }

    #[test]
    fn account_write() {
        let state = SuperState {
            id: 0,
            vars: SuperVars::default(),
            accounts: Vec::from([
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()}),
                (SuperMeta{address: ACCOUNT1.clone(), is_signer: true, writable: true, executable: false, owner: PROGRAM_ID.clone()},
                 SuperAccount{lamports: 1000000000, data: vec![0; 40]}),
            ])
        };
        let instr = SuperInstruction {
            program: 0,
            accounts: Vec::from([1]),
            data: Vec::from([1]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }

    #[test]
    fn program_call_run() {
        let state = SuperState {
            id: 0,
            vars: SuperVars::default(),
            accounts: Vec::from([
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()}),
                (SuperMeta{address: ACCOUNT1.clone(), is_signer: true, writable: true, executable: false, owner: PROGRAM_ID.clone()},
                 SuperAccount{lamports: 1000000000, data: vec![0; 40]}),
            ])
        };
        let instr = SuperInstruction {
            program: 0,
            accounts: Vec::from([0, 1]),
            data: Vec::from([5]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }

    #[test]
    fn program_signed_call_run() {
        let state = SuperState {
            id: 0,
            vars: SuperVars::default(),
            accounts: Vec::from([
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()}),
                (SuperMeta{address: ACCOUNT5.clone(), is_signer: false, writable: true, executable: false, owner: PROGRAM_ID.clone()},
                 SuperAccount{lamports: 1000000000, data: vec![0; 40]}),
            ])
        };
        let instr = SuperInstruction {
            program: 0,
            accounts: Vec::from([0, 1]),
            data: Vec::from([6]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }

    fn sysvars_read_run() {
        let state = SuperState {
            id: 0,
            vars: SuperVars{
                lamports_per_byte_year: 1000,
                slot: 99,
                timestamp: 169999999
            },
            accounts: Vec::from([
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()})
            ])
        };
        let instr = SuperInstruction {
            program: 0,
            accounts: Vec::from([]),
            data: Vec::from([8]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }


    #[test]
    fn account_create() {
        let state = SuperState {
            id: 0,
            vars: SuperVars{
                lamports_per_byte_year: 1000,
                slot: 99,
                timestamp: 169999999
            },
            accounts: Vec::from([
                (SuperMeta{address: ACCOUNT1, is_signer: true, writable: true, executable: false, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000000, data: Vec::new()}),
                (SuperMeta{address: ACCOUNT2, is_signer: true, writable: true, executable: false, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 0, data: Vec::new()}),
                (SuperMeta{address: ACCOUNT5, is_signer: false, writable: true, executable: false, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 0, data: Vec::new()}),
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()}),
                (SuperMeta{address: SYSTEM_PROGRAM, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()})
            ])
        };
        let instr = SuperInstruction {
            program: 3,
            accounts: Vec::from([0, 1, 2, 4]),
            data: Vec::from([2, 50, 0, 0, 0]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }

    #[test]
    fn transfer_sol_run() {
        let state = SuperState {
            id: 0,
            vars: SuperVars{
                lamports_per_byte_year: 1000,
                slot: 99,
                timestamp: 169999999
            },
            accounts: Vec::from([
                (SuperMeta{address: ACCOUNT1, is_signer: true, writable: true, executable: false, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()}),
                (SuperMeta{address: ACCOUNT2, is_signer: false, writable: true, executable: false, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 101, data: vec![0; 40]}),
                (SuperMeta{address: PROGRAM_ID, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()}),
                (SuperMeta{address: SYSTEM_PROGRAM, is_signer: false, writable: false, executable: true, owner: SYSTEM_PROGRAM.clone()},
                 SuperAccount{lamports: 1000000, data: Vec::new()})
            ])
        };
        let instr = SuperInstruction {
            program: 3,
            accounts: Vec::from([1, 0, 3]),
            data: Vec::from([4, 0, 1, 0, 0]),
        };
        let tx = SuperTransaction {
            instructions: Vec::from([instr]),
            state,
        };
        let reporter = execute(tx);
        println!("Result: {:?}", reporter.error);
        #[cfg(feature = "reported")]
        print!("Reporter: {:?}", reporter);
    }
}



