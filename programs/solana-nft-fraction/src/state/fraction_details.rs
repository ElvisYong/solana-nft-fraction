use anchor_lang::prelude::*;

#[account]
pub struct FractionDetails {
    /// The authority that can withdraw the NFT from the vault. (32)
    pub withdraw_authority: Pubkey,
    /// The number of shares that exist for this NFT. (8)
    pub shares_amount: u64,
}

impl FractionDetails {
    pub const LEN: usize = 32 + 8;
}