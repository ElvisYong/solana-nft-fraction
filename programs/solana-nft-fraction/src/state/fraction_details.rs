use anchor_lang::prelude::*;

#[account]
pub struct FractionDetails {
    /// The number of shares that exist for this NFT. (8)
    pub shares_amount: u64,
}

impl FractionDetails {
    pub const LEN: usize = 8;
}