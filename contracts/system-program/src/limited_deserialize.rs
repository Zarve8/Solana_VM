use solana_program::instruction::InstructionError;


pub const PACKET_DATA_SIZE: usize = 1280 - 40 - 8;

pub fn limited_deserialize<T>(instruction_data: &[u8]) -> Result<T, InstructionError>
    where
        T: serde::de::DeserializeOwned,
{
    solana_program::program_utils::limited_deserialize(
        instruction_data, PACKET_DATA_SIZE as u64,
    )
}