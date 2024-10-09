use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::instruction::VaultInstruction;

#[allow(clippy::too_many_arguments)]
pub fn create_token_metadata(
    program_id: &Pubkey,
    mint_account: &Pubkey,
    mint_authority: &Pubkey,
    metadata: &Pubkey,
    payer: &Pubkey,
    token_program_id: &Pubkey,
    name: String,
    symbol: String,
    uri: String,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*mint_account, true),
        AccountMeta::new_readonly(*mint_authority, true),
        AccountMeta::new(*metadata, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*token_program_id, false),
        AccountMeta::new_readonly(crate::inline_mpl_token_metadata::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data: VaultInstruction::CreateTokenMetadata { name, symbol, uri }
            .try_to_vec()
            .unwrap(),
    }
}
