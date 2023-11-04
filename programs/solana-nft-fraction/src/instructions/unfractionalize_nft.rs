use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{token::{TokenAccount, Token}, metadata::Metadata as TokenMetadata, associated_token::AssociatedToken};
use mpl_token_metadata::{instructions::{CreateV1CpiBuilder, TransferV1CpiBuilder, MintV1CpiBuilder, BurnV1CpiBuilder}, types::TokenStandard, accounts::Metadata};

use crate::{state::fraction_details::FractionDetails, MyError};

#[derive(Accounts)]
pub struct UnfractionalizeNft<'info> {
    /// The user who is fractionalizing the NFT
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA that holds the fraction account details
    /// We will use the nft_vault as the seed for the pda
    #[account(
        init_if_needed, 
        space = FractionDetails::LEN,
        payer = user, 
        seeds = [
            b"fraction", 
            nft_vault.key().as_ref(),
            ],
        bump
    )]
    pub fraction_account: Account<'info, FractionDetails>,

    /// The pda vault thats holding the NFT
    #[account(
        init_if_needed, 
        payer = user, 
        token::mint = nft_mint,
        token::authority = fraction_account,
        seeds = [
            b"nft_vault", 
            fraction_token_mint.key().as_ref(),
            ],
        bump
    )]
    pub nft_vault: Account<'info, TokenAccount>,
    
    /// The user account to hold the nft
    /// CHECK: Checking in the program
    #[account(mut)]
    pub user_nft_account: UncheckedAccount<'info>,

    /// The NFT Mint Account
    /// CHECK: Account checked in fractionalize_nft_handler
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,

    /// CHECK: Will be created by the fractionalize_nft_handler
    #[account(mut)]
    pub nft_metadata_account: UncheckedAccount<'info>,

    /// Metadata account of the Fractionalized NFT Token.
    /// This account must be uninitialized.
    ///
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub fraction_token_metadata: AccountInfo<'info>,

    /// Destination token account
    /// CHECK: Account checked in CPI
    #[account(mut)]
    pub user_fraction_token: Account<'info, TokenAccount>,

    /// The account will be initialized if necessary.
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
    let signer_seeds = [b"fraction", nft_vault.as_ref(), &[ctx.bumps.fraction_account]];

    BurnV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .authority(&ctx.accounts.user)
        .metadata(&ctx.accounts.fraction_token_metadata)
        .mint(&ctx.accounts.fraction_token_mint)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .amount(*shares_amount)
        .invoke_signed(&[&signer_seeds])?;
    msg!("Burned the shares");

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

    Ok(())
}


