use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum VaultInstruction {
    CreateTokenMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
}
