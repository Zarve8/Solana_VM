use super_lib::prelude::SuperKey;
use crate::types::block_data::BlockData;

pub trait BlockManager {
    fn get_block(&mut self, key: &SuperKey) -> Option<BlockData>;
    fn set_block(&mut self, key: &SuperKey, block: &BlockData);
}
