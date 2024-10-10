use jito_bytemuck::types::PodU16;

#[derive(
    Debug, Clone, Copy, Zeroable, ShankType, Pod, Default, AccountDeserialize, ShankAccount,
)]
#[repr(C)]
pub struct Resolver {
    pub admin: Pubkey,

    pub index: PodU16,
}
