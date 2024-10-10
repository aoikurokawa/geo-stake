use solana_program::pubkey::Pubkey;

use crate::resolver::Resolver;

#[derive(
    Debug, Clone, Copy, Zeroable, ShankType, Pod, Default, AccountDeserialize, ShankAccount,
)]
#[repr(C)]
pub struct ResolverTable {
    pub ncn: Pubkey,

    pub vault: Pubkey,

    pub table: [Resolver; 32],
}
