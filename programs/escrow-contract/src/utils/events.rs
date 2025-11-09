use anchor_lang::prelude::*;

#[event]
pub struct EscrowInitialised {
    pub initialiser: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
}

#[event]
pub struct EscrowAccepted {
    pub taker: Pubkey,
}
#[event]
pub struct EscrowCancelled {
    pub initialiser: Pubkey,
    pub escrow_account: Pubkey,
}