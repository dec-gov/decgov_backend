use hex::FromHexError;

pub fn from_hex(data: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(&data[2..])
}

pub fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}
