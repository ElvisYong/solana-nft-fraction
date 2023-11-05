use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata as TokenMetadata,
    token::{Token, TokenAccount},
};
use mpl_token_metadata::instructions::{BurnV1CpiBuilder, TransferV1CpiBuilder};

use crate::{state::fraction_details::FractionDetails, MyError};

#[derive(Accounts)]
pub struct UnfractionalizeNft<'info> {
    /// The user who is unfractionalizing the NFT
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA that holds the fraction account details
    /// We will use the nft_vault as the seed for the pda
    #[account(
        mut,
        seeds = [
            b"fraction", 
            nft_vault.key().as_ref(),
            ],
        bump
    )]
    pub fraction_account: Account<'info, FractionDetails>,

    /// The pda vault thats holding the NFT
    #[account(
        mut,
        seeds = [
            b"nft_vault", 
            fraction_token_mint.key().as_ref(),
            ],
        bump
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    /// The user account to hold the unfractionalized withdrawn nft
    /// CHECK: Checking in the program
    #[account(mut)]
    pub user_nft_account: UncheckedAccount<'info>,

    /// The current NFT Mint Account
    /// CHECK: Account checked in fractionalize_nft_handler
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,

    /// The current nft metadata account
    /// CHECK: Will be created by the fractionalize_nft_handler
    #[account(mut)]
    pub nft_metadata_account: AccountInfo<'info>,

    /// Metadata account of the Fractionalized NFT Token.
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub fraction_token_metadata: AccountInfo<'info>,

    /// The SPL Token that the user is going to burn to withdraw the NFT
    /// CHECK: Account checked in CPI
    #[account(mut)]
    pub user_fraction_token: Account<'info, TokenAccount>,

    /// The current token mint of the fraction token
    ///
    /// Must be a signer if:
    ///   * the token mint account does not exist.
    ///
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub fraction_token_mint: AccountInfo<'info>,

    /// Token Metadata Program
    /// CHECK: account checked in CPI
    pub token_metadata_program: Program<'info, TokenMetadata>,

    /// spl token program
    pub token_program: Program<'info, Token>,

    /// spl ata program
    pub ata_program: Program<'info, AssociatedToken>,

    /// CHECK: account constraints for the system program
    #[account(address = sysvar::instructions::id())]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// Solana native system program
    pub system_program: Program<'info, System>,
}

pub fn unfractionalize_nft_handler(ctx: Context<UnfractionalizeNft>) -> Result<()> {
    // 1. Check if user is the owner of the spl token
    if ctx.accounts.user.key != &ctx.accounts.user_fraction_token.owner {
        return err!(MyError::WrongOwner);
    }

    // 2. Check that user has all the shares
    let shares_amount = &ctx.accounts.fraction_account.shares_amount;
    if ctx.accounts.user_fraction_token.amount != *shares_amount {
        return err!(MyError::NotEnoughShares);
    }

    // 3. Burn the shares
    let nft_vault = ctx.accounts.nft_vault.key();
    let signer_seeds = [
        b"fraction",
        nft_vault.as_ref(),
        &[ctx.bumps.fraction_account],
    ];

    // Log the accounts
    msg!("user: {}", ctx.accounts.user.key());
    msg!("fraction_account: {}", ctx.accounts.fraction_account.key());
    msg!("nft_vault: {}", ctx.accounts.nft_vault.key());
    msg!("user_nft_account: {}", ctx.accounts.user_nft_account.key());
    msg!("nft_mint: {}", ctx.accounts.nft_mint.key());
    msg!("nft_metadata_account: {}", ctx.accounts.nft_metadata_account.key());
    msg!("fraction_token_metadata: {}", ctx.accounts.fraction_token_metadata.key());
    msg!("user_fraction_token: {}", ctx.accounts.user_fraction_token.key());
    msg!("fraction_token_mint: {}", ctx.accounts.fraction_token_mint.key());
    msg!("token_metadata_program: {}", ctx.accounts.token_metadata_program.key());
    msg!("token_program: {}", ctx.accounts.token_program.key());
    msg!("ata_program: {}", ctx.accounts.ata_program.key());
    msg!("sysvar_instructions: {}", ctx.accounts.sysvar_instructions.key());
    msg!("system_program: {}", ctx.accounts.system_program.key());

    msg!("Transferring the NFT back to the user");
    TransferV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .token(&ctx.accounts.nft_vault.to_account_info())
        .token_owner(&ctx.accounts.fraction_account.to_account_info())
        .destination_token(&ctx.accounts.user_nft_account)
        .destination_owner(&ctx.accounts.user)
        .mint(&ctx.accounts.nft_mint)
        .metadata(&ctx.accounts.nft_metadata_account)
        .authority(&ctx.accounts.fraction_account.to_account_info())
        .payer(&ctx.accounts.user)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .spl_ata_program(&ctx.accounts.ata_program)
        .invoke_signed(&[&signer_seeds])?;
    msg!("Transferred the NFT back to the user");


    msg!("Burning the shares");
    BurnV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .token(&ctx.accounts.user_fraction_token.to_account_info())
        .authority(&ctx.accounts.user)
        .metadata(&ctx.accounts.fraction_token_metadata)
        .mint(&ctx.accounts.fraction_token_mint)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .amount(*shares_amount)
        .invoke_signed(&[&signer_seeds])?;
    msg!("Burned the shares");


    Ok(())
}
