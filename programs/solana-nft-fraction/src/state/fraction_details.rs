use anchor_lang::prelude::*;

#[account]
pub struct FractionDetails {
    /// The creator of the lockup record
    pub creator: Pubkey,
    /// The number of shares that exist for this NFT.
    pub shares_amount: u64,
}
