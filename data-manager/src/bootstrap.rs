


#[cfg(test)]
mod bootstrap {
    use super_lib::prelude::SuperKey;
    use crate::prelude::{AccountData, AccountManager, AccountMeta, DataManager};
    const SYSTEM_PROGRAM: SuperKey = SuperKey([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    const TOKEN_PROGRAM: SuperKey = SuperKey([6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169]);

    const WALLET: SuperKey = SuperKey([80, 247, 130, 50, 210, 73, 58, 239, 231, 177, 3, 109, 113, 232, 19,  49, 230, 54,  34, 156, 3, 13, 30, 205, 151, 81, 46, 195, 243, 240, 111, 165]);

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
    fn fill() {
        let manager = DataManager::new("../db.redb");
        manager.set_account_meta("11111111111111111111111111111111", &AccountMeta {
            address: SYSTEM_PROGRAM,
            owner: SYSTEM_PROGRAM,
            executable: true,
            lamports: 1000000000,
        });
        manager.set_account_meta("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", &AccountMeta {
            address: TOKEN_PROGRAM,
            owner: SYSTEM_PROGRAM,
            executable: true,
            lamports: 1000000000,
        });
        manager.set_account_meta("FaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa", &AccountMeta {
            address: PROGRAM_ID,
            owner: SYSTEM_PROGRAM,
            executable: true,
            lamports: 1000000000,
        });

        manager.set_account_meta("6T4WnzLmMKzfgrjFEw9gUXu284WqHsaksjtH6MLRRBxg", &AccountMeta {
            address: WALLET,
            owner: SYSTEM_PROGRAM,
            executable: false,
            lamports: 10000000000,
        });
    }

    // #[test]
    // fn fill() {
    //     let manager = DataManager::new("../db.redb");
    //     manager.set_account_meta("6yk2s4VtX1i7xN7R15WHGtU4AoMtStSBY98H87LtirW", &AccountMeta {
    //         address: ACCOUNT1,
    //         owner: SYSTEM_PROGRAM,
    //         executable: false,
    //         lamports: 987654321,
    //     });
    //     manager.set_account_meta("At9gMnanMWRXeV5vWLxBCdhKBN8bPzLmQT1nKRLcXWr", &AccountMeta {
    //         address: ACCOUNT2,
    //         owner: PROGRAM_ID,
    //         executable: false,
    //         lamports: 10000000000,
    //     });
    //     manager.set_account_meta("FaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa", &AccountMeta {
    //         address: PROGRAM_ID,
    //         owner: SYSTEM_PROGRAM,
    //         executable: true,
    //         lamports: 123123,
    //     });
    //
    //     manager.set_account_data("At9gMnanMWRXeV5vWLxBCdhKBN8bPzLmQT1nKRLcXWr", &AccountData {
    //         bytes: Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    //     });
    // }
}