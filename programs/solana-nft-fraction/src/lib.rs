use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("2NuYbQqS8SBtzBt21t2BL1CtGtWfNegvAXj9gYktn9oP");

#[program]
pub mod solana_nft_fraction {
    use super::*;

    pub fn fractionalize_nft(ctx: Context<FractionalizeNft>, share_amount: u64) -> Result<()> {
        fractionalize_nft_handler(ctx, share_amount)
    }
}
