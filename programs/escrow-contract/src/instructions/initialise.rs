use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::states::escrow::*;
use crate::utils::events::EscrowInitialised;

/**
 * Function to initialise the escrow
 * - create escrow account
 * - transfer `amount_a` from initialiser's token account into the vault
 * - set vault owner to PDA
 */
pub fn initialise(
    ctx: Context<Initialise>,
    amount_a: u64,
    amount_b: u64,
) -> Result<()> {
    let initialiser = &ctx.accounts.initialiser;
    let initialiser_token_account = &ctx.accounts.initialiser_token_account;
    let initialiser_receive_token_account = &ctx.accounts.initialiser_receive_token_account;
    let vault_account = &ctx.accounts.vault_account;
    let mint_a = &ctx.accounts.mint_a;
    
    // Save escrow details
    let escrow = &mut ctx.accounts.escrow;
    escrow.initialiser = initialiser.key();
    escrow.initialiser_token_account = initialiser_token_account.key();
    escrow.initialiser_receive_token_account = initialiser_receive_token_account.key();
    escrow.vault_account = vault_account.key();
    escrow.amount_a = amount_a;
    escrow.amount_b = amount_b;
    escrow.bump = ctx.bumps.escrow_pda;
    escrow.is_active = true;

    // Transfer token_a from initialiser to vault
    let cpi_accounts = TransferChecked {
        mint: mint_a.to_account_info(),
        from: initialiser_token_account.to_account_info(),
        to: vault_account.to_account_info(),
        authority: initialiser.to_account_info(),
    };

    let decimals = mint_a.decimals;
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer_checked(cpi_context, amount_a, decimals)?;

    emit!(EscrowInitialised {
        initialiser: initialiser.key(),
        amount_a,
        amount_b
    });

    Ok(())
}


#[derive(Accounts)]
pub struct Initialise<'info> {
    #[account(mut)]
    pub initialiser: Signer<'info>,

    /// The token account of initialiser from which token A will be moved (source)
    #[account(
        mut, 
        constraint = initialiser_token_account.owner == initialiser.key()
    )]
    pub initialiser_token_account: Account<'info, TokenAccount>,

    /// The token account where initialiser expects to receive token B when taker accepts
    #[account(
        mut, 
        constraint = initialiser_receive_token_account.owner == initialiser.key()
    )]
    pub initialiser_receive_token_account: Account<'info, TokenAccount>,

    /// Vault token account owned by initialiser initially, after transfer we set owner to PDA
    /// Vault must have same mint as initialiser_token_account.mint
    #[account(
        mut, 
        constraint = vault_account.mint == initialiser_token_account.mint
    )]
    pub vault_account: Account<'info, TokenAccount>,

    #[account(address = vault_account.mint)]
    pub mint_a: Account<'info, Mint>,

    /// Escrow PDA account to store metadata
    #[account(
        init,
        payer = initialiser,
        space = 8 + Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,

    /// The PDA acting as authority once set on token vault.
    /// This account is only used as a PDA authority (not a signer). Anchor expects it as AccountInfo.
    /// We derive it with seeds: ["escrow", escrow.key()]
    /// We set the bump on the Escrow struct.
    /// Provide the PDA's token account's owner will be the PDA address.
    /// The `escrow_pda` is an unchecked account to allow program to sign with it via seeds.
    #[account(
        seeds = [b"escrow", escrow.key().as_ref()],
        bump
    )]
    /// CHECK: PDA, no data stored here
    pub escrow_pda: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}