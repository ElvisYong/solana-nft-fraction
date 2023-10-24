use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("796GNwLYHzPyjcR5T6vhcge7RMfYr7kE8QDFBN23hdvm");

#[program]
pub mod solana_nft_fraction {
    use super::*;

    pub fn fractionalize_nft(ctx: Context<FractionalizeNft>, share_amount: u64) -> Result<()> {
        fractionalize_nft_handler(ctx, share_amount)
    }

    pub fn mint_fraction(ctx: Context<MintFraction>, share_amount: u64) -> Result<()> {
        mint_fraction_handler(ctx, share_amount)
    }
}
