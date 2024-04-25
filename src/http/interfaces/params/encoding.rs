use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub enum EncodingType {
    Base58,
    Base64,
    ZSTD,
    JsonParsed
}

impl EncodingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EncodingType::Base58 => "base58",
            EncodingType::Base64=> "base64",
            EncodingType::ZSTD => "base64+zstd",
            EncodingType::JsonParsed => "jsonParsed"
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "base58" => EncodingType::Base58,
            "base64" => EncodingType::Base64,
            "base64+zstd" => EncodingType::ZSTD,
            "jsonParsed" => EncodingType::JsonParsed,
            _ => panic!("Invalid encoding")
        }
    }
}