use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Escrow account is not active")]
    NotActive,
}