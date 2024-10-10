use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub fn process_set_resolver(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}
