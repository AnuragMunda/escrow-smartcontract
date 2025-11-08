use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub initialiser: Pubkey,
    pub initialiser_token_account: Pubkey,
    pub initialiser_receive_token_account: Pubkey,
    pub vault_account: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub bump: u8,
    pub is_active: bool,
}