use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::fraction_details::FractionDetails;

pub fn init_fractionalize_handler(
    ctx: Context<InitFractionalize>,
    shares_amount: u64,
) -> Result<()> {
}

#[derive(Accounts)]
pub struct InitFractionalize<'info> {
    #[account(
        init, 
        payer = creator, 
        space = Details::LEN,
        seeds = [
            b"fraction", 
            collection_address.key().as_ref(),
            creator.key().as_ref()
        ],
        bump
    )]
    pub fraction_details: Account<'info, FractionDetails>,

    #[account(mint::decimals = 0)]
    pub collection_address: Account<'info, Mint>,
}
