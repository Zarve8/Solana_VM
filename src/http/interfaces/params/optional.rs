use serde::{Deserialize, Serialize};
use crate::http::interfaces::params::commitment::CommitmentType;
use crate::http::interfaces::params::data_slice::DataSliceParam;
use crate::http::interfaces::params::encoding::EncodingType;


#[derive(Deserialize, Serialize, Debug)]
pub struct OptionalParams {
    pub encoding: Option<String>,
    pub commitment: Option<String>,
    pub minContextSlot: Option<u64>,
    pub dataSlice: Option<DataSliceParam>
}

impl OptionalParams {
    pub fn default() -> Self {
        OptionalParams {
            encoding: None,
            commitment: None,
            minContextSlot: None,
            dataSlice: None
        }
    }

    pub fn validate(&self) {
        match &self.encoding {
            Some(encoding) => {EncodingType::from_str(encoding);},
            None => {}
        }
        match &self.commitment {
            Some(commitment) => {CommitmentType::from_str(commitment);}
            None => {}
        }
        match &self.dataSlice {
            Some(param) => {param.validate();}
            None => {}
        }
    }

    pub fn from_option(option: Option<OptionalParams>) -> OptionalParams {
        match option {
            Some(value) => value,
            None => Self::default()
        }
    }
}