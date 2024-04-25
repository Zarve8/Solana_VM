


pub mod prelude {
    use super_lib::prelude::*;

    #[cfg(feature = "executor")]
    pub fn get_program_path(id: VMID, key: &SuperKey) -> String {
        String::from("contract")
    }
}


