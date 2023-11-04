use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("5FYYwBNgxgGdUWWrY1Mxo53nwLFzH3q8pwHQD3BNre8x");

#[program]
pub mod solana_nft_fraction {
    use super::*;

    pub fn fractionalize_nft(ctx: Context<FractionalizeNft>, share_amount: u64) -> Result<()> {
        fractionalize_nft_handler(ctx, share_amount)
    }

    pub fn unfractionalize_nft(ctx: Context<UnfractionalizeNft>) -> Result<()> {
        unfractionalize_nft_handler(ctx)
    }
}

#[error_code]
pub enum MyError {
    #[msg("SPL token owner does not belong to the user")]
    WrongOwner,
    #[msg("Not enough shares to unfractionalize")]
    NotEnoughShares,
}