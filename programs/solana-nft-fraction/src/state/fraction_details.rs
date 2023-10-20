use anchor_lang::prelude::*;

#[account]
pub struct FractionDetails {
    /// The vault account for the fractionalized NFT. (32)
    pub nft_vault_account: Pubkey,
    /// The nft mint for the fractionalized NFT. (32)
    pub nft_mint: Pubkey,
    /// The spl token mint for the fractionalized NFT. (32)
    pub spl_token_mint: Pubkey,
    /// The authority that can withdraw the NFT from the vault. (32)
    pub withdraw_authority: Pubkey,
    /// The number of shares that exist for this NFT. (8)
    pub shares_amount: u64,
}

impl FractionDetails {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8;
}