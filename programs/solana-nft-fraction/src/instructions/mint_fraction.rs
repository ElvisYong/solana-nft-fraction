use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{
    associated_token::AssociatedToken, metadata::Metadata as TokenMetadata, token::Token,
};
use mpl_token_metadata::instructions::MintV1CpiBuilder;

use crate::state::fraction_details::FractionDetails;

pub fn mint_fraction_handler(ctx: Context<MintFraction>, share_amount: u64) -> Result<()> {
    let signer_seeds = [
        b"fraction",
        ctx.accounts.nft_mint.key.as_ref(),
        &[ctx.bumps.fraction_account],
    ];

    msg!("Minting fraction token...");
    // Careful of authority, we might need to create a pda authority just for signing as program
    // For now we will use the user as the authority
    MintV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .token(&ctx.accounts.user_token_account)
        .token_owner(Some(&&ctx.accounts.user))
        .metadata(&ctx.accounts.fraction_token_metadata)
        .mint(&ctx.accounts.token_mint)
        .authority(&ctx.accounts.user)
        .payer(&ctx.accounts.user)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .spl_ata_program(&ctx.accounts.ata_program)
        .amount(share_amount)
        .invoke_signed(&[&signer_seeds])?;
    msg!(
        "Fraction token minted to: {}",
        ctx.accounts.user_token_account.key()
    );

    Ok(())
}

#[derive(Accounts)]
pub struct MintFraction<'info> {
    /// The user who is fractionalizing the NFT
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA for the Fractionalized NFT Token.
    #[account(
        mut,
        seeds = [
            b"fraction", 
            nft_mint.key().as_ref(),
            ],
        bump
    )]
    pub fraction_account: Account<'info, FractionDetails>,

    /// Metadata account of the Fractionalized NFT Token.
    /// This account must be uninitialized.
    ///
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub fraction_token_metadata: UncheckedAccount<'info>,

    /// The NFT Mint Account
    /// CHECK: Account checked in mint_nft_handler
    pub nft_mint: AccountInfo<'info>,

    /// The account will be initialized if necessary.
    ///
    /// Must be a signer if:
    ///   * the token mint account does not exist.
    ///
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub token_mint: Signer<'info>,

    /// Destination token account
    /// CHECK: Account checked in CPI
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,

    /// Token Metadata Program
    /// CHECK: account checked in CPI
    pub token_metadata_program: Program<'info, TokenMetadata>,

    /// spl_ata program
    pub ata_program: Program<'info, AssociatedToken>,

    /// spl token program
    pub token_program: Program<'info, Token>,

    /// CHECK: account constraints for the system program
    #[account(address = sysvar::instructions::id())]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// Solana native system program
    pub system_program: Program<'info, System>,
}
