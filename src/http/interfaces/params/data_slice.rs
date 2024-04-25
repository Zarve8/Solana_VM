use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DataSliceParam {
    offset: Option<u64>,
    length: Option<u64>
}

impl DataSliceParam {
    pub fn validate(&self) {
        //TODO
    }
}