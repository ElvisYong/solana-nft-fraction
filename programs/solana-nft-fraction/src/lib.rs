use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("DpNF7npTTAfuGpAd7ghu7APkdYWw2CMSR2XHmjaUysMv");

#[program]
pub mod solana_nft_fraction {
    use super::*;

    pub fn fractionalize_nft(ctx: Context<FractionalizeNft>, share_amount: u64) -> Result<()> {
        fractionalize_nft_handler(ctx, share_amount)
    }
}
