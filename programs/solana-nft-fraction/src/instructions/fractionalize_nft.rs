use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, self, Transfer}, associated_token};

use crate::state::fraction_details::FractionDetails;

pub fn fractionalize_nft_handler(
    ctx: Context<FractionalizeNft>,
    // shares_amount: u64,
) -> Result<()> {
    // Transfer NFT to vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.nft_vault.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred to vault");

    // Freeze the NFT in the vault
    // Set the withdraw authority of the vault to the signer

    Ok(())
}

#[derive(Accounts)]
pub struct FractionalizeNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, 
        space = FractionDetails::LEN,
        payer = signer, 
        seeds = [
            b"fraction_account", 
            ],
        bump
    )]
    pub fraction_account: Account<'info, FractionDetails>,

    #[account(
        init, 
        payer = signer, 
        token::mint = token_mint,
        token::authority = fraction_account,
        seeds = [
            b"nft_vault", 
            ],
        bump
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    pub token_account: Account<'info, TokenAccount>,
    
    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
