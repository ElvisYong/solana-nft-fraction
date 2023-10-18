pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("2FVnCxEJWcuxBVBZSphHPhLt3LyuXtbpDHubm4rXu1tP");

#[program]
pub mod solana_nft_fraction {
    // use crate::instruction::LockAndFractionalize;

    use super::*;


    pub fn init_fractionalize(ctx: Context<InitFractionalize>, shares_amount: u64) -> Result<()> {
        init_fractionalize_handler(ctx, shares_amount)
    }

    // pub fn lock_and_fractionalize(
    //     ctx: Context<LockAndFractionalize>,
    //     shares_amount: u64,
    // ) -> Result<()> { 
    //     lock_and_fractionalize(ctx, shares_amount)
    // }
}
