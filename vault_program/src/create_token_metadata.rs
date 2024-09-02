use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
    program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, rent::Rent,
    system_instruction, sysvar::Sysvar,
};
use vault_sdk::inline_mpl_token_metadata::instruction::create_metadata_accounts_v3;

pub fn process_create_token_metadata(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    symbol: String,
    uri: String,
) -> ProgramResult {
    let [mint_account, mint_authority, metadata, payer, token_program, mpl_token_metadata_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // First create the account for the Mint
    //
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(spl_token_2022::state::Mint::LEN),
            spl_token_2022::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Now initialize that account as a Mint (standard Mint)
    //
    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &spl_token_2022::instruction::initialize_mint2(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            None,
            9,
        )?,
        &[mint_account.clone()],
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata.key);

    let new_metadata_instruction = create_metadata_accounts_v3(
        *mpl_token_metadata_program.key,
        *metadata.key,
        *mint_account.key,
        *mint_authority.key,
        *payer.key,
        *mint_authority.key,
        name,
        symbol,
        uri,
    );

    invoke(
        &new_metadata_instruction,
        &[
            metadata.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}
