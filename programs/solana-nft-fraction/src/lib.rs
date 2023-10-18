use anchor_lang::prelude::*;

declare_id!("2FVnCxEJWcuxBVBZSphHPhLt3LyuXtbpDHubm4rXu1tP");

#[program]
pub mod solana_nft_fraction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
