use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{token::{TokenAccount, Token}, metadata::Metadata as TokenMetadata, associated_token::AssociatedToken};
use mpl_token_metadata::{instructions::{CreateV1CpiBuilder, MintV1CpiBuilder}, types::TokenStandard, accounts::Metadata};

use crate::state::fraction_details::FractionDetails;

pub fn fractionalize_nft_handler(
    ctx: Context<FractionalizeNft>,
    shares_amount: u64,
) -> Result<()> {
    let fraction_account = &mut ctx.accounts.fraction_account;
    fraction_account.nft_vault_account = ctx.accounts.nft_vault.key();
    fraction_account.nft_mint = ctx.accounts.nft_mint.key();
    fraction_account.spl_token_mint = ctx.accounts.token_mint.key();
    fraction_account.withdraw_authority = ctx.accounts.user.key();
    fraction_account.shares_amount = shares_amount;
    msg!("Created fraction account");

    let signer_seeds = [b"fraction", ctx.accounts.nft_mint.key.as_ref(), &[ctx.bumps.fraction_account], ];
    let nft_metadata_acc = Metadata::try_from(&ctx.accounts.nft_metadata_account.to_account_info())?;

    // This is because the name and symbol are padded with null characters
    // https://stackoverflow.com/questions/49406517/how-to-remove-trailing-null-characters-from-string
    let token_name = format!("{}-fx", nft_metadata_acc.name.trim_matches(char::from(0)));
    let token_symbol = format!("{}-fx", nft_metadata_acc.symbol.trim_matches(char::from(0)));

    // Print all the accounts
    msg!("User: {}", ctx.accounts.user.key());
    msg!("Fraction Account: {}", ctx.accounts.fraction_account.key());
    msg!("NFT Vault: {}", ctx.accounts.nft_vault.key());
    msg!("NFT Account: {}", ctx.accounts.nft_account.key());
    msg!("NFT Mint: {}", ctx.accounts.nft_mint.key());
    msg!("NFT Metadata: {}", ctx.accounts.nft_metadata_account.to_account_info().key());
    msg!("Fraction Token Metadata: {}", ctx.accounts.fraction_token_metadata.to_account_info().key());


    msg!("Creating NFT Fraction Token...");
    // Careful of authority, we might need to create a pda authority just for signing as program
    CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
    .metadata(&ctx.accounts.fraction_token_metadata)
    .mint(&ctx.accounts.token_mint, true)
    .name(token_name)
    .uri(nft_metadata_acc.uri) // TODO: Add uri
    .symbol(token_symbol)
    .payer(&ctx.accounts.user)
    .update_authority(&ctx.accounts.user, true)
    .authority(&ctx.accounts.user)
    .token_standard(TokenStandard::Fungible)
    .print_supply(mpl_token_metadata::types::PrintSupply::Limited(shares_amount))
    .system_program(&ctx.accounts.system_program)
    .sysvar_instructions(&ctx.accounts.sysvar_instructions)
    .spl_token_program(&ctx.accounts.token_program)
    .seller_fee_basis_points(0) // Fee to creators of this token
    .invoke_signed(&[&signer_seeds])?;
    msg!("Fraction token created");


    // msg!("Transfering NFT to vault");
    // Transfer NFT to vault
    // token::transfer(
    //     CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         Transfer {
    //             from: ctx.accounts.nft_account.to_account_info(),
    //             to: ctx.accounts.nft_vault.to_account_info(),
    //             authority: ctx.accounts.user.to_account_info(),
    //         }
    //     ),
    //     1
    // )?;
    // msg!("NFT transferred to vault");

    // Freeze the NFT in the vault

    // Set the withdraw authority of the vault to the signer

    // Send created spl token to the user

    Ok(())
}

#[derive(Accounts)]
pub struct FractionalizeNft<'info> {
    /// The user who is fractionalizing the NFT
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA that holds the fraction account details
    #[account(
        init_if_needed, 
        space = FractionDetails::LEN,
        payer = user, 
        seeds = [
            b"fraction", 
            nft_mint.key().as_ref(),
            ],
        bump
    )]
    pub fraction_account: Account<'info, FractionDetails>,

    /// The pda vault thats going to hold the NFT
    #[account(
        init_if_needed, 
        payer = user, 
        token::mint = nft_mint,
        token::authority = fraction_account,
        seeds = [
            b"nft_vault", 
            nft_mint.key().as_ref(),
            ],
        bump
    )]
    pub nft_vault: Account<'info, TokenAccount>,
    
    /// The original account that holds the NFT token
    /// CHECK: Checking in the program
    pub nft_account: AccountInfo<'info>,

    /// The NFT Mint Account
    /// CHECK: Account checked in fractionalize_nft_handler
    pub nft_mint: AccountInfo<'info>,

    /// CHECK: Will be created by the fractionalize_nft_handler
    pub nft_metadata_account: UncheckedAccount<'info>,

    /// Metadata account of the Fractionalized NFT Token.
    /// This account must be uninitialized.
    ///
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub fraction_token_metadata: UncheckedAccount<'info>,

    /// The account will be initialized if necessary.
    ///
    /// Must be a signer if:
    ///   * the token mint account does not exist.
    ///
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub token_mint: Signer<'info>,

    /// Token Metadata Program
    /// CHECK: account checked in CPI
    pub token_metadata_program: Program<'info, TokenMetadata>,
    
    /// spl token program
    pub token_program: Program<'info, Token>,

    /// CHECK: account constraints for the system program
    #[account(address = sysvar::instructions::id())]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// Solana native system program
    pub system_program: Program<'info, System>,
}