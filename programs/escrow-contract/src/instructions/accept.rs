use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::states::escrow::Escrow;


/**
 * Accept Escrow instruction
 * - taker transfer amount_b (token B) to initialiser_receive_token_account
 * - program transfers amount_a (token A) from vault (PDA owned) to taker_receive_token_account
 */
pub fn accept(ctx: Context<Accept>) -> Result<()> {
    let taker_token_account = &ctx.accounts.taker_token_account;
    let taker_receive_token_account = &ctx.accounts.taker_receive_token_account;
    let mint_a = &ctx.accounts.mint_a;
    let mint_b = &ctx.accounts.mint_b;
    let initialiser_recieve_token_account = &ctx.accounts.initialiser_receive_token_account;
    let vault_account = &ctx.accounts.vault_account;
    let escrow = &mut ctx.accounts.escrow;
    let decimals_a = mint_a.decimals;
    let decimals_b = mint_b.decimals;

    // From taker -> initialiser
    let cpi_accounts = TransferChecked {
        from: taker_token_account.to_account_info(),
        mint: mint_b.to_account_info(),
        to: initialiser_recieve_token_account.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts
    );

    token::transfer_checked(
        cpi_context,
        escrow.amount_b,
        decimals_b,
    )?;

    // From vault -> taker
    let binding = escrow.key();

    let seeds = &[
        b"escrow".as_ref(),
        binding.as_ref(),
        &[escrow.bump],
    ];

    let signer = &[&seeds[..]];

    let cpi_accounts = TransferChecked {
        from: vault_account.to_account_info(),
        mint: mint_a.to_account_info(),
        authority: ctx.accounts.escrow_pda.to_account_info(),
        to: taker_receive_token_account.to_account_info(),
    };

    token::transfer_checked(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer),
            escrow.amount_a,
            decimals_a,
        )?;

        escrow.is_active = false;

        Ok(())
}

#[derive(Accounts)]
pub struct Accept<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    /// The token account of taker from which token B will be moved
    #[account(mut)]
    pub taker_token_account: Account<'info, TokenAccount>,

    /// The token account of where taker will recieve token A
    #[account(mut)]
    pub taker_receive_token_account: Account<'info, TokenAccount>,

    /// The initialiser's receive account
    #[account(mut)]
    pub initialiser_receive_token_account: Account<'info, TokenAccount>,

    /// Vault account to hold token A
    #[account(
        mut,
        constraint = vault_account.owner == escrow_pda.key(),
    )]
    pub vault_account: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        seeds = [b"escrow", escrow.key().as_ref()],
        bump = escrow.bump
    )]
    /// CHECK: PDA
    pub escrow_pda: UncheckedAccount<'info>,

    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    pub token_program: Program<'info, Token>,
}