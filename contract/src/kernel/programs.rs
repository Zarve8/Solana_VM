use solana_program::pubkey::Pubkey;

//FaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa
const PROGRAM_ID_BYTES: [u8; 32] = [216,137,15,246,105,235,18,242,111,141,103,128,249,105,115,18,16,140,15,252,183,174,191,35,208,224,18,83,70,53,156,163];
pub const SELF_PROGRAM_ID: Pubkey = Pubkey::new_from_array(PROGRAM_ID_BYTES);
