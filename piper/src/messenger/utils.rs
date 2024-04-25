pub fn u32_to_bytes(x: usize) -> [u8; 4] {
    (x as u32).to_ne_bytes()
}

pub fn bytes_to_u32(b: [u8; 4]) -> usize {
    u32::from_ne_bytes(b) as usize
}

pub const MESSAGE_CODE: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

