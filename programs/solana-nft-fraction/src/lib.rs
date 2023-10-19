use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("2FVnCxEJWcuxBVBZSphHPhLt3LyuXtbpDHubm4rXu1tP");

#[program]
pub mod solana_nft_fraction {
    use super::*;

    pub fn fractionalize_nft(ctx: Context<FractionalizeNft>) -> Result<()> {
        fractionalize_nft_handler(ctx)
    }
}
