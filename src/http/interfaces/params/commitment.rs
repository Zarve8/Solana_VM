use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub enum CommitmentType {
    Processed,
    Confirmed,
    Finalized
}

impl CommitmentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommitmentType::Processed => "Processed",
            CommitmentType::Confirmed => "Confirmed",
            CommitmentType::Finalized => "Finalized",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Processed" => CommitmentType::Processed,
            "Confirmed" => CommitmentType::Confirmed,
            "Finalized" => CommitmentType::Finalized,
            _ => panic!("Invalid commitment")
        }
    }
}