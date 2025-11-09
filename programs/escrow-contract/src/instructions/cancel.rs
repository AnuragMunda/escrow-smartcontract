use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::states::escrow::Escrow;
use crate::utils::errors::EscrowError;

/**
 * Escrow Cancel instruction
 * - Only initialiser can cancel when active
 * - transfer token A from vault back to initialiser_token_account
 */
pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    let initialiser = &ctx.accounts.initialiser;

    // verification
    require!(escrow.is_active, EscrowError::NotActive);
    require_keys_eq!(
        escrow.initialiser,
        initialiser.key(),
        EscrowError::Unauthorized
    );

    let vault_account = &ctx.accounts.vault_account;
    let mint_a = &ctx.accounts.mint_a;
    let initialiser_token_account = &ctx.accounts.initialiser_token_account;

    // Prepare context for transfer
    let cpi_accounts = TransferChecked {
        from: vault_account.to_account_info(),
        mint: mint_a.to_account_info(),
        to: initialiser_token_account.to_account_info(),
        authority: ctx.accounts.escrow_pda.to_account_info(),
    };

    let binding = escrow.key();

    let seeds = &[
        b"escrow".as_ref(),
        binding.as_ref(),
        &[escrow.bump],
    ];

    let signer = &[&seeds[..]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );

    // transfer token A from vault -> initialiser_token_account
    token::transfer_checked(cpi_context, escrow.amount_a, mint_a.decimals)?;

    escrow.is_active = false;
    Ok(())
}

/**
 * Accounts for Cancel instructions
 */
#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub initialiser: Signer<'info>,

    #[account(
        mut,
        constraint = initialiser_token_account.owner == initialiser.key()
    )]
    pub initialiser_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,

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
